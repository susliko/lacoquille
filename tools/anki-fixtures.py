#!/usr/bin/env python3
"""Generate Anki package fixtures for the import harness.

Every container shape and schema the importer promises to read:
  schema11.apkg     zip · collection.anki2 (models/decks as col JSON) · revlog · media JSON
  schema18.apkg     zip · collection.anki21b (notetypes/fields/decks tables) · revlog · media protobuf
  nohistory.apkg    zip · schema 11 · no revlog · all cards new (export without scheduling)
  colpkg-zstd       whole-file zstd frame around a tar of collection.anki21b + media
  corrupt.apkg      a zip with no collection inside (friendly-error path)

Written to the directory given as argv[1] (default /tmp/anki-fixtures).
Column layouts mirror real Anki exports closely enough to catch drift
in the reader; the reader's own defensive fallbacks are exercised by
the corrupt package.
"""

import json
import os
import sqlite3
import struct
import sys
import tarfile
import io
import zipfile

try:
    import zstandard
except ImportError:
    zstandard = None

OUT = sys.argv[1] if len(sys.argv) > 1 else "/tmp/anki-fixtures"
os.makedirs(OUT, exist_ok=True)

CRT = 1590000000  # 2020-05-20 — review due days count from here
MID = 1590000000000  # note type id
DID = 1590000000001  # deck id ("French::Core 2k")

MODELS_11 = json.dumps(
    {
        str(MID): {
            "id": MID,
            "name": "Basic",
            "type": 0,
            "flds": [{"name": "Front", "ord": 0}, {"name": "Back", "ord": 1}],
            "tmpls": [{"name": "Card 1"}, {"name": "Card 2"}],
        }
    }
)
DECKS_11 = json.dumps(
    {"1": {"id": 1, "name": "Default"}, str(DID): {"id": DID, "name": "French::Core 2k"}}
)


def schema11_db(with_revlog=True, reset_schedule=False):
    """reset_schedule mirrors Anki's export without scheduling: every
    card is written back as new (no type, no interval, no reps)."""
    path = os.path.join(OUT, "_schema11.sqlite3")
    if os.path.exists(path):
        os.remove(path)
    db = sqlite3.connect(path)
    c = db.cursor()
    c.executescript(
        """
        CREATE TABLE col (id integer primary key, crt integer not null, mod integer not null,
            scm integer not null, dty integer not null, usn integer not null, ls integer not null,
            config text not null, models text not null, decks text not null, dconf text not null,
            tags text not null);
        CREATE TABLE notes (
            id integer primary key, guid text not null, mid integer not null, mod integer not null,
            usn integer not null, tags text not null, flds text not null,
            sfld integer not null, csum integer not null, flags integer not null, data text not null);
        CREATE TABLE cards (
            id integer primary key, nid integer not null, did integer not null not null default 0,
            ord integer not null not null default 0, mod integer not null not null default 0,
            usn integer not null not null default 0, type integer not null not null default 0,
            queue integer not null not null default 0, due integer not null not null default 0,
            ivl integer not null not null default 0, factor integer not null not null default 0,
            reps integer not null not null default 0, lapses integer not null not null default 0,
            left integer not null not null default 0, odue integer not null not null default 0,
            odid integer not null not null default 0, flags integer not null not null default 0,
            data text not null not null default '');
        CREATE TABLE revlog (
            id integer primary key, cid integer not null, usn integer not null,
            ease integer not null, ivl integer not null, lastIvl integer not null,
            factor integer not null, time integer not null, type integer not null);
        """
    )
    c.execute(
        "INSERT INTO col VALUES (1,?,?,0,0,0,0,'{}',?,?, '{}','')",
        (CRT, CRT + 100, MODELS_11, DECKS_11),
    )
    # three notes: a review card, a new card, a learning card
    notes = [
        (1700000000001, "guidA", MID, 0, "une goéland", "a gull"),
        (1700000000002, "guidB", MID, 0, "la marée", "the tide"),
        (1700000000003, "guidC", MID, 0, "à contretemps", "out of step; at the wrong moment"),
    ]
    for nid, guid, mid, mod, front, back in notes:
        c.execute(
            "INSERT INTO notes VALUES (?,?,?,?,0,'',?,?,0,0,'')",
            (nid, guid, mid, mod, f"{front}\x1f{back}", front),
        )
    cards = [
        # review: due in 1234 days (Anki stores a day count for review
        # cards, not an epoch), ivl 12d, ease 2500, 15 reps 1 lapse
        (1700000900001, 1700000000001, DID, 0, 2, 2, 1234, 12, 2500, 15, 1),
        # new
        (1700000900002, 1700000000002, DID, 0, 0, 0, 0, 0, 0, 0, 0),
        # learning: due = epoch seconds, ivl 0
        (1700000900003, 1700000000003, DID, 0, 1, 1, 1700000100, 0, 0, 2, 0),
    ]
    for cid, nid, did, ord_, type_, queue, due, ivl, factor, reps, lapses in cards:
        if reset_schedule:
            type_ = queue = due = ivl = factor = reps = lapses = 0
        c.execute(
            "INSERT INTO cards (id, nid, did, ord, mod, usn, type, queue, due, ivl, factor, reps,"
            " lapses)"
            " VALUES (?,?,?,?,0,0,?,?,?,?,?,?,?)",
            (cid, nid, did, ord_, type_, queue, due, ivl, factor, reps, lapses),
        )
    if with_revlog:
        rows = []
        for i, (cid, ease, ivl, last, factor, t) in enumerate(
            [
                (1700000900001, 3, 12, 7, 2500, 4210),
                (1700000900001, 3, 7, 4, 2500, 3800),
                (1700000900003, 2, 0, 0, 0, 6100),
                (1700000900001, 4, 4, 2, 2650, 2900),
                (1700000900001, 3, 2, 1, 2500, 5300),
            ]
        ):
            # earliest id = 1591000000000 (June 2020) — the History fact
            rows.append((1591000000000 + i * 86400000, cid, 0, ease, ivl, last, factor, t, 0))
        c.executemany("INSERT INTO revlog VALUES (?,?,?,?,?,?,?,?,?)", rows)
    db.commit()
    db.close()
    with open(path, "rb") as f:
        return f.read()


def schema18_db(with_cloze=True):
    path = os.path.join(OUT, "_schema18.sqlite3")
    if os.path.exists(path):
        os.remove(path)
    db = sqlite3.connect(path)
    c = db.cursor()
    c.executescript(
        """
        CREATE TABLE col (id integer primary key, crt integer not null, mod integer not null,
            scm integer not null, dty integer not null, usn integer not null, ls integer not null,
            config blob not null);
        CREATE TABLE notetypes (
            id integer not null, name text not null collate nocase,
            mtimeSec integer not null, usn integer not null, config blob not null,
            primary key (id));
        CREATE TABLE fields (
            ntid integer not null, ord integer not null, name text not null collate nocase,
            config blob not null, primary key (ntid, ord)) without rowid;
        CREATE TABLE templates (
            ntid integer not null, ord integer not null, name text not null collate nocase,
            config blob not null, primary key (ntid, ord)) without rowid;
        CREATE TABLE decks (
            id integer primary key, name text not null collate nocase,
            mtimeSec integer not null, usn integer not null, common blob not null,
            browser blob not null, config blob not null);
        CREATE TABLE notes (
            id integer primary key, guid text not null, mid integer not null, mod integer not null,
            usn integer not null, tags text not null, flds text not null,
            sfld integer not null, csum integer not null, flags integer not null, data text not null);
        CREATE TABLE cards (
            id integer primary key, nid integer not null, did integer not null not null default 0,
            ord integer not null not null default 0, mod integer not null not null default 0,
            usn integer not null not null default 0, type integer not null not null default 0,
            queue integer not null not null default 0, due integer not null not null default 0,
            ivl integer not null not null default 0, factor integer not null not null default 0,
            reps integer not null not null default 0, lapses integer not null not null default 0,
            left integer not null not null default 0, odue integer not null not null default 0,
            odid integer not null not null default 0, flags integer not null not null default 0,
            data text not null not null default '');
        CREATE TABLE revlog (
            id integer primary key, cid integer not null, usn integer not null,
            ease integer not null, ivl integer not null, lastIvl integer not null,
            factor integer not null, time integer not null, type integer not null);
        """
    )
    c.execute("INSERT INTO col VALUES (1,?,0,0,0,0,0,'{}')", (CRT,))
    basic_cfg = json.dumps({"type": 0, "sortf": 0}).encode()
    cloze_cfg = json.dumps({"type": 1, "sortf": 0}).encode()
    mid_cloze = MID + 7
    c.execute("INSERT INTO notetypes VALUES (?,?,0,0,?)", (MID, "Basic", basic_cfg))
    c.execute("INSERT INTO notetypes VALUES (?,?,0,0,?)", (mid_cloze, "Cloze French", cloze_cfg))
    for i, name in enumerate(["Front", "Back"]):
        c.execute("INSERT INTO fields VALUES (?,?,?,'{}')", (MID, i, name))
    c.execute("INSERT INTO fields VALUES (?,0,'Text','{}')", (mid_cloze,))
    c.execute("INSERT INTO fields VALUES (?,1,'Back Extra','{}')", (mid_cloze,))
    for i, name in enumerate(["Card 1", "Card 2"]):
        c.execute("INSERT INTO templates VALUES (?,?,?,'{}')", (MID, i, name))
    c.execute("INSERT INTO templates VALUES (?,0,'Cloze','{}')", (mid_cloze,))
    c.execute("INSERT INTO decks VALUES (1,'Default',0,0,'{}','{}','{}')")
    c.execute("INSERT INTO decks VALUES (?,'French::Core 2k',0,0,'{}','{}','{}')", (DID,))
    notes = [
        # a review card with HTML + sound to clean — flds is
        # \x1f-joined (two fields: Front, Back), sfld is the sort text
        (1700000000011, "guidD", MID, "un oiseau marin<br>seabird [sound:bird.mp3]\x1f<b>a seabird</b>", "un oiseau marin"),
        # a cloze note with two deletions — first (c1) becomes the card
        (1700000000012, "guidE", mid_cloze, "Le {{c1::phare::lighthouse}} guide les {{c2::navires}}.", ""),
    ]
    for nid, guid, mid, flds, sfld in notes:
        c.execute("INSERT INTO notes VALUES (?,?,?,?,0,'',?,?,0,0,'')", (nid, guid, mid, 0, flds, sfld))
    c.execute(
        "INSERT INTO cards (id, nid, did, ord, mod, usn, type, queue, due, ivl, factor, reps, lapses)"
        " VALUES (?,?,?,?,0,0,?,?,?,?,?,?,?)",
        (1700000900011, 1700000000011, DID, 0, 2, 2, 30, 30, 2350, 22, 2),
    )
    c.execute(
        "INSERT INTO cards (id, nid, did, ord, mod, usn, type, queue, due, ivl, factor, reps, lapses)"
        " VALUES (?,?,?,?,0,0,?,?,?,?,?,?,?)",
        (1700000900012, 1700000000012, DID, 0, 0, 0, 0, 0, 0, 0, 0),
    )
    c.execute(
        "INSERT INTO revlog VALUES (1593000000000,1700000900011,0,3,30,20,2350,5100,0)"
    )
    db.commit()
    db.close()
    with open(path, "rb") as f:
        return f.read()


def media_json():
    return json.dumps({"0": "bird.mp3", "1": "img/lighthouse.png"}).encode()


def media_protobuf():
    # the .anki21b map: repeated { name: string (field 1) } — walkable
    # wire format, which the reader only counts
    out = bytearray()
    for name in ["bird.mp3", "img/lighthouse.png"]:
        nb = name.encode()
        out += bytes([0x0A, len(nb)]) + nb
    return bytes(out)


def zip_apkg(name, coll_bytes, media, coll_member="collection.anki2", compress=zipfile.ZIP_STORED):
    path = os.path.join(OUT, name)
    with zipfile.ZipFile(path, "w", compress) as z:
        z.writestr(coll_member, coll_bytes, compress_type=compress)
        z.writestr("media", media)
    return path


def tar_zstd_colpkg(name, coll_bytes, media):
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=tarfile.USTAR_FORMAT) as t:
        for member, data in [("collection.anki21b", coll_bytes), ("media", media)]:
            ti = tarfile.TarInfo(member)
            ti.size = len(data)
            t.addfile(ti, io.BytesIO(data))
    frame = zstandard.ZstdCompressor().compress(buf.getvalue())
    with open(os.path.join(OUT, name), "wb") as f:
        f.write(frame)
    return os.path.join(OUT, name)


def main():
    made = []

    # schema 11, stored entries (old exporter)
    made.append(zip_apkg("schema11.apkg", schema11_db(), media_json()))
    # schema 18, deflated (current exporter); .anki21b content is plain sqlite here
    made.append(
        zip_apkg(
            "schema18.apkg",
            schema18_db(),
            media_protobuf(),
            coll_member="collection.anki21b",
            compress=zipfile.ZIP_DEFLATED,
        )
    )
    # export without scheduling: no revlog at all, cards all new
    made.append(zip_apkg("nohistory.apkg", schema11_db(with_revlog=False, reset_schedule=True), media_json()))

    if zstandard:
        made.append(tar_zstd_colpkg("colpkg.tar.zst", schema18_db(), media_protobuf()))
    else:
        print("zstandard missing — skipping the tar+zstd colpkg fixture")

    # a zip that holds no collection
    bad = os.path.join(OUT, "corrupt.apkg")
    with zipfile.ZipFile(bad, "w") as z:
        z.writestr("media", b"{}")
    made.append(bad)

    for m in made:
        print("wrote", m)


if __name__ == "__main__":
    main()
