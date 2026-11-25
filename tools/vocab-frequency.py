#!/usr/bin/env python3
"""Build the vocabulary frequency list for the vocab-size estimate.

Source: Lexique383 (http://www.lexique.org, CC-BY-SA) — 142k French
word forms with lemma-level frequencies in a film-subtitle corpus
(freqlemfilms2) and a book corpus (freqlemlivres), both per-million.

We rank LEMMAS (a verb and its 40 conjugations are one word), keep
open-class + function-word lemmas, drop proper nouns and
onomatopoeia, and emit the top N as a plain rank-ordered array:
index + 1 = rank. Band edges over that array define the strata the
spot-check samples from (population sizes for the binomial estimate).

A pool of French-looking pseudowords (fixed seed) is generated with
a trigram model over the emitted lemmas and filtered against every
real word Lexique knows — they drive the over-claim correction.

Usage:
  python3 tools/vocab-frequency.py <Lexique383.tsv> [out.json] [en_50k.txt]

The generated JSON is committed; re-running needs the TSVs only when
the list itself must change.
"""

import csv
import json
import random
import re
import sys
from collections import defaultdict

N_WORDS = 20_000
EDGES = [1_000, 3_000, 6_000, 12_000, 20_000]
N_PSEUDO = 120
SEED = 7

# POS worth counting as vocabulary. NAM (proper nouns) and ONO
# (onomatopoeia) are excluded; everything else — verbs, nouns,
# adjectives, adverbs, pronouns, articles, prepositions — is fair.
EXCLUDE_CGRAM = {"NAM", "ONO"}

ALLOWED = re.compile(r"^[a-zàâäéèêëîïôöùûüçœæ']+$")


def main() -> None:
    tsv = sys.argv[1] if len(sys.argv) > 1 else "Lexique383.tsv"
    out = sys.argv[2] if len(sys.argv) > 2 else "src/data/vocab/frequency.json"
    en_path = sys.argv[3] if len(sys.argv) > 3 else ""
    english: set[str] = set()
    if en_path:
        with open(en_path, encoding="utf-8") as f:
            for line in f:
                english.add(line.split()[0].lower())
        english = set(sorted(english)[:20_000])  # top forms only: "space" yes, "miler" noise no

    # lemma → best per-source frequency (a lemma spans POS and
    # inflection rows; the lemma-freq columns repeat, take the max)
    films: dict[str, float] = {}
    livres: dict[str, float] = {}
    all_ortho: set[str] = set()

    with open(tsv, encoding="utf-8") as f:
        for row in csv.DictReader(f, delimiter="\t", quoting=csv.QUOTE_NONE):
            lemme = row["lemme"].lower()
            ortho = row["ortho"].lower()
            all_ortho.add(ortho)
            if row["cgram"] in EXCLUDE_CGRAM:
                continue
            if not ALLOWED.match(lemme) or len(lemme) < 2 or len(lemme) > 20:
                continue
            try:
                ff = float(row["freqlemfilms2"] or 0)
                fl = float(row["freqlemlivres"] or 0)
            except ValueError:
                continue
            if ff > films.get(lemme, -1):
                films[lemme] = ff
            if fl > livres.get(lemme, -1):
                livres[lemme] = fl

    # rank lemmas by summed per-million frequency across both corpora
    combined = {m: films.get(m, 0) + livres.get(m, 0) for m in films}
    words = sorted(combined, key=lambda m: combined[m], reverse=True)[:N_WORDS]
    print(f"lemmas kept: {len(combined)}, emitted: {len(words)}")

    pseudos = make_pseudowords(words, all_ortho, english)

    payload = {
        "meta": {
            "source": "Lexique383",
            "license": "CC-BY-SA-4.0 (New, Pallier, Brysbaert — lexique.org)",
            "metric": "freqlemfilms2 + freqlemlivres (per-million, lemma level)",
            "edges": EDGES,
        },
        "words": words,
        "pseudowords": pseudos,
    }
    with open(out, "w", encoding="utf-8") as f:
        json.dump(payload, f, ensure_ascii=False, separators=(",", ":"))
    print(f"wrote {out}: {len(words)} words, {len(pseudos)} pseudowords")

    # sanity: where do a few known words rank?
    for probe in ["chose", "frère", "souris", "tempête", "brume", "foulard"]:
        if probe in words:
            print(f"  {probe!r}: rank {words.index(probe) + 1}")


def make_pseudowords(words: list[str], real: set[str], english: set[str]) -> list[str]:
    """Trigram model over the emitted lemmas → French-looking nonwords."""
    rng = random.Random(SEED)

    initials: dict[str, int] = defaultdict(int)
    tris: dict[str, dict[str, int]] = defaultdict(lambda: defaultdict(int))
    bis: dict[str, dict[str, int]] = defaultdict(lambda: defaultdict(int))
    for w in words:
        pad = "^^" + w + "$"
        initials[w[0]] += 1
        for i in range(2, len(pad)):
            tris[pad[i - 2 : i]][pad[i]] += 1
        for i in range(1, len(pad)):
            bis[pad[i - 1 : i]][pad[i]] += 1

    def draw(d: dict[str, int]) -> str:
        return rng.choices(list(d), weights=list(d.values()))[0]

    def gen() -> str:
        c = draw(initials)
        out = [c]
        prev2 = "^" + c
        for _ in range(11):
            nxt = draw(tris[prev2]) if tris[prev2] else draw(bis[prev2[-1:]])
            if nxt == "$":
                break
            out.append(nxt)
            prev2 = (prev2 + nxt)[-2:]
        return "".join(out)

    vowels = set("aàâeéèêëiîïoôuùûüyœ")
    # real forms joined once for substring (fragment) rejection — a
    # pseudoword must not occur inside any real French word
    blob = " \uffff ".join(real)
    out: set[str] = set()
    for _ in range(80_000):
        if len(out) >= N_PSEUDO:
            break
        cand = gen()
        if not (5 <= len(cand) <= 9):
            continue
        if len(vowels & set(cand)) < 2:
            continue
        if cand in real or cand in words:
            continue
        if cand in english:
            continue
        if cand in blob:
            continue
        out.add(cand)
    return sorted(out)


if __name__ == "__main__":
    main()
