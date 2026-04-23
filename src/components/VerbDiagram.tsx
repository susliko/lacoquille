import { createSignal, createMemo, For, Show, onMount, onCleanup } from "solid-js";
import type { DiagramData, DiagramNode, DiagramEdge, MoodId } from "./GrammarMap/types";
import {
  TIME_COLUMNS, MOOD_LABELS, MOOD_ORDER, MOOD_HUE,
  EDGE_COLOR, EDGE_DASH, EDGE_LABEL,
} from "./GrammarMap/types";
import { VERBS, PERSONS, VERB_OPTIONS } from "../data/conjugations/index";
import type { TenseId, Person } from "../data/conjugations/types";

// ── Desktop layout (time=X, mood=Y) ──────────────────────────
const DK_CARD_W = 140;
const DK_CARD_H = 62;
const DK_COL_GAP = 16;
const DK_LANE_GAP = 14;
const DK_LANE_HEADER_W = 96;
const DK_COL_HEADER_H = 28;
const DK_PAD = 12;

const DK_NUM_COLS = 7;
const DK_COL_W = DK_CARD_W + DK_COL_GAP;
const DK_LANE_H = DK_CARD_H + DK_LANE_GAP;

const DK_SVG_W = DK_LANE_HEADER_W + DK_PAD * 2 + DK_NUM_COLS * DK_COL_W;
const DK_SVG_H = DK_COL_HEADER_H + DK_PAD * 2 + MOOD_ORDER.length * DK_LANE_H;

function dkColX(col: number) { return DK_LANE_HEADER_W + DK_PAD + col * DK_COL_W; }
function dkLaneY(row: number) { return DK_COL_HEADER_H + DK_PAD + row * DK_LANE_H; }

// ── Mobile layout (mood=X, time=Y) — transposed ──────────────
// 4 mood columns × ~78px ≈ 362px total → fits 375px phones
const MB_CARD_W = 74;
const MB_CARD_H = 58;
const MB_COL_GAP = 4;
const MB_ROW_GAP = 5;
const MB_TIME_LABEL_W = 46;
const MB_MOOD_HEADER_H = 26;

const MB_COL_W = MB_CARD_W + MB_COL_GAP;
const MB_ROW_H = MB_CARD_H + MB_ROW_GAP;

const MB_SVG_W = MB_TIME_LABEL_W + MOOD_ORDER.length * MB_COL_W;
const MB_SVG_H = MB_MOOD_HEADER_H + Object.keys(TIME_COLUMNS).length * MB_ROW_H;

const MB_TIME_LABELS: Record<string, string> = {
  "far-past": "ant.", "past": "past", "near-past": "rec.",
  "present": "now", "near-future": "near", "future": "fut.", "far-future": "f.ant.",
};

function mbColX(moodIdx: number) { return MB_TIME_LABEL_W + moodIdx * MB_COL_W; }
function mbRowY(timeIdx: number) { return MB_MOOD_HEADER_H + timeIdx * MB_ROW_H; }

// ── Types ─────────────────────────────────────────────────────
interface NodePos { node: DiagramNode; x: number; y: number; cx: number; cy: number; }

interface Props {
  data: DiagramData;
  tenseTitles: Record<string, string>;
  tenseRules:  Record<string, string>;
}

// ── Verbal periphrases — computed dynamically ─────────────────
const PERIPHRASES: Record<string, (inf: string, person: string) => string> = {
  "futur-proche": (inf, person) => {
    const f = VERBS["aller"]?.forms["present-indicatif"]?.[person as Person];
    return f ? `${f} ${inf}` : "—";
  },
  "passe-recent": (inf, person) => {
    const venir: Record<string, string> = {
      je: "viens", tu: "viens", il: "vient",
      nous: "venons", vous: "venez", ils: "viennent",
    };
    return `${venir[person] ?? "—"} de ${inf}`;
  },
  "present-progressif": (inf, person) => {
    const f = VERBS["être"]?.forms["present-indicatif"]?.[person as Person];
    return f ? `${f} en train de ${inf}` : "—";
  },
};

// ── Edge path — desktop ───────────────────────────────────────
function edgePath(from: NodePos, to: NodePos, edge: DiagramEdge): string {
  const dx = to.cx - from.cx;
  const dy = to.cy - from.cy;

  // Cross-lane edges: bezier from card top/bottom
  if (edge.type === "mood-swap" || edge.type === "stem-share") {
    const fromY = dy > 0 ? from.y + DK_CARD_H : from.y;
    const toY   = dy > 0 ? to.y               : to.y + DK_CARD_H;
    return `M ${from.cx} ${fromY} C ${from.cx} ${from.cy + dy * 0.5}, ${from.cx} ${from.cy + dy * 0.5}, ${to.cx} ${toY}`;
  }

  // Same-lane (horizontal) edges
  if (Math.abs(dy) < 4) {
    const goingRight = dx > 0;
    // Connect card edges, not centers, so the line stays between the cards
    const startX = goingRight ? from.x + DK_CARD_W : from.x;
    const endX   = goingRight ? to.x               : to.x + DK_CARD_W;

    // Y offset per type so overlapping same-lane lines stay visually separate
    const yOff = edge.type === "auxiliary-compound" ? -8
               : edge.type === "aspect-pair"         ?  8
               : 0;
    const y = from.cy + yOff;

    // Count skipped columns — if > 1, arc above the row to avoid intermediate cards
    const fromCol = TIME_COLUMNS[from.node.timePosition];
    const toCol   = TIME_COLUMNS[to.node.timePosition];
    if (Math.abs(toCol - fromCol) > 1) {
      const arcTop = from.y - 18;
      return `M ${startX} ${y} C ${startX} ${arcTop}, ${endX} ${arcTop}, ${endX} ${y}`;
    }

    return `M ${startX} ${y} L ${endX} ${y}`;
  }

  // Generic diagonal
  const cp1y = from.cy + dy * 0.4;
  const cp2y = to.cy - dy * 0.4;
  return `M ${from.cx} ${from.cy} C ${from.cx} ${cp1y}, ${to.cx} ${cp2y}, ${to.cx} ${to.cy}`;
}

// ── Edge path — mobile (transposed: mood=col, time=row) ──────
function edgePathMobile(from: NodePos, to: NodePos, edge: DiagramEdge): string {
  const dx = to.cx - from.cx;
  const dy = to.cy - from.cy;

  if (Math.abs(dx) < 4) {
    // Same column (same mood) → vertical; connect card edges not centers
    const goingDown = dy > 0;
    const startY = goingDown ? from.y + MB_CARD_H : from.y;
    const endY   = goingDown ? to.y               : to.y + MB_CARD_H;

    // X offset by type to separate overlapping same-column lines
    const xOff = edge.type === "auxiliary-compound" ? -6
               : edge.type === "aspect-pair"         ?  6
               : 0;
    const x = from.cx + xOff;

    // Arc to the side if skipping 2+ rows
    const fromRow = TIME_COLUMNS[from.node.timePosition];
    const toRow   = TIME_COLUMNS[to.node.timePosition];
    if (Math.abs(toRow - fromRow) > 1) {
      const arcLeft = from.x - 14;
      return `M ${x} ${startY} C ${arcLeft} ${startY}, ${arcLeft} ${endY}, ${x} ${endY}`;
    }

    return `M ${x} ${startY} L ${x} ${endY}`;
  }

  if (Math.abs(dy) < 4) {
    // Same row (same time) → horizontal; connect card edges
    const fx = dx > 0 ? from.x + MB_CARD_W : from.x;
    const tx = dx > 0 ? to.x               : to.x + MB_CARD_W;
    return `M ${fx} ${from.cy} L ${tx} ${to.cy}`;
  }

  // Diagonal
  return `M ${from.cx} ${from.cy} C ${from.cx} ${to.cy}, ${to.cx} ${from.cy}, ${to.cx} ${to.cy}`;
}

// ── Component ─────────────────────────────────────────────────
export default function VerbDiagram({ data, tenseTitles, tenseRules }: Props) {
  const [selectedPerson, setSelectedPerson] = createSignal<string>("je");
  const [selectedVerb,   setSelectedVerb]   = createSignal<string>("parler");
  const [showLiterary,   setShowLiterary]   = createSignal(false);
  const [activeNode,     setActiveNode]     = createSignal<string | null>(null);
  const [activeEdge,     setActiveEdge]     = createSignal<DiagramEdge | null>(null);
  const [isMobile,       setIsMobile]       = createSignal(
    typeof window !== "undefined" ? window.innerWidth < 768 : false
  );
  const [highlightedTense, _setHighlightedTense] = createSignal<string | null>(null);

  onMount(() => {
    const check = () => setIsMobile(window.innerWidth < 768);
    window.addEventListener("resize", check);
    onCleanup(() => window.removeEventListener("resize", check));
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

  // Desktop: time=col, mood=row
  const dkNodePositions = createMemo<NodePos[]>(() =>
    visibleNodes().map(node => {
      const col = TIME_COLUMNS[node.timePosition];
      const row = MOOD_ORDER.indexOf(node.lane as MoodId);
      const x = dkColX(col), y = dkLaneY(row);
      return { node, x, y, cx: x + DK_CARD_W / 2, cy: y + DK_CARD_H / 2 };
    })
  );
  const dkPosMap = createMemo(() => {
    const m = new Map<string, NodePos>();
    for (const p of dkNodePositions()) m.set(p.node.id, p);
    return m;
  });

  // Mobile: mood=col, time=row
  const mbNodePositions = createMemo<NodePos[]>(() =>
    visibleNodes().map(node => {
      const col = MOOD_ORDER.indexOf(node.lane as MoodId);
      const row = TIME_COLUMNS[node.timePosition];
      const x = mbColX(col), y = mbRowY(row);
      return { node, x, y, cx: x + MB_CARD_W / 2, cy: y + MB_CARD_H / 2 };
    })
  );
  const mbPosMap = createMemo(() => {
    const m = new Map<string, NodePos>();
    for (const p of mbNodePositions()) m.set(p.node.id, p);
    return m;
  });

  function getConjugation(tenseId: string): string {
    const verb   = VERBS[selectedVerb()];
    const person = selectedPerson();
    if (!verb) return "—";
    if (tenseId in PERIPHRASES) return PERIPHRASES[tenseId]!(verb.infinitive, person);
    const forms = verb.forms[tenseId as TenseId];
    if (!forms) return "—";
    return forms[person as keyof typeof forms] ?? "—";
  }

  const activeNodeData = createMemo<DiagramNode | null>(() =>
    activeNode() ? (data.nodes.find(n => n.id === activeNode()) ?? null) : null
  );

  // Short title for diagram cards (diagram.json title wins)
  function nodeTitle(node: DiagramNode): string {
    return (node as any).title ?? tenseTitles[node.id] ?? node.id;
  }

  // Full title for detail panels (content page title wins)
  function nodePanelTitle(node: DiagramNode): string {
    return tenseTitles[node.id] ?? (node as any).title ?? node.id;
  }

  function openNode(id: string) {
    setActiveNode(activeNode() === id ? null : id);
    setActiveEdge(null);
  }

  // ── Shared panels ────────────────────────────────────────────
  const Panels = () => (
    <>
      <Show when={activeEdge()}>
        {edge => (
          <aside class="edge-panel" role="complementary" aria-live="polite">
            <button class="panel-close" onClick={() => setActiveEdge(null)} aria-label="Close">✕</button>
            <strong class="panel-type-tag">{edge().type.replace(/-/g, " ")}</strong>
            <p style={{ color: EDGE_COLOR[edge().type], "font-weight": "500", "margin-bottom": "0.25rem" }}>
              {EDGE_LABEL[edge().type]}
            </p>
            <p style={{ color: "var(--text-2)", "font-size": "0.875rem", margin: "0" }}>
              {edge().label}
            </p>
            <p class="panel-edge-nodes">
              <a href={`/verbs/tenses/${edge().from}`}>
                {nodeTitle(data.nodes.find(n => n.id === edge().from)!)}
              </a>
              {" ↔ "}
              <a href={`/verbs/tenses/${edge().to}`}>
                {nodeTitle(data.nodes.find(n => n.id === edge().to)!)}
              </a>
            </p>
          </aside>
        )}
      </Show>

      <Show when={activeNodeData()}>
        {(nodeData: () => DiagramNode) => {
          const tenseId = () => nodeData().id;
          const isPeriph = () => tenseId() in PERIPHRASES;
          const forms = () => {
            if (isPeriph()) return null;
            return VERBS[selectedVerb()]?.forms[tenseId() as TenseId];
          };

          return (
            <aside class="node-panel" role="complementary" aria-live="polite">
              <button class="panel-close" onClick={() => setActiveNode(null)} aria-label="Close">✕</button>
              <h3 class="panel-title">{nodePanelTitle(nodeData())}</h3>
              <p class="panel-rule">{tenseRules[tenseId()] ?? ""}</p>

              <table class="panel-conj-table">
                <tbody>
                  {PERSONS.map(person => {
                    const form = isPeriph()
                      ? (PERIPHRASES[tenseId()]?.(VERBS[selectedVerb()]?.infinitive ?? "", person) ?? "—")
                      : (forms()?.[person] ?? null);
                    if (form === null) return null;
                    const active = () => selectedPerson() === person;
                    return (
                      <tr class={active() ? "active-row" : ""} onClick={() => setSelectedPerson(person)} style={{ cursor: "pointer" }}>
                        <th scope="row">{person}</th>
                        <td>{form}</td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>

              <a href={`/verbs/tenses/${tenseId()}`} class="panel-learn-more">
                Full reference page →
              </a>
            </aside>
          );
        }}
      </Show>
    </>
  );

  // ── Node card (SVG) — shared renderer with size params ───────
  const NodeCard = (props: {
    node: DiagramNode;
    x: number; y: number;
    cardW: number; cardH: number;
    fontSize: { title: number; form: number };
  }) => {
    const { node, x, y, cardW, cardH, fontSize } = props;
    const isActive     = () => activeNode() === node.id;
    const isHighlighted = () => highlightedTense() === node.id;
    const activated    = () => isActive() || isHighlighted();
    const hue = MOOD_HUE[node.lane as MoodId];
    const isPresent = () => node.id === "present-indicatif";
    const isLiterary = () => node.literary;

    return (
      <g
        class="diagram-node"
        onClick={() => openNode(node.id)}
        style={{ cursor: "pointer" }}
        role="button" tabIndex={0}
        aria-label={`${nodeTitle(node)}: ${getConjugation(node.id)}`}
        aria-pressed={isActive()}
        onKeyDown={e => e.key === "Enter" && openNode(node.id)}
      >
        {/* Opaque base — blocks edge lines from showing through */}
        <rect x={x} y={y} width={cardW} height={cardH} rx="6" fill="var(--surface-2)" />
        {/* Beginner's start indicator: gold ring on présent */}
        {isPresent() && (
          <rect x={x - 2} y={y - 2} width={cardW + 4} height={cardH + 4} rx="8"
            fill="none"
            stroke="var(--gold)"
            stroke-width="1.5"
            opacity="0.5"
          />
        )}
        {/* Color overlay */}
        <rect x={x} y={y} width={cardW} height={cardH} rx="6"
          fill={activated() ? `hsl(${hue} 55% 45% / 0.18)` : isLiterary() ? "none" : "none"}
          stroke={activated() ? `hsl(${hue} 65% 55%)` : isLiterary() ? `hsl(${hue} 40% 50% / 0.2)` : `hsl(${hue} 40% 50% / 0.4)`}
          stroke-width={activated() ? 1.5 : 1}
          opacity={isLiterary() ? 0.65 : 1}
        />
        {/* Left accent stripe */}
        <line
          x1={x + 4} y1={y + 9} x2={x + 4} y2={y + cardH - 9}
          stroke={`hsl(${hue} 65% 55%)`}
          stroke-width="3" stroke-linecap="round"
          opacity={activated() ? 1 : isLiterary() ? 0.35 : 0.55}
        />
        {/* Tense name */}
        <text x={x + 11} y={y + 14 + fontSize.title * 0.35}
          font-size={String(fontSize.title)} font-weight="600"
          fill={`hsl(${hue} 65% 62%)`}
          opacity={isLiterary() ? 0.7 : 1}
        >
          {nodeTitle(node)}
        </text>
        {/* Conjugated form */}
        <text x={x + cardW / 2 + 3} y={y + cardH * 0.68}
          text-anchor="middle" font-size={String(fontSize.form)} font-weight="700"
          fill="var(--text)"
          opacity={isLiterary() ? 0.7 : 1}
        >
          {getConjugation(node.id)}
        </text>
        {/* Literary lock icon */}
        {isLiterary() && (
          <g opacity={0.55} transform={`translate(${x + cardW - 16}, ${y + 6})`}>
            <rect x="0" y="4" width="10" height="7" rx="1.5" fill="var(--text-2)" />
            <path d="M2.5 4V2.5a2.5 2.5 0 0 1 5 0V4" stroke="var(--text-2)" stroke-width="1.5" fill="none" stroke-linecap="round" />
          </g>
        )}
        {/* Beginner's start label */}
        {isPresent() && (
          <text x={x + cardW / 2} y={y + cardH + 10}
            text-anchor="middle" font-size="7"
            fill="var(--amber)"
            opacity={0.7}
          >
            start here
          </text>
        )}
      </g>
    );
  };

  // ── Legend ────────────────────────────────────────────────────
  const Legend = () => (
    <details class="diagram-legend">
      <summary>Key</summary>
      <div class="legend-section">
        <p class="legend-section-title">Difficulty</p>
        <div class="legend-grid">
          <div class="legend-item">
            <span class="legend-dot" style={{ background: "var(--amber)", opacity: "0.6" }} />
            <span>Présent — start here</span>
          </div>
          <div class="legend-item">
            <svg width="12" height="12" viewBox="0 0 16 16" fill="none" aria-hidden="true" style={{ opacity: "0.65", display: "inline-block", "vertical-align": "middle" }}>
              <rect x="3" y="7" width="10" height="8" rx="2" fill="var(--text-2)" />
              <path d="M5 7V5a3 3 0 0 1 6 0v2" stroke="var(--text-2)" stroke-width="1.5" fill="none" stroke-linecap="round" />
            </svg>
            <span>Literary tense — rare in speech</span>
          </div>
        </div>
      </div>
      <div class="legend-section">
        <p class="legend-section-title">Connections</p>
        <div class="legend-grid">
          {(Object.entries(EDGE_COLOR) as [keyof typeof EDGE_COLOR, string][]).map(([type, color]) => (
            <div class="legend-item">
              <svg width="32" height="10" aria-hidden="true">
                <line x1="0" y1="5" x2="32" y2="5"
                  stroke={color} stroke-width="2.5"
                  stroke-dasharray={EDGE_DASH[type]}
                  stroke-linecap="round"
                />
              </svg>
              <span>{type.replace(/-/g, " ")}</span>
            </div>
          ))}
        </div>
      </div>
    </details>
  );

  // ── Controls bar ──────────────────────────────────────────────
  const Controls = () => (
    <div class="diagram-controls">
      <div class="control-group">
        <span class="control-label">Verb</span>
        <select class="verb-select" value={selectedVerb()} onChange={e => setSelectedVerb(e.currentTarget.value)}>
          {VERB_OPTIONS.map(opt => <option value={opt.value}>{opt.label}</option>)}
        </select>
      </div>
      <div class="control-group">
        <span class="control-label">Person</span>
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

  // ── Render ────────────────────────────────────────────────────
  return (
    <div class="verb-diagram">
      <Controls />

      {/* ── MOBILE: transposed SVG (mood=col, time=row) ── */}
      <Show when={isMobile()}>
        <div class="diagram-scroll-wrap">
          <svg width={MB_SVG_W} height={MB_SVG_H}
            viewBox={`0 0 ${MB_SVG_W} ${MB_SVG_H}`}
            class="grammar-map-svg"
            role="img" aria-label="French verb tense diagram"
          >
            <defs>
              <marker id="mb-arr-aux" viewBox="0 0 8 6" refX="7" refY="3"
                markerWidth="5" markerHeight="4" orient="auto" markerUnits="userSpaceOnUse">
                <path d="M 0 0 L 8 3 L 0 6 Z" fill={EDGE_COLOR["auxiliary-compound"]} />
              </marker>
              <marker id="mb-arr-ant" viewBox="0 0 8 6" refX="7" refY="3"
                markerWidth="5" markerHeight="4" orient="auto" markerUnits="userSpaceOnUse">
                <path d="M 0 0 L 8 3 L 0 6 Z" fill={EDGE_COLOR["anteriority"]} />
              </marker>
            </defs>
            {/* Mood column backgrounds */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <rect
                  x={mbColX(i())} y={MB_MOOD_HEADER_H - 4}
                  width={MB_CARD_W} height={MB_SVG_H - MB_MOOD_HEADER_H + 4}
                  fill={`hsl(${MOOD_HUE[mood]} 55% 50% / 0.04)`} rx="4"
                />
              )}
            </For>

            {/* Mood column headers */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <text
                  x={mbColX(i()) + MB_CARD_W / 2} y={MB_MOOD_HEADER_H - 8}
                  text-anchor="middle" font-size="8" font-weight="700"
                  fill={`hsl(${MOOD_HUE[mood]} 60% 58%)`}
                >
                  {mood === "indicatif" ? "Ind." : mood === "conditionnel" ? "Cond." : mood === "subjonctif" ? "Subj." : "Imp."}
                </text>
              )}
            </For>

            {/* Time row labels */}
            {Object.entries(MB_TIME_LABELS).map(([pos, label]) => {
              const row = TIME_COLUMNS[pos as keyof typeof TIME_COLUMNS];
              return (
                <text
                  x={MB_TIME_LABEL_W - 4}
                  y={mbRowY(row) + MB_CARD_H / 2 + 3}
                  text-anchor="end" font-size="7.5" fill="var(--text-2)"
                >
                  {label}
                </text>
              );
            })}

            {/* Edges */}
            <For each={visibleEdges()}>
              {edge => {
                const from = mbPosMap().get(edge.from);
                const to   = mbPosMap().get(edge.to);
                if (!from || !to) return null;
                const isActive = () => activeEdge() === edge;
                const edgeColor = EDGE_COLOR[edge.type];
                const d = edgePathMobile(from, to, edge);
                return (
                  <g class="diagram-edge"
                    onClick={() => setActiveEdge(isActive() ? null : edge)}
                    style={{ cursor: "pointer" }}
                    role="button" tabIndex={0}
                    aria-label={`${edge.type}: ${edge.label}`}
                    onKeyDown={e => e.key === "Enter" && setActiveEdge(isActive() ? null : edge)}
                  >
                    <path d={d} fill="none" stroke={edgeColor}
                      stroke-width={isActive() ? 2.5 : 1.5}
                      stroke-dasharray={EDGE_DASH[edge.type]}
                      stroke-linecap="round"
                      opacity={isActive() ? 1 : 0.6}
                      marker-end={
                        edge.type === "auxiliary-compound" ? "url(#mb-arr-aux)"
                        : undefined
                      }
                    />
                    <path d={d} fill="none" stroke="transparent" stroke-width="12" />
                  </g>
                );
              }}
            </For>

            {/* Node cards */}
            <For each={mbNodePositions()}>
              {({ node, x, y }) => (
                <NodeCard node={node} x={x} y={y}
                  cardW={MB_CARD_W} cardH={MB_CARD_H}
                  fontSize={{ title: 8, form: 11 }}
                />
              )}
            </For>
          </svg>
        </div>
        <Panels />
        <Legend />
      </Show>

      {/* ── DESKTOP: horizontal SVG (time=col, mood=row) ── */}
      <Show when={!isMobile()}>
        <div class="diagram-scroll-wrap">
          <svg width={DK_SVG_W} height={DK_SVG_H}
            viewBox={`0 0 ${DK_SVG_W} ${DK_SVG_H}`}
            class="grammar-map-svg"
            role="img" aria-label="French verb tense diagram"
          >
            <defs>
              <marker id="dk-arr-aux" viewBox="0 0 8 6" refX="7" refY="3"
                markerWidth="6" markerHeight="5" orient="auto" markerUnits="userSpaceOnUse">
                <path d="M 0 0 L 8 3 L 0 6 Z" fill={EDGE_COLOR["auxiliary-compound"]} />
              </marker>
              <marker id="dk-arr-ant" viewBox="0 0 8 6" refX="7" refY="3"
                markerWidth="6" markerHeight="5" orient="auto" markerUnits="userSpaceOnUse">
                <path d="M 0 0 L 8 3 L 0 6 Z" fill={EDGE_COLOR["anteriority"]} />
              </marker>
            </defs>
            {/* Lane backgrounds */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <rect
                  x={DK_LANE_HEADER_W} y={dkLaneY(i()) - DK_LANE_GAP / 2}
                  width={DK_SVG_W - DK_LANE_HEADER_W} height={DK_LANE_H}
                  fill={`hsl(${MOOD_HUE[mood]} 55% 50% / 0.04)`} rx="4"
                />
              )}
            </For>

            {/* Column headers */}
            {(Object.entries({
              "far-past": "anterior", "past": "past", "near-past": "recent past",
              "present": "present", "near-future": "near future",
              "future": "future", "far-future": "future anterior",
            }) as [string, string][]).map(([pos, label]) => {
              const col = TIME_COLUMNS[pos as keyof typeof TIME_COLUMNS];
              return (
                <text x={dkColX(col) + DK_CARD_W / 2} y={DK_COL_HEADER_H - 7}
                  text-anchor="middle" font-size="9" fill="var(--text-2)">
                  {label}
                </text>
              );
            })}

            {/* Lane labels */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <text x={DK_LANE_HEADER_W - 8} y={dkLaneY(i()) + DK_CARD_H / 2 + 4}
                  text-anchor="end" font-size="10" font-weight="600"
                  fill={`hsl(${MOOD_HUE[mood]} 60% 55%)`}
                >
                  {MOOD_LABELS[mood]}
                </text>
              )}
            </For>

            {/* Edges */}
            <For each={visibleEdges()}>
              {edge => {
                const from = dkPosMap().get(edge.from);
                const to   = dkPosMap().get(edge.to);
                if (!from || !to) return null;
                const isActive = () => activeEdge() === edge;
                const edgeColor = EDGE_COLOR[edge.type];
                const d = edgePath(from, to, edge);
                return (
                  <g class="diagram-edge"
                    onClick={() => setActiveEdge(isActive() ? null : edge)}
                    style={{ cursor: "pointer" }}
                    role="button" tabIndex={0}
                    aria-label={`${edge.type}: ${edge.label}`}
                    onKeyDown={e => e.key === "Enter" && setActiveEdge(isActive() ? null : edge)}
                  >
                    <path d={d} fill="none" stroke={edgeColor}
                      stroke-width={isActive() ? 3 : 2}
                      stroke-dasharray={EDGE_DASH[edge.type]}
                      stroke-linecap="round"
                      opacity={isActive() ? 1 : 0.55}
                      marker-end={
                        edge.type === "auxiliary-compound" ? "url(#dk-arr-aux)"
                        : undefined
                      }
                    />
                    <path d={d} fill="none" stroke="transparent" stroke-width="16" />
                  </g>
                );
              }}
            </For>

            {/* Node cards */}
            <For each={dkNodePositions()}>
              {({ node, x, y }) => (
                <NodeCard node={node} x={x} y={y}
                  cardW={DK_CARD_W} cardH={DK_CARD_H}
                  fontSize={{ title: 10, form: 14 }}
                />
              )}
            </For>
          </svg>
        </div>
        <Panels />
        <Legend />
      </Show>
    </div>
  );
}
