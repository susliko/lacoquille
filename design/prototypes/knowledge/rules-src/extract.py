#!/usr/bin/env python3
"""Re-extract taxonomy.json + topics.json from the real sources.

Run from anywhere; writes next to build.py (../taxonomy.json, ../topics.json).
- taxonomy.ts (RAW + SLUG_MAP) -> taxonomy.json: the 1:1 grid contract
- topics.rs                   -> topics.json: catalog blurbs, joined by slug
"""
import json, re, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]   # repo root
KDIR = Path(__file__).resolve().parents[1]

ts = (ROOT / "src/components/progress/taxonomy.ts").read_text(encoding="utf-8")
raw = re.search(r"const RAW = `(.*?)`;", ts, re.S).group(1).strip()
sl = re.search(r'const SLUG_MAP: Record<string, string\[\]> = \{(.*?)\n\};', ts, re.S).group(1)
slug_map = {m.group(1): re.findall(r'"([^"]+)"', m.group(2))
            for m in re.finditer(r'"([^"]+)":\s*\[([^\]]*)\]', sl)}

rs = (ROOT / "lacq/src/exercises/topics.rs").read_text(encoding="utf-8")
topics = []
for m in re.finditer(r'Topic \{\s*\n\s*slug: "([^"]+)",\s*\n\s*title: "([^"]+)",\s*\n\s*blurb: "((?:[^"\\]|\\.)*)"', rs):
    topics.append({"slug": m.group(1), "title": m.group(2), "blurb": m.group(3).replace('\\"', '"')})

by_slug = {t["slug"]: t for t in topics}
missing = {s for v in slug_map.values() for s in v if s not in by_slug}
if missing:
    sys.exit(f"FAIL: SLUG_MAP references unknown catalog slugs: {sorted(missing)}")

entries = []
for i, line in enumerate(raw.split("\n")):
    lv, code, _tier, name = [p.strip() for p in line.split("|")]
    cat, sub = code[0], (code[1] if len(code) > 1 else "")
    slugs = slug_map.get(f"{lv}|{code}|{name}", [])
    if not slugs:
        sys.exit(f"FAIL: taxonomy entry without slugs (catalog grew past it?): {lv}|{code}|{name}")
    e = {"id": f"t{i}", "lv": lv, "cat": cat, "sub": sub, "name": name, "slugs": slugs,
         "blurb": by_slug[slugs[0]]["blurb"]}
    entries.append(e)

json.dump(topics, open(KDIR / "topics.json", "w"), ensure_ascii=False, indent=1)
json.dump(entries, open(KDIR / "taxonomy.json", "w"), ensure_ascii=False, indent=1)
drillable = sum(1 for e in entries if e["slugs"])
print(f"topics.json: {len(topics)} catalog topics")
print(f"taxonomy.json: {len(entries)} entries, {drillable} drillable, "
      f"{len(entries) - drillable} structural")
