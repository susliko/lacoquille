import { createSignal, createMemo, For, Show, onMount, onCleanup } from "solid-js";
import type { DiagramData, DiagramNode, DiagramEdge, MoodId } from "./GrammarMap/types";
import {
  TIME_COLUMNS, MOOD_LABELS, MOOD_ORDER, MOOD_HUE,
  EDGE_COLOR, EDGE_DASH, EDGE_LABEL,
} from "./GrammarMap/types";
import { AspectIcon } from "./GrammarMap/AspectIcon";
import { VERBS, PERSONS, VERB_OPTIONS } from "../data/conjugations/index";
import type { TenseId, Person } from "../data/conjugations/types";

// ── Layout constants ──────────────────────────────────────────
const CARD_W = 140;
const CARD_H = 62;        // shorter now — no description text
const COL_GAP = 16;
const LANE_GAP = 14;
const LANE_HEADER_W = 96;
const COL_HEADER_H = 28;
const PAD = 12;

const NUM_COLS = 7;
const COL_W = CARD_W + COL_GAP;
const LANE_H = CARD_H + LANE_GAP;

function colX(col: number) { return LANE_HEADER_W + PAD + col * COL_W; }
function laneY(row: number) { return COL_HEADER_H + PAD + row * LANE_H; }

const SVG_W = LANE_HEADER_W + PAD * 2 + NUM_COLS * COL_W;
const SVG_H = COL_HEADER_H + PAD * 2 + MOOD_ORDER.length * LANE_H;

interface NodePos { node: DiagramNode; x: number; y: number; cx: number; cy: number; }

interface Props {
  data: DiagramData;
  tenseTitles: Record<string, string>;
  tenseRules:  Record<string, string>;
}

// ── Verbal periphrases — computed from aller/venir/être ───────
const PERIPHRASES: Record<string, (verbInf: string, person: string) => string> = {
  "futur-proche": (inf, person) => {
    const f = VERBS["aller"]?.forms["present-indicatif"]?.[person as Person];
    return f ? `${f} ${inf}` : "—";
  },
  "passe-recent": (inf, person) => {
    // venir conjugated inline (common irregular)
    const venir: Record<string, string> = {
      je: "viens", tu: "viens", il: "vient",
      nous: "venons", vous: "venez", ils: "viennent",
    };
    return `${venir[person] ?? "—"} de ${inf}`;
  },
  "present-progressif": (inf, person) => {
    const etre = VERBS["être"]?.forms["present-indicatif"]?.[person as Person];
    return etre ? `${etre} en train de ${inf}` : "—";
  },
};

function edgePath(from: NodePos, to: NodePos, edge: DiagramEdge): string {
  const dy = to.cy - from.cy;

  // Cross-lane edges (mood-swap, stem-share): elbow from bottom/top of card
  if (edge.type === "mood-swap" || edge.type === "stem-share") {
    const midX = from.cx;
    const fromY = dy > 0 ? from.y + CARD_H : from.y;
    const toY   = dy > 0 ? to.y            : to.y + CARD_H;
    return `M ${from.cx} ${fromY} C ${midX} ${from.cy + dy * 0.5}, ${midX} ${from.cy + dy * 0.5}, ${to.cx} ${toY}`;
  }

  // Same-lane horizontal edges: offset slightly from center to avoid overlap
  if (Math.abs(dy) < 4) {
    const yOff = from.cy - 4;
    return `M ${from.x + CARD_W} ${yOff} L ${to.x} ${yOff}`;
  }

  // Generic diagonal
  const cp1y = from.cy + dy * 0.4;
  const cp2y = to.cy - dy * 0.4;
  return `M ${from.cx} ${from.cy} C ${from.cx} ${cp1y}, ${to.cx} ${cp2y}, ${to.cx} ${to.cy}`;
}

export default function VerbDiagram({ data, tenseTitles, tenseRules }: Props) {
  // ── Reactive state ────────────────────────────────────────
  const [selectedPerson, setSelectedPerson] = createSignal<string>("je");
  const [selectedVerb,   setSelectedVerb]   = createSignal<string>("parler");
  const [showLiterary,   setShowLiterary]   = createSignal(false);
  const [activeNode,     setActiveNode]     = createSignal<string | null>(null);
  const [activeEdge,     setActiveEdge]     = createSignal<DiagramEdge | null>(null);
  const [activeMood,     setActiveMood]     = createSignal<MoodId>("indicatif");
  const [isMobile,       setIsMobile]       = createSignal(
    typeof window !== "undefined" ? window.innerWidth < 768 : false
  );

  // Tier-1 linking: will be wired to example-sentence verb tokens
  const [highlightedTense, _setHighlightedTense] = createSignal<string | null>(null);

  onMount(() => {
    const check = () => setIsMobile(window.innerWidth < 768);
    window.addEventListener("resize", check);
    onCleanup(() => window.removeEventListener("resize", check));
  });

  // ── Derived ───────────────────────────────────────────────
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

  const nodePositions = createMemo<NodePos[]>(() =>
    visibleNodes().map(node => {
      const col = TIME_COLUMNS[node.timePosition];
      const row = MOOD_ORDER.indexOf(node.lane as MoodId);
      const x = colX(col);
      const y = laneY(row);
      return { node, x, y, cx: x + CARD_W / 2, cy: y + CARD_H / 2 };
    })
  );

  const posMap = createMemo(() => {
    const m = new Map<string, NodePos>();
    for (const p of nodePositions()) m.set(p.node.id, p);
    return m;
  });

  // Tenses in the active mood lane, sorted by time column
  const mobileLaneNodes = createMemo(() =>
    visibleNodes()
      .filter(n => n.lane === activeMood())
      .sort((a, b) => TIME_COLUMNS[a.timePosition] - TIME_COLUMNS[b.timePosition])
  );

  function getConjugation(tenseId: string): string {
    const verb   = VERBS[selectedVerb()];
    const person = selectedPerson();
    if (!verb) return "—";

    // Periphrastic tenses — computed, not stored
    if (tenseId in PERIPHRASES) {
      return PERIPHRASES[tenseId]!(verb.infinitive, person);
    }

    const forms = verb.forms[tenseId as TenseId];
    if (!forms) return "—";
    return forms[person as keyof typeof forms] ?? "—";
  }

  const activeNodeData = createMemo<DiagramNode | null>(() =>
    activeNode() ? (data.nodes.find(n => n.id === activeNode()) ?? null) : null
  );

  function nodeTitle(node: DiagramNode): string {
    return tenseTitles[node.id] ?? (node as any).title ?? node.id;
  }

  // ── Shared panels (used by both desktop + mobile) ─────────
  const Panels = () => (
    <>
      <Show when={activeEdge()}>
        {edge => (
          <aside class="edge-panel" role="complementary" aria-live="polite">
            <button class="panel-close" onClick={() => setActiveEdge(null)} aria-label="Close">✕</button>
            <strong class="panel-type-tag">{edge().type.replace(/-/g, " ")}</strong>
            <p style={{ color: EDGE_COLOR[edge().type], "font-weight": "500" }}>
              {EDGE_LABEL[edge().type]}
            </p>
            <p style={{ "margin-top": "0.4rem", color: "var(--text-2)", "font-size": "0.9rem" }}>
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
          const verb    = () => VERBS[selectedVerb()];
          const forms   = () => {
            if (tenseId() in PERIPHRASES) return null; // periphrases shown differently
            return verb()?.forms[tenseId() as TenseId];
          };

          return (
            <aside class="node-panel" role="complementary" aria-live="polite">
              <button class="panel-close" onClick={() => setActiveNode(null)} aria-label="Close">✕</button>
              <h3 class="panel-title">{nodeTitle(nodeData())}</h3>
              <p class="panel-rule">{tenseRules[tenseId()] ?? ""}</p>

              <Show when={forms()}>
                {f => (
                  <table class="panel-conj-table">
                    <tbody>
                      {PERSONS.map(person => {
                        const form = f()[person];
                        if (form === null) return null;
                        const active = () => selectedPerson() === person;
                        return (
                          <tr
                            class={active() ? "active-row" : ""}
                            onClick={() => setSelectedPerson(person)}
                            style={{ cursor: "pointer" }}
                          >
                            <th scope="row">{person}</th>
                            <td>{form}</td>
                          </tr>
                        );
                      })}
                    </tbody>
                  </table>
                )}
              </Show>

              <Show when={tenseId() in PERIPHRASES}>
                <table class="panel-conj-table">
                  <tbody>
                    {PERSONS.map(person => {
                      const form = getConjugation(tenseId());
                      const verbObj = VERBS[selectedVerb()];
                      const active = () => selectedPerson() === person;
                      const personForm = PERIPHRASES[tenseId()]?.(verbObj?.infinitive ?? "", person) ?? "—";
                      return (
                        <tr class={active() ? "active-row" : ""} onClick={() => setSelectedPerson(person)} style={{ cursor: "pointer" }}>
                          <th scope="row">{person}</th>
                          <td>{personForm}</td>
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </Show>

              <a href={`/verbs/tenses/${tenseId()}`} class="panel-learn-more">
                Full reference page →
              </a>
            </aside>
          );
        }}
      </Show>
    </>
  );

  // ── Mobile tense card ─────────────────────────────────────
  const MobileCard = (node: DiagramNode) => {
    const hue   = MOOD_HUE[node.lane as MoodId];
    const isActive = () => activeNode() === node.id;
    return (
      <button
        class={`tense-card-mobile${isActive() ? " active" : ""}`}
        style={{ "--hue": hue } as any}
        onClick={() => {
          setActiveNode(isActive() ? null : node.id);
          setActiveEdge(null);
        }}
        type="button"
        aria-expanded={isActive()}
      >
        <div class="tcm-stripe" />
        <div class="tcm-body">
          <div class="tcm-header">
            <span class="tcm-title">{nodeTitle(node)}</span>
            <span class="tcm-aspect">
              <AspectIcon aspect={node.aspect} size={13} color={`hsl(${hue} 55% 55%)`} />
            </span>
          </div>
          <div class="tcm-form">{getConjugation(node.id)}</div>
          <div class="tcm-rule">{tenseRules[node.id] ?? ""}</div>
        </div>
      </button>
    );
  };

  // ── Render ─────────────────────────────────────────────────
  return (
    <div class="verb-diagram">
      {/* Controls — shared */}
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
              <button
                class={`person-pill${selectedPerson() === p ? " active" : ""}`}
                onClick={() => setSelectedPerson(p)}
                type="button"
              >{p}</button>
            ))}
          </div>
        </div>

        <label class="literary-toggle">
          <input type="checkbox" checked={showLiterary()} onChange={e => setShowLiterary(e.currentTarget.checked)} />
          <span class="literary-toggle-label">Literary tenses</span>
        </label>
      </div>

      {/* ── MOBILE VIEW ── */}
      <Show when={isMobile()}>
        <div class="diagram-mobile">
          {/* Mood tabs */}
          <div class="mood-tabs" role="tablist">
            <For each={MOOD_ORDER}>
              {mood => (
                <button
                  class={`mood-tab${activeMood() === mood ? " active" : ""}`}
                  style={{ "--hue": MOOD_HUE[mood] } as any}
                  role="tab"
                  aria-selected={activeMood() === mood}
                  onClick={() => setActiveMood(mood)}
                  type="button"
                >{MOOD_LABELS[mood]}</button>
              )}
            </For>
          </div>

          {/* Tense cards for active mood */}
          <div class="tense-cards-mobile" role="tabpanel">
            <For each={mobileLaneNodes()}>
              {node => <MobileCard {...node} />}
            </For>
          </div>

          <Panels />

          {/* Mobile legend */}
          <details class="diagram-legend">
            <summary>Connection types</summary>
            <div class="legend-grid">
              <For each={Object.entries(EDGE_COLOR) as [keyof typeof EDGE_COLOR, string][]}>
                {([type, color]) => (
                  <div class="legend-item">
                    <svg width="28" height="10" aria-hidden="true">
                      <line x1="0" y1="5" x2="28" y2="5"
                        stroke={color} stroke-width="2.5"
                        stroke-dasharray={EDGE_DASH[type]}
                        stroke-linecap="round"
                      />
                    </svg>
                    <span>{type.replace(/-/g, " ")}</span>
                  </div>
                )}
              </For>
            </div>
          </details>
        </div>
      </Show>

      {/* ── DESKTOP SVG VIEW ── */}
      <Show when={!isMobile()}>
        <div class="diagram-scroll-wrap">
          <svg
            width={SVG_W} height={SVG_H}
            viewBox={`0 0 ${SVG_W} ${SVG_H}`}
            class="grammar-map-svg"
            role="img"
            aria-label="French verb tense diagram"
          >
            {/* Lane backgrounds */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <rect
                  x={LANE_HEADER_W} y={laneY(i()) - LANE_GAP / 2}
                  width={SVG_W - LANE_HEADER_W} height={LANE_H}
                  fill={`hsl(${MOOD_HUE[mood]} 55% 50% / 0.04)`}
                  rx="4"
                />
              )}
            </For>

            {/* Column headers */}
            <For each={Object.entries({
              "far-past": "anterior", "past": "past", "near-past": "recent past",
              "present": "present", "near-future": "near future",
              "future": "future", "far-future": "future anterior",
            })}>
              {([pos, label]) => {
                const col = TIME_COLUMNS[pos as keyof typeof TIME_COLUMNS];
                return (
                  <text x={colX(col) + CARD_W / 2} y={COL_HEADER_H - 7}
                    text-anchor="middle" font-size="9" fill="var(--text-2)">
                    {label}
                  </text>
                );
              }}
            </For>

            {/* Lane labels */}
            <For each={MOOD_ORDER}>
              {(mood, i) => (
                <text
                  x={LANE_HEADER_W - 8} y={laneY(i()) + CARD_H / 2 + 4}
                  text-anchor="end" font-size="10" font-weight="600"
                  fill={`hsl(${MOOD_HUE[mood]} 60% 55%)`}
                >
                  {MOOD_LABELS[mood]}
                </text>
              )}
            </For>

            {/* Edges — colored by type */}
            <For each={visibleEdges()}>
              {edge => {
                const from = posMap().get(edge.from);
                const to   = posMap().get(edge.to);
                if (!from || !to) return null;
                const isActive = () => activeEdge() === edge;
                const edgeColor = EDGE_COLOR[edge.type];
                const d = edgePath(from, to, edge);
                return (
                  <g
                    class="diagram-edge"
                    onClick={() => setActiveEdge(isActive() ? null : edge)}
                    style={{ cursor: "pointer" }}
                    role="button" tabIndex={0}
                    aria-label={`${edge.type}: ${edge.label}`}
                    onKeyDown={e => e.key === "Enter" && setActiveEdge(isActive() ? null : edge)}
                  >
                    <path d={d} fill="none"
                      stroke={edgeColor}
                      stroke-width={isActive() ? 3 : 2}
                      stroke-dasharray={EDGE_DASH[edge.type]}
                      stroke-linecap="round"
                      opacity={isActive() ? 1 : 0.55}
                    />
                    {/* Wide invisible hit target */}
                    <path d={d} fill="none" stroke="transparent" stroke-width="16" />
                  </g>
                );
              }}
            </For>

            {/* Node cards */}
            <For each={nodePositions()}>
              {({ node, x, y }) => {
                const isActive     = () => activeNode() === node.id;
                const isHighlighted = () => highlightedTense() === node.id;
                const hue = MOOD_HUE[node.lane as MoodId];
                const activated = () => isActive() || isHighlighted();

                return (
                  <g
                    class="diagram-node"
                    onClick={() => { setActiveNode(isActive() ? null : node.id); setActiveEdge(null); }}
                    style={{ cursor: "pointer" }}
                    role="button" tabIndex={0}
                    aria-label={`${nodeTitle(node)}: ${getConjugation(node.id)}`}
                    onKeyDown={e => e.key === "Enter" && setActiveNode(isActive() ? null : node.id)}
                  >
                    {/* Card background */}
                    <rect
                      x={x} y={y} width={CARD_W} height={CARD_H} rx="7"
                      fill={activated() ? `hsl(${hue} 50% 45% / 0.14)` : "var(--surface-2)"}
                      stroke={activated() ? `hsl(${hue} 65% 55%)` : `hsl(${hue} 40% 50% / 0.35)`}
                      stroke-width={activated() ? 1.5 : 1}
                    />
                    {/* Left accent stripe */}
                    <line
                      x1={x + 4} y1={y + 10} x2={x + 4} y2={y + CARD_H - 10}
                      stroke={`hsl(${hue} 65% 55%)`}
                      stroke-width="3"
                      stroke-linecap="round"
                      opacity={activated() ? 1 : 0.6}
                    />
                    {/* Tense name */}
                    <text x={x + 14} y={y + 17}
                      font-size="10" font-weight="600"
                      fill={`hsl(${hue} 60% 60%)`}
                    >
                      {nodeTitle(node)}
                    </text>
                    {/* Conjugated form */}
                    <text x={x + CARD_W / 2 + 4} y={y + 43}
                      text-anchor="middle" font-size="15" font-weight="700"
                      fill={activated() ? "var(--text)" : "var(--text)"}
                    >
                      {getConjugation(node.id)}
                    </text>
                    {/* Aspect icon */}
                    <g transform={`translate(${x + CARD_W - 18}, ${y + 7})`}>
                      <AspectIcon aspect={node.aspect} size={11} color={`hsl(${hue} 55% 55%)`} />
                    </g>
                  </g>
                );
              }}
            </For>
          </svg>
        </div>

        <Panels />

        {/* Desktop legend */}
        <details class="diagram-legend">
          <summary>Connection types</summary>
          <div class="legend-grid">
            <For each={Object.entries(EDGE_COLOR) as [keyof typeof EDGE_COLOR, string][]}>
              {([type, color]) => (
                <div class="legend-item">
                  <svg width="36" height="10" aria-hidden="true">
                    <line x1="0" y1="5" x2="36" y2="5"
                      stroke={color} stroke-width="2.5"
                      stroke-dasharray={EDGE_DASH[type]}
                      stroke-linecap="round"
                    />
                  </svg>
                  <span>{type.replace(/-/g, " ")}</span>
                </div>
              )}
            </For>
          </div>
        </details>
      </Show>
    </div>
  );
}
