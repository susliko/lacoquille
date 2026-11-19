import { createSignal, createMemo, createEffect, For, Show, onMount, onCleanup } from "solid-js";
import type {
  DiagramData, DiagramNode, DiagramEdge, MoodId, EdgeType, TimePosition,
} from "./GrammarMap/types";
import {
  TIME_COLUMNS, MOOD_LABELS, MOOD_ORDER, MOOD_FILL,
  EDGE_COLOR, EDGE_DASH, EDGE_LABEL,
} from "./GrammarMap/types";
import { VERBS, PERSONS, VERB_OPTIONS } from "../data/conjugations/index";
import type { TenseId, Person } from "../data/conjugations/types";

// ── Time axis ─────────────────────────────────────────────────
const TIME_ORDER = Object.keys(TIME_COLUMNS) as TimePosition[];

const TIME_DIVIDERS: Record<TimePosition, string> = {
  "far-past":    "before the past",
  "past":        "past",
  "near-past":   "recent past",
  "present":     "present",
  "near-future": "near future",
  "future":      "future",
  "far-future":  "future anterior",
};

// Which card sits on top when two tenses share a (mood × time) cell:
// the simple/base tense first, derived or periphrastic below it.
const CELL_ORDER: Record<string, number> = {
  "present-indicatif": 0, "present-progressif": 1,
  "passe-compose": 0, "passe-recent": 1,
  "imparfait": 0, "passe-simple": 1,
  "subjonctif-passe": 0, "subjonctif-imparfait": 1,
  "plus-que-parfait": 0, "passe-anterieur": 1,
  "futur-simple": 0, "futur-proche": 1,
  "futur-anterieur": 0, "conditionnel-present": 0,
  "conditionnel-passe": 0, "subjonctif-present": 0,
  "imperatif-present": 0, "imperatif-passe": 1,
};

// ── Line vocabulary (human wording) ───────────────────────────
const EDGE_NAME: Record<EdgeType, string> = {
  "auxiliary-compound": "compound tense",
  "aspect-pair":        "aspect pair",
  "stem-share":         "same stem",
  "mood-swap":          "mood shift",
  "anteriority":        "anteriority",
};

// ── Text fitting (SVG <text> does not wrap — we do it ourselves,
//    measured with canvas, shrinking to a floor, then ellipsing) ─
interface FittedText { size: number; lines: string[]; }

let measureCtx: CanvasRenderingContext2D | null | undefined;
function measure(text: string, size: number, weight: number): number {
  if (typeof document !== "undefined") {
    if (measureCtx === undefined) {
      measureCtx = document.createElement("canvas").getContext("2d");
    }
    if (measureCtx) {
      measureCtx.font = `${weight} ${size}px Manrope, ui-sans-serif, system-ui, sans-serif`;
      return measureCtx.measureText(text).width;
    }
  }
  return text.length * size * (weight >= 600 ? 0.62 : 0.56);
}

function wrapWords(text: string, maxW: number, size: number, weight: number): string[] {
  const words = text.split(/\s+/).filter(Boolean);
  const lines: string[] = [];
  let cur = "";
  for (let w of words) {
    // A single word wider than the line: hard-break it by characters.
    while (measure(w, size, weight) > maxW && w.length > 1) {
      let k = w.length;
      while (k > 1 && measure(w.slice(0, k) + "-", size, weight) > maxW) k--;
      const head = w.slice(0, k) + "-";
      const rest = w.slice(k);
      if (cur) { lines.push(cur); cur = ""; }
      lines.push(head);
      w = rest;
    }
    if (!w) continue;
    if (!cur) { cur = w; continue; }
    if (measure(cur + " " + w, size, weight) <= maxW) cur = cur + " " + w;
    else { lines.push(cur); cur = w; }
  }
  if (cur) lines.push(cur);
  return lines;
}

function fitText(
  text: string, maxW: number, maxLines: number,
  maxSize: number, minSize: number, weight: number,
): FittedText {
  const t = text ?? "";
  for (let s = maxSize; s >= minSize - 0.01; s -= 0.5) {
    const lines = wrapWords(t, maxW, s, weight);
    if (lines.length <= maxLines) return { size: s, lines };
  }
  const full = wrapWords(t, maxW, minSize, weight);
  const lines = full.slice(0, maxLines);
  if (full.length > maxLines) {
    let last = lines[lines.length - 1] ?? "";
    while (last && measure(last + "…", minSize, weight) > maxW) last = last.slice(0, -1);
    lines[lines.length - 1] = last + "…";
  }
  return { size: minSize, lines };
}

// ── Mobile layout (mood = column, time = row divider) ─────────
const MB_CARD_W = 88;
const MB_COL_GAP = 4;
const MB_COL_W = MB_CARD_W + MB_COL_GAP;            // 92
const MB_MOOD_HEADER_H = 26;
const MB_DIVIDER_H = 20;
const MB_ROW_GAP = 10;
const MB_CELL_GAP = 6;
const MB_SVG_W = MOOD_ORDER.length * MB_COL_W;      // 368

const MB_CFG = {
  titleW: MB_CARD_W - 20, titleMax: 8.5, titleMin: 7,
  formW: MB_CARD_W - 12, formMax: 11, formMin: 9.5, formMaxLines: 3,
  padT: 7, padB: 8, gap: 3, minH: 46,
};

// ── Desktop layout (time = column, mood = lane row) ───────────
const DK_CARD_W = 140;
const DK_COL_GAP = 16;
const DK_COL_W = DK_CARD_W + DK_COL_GAP;            // 156
const DK_LANE_HEADER_W = 96;
const DK_COL_HEADER_H = 28;
const DK_PAD = 12;
const DK_LANE_GAP = 16;
const DK_CELL_GAP = 6;
const DK_SVG_W = DK_LANE_HEADER_W + DK_PAD * 2 + 7 * DK_COL_W;

const DK_CFG = {
  titleW: DK_CARD_W - 24, titleMax: 10.5, titleMin: 8.5,
  formW: DK_CARD_W - 14, formMax: 14, formMin: 11, formMaxLines: 2,
  padT: 9, padB: 10, gap: 4, minH: 50,
};

// ── Types ─────────────────────────────────────────────────────
interface CardMetrics { node: DiagramNode; title: FittedText; h: number; }
interface Box { node: DiagramNode; x: number; y: number; w: number; h: number; cx: number; cy: number; m: CardMetrics; }
interface LaneRect { mood: MoodId; y: number; h: number; }
interface Divider { time: TimePosition; lineY: number; textY: number; }

interface Props {
  data: DiagramData;
  tenseTitles: Record<string, string>;
  tenseRules:  Record<string, string>;
}

// ── Verbal periphrases — computed dynamically ─────────────────
const PERIPHRASES: Record<string, (inf: string, person: string) => string> = {
  "futur-proche": (inf, person) => {
    const key = (person === "elle" || person === "on") ? "il" : person;
    const f = VERBS["aller"]?.forms["present-indicatif"]?.[key as Person];
    return f ? `${f} ${inf}` : "—";
  },
  "passe-recent": (inf, person) => {
    const venir: Record<string, string> = {
      je: "viens", tu: "viens", il: "vient",
      nous: "venons", vous: "venez", ils: "viennent",
    };
    const key = (person === "elle" || person === "on") ? "il" : person;
    return `${venir[key] ?? "—"} de ${inf}`;
  },
  "present-progressif": (inf, person) => {
    const key = (person === "elle" || person === "on") ? "il" : person;
    const f = VERBS["être"]?.forms["present-indicatif"]?.[key as Person];
    return f ? `${f} en train de ${inf}` : "—";
  },
};

// ── Edge paths ────────────────────────────────────────────────
function edgePathMb(f: Box, t: Box, edge: DiagramEdge): string {
  const dx = t.cx - f.cx;
  const dy = t.cy - f.cy;
  const off = edge.type === "auxiliary-compound" ? -5
            : edge.type === "aspect-pair"         ?  5 : 0;

  if (Math.abs(dx) < 4) { // same mood column — vertical
    const down = dy > 0;
    const sy = down ? f.y + f.h : f.y;
    const ey = down ? t.y : t.y + t.h;
    return `M ${f.cx + off} ${sy} L ${f.cx + off} ${ey}`;
  }
  if (Math.abs(dy) < 4) { // same time row — horizontal
    const right = dx > 0;
    const sx = right ? f.x + f.w : f.x;
    const ex = right ? t.x : t.x + t.w;
    return `M ${sx} ${f.cy + off} L ${ex} ${f.cy + off}`;
  }
  return `M ${f.cx} ${f.cy} L ${t.cx} ${t.cy}`;
}

function edgePathDk(f: Box, t: Box, edge: DiagramEdge): string {
  const dx = t.cx - f.cx;
  const dy = t.cy - f.cy;

  // Cross-lane edges: bezier from card top/bottom
  if (f.node.lane !== t.node.lane) {
    const fromY = dy > 0 ? f.y + f.h : f.y;
    const toY   = dy > 0 ? t.y : t.y + t.h;
    const midY = (f.cy + t.cy) / 2;
    return `M ${f.cx} ${fromY} C ${f.cx} ${midY}, ${t.cx} ${midY}, ${t.cx} ${toY}`;
  }

  // Stacked within the same cell — short vertical connector
  if (Math.abs(dx) < 4) {
    const down = dy > 0;
    const sy = down ? f.y + f.h : f.y;
    const ey = down ? t.y : t.y + t.h;
    return `M ${f.cx} ${sy} L ${f.cx} ${ey}`;
  }

  // Same lane, same row — horizontal, offset per type
  if (Math.abs(dy) < 4) {
    const goingRight = dx > 0;
    const startX = goingRight ? f.x + f.w : f.x;
    const endX   = goingRight ? t.x : t.x + t.w;
    const yOff = edge.type === "auxiliary-compound" ? -8
               : edge.type === "aspect-pair"         ?  8 : 0;
    const y = f.cy + yOff;
    const fromCol = TIME_COLUMNS[f.node.timePosition];
    const toCol   = TIME_COLUMNS[t.node.timePosition];
    if (Math.abs(toCol - fromCol) > 1) {
      const arcTop = Math.min(f.y, t.y) - 16;
      return `M ${startX} ${y} C ${startX} ${arcTop}, ${endX} ${arcTop}, ${endX} ${y}`;
    }
    return `M ${startX} ${y} L ${endX} ${y}`;
  }

  // Generic diagonal
  const cp1y = f.cy + dy * 0.4;
  const cp2y = t.cy - dy * 0.4;
  return `M ${f.cx} ${f.cy} C ${f.cx} ${cp1y}, ${t.cx} ${cp2y}, ${t.cx} ${t.cy}`;
}

// ── Component ─────────────────────────────────────────────────
export default function VerbDiagram({ data, tenseTitles, tenseRules }: Props) {
  const [selectedPerson, setSelectedPerson] = createSignal<string>("je");
  const [selectedVerb,   setSelectedVerb]   = createSignal<string>("parler");
  const [showLiterary,   setShowLiterary]   = createSignal(false);
  const [activeEdge,     setActiveEdge]     = createSignal<DiagramEdge | null>(null);
  const [selectedTense,  setSelectedTense]  = createSignal<string | null>(null);
  const [isMobile,       setIsMobile]       = createSignal(
    typeof window !== "undefined" ? window.innerWidth < 768 : false
  );
  const [verbDropdownOpen, setVerbDropdownOpen] = createSignal(false);
  // bumped when webfonts finish loading — text metrics change
  const [fontTick, setFontTick] = createSignal(0);

  let captionRef: HTMLDivElement | undefined;

  onMount(() => {
    const check = () => setIsMobile(window.innerWidth < 768);
    window.addEventListener("resize", check);
    const handleClickOutside = (e: MouseEvent) => {
      const target = e.target as Element;
      if (!target.closest(".verb-dropdown")) setVerbDropdownOpen(false);
    };
    document.addEventListener("click", handleClickOutside);
    document.fonts?.ready.then(() => setFontTick(t => t + 1));
    onCleanup(() => {
      window.removeEventListener("resize", check);
      document.removeEventListener("click", handleClickOutside);
    });
  });

  // ── Derived ─────────────────────────────────────────────────
  const visibleNodes = createMemo(() =>
    data.nodes.filter(n => showLiterary() || !n.literary)
  );
  const visibleEdges = createMemo(() =>
    data.edges.filter(e => {
      if (e.literary && !showLiterary()) return false;
      return visibleNodes().some(n => n.id === e.from)
          && visibleNodes().some(n => n.id === e.to);
    })
  );

  function conjugationText(tenseId: string, person: string): string {
    const verb = VERBS[selectedVerb()];
    let p = person;
    if (!verb) return "—";
    if (tenseId in PERIPHRASES) return PERIPHRASES[tenseId]!(verb.infinitive, p);
    const forms = verb.forms[tenseId as TenseId];
    if (!forms) return "—";
    if ((p === "elle" || p === "on") && !forms[p as Person]) p = "il";
    return forms[p as keyof typeof forms] ?? "—";
  }

  // Full, human title — the page's tense names, not cramped abbreviations.
  function titleOf(id: string): string {
    return tenseTitles[id] ?? (data.nodes.find(n => n.id === id) as any)?.title ?? id;
  }

  // ── Layout: cards measured & fitted, cells stack (no overlap) ─
  const layout = createMemo(() => {
    fontTick();
    const mb = isMobile();
    const cfg = mb ? MB_CFG : DK_CFG;

    const nodes = visibleNodes().slice().sort((a, b) => {
      const ca = CELL_ORDER[a.id] ?? 5;
      const cb = CELL_ORDER[b.id] ?? 5;
      if (ca !== cb) return ca - cb;
      return a.id < b.id ? -1 : 1;
    });

    // Measure every card. Form height uses the WORST person so rows
    // don't reflow when the learner toggles je → nous → ils.
    const metrics = new Map<string, CardMetrics>();
    for (const n of nodes) {
      const title = fitText(titleOf(n.id), cfg.titleW, 2, cfg.titleMax, cfg.titleMin, 600);
      let formLines = 1;
      for (const p of PERSONS) {
        const t = conjugationText(n.id, p);
        if (t === "—") continue;
        formLines = Math.max(formLines,
          fitText(t, cfg.formW, cfg.formMaxLines, cfg.formMax, cfg.formMin, 700).lines.length);
      }
      const h = Math.max(cfg.minH,
        cfg.padT
        + title.lines.length * title.size * 1.16
        + cfg.gap
        + formLines * cfg.formMax * 1.22
        + cfg.padB);
      metrics.set(n.id, { node: n, title, h });
    }

    // Group by cell (mood × time)
    const cells = new Map<string, DiagramNode[]>();
    for (const n of nodes) {
      const k = `${n.lane}|${n.timePosition}`;
      const arr = cells.get(k);
      if (arr) arr.push(n); else cells.set(k, [n]);
    }
    const cellH = (cell: DiagramNode[]) =>
      cell.reduce((s, n) => s + metrics.get(n.id)!.h, 0) + (cell.length - 1) * (mb ? MB_CELL_GAP : DK_CELL_GAP);

    const boxes = new Map<string, Box>();

    if (mb) {
      const dividers: Divider[] = [];
      let y = MB_MOOD_HEADER_H;
      for (const time of TIME_ORDER) {
        const laneCells = MOOD_ORDER.map(mood => cells.get(`${mood}|${time}`) ?? []);
        const rowH = Math.max(0, ...laneCells.map(cellH));
        if (rowH === 0) continue;
        dividers.push({ time, lineY: y + 1.5, textY: y + 12 });
        const top = y + MB_DIVIDER_H;
        laneCells.forEach((cell, mi) => {
          let cy = top;
          for (const n of cell) {
            const m = metrics.get(n.id)!;
            const x = mi * MB_COL_W;
            boxes.set(n.id, { node: n, x, y: cy, w: MB_CARD_W, h: m.h, cx: x + MB_CARD_W / 2, cy: cy + m.h / 2, m });
            cy += m.h + MB_CELL_GAP;
          }
        });
        y = top + rowH + MB_ROW_GAP;
      }
      return { mb: true as const, boxes, svgW: MB_SVG_W, svgH: y - MB_ROW_GAP + 6, dividers, lanes: [] as LaneRect[] };
    }

    const lanes: LaneRect[] = [];
    let y = DK_COL_HEADER_H + DK_PAD;
    for (const mood of MOOD_ORDER) {
      let laneH = 0;
      for (const time of TIME_ORDER) {
        const cell = cells.get(`${mood}|${time}`) ?? [];
        if (!cell.length) continue;
        laneH = Math.max(laneH, cellH(cell));
      }
      laneH = Math.max(laneH, 46);
      lanes.push({ mood, y, h: laneH });
      for (const time of TIME_ORDER) {
        const cell = cells.get(`${mood}|${time}`) ?? [];
        let cy = y;
        const x = DK_LANE_HEADER_W + DK_PAD + TIME_COLUMNS[time] * DK_COL_W;
        for (const n of cell) {
          const m = metrics.get(n.id)!;
          boxes.set(n.id, { node: n, x, y: cy, w: DK_CARD_W, h: m.h, cx: x + DK_CARD_W / 2, cy: cy + m.h / 2, m });
          cy += m.h + DK_CELL_GAP;
        }
      }
      y += laneH + DK_LANE_GAP;
    }
    return { mb: false as const, boxes, svgW: DK_SVG_W, svgH: y - DK_LANE_GAP + DK_PAD, dividers: [] as Divider[], lanes };
  });

  // ── Interactions ─────────────────────────────────────────────
  function toggleSelect(id: string) {
    setActiveEdge(null);
    setSelectedTense(t => (t === id ? null : id));
  }
  function toggleEdge(e: DiagramEdge) {
    setSelectedTense(null);
    setActiveEdge(cur => (cur === e ? null : e));
  }

  // Bring the explanation strip into view when it appears
  createEffect(() => {
    const c = caption();
    if (c && captionRef) captionRef.scrollIntoView({ block: "nearest", behavior: "smooth" });
  });

  type Caption =
    | { kind: "node"; node: DiagramNode; rel: DiagramEdge[] }
    | { kind: "edge"; edge: DiagramEdge }
    | null;

  const caption = createMemo<Caption>(() => {
    const selId = selectedTense();
    if (selId) {
      const node = data.nodes.find(n => n.id === selId);
      if (node) return { kind: "node", node, rel: visibleEdges().filter(e => e.from === selId || e.to === selId) };
    }
    const e = activeEdge();
    if (e) return { kind: "edge", edge: e };
    return null;
  });

  const nodeCaption = () => {
    const c = caption();
    return c && c.kind === "node" ? c : null;
  };
  const edgeCaption = () => {
    const c = caption();
    return c && c.kind === "edge" ? c : null;
  };

  function edgePhrase(e: DiagramEdge, thisId: string): string {
    const otherId = e.from === thisId ? e.to : e.from;
    const other = titleOf(otherId);
    const isFrom = e.from === thisId;
    switch (e.type) {
      case "auxiliary-compound": return isFrom ? `Builds ${other} — ${e.label}` : `Built from ${other} — ${e.label}`;
      case "aspect-pair":        return `Same moment as ${other} — ongoing vs completed`;
      case "stem-share":         return `Shares its stem with ${other} — different endings`;
      case "mood-swap":          return `Same idea in another mood: ${other} (${e.label})`;
      case "anteriority":        return isFrom ? `Happens before ${other} (${e.label})` : `${other} happens before this (${e.label})`;
    }
  }

  // ── Node card ────────────────────────────────────────────────
  const NodeCard = (p: { b: Box }) => {
    const b = () => p.b;
    const node = () => b().node;
    const cfg = () => (layout().mb ? MB_CFG : DK_CFG);
    const laneFill = () => MOOD_FILL[node().lane as MoodId];
    const selected = () => selectedTense() === node().id;
    const isPresent = () => node().id === "present-indicatif";
    const lit = () => !!node().literary;
    const form = () => fitText(
      conjugationText(node().id, selectedPerson()),
      cfg().formW, cfg().formMaxLines, cfg().formMax, cfg().formMin, 700,
    );

    const titleX = () => b().x + 12;
    const titleY0 = () => b().y + cfg().padT + b().m.title.size * 0.85;
    const formFit = () => form();
    const formBaselineLast = () => b().y + b().h - cfg().padB - formFit().size * 0.3;

    return (
      <g
        class="diagram-node"
        onClick={() => toggleSelect(node().id)}
        onKeyDown={e => e.key === "Enter" && toggleSelect(node().id)}
        style={{ cursor: "pointer" }}
        role="button" tabIndex={0}
        aria-label={`${titleOf(node().id)} — ${selectedPerson()}: ${conjugationText(node().id, selectedPerson())}. Activates details below.`}
        aria-pressed={selected()}
      >
        {/* Opaque base — hides lines passing underneath */}
        <rect x={b().x} y={b().y} width={b().w} height={b().h} rx="6" fill="var(--surface-2)" />
        {/* Présent — beginner's start ring */}
        {isPresent() && (
          <rect x={b().x - 2.5} y={b().y - 2.5} width={b().w + 5} height={b().h + 5} rx="8"
            fill="none" stroke="var(--accent)" stroke-width="1.5" opacity="0.85" />
        )}
        {/* Mood-tinted frame */}
        <rect x={b().x} y={b().y} width={b().w} height={b().h} rx="6"
          fill={laneFill()}
          fill-opacity={selected() ? 0.16 : 0}
          stroke={laneFill()}
          stroke-opacity={selected() ? 1 : lit() ? 0.3 : 0.45}
          stroke-width={selected() ? 1.8 : 1}
          stroke-dasharray={lit() && !selected() ? "4 3" : undefined}
          opacity={lit() ? 0.72 : 1}
        />
        {/* Left accent stripe */}
        <line
          x1={b().x + 4.5} y1={b().y + 9} x2={b().x + 4.5} y2={b().y + b().h - 9}
          stroke={laneFill()} stroke-width="3" stroke-linecap="round"
          opacity={selected() ? 1 : lit() ? 0.35 : 0.55}
        />
        {/* Tense name — measured, wrapped, ellipsised */}
        <text
          font-size={String(b().m.title.size)} font-weight="600"
          fill="var(--accent-ink)" opacity={lit() ? 0.75 : 1}
        >
          <For each={b().m.title.lines}>
            {(line, i) => (
              <tspan x={titleX()} y={titleY0() + i() * b().m.title.size * 1.16}>{line}</tspan>
            )}
          </For>
        </text>
        {/* Conjugated form — bottom-anchored, centred */}
        <text
          x={b().cx} text-anchor="middle"
          font-size={String(formFit().size)} font-weight="700"
          fill="var(--text)" opacity={lit() ? 0.75 : 1}
        >
          <For each={formFit().lines}>
            {(line, i) => (
              <tspan
                x={b().cx}
                y={formBaselineLast() - (formFit().lines.length - 1 - i()) * formFit().size * 1.22}
              >{line}</tspan>
            )}
          </For>
        </text>
      </g>
    );
  };

  // ── Edge (line between cards) ────────────────────────────────
  const EdgeEl = (p: { edge: DiagramEdge; marker?: string; labelMaxW: number; labelMax: number; labelMin: number }) => {
    const layout_ = layout();
    const f = () => layout_.boxes.get(p.edge.from);
    const t = () => layout_.boxes.get(p.edge.to);
    const active = () => activeEdge() === p.edge;
    const related = () => {
      const s = selectedTense();
      return !!s && (p.edge.from === s || p.edge.to === s);
    };
    const color = () => EDGE_COLOR[p.edge.type];
    const d = () => {
      const a = f(), b = t();
      if (!a || !b) return "";
      return layout_.mb ? edgePathMb(a, b, p.edge) : edgePathDk(a, b, p.edge);
    };
    const label = () => fitText(p.edge.label, p.labelMaxW, 2, p.labelMax, p.labelMin, 500);
    // Label placement: beside vertical lines, above everything else
    const labelPos = () => {
      const a = f(), b = t();
      if (!a || !b) return null;
      const vertical = Math.abs(a.cx - b.cx) < 4;
      if (vertical) {
        return { x: Math.max(a.cx, b.cx) + 7, y: (a.cy + b.cy) / 2 + 3, anchor: "start" as const };
      }
      return { x: (a.cx + b.cx) / 2, y: (a.cy + b.cy) / 2 - 6, anchor: "middle" as const };
    };

    return (
      <Show when={f() && t()}>
        <g
          class="diagram-edge"
          onClick={ev => { ev.stopPropagation(); toggleEdge(p.edge); }}
          onKeyDown={e => e.key === "Enter" && toggleEdge(p.edge)}
          style={{ cursor: "pointer" }}
          role="button" tabIndex={0}
          aria-label={`${EDGE_NAME[p.edge.type]}: ${p.edge.label}`}
        >
          <path d={d()} fill="none" stroke={color()}
            stroke-width={active() ? 2.5 : 1.5}
            stroke-dasharray={EDGE_DASH[p.edge.type]}
            stroke-linecap="round"
            opacity={active() ? 1 : selectedTense() ? (related() ? 0.9 : 0.08) : 0.4}
            marker-end={p.marker && (active() || related()) ? p.marker : undefined}
          />
          {/* generous invisible hit area */}
          <path d={d()} fill="none" stroke="transparent" stroke-width="16" />
          {/* label only on the active line — noise by default */}
          <Show when={active()}>
            {(() => {
              const pos = labelPos();
              const fit = label();
              if (!pos) return null;
              return (
                <text
                  x={pos.x} y={pos.y} text-anchor={pos.anchor}
                  font-size={String(fit.size)} font-style="italic" font-weight="500"
                  fill={color()}
                  paint-order="stroke" stroke="var(--card)" stroke-width="3.5"
                  stroke-linejoin="round"
                  style={{ "pointer-events": "none" }}
                >
                  <For each={fit.lines}>
                    {(line, i) => (
                      <tspan x={pos.x} y={pos.y + i() * fit.size * 1.2}
                        dy={i() === 0 ? -((fit.lines.length - 1) * fit.size * 1.2) / 2 : undefined}
                      >{line}</tspan>
                    )}
                  </For>
                </text>
              );
            })()}
          </Show>
        </g>
      </Show>
    );
  };

  // ── Legend ────────────────────────────────────────────────────
  const Legend = () => (
    <details class="diagram-legend" open={!isMobile()}>
      <summary>What the lines &amp; marks mean</summary>
      <div class="legend-section">
        <p class="legend-section-title">Lines</p>
        <div class="legend-grid">
          {(Object.keys(EDGE_COLOR) as EdgeType[]).map(t => (
            <div class="legend-item legend-item-wide">
              <svg width="30" height="8" aria-hidden="true" style={{ "flex-shrink": "0" }}>
                <line x1="0" y1="4" x2="30" y2="4"
                  stroke={EDGE_COLOR[t]} stroke-width="2.5"
                  stroke-dasharray={EDGE_DASH[t]} stroke-linecap="round" />
              </svg>
              <span><strong>{EDGE_NAME[t]}</strong> — {EDGE_LABEL[t]}</span>
            </div>
          ))}
        </div>
      </div>
      <div class="legend-section">
        <p class="legend-section-title">Marks</p>
        <div class="legend-grid">
          <div class="legend-item">
            <span class="legend-ring" aria-hidden="true" />
            <span><strong>Présent</strong> — the starting point</span>
          </div>
          <div class="legend-item">
            <svg width="26" height="10" aria-hidden="true" style={{ "flex-shrink": "0" }}>
              <rect x="1" y="1" width="24" height="8" rx="3" fill="none"
                stroke="var(--text-2)" stroke-width="1.2" stroke-dasharray="4 3" opacity="0.7" />
            </svg>
            <span><strong>Dashed border</strong> — literary tense, rare in speech (toggle above)</span>
          </div>
        </div>
      </div>
    </details>
  );

  // ── Controls bar ──────────────────────────────────────────────
  const Controls = () => (
    <div class="diagram-controls">
      <div class="control-group">
        <span class="control-label">Verb</span>
        <div class="verb-dropdown">
          <button
            class="verb-dropdown-trigger"
            type="button"
            aria-expanded={verbDropdownOpen()}
            aria-haspopup="listbox"
            aria-controls="verb-dropdown-menu"
            onClick={() => setVerbDropdownOpen(o => !o)}
          >
            {VERB_OPTIONS.find(o => o.value === selectedVerb())?.label ?? selectedVerb()}
            <span class="verb-dropdown-arrow" aria-hidden="true">▾</span>
          </button>
          <Show when={verbDropdownOpen()}>
            <div id="verb-dropdown-menu" class="verb-dropdown-menu" role="listbox">
              {VERB_OPTIONS.map(opt => (
                <button
                  class={`verb-dropdown-item${selectedVerb() === opt.value ? " active" : ""}`}
                  role="option"
                  aria-selected={selectedVerb() === opt.value}
                  type="button"
                  onClick={() => {
                    setSelectedVerb(opt.value);
                    setVerbDropdownOpen(false);
                  }}
                >
                  {opt.label}
                </button>
              ))}
            </div>
          </Show>
        </div>
      </div>
      <div class="control-group">
        <div class="person-pills">
          {PERSONS.map(p => (
            <button class={`person-pill${selectedPerson() === p ? " active" : ""}`}
              onClick={() => setSelectedPerson(p)} type="button">{p}</button>
          ))}
        </div>
      </div>
      <label class="literary-toggle">
        <input type="checkbox" checked={showLiterary()} onChange={e => setShowLiterary(e.currentTarget.checked)} />
        <span class="literary-toggle-label">Literary tenses</span>
      </label>
    </div>
  );

  // ── Explanation strip under the diagram ───────────────────────
  const Caption = () => (
    <div class="diagram-caption" ref={captionRef} aria-live="polite">
      <Show
        when={nodeCaption()}
        fallback={
          <Show when={edgeCaption()} fallback={
            <p class="cap-hint">Tap any card to inspect a tense · tap a line to see what it connects.</p>
          }>
            {(c) => (
              <div>
                <div class="cap-head">
                  <strong class="cap-title">
                    {titleOf(c().edge.from)} → {titleOf(c().edge.to)}
                  </strong>
                  <span class="cap-chip">{EDGE_NAME[c().edge.type]}</span>
                </div>
                <p class="cap-rule">{EDGE_LABEL[c().edge.type]}.</p>
                <p class="cap-rule cap-note">On the diagram: <em>{c().edge.label}</em></p>
              </div>
            )}
          </Show>
        }
      >
        {(c) => (
          <div>
            <div class="cap-head">
              <strong class="cap-title">{titleOf(c().node.id)}</strong>
              {c().node.literary && <span class="cap-chip">literary</span>}
            </div>
            <Show when={tenseRules[c().node.id]}>
              <p class="cap-rule">{tenseRules[c().node.id]}</p>
            </Show>
            <p class="cap-form">
              {selectedPerson()} → <strong>{conjugationText(c().node.id, selectedPerson())}</strong>
            </p>
            <Show when={c().rel.length > 0}>
              <ul class="cap-rel">
                {c().rel.map(e => (
                  <li>
                    <svg class="cap-swatch" width="26" height="8" aria-hidden="true">
                      <line x1="0" y1="4" x2="26" y2="4"
                        stroke={EDGE_COLOR[e.type]} stroke-width="2.5"
                        stroke-dasharray={EDGE_DASH[e.type]} stroke-linecap="round" />
                    </svg>
                    <span>{edgePhrase(e, c().node.id)}</span>
                  </li>
                ))}
              </ul>
            </Show>
            <a class="cap-link" href={`/knowledge/verbs/tenses/${c().node.id}`}>
              Open the full {titleOf(c().node.id)} page →
            </a>
          </div>
        )}
      </Show>
    </div>
  );

  // ── Render ────────────────────────────────────────────────────
  return (
    <div class="verb-diagram">
      <Controls />

      {/* ── MOBILE: mood = column, time = row divider ── */}
      <Show when={layout().mb}>
        <div class="diagram-scroll-wrap">
          <svg width={layout().svgW} height={layout().svgH}
            viewBox={`0 0 ${layout().svgW} ${layout().svgH}`}
            class="grammar-map-svg"
            role="img" aria-label="French verb tense map"
          >
            <defs>
              <marker id="mb-arr-aux" viewBox="0 0 8 6" refX="7" refY="3"
                markerWidth="5" markerHeight="4" orient="auto" markerUnits="userSpaceOnUse">
                <path d="M 0 0 L 8 3 L 0 6 Z" fill={EDGE_COLOR["auxiliary-compound"]} />
              </marker>
            </defs>

            {/* Mood column tints */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <rect
                  x={i() * MB_COL_W} y={MB_MOOD_HEADER_H - 2}
                  width={MB_CARD_W} height={layout().svgH - MB_MOOD_HEADER_H + 2}
                  fill={MOOD_FILL[mood]} fill-opacity="0.05" rx="4"
                />
              )}
            </For>

            {/* Mood headers — full words fit now */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <text
                  x={i() * MB_COL_W + MB_CARD_W / 2} y={17}
                  text-anchor="middle" font-size="8" font-weight="700"
                  letter-spacing="0.06em" fill="var(--muted)"
                >
                  {MOOD_LABELS[mood].toUpperCase()}
                </text>
              )}
            </For>

            {/* Time dividers */}
            <For each={layout().dividers}>
              {(d) => (
                <g>
                  <line x1="0" y1={d.lineY} x2={layout().svgW} y2={d.lineY}
                    stroke="var(--line)" stroke-width="1" opacity="0.7" />
                  <text x="2" y={d.textY} font-size="7.5" font-weight="600"
                    letter-spacing="0.1em" fill="var(--faint)">
                    {TIME_DIVIDERS[d.time].toUpperCase()}
                  </text>
                </g>
              )}
            </For>

            {/* Edges */}
            <For each={visibleEdges()}>
              {(edge) => (
                <EdgeEl edge={edge} marker="url(#mb-arr-aux)"
                  labelMaxW={120} labelMax={8.5} labelMin={7.5} />
              )}
            </For>

            {/* Cards */}
            <For each={[...layout().boxes.values()]}>
              {(b) => <NodeCard b={b} />}
            </For>
          </svg>
        </div>
        <Caption />
        <Legend />
      </Show>

      {/* ── DESKTOP: time = column, mood = lane ── */}
      <Show when={!layout().mb}>
        <div class="diagram-scroll-wrap">
          <svg width={layout().svgW} height={layout().svgH}
            viewBox={`0 0 ${layout().svgW} ${layout().svgH}`}
            class="grammar-map-svg"
            role="img" aria-label="French verb tense map"
          >
            <defs>
              <marker id="dk-arr-aux" viewBox="0 0 8 6" refX="7" refY="3"
                markerWidth="6" markerHeight="5" orient="auto" markerUnits="userSpaceOnUse">
                <path d="M 0 0 L 8 3 L 0 6 Z" fill={EDGE_COLOR["auxiliary-compound"]} />
              </marker>
            </defs>

            {/* Lane backgrounds */}
            <For each={layout().lanes}>
              {(lane) => (
                <rect
                  x={DK_LANE_HEADER_W} y={lane.y - 7}
                  width={layout().svgW - DK_LANE_HEADER_W} height={lane.h + 14}
                  fill={MOOD_FILL[lane.mood]} fill-opacity="0.05" rx="4"
                />
              )}
            </For>

            {/* Column headers */}
            <For each={TIME_ORDER}>
              {(time) => (
                <text
                  x={DK_LANE_HEADER_W + DK_PAD + TIME_COLUMNS[time] * DK_COL_W + DK_CARD_W / 2}
                  y={DK_COL_HEADER_H - 8}
                  text-anchor="middle" font-size="9" font-weight="600"
                  letter-spacing="0.08em" fill="var(--text-2)">
                  {TIME_DIVIDERS[time].toUpperCase()}
                </text>
              )}
            </For>

            {/* Lane labels */}
            <For each={layout().lanes}>
              {(lane) => (
                <text
                  x={DK_LANE_HEADER_W - 8} y={lane.y + lane.h / 2 + 4}
                  text-anchor="end" font-size="10" font-weight="600"
                  fill="var(--muted)"
                >
                  {MOOD_LABELS[lane.mood]}
                </text>
              )}
            </For>

            {/* Edges */}
            <For each={visibleEdges()}>
              {(edge) => (
                <EdgeEl edge={edge} marker="url(#dk-arr-aux)"
                  labelMaxW={200} labelMax={10.5} labelMin={9} />
              )}
            </For>

            {/* Cards */}
            <For each={[...layout().boxes.values()]}>
              {(b) => <NodeCard b={b} />}
            </For>
          </svg>
        </div>
        <Caption />
        <Legend />
      </Show>
    </div>
  );
}
