#!/usr/bin/env python3
"""Build rules.json from rules-src chunks and inject into 1-manuel.html.

- keys rules by taxonomy entry id (t0..t120)
- resolves `see` name references to entry ids
- refuses to build if any name is missing, unknown or duplicated
- greps for the strongest AI-tells (humanizer) before writing
"""
import json, re, sys, importlib.util
from pathlib import Path

HERE = Path(__file__).parent          # rules-src/
KDIR  = HERE.parent                   # knowledge/
taxonomy = json.load(open(KDIR / "taxonomy.json", encoding="utf-8"))

def load(name):
    spec = importlib.util.spec_from_file_location(name, HERE / f"{name}.py")
    m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
    return m.R

rules = {**load("rules_a"), **load("rules_b"), **load("rules_c")}

names = [e["name"] for e in taxonomy]
assert len(names) == len(set(names)) == 121, "taxonomy names not unique/complete"

missing = [n for n in names if n not in rules]
unknown = [n for n in rules if n not in set(names)]
if missing or unknown:
    sys.exit(f"FAIL missing={missing}\n     unknown={unknown}")

# humanizer self-check on the strongest tells
blob = json.dumps(rules, ensure_ascii=False)
tells = {
    "not just": "not just", "isn't just": "isn't just", "not only": "not only",
    "delve": "delve", "testament": "testament", "showcas": "showcas",
    "dive in": "dive in", "at its core": "at its core",
    "crucial": "crucial", "pivotal": "pivotal", "elevate": "elevate",
    "landscape": "landscape", "in conclusion": "in conclusion",
}
hits = {label: blob.count(pat) for label, pat in tells.items() if pat in blob.lower()}
print("AI-tell hits:", hits or "none")
if hits:
    sys.exit("fix AI tells first")

# validate shape + collect stats
by_id, n_rules, longest = {}, 0, ("", 0)
for e in taxonomy:
    r = rules[e["name"]]
    assert r.get("rules"), f"{e['name']}: no rules"
    for x in r["rules"]:
        assert x.get("h") and x.get("p") and x.get("ex", {}).get("fr") and x["ex"].get("en"), f"{e['name']}: bad rule {x}"
        n_rules += 1
        if len(x["p"]) > longest[1]:
            longest = (f"{e['name']} — {x['h']}", len(x["p"]))
    if r.get("see"):
        for s in r["see"]:
            assert s in rules, f"{e['name']}: unknown see-ref {s!r}"
    by_id[e["id"]] = {
        "rules": r["rules"],
        **({"exc": r["exc"]} if r.get("exc") else {}),
        **({"see": [next(t["id"] for t in taxonomy if t["name"] == s) for s in r["see"]]} if r.get("see") else {}),
    }

json.dump(by_id, open(KDIR / "rules.json", "w"), ensure_ascii=False, separators=(",", ":"))
print(f"rules.json: {len(by_id)} topics, {n_rules} rules, "
      f"{sum(1 for r in by_id.values() if 'exc' in r)} exceptions, "
      f"{sum(1 for r in by_id.values() if 'see' in r)} see-also sets")
print("longest rule paragraph:", longest)

# inject into the prototype
html = open(KDIR / "1-manuel.html", encoding="utf-8").read()
rules_js = json.dumps(by_id, ensure_ascii=False).replace("</", "<\\/")
lines = html.split("\n")
out, done = [], 0
for ln in lines:
    if ln.startswith("const RULES"):
        out.append("const RULES   = " + rules_js + ";"); done += 1
    else:
        out.append(ln)
html = "\n".join(out)
old_r = "const r = drillable(e) ? RULES[e.slugs[0]] : undefined;"
new_r = "const r = RULES[e.id];"
if old_r in html:
    html = html.replace(old_r, new_r)
assert new_r in html
old_see = """        ${r.see.map(s=>{
          const t = ENTRIES.find(x => x.slugs && x.slugs.includes(s));
          return `<a data-eid="${t ? t.id : ""}">${esc(t ? t.name : s)}</a>`;
        }).join("")}"""
new_see = """        ${r.see.map(id => {
          const t = ENTRIES.find(x => x.id === id);
          return t ? `<a data-eid="${t.id}">${esc(t.name)}</a>` : "";
        }).join("")}"""
if old_see in html:
    html = html.replace(old_see, new_see)
assert new_see in html
open(KDIR / "1-manuel.html", "w", encoding="utf-8").write(html)
print(f"injected into 1-manuel.html ({len(html)} bytes, RULES line replaced: {done})")
