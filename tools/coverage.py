#!/usr/bin/env python3
"""Exercise coverage report — how much of the progress grid has drills.

Reads two sources of truth (no server needed):

  src/components/progress/taxonomy.ts   the 121-entry grid skeleton and
                                        the SLUG_MAP wiring entries →
                                        catalog slugs
  lacq/src/exercises/topics.rs          the catalog (slug → family)

and prints, per CEFR level:

  - entries lit (wired to at least one catalog slug)
  - entries dark (pure structure for now)
  - the catalog family behind each slug

Also flags orphans: slugs wired in SLUG_MAP but missing from the
catalog (typo risk), and catalog slugs no entry points at (dead
drills). Exit code is 0 always — this is a report, not a gate; the
compile-time gate for pool sizes lives in families.rs (coverage_gate).

Usage:  python3 tools/coverage.py
"""

import re
import sys
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TAXONOMY = ROOT / "src/components/progress/taxonomy.ts"
TOPICS = ROOT / "lacq/src/exercises/topics.rs"

LEVELS = ["A1", "A2", "B1", "B2", "C1", "C2"]


def parse_taxonomy():
    src = TAXONOMY.read_text()
    raw = re.search(r"const RAW = `(.*?)`", src, re.S).group(1)
    entries = []
    for line in raw.strip().split("\n"):
        parts = line.strip().split("|")
        if len(parts) == 4:
            entries.append({"lv": parts[0], "code": parts[1], "name": parts[3]})
    slug_map = dict(re.findall(r'"([^"]+)":\s*\[([^\]]+)\]', src))
    # entry key is lv|code|name
    for e in entries:
        key = f'{e["lv"]}|{e["code"]}|{e["name"]}'
        slugs = re.findall(r'"([^"]+)"', slug_map.get(key, ""))
        e["slugs"] = slugs
    return entries


def parse_topics():
    src = TOPICS.read_text()
    block = re.search(r"pub const TOPICS: &\[Topic\] = &\[(.*?)\n\];", src, re.S).group(1)
    topics = {}
    for m in re.finditer(r'slug:\s*"([^"]+)"[^}]*?family:\s*Family::(\w+)', block, re.S):
        topics[m.group(1)] = m.group(2)
    return topics


def main():
    entries = parse_taxonomy()
    topics = parse_topics()

    wired = {s for e in entries for s in e["slugs"]}
    orphan_wires = wired - set(topics)
    dead_slugs = set(topics) - wired

    total = len(entries)
    lit = sum(1 for e in entries if e["slugs"])
    print(f"grid coverage: {lit}/{total} entries lit\n")

    per_level = defaultdict(lambda: [0, 0])
    for e in entries:
        per_level[e["lv"]][1] += 1
        if e["slugs"]:
            per_level[e["lv"]][0] += 1
    for lv in LEVELS:
        lit_n, tot_n = per_level[lv]
        bar = "█" * lit_n + "░" * (tot_n - lit_n)
        print(f"  {lv}  {bar}  {lit_n:>2}/{tot_n}")

    print("\ncatalog slugs:")
    for slug in sorted(topics):
        star = "  (unwired!)" if slug in dead_slugs else ""
        print(f"  {slug:<18} {topics[slug]}{star}")

    if orphan_wires:
        print("\nORPHAN WIRES — SLUG_MAP points at unknown catalog slugs:")
        for s in sorted(orphan_wires):
            print(f"  {s}")

    print("\ndark entries (no drills yet):")
    for e in entries:
        if not e["slugs"]:
            print(f'  {e["lv"]} {e["code"]:<3} {e["name"]}')

    return 0


if __name__ == "__main__":
    sys.exit(main())
