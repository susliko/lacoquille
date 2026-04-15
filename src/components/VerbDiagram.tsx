import { createSignal, createMemo, For, Show } from "solid-js";
import type { DiagramData, DiagramNode, DiagramEdge, MoodId } from "./GrammarMap/types";
import {
  TIME_COLUMNS, TIME_LABELS, MOOD_LABELS, MOOD_ORDER, MOOD_HUE, EDGE_DASH,
} from "./GrammarMap/types";
import { AspectIcon } from "./GrammarMap/AspectIcon";
import { VERBS, PERSONS } from "../data/conjugations/index";
import type { TenseId } from "../data/conjugations/types";

// ── Layout constants ──────────────────────────────────────────
const CARD_W = 130;
const CARD_H = 76;
const COL_GAP = 24;
const LANE_GAP = 20;
const LANE_HEADER_W = 100;
const COL_HEADER_H = 32;
const PAD = 16;

const NUM_COLS = 7; // far-past … far-future
const COL_W = CARD_W + COL_GAP;
const LANE_H = CARD_H + LANE_GAP;

function colX(col: number) { return LANE_HEADER_W + PAD + col * COL_W; }
function laneY(row: number) { return COL_HEADER_H + PAD + row * LANE_H; }

const SVG_W = LANE_HEADER_W + PAD * 2 + NUM_COLS * COL_W;
const SVG_H = COL_HEADER_H + PAD * 2 + MOOD_ORDER.length * LANE_H;

// ── Types ─────────────────────────────────────────────────────
interface NodePos { node: DiagramNode; x: number; y: number; cx: number; cy: number; }

interface Props {
  data: DiagramData;
  /** tense titles keyed by slug, loaded from content collection */
  tenseTitles: Record<string, string>;
  /** one-line rules keyed by slug */
  tenseRules: Record<string, string>;
}

export default function VerbDiagram({ data, tenseTitles, tenseRules }: Props) {
  // ── Global controls ───────────────────────────────────────
  const [selectedPerson, setSelectedPerson] = createSignal<string>("je");
  const [selectedVerb, setSelectedVerb] = createSignal<string>("parler");
  const [showLiterary, setShowLiterary] = createSignal(false);
  const [activeNode, setActiveNode] = createSignal<string | null>(null);
  const [activeEdge, setActiveEdge] = createSignal<DiagramEdge | null>(null);
  // Tier-1 linking: sentence verb-tokens will call this to highlight the owning node
  const [highlightedTense, _setHighlightedTense] = createSignal<string | null>(null);

  // ── Derived data ──────────────────────────────────────────
  const visibleNodes = createMemo(() =>
    data.nodes.filter(n => showLiterary() || !n.literary)
  );

  const visibleEdges = createMemo(() =>
    data.edges.filter(e => {
      if (e.literary && !showLiterary()) return false;
      const fromVisible = visibleNodes().some(n => n.id === e.from);
      const toVisible = visibleNodes().some(n => n.id === e.to);
      return fromVisible && toVisible;
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

  function getPosById(id: string): NodePos | undefined {
    return nodePositions().find(p => p.node.id === id);
  }

  function getConjugation(tenseId: string): string {
    const verb = VERBS[selectedVerb()];
    if (!verb) return "—";
    const forms = verb.forms[tenseId as TenseId];
    if (!forms) return "—";
    const person = selectedPerson() as keyof typeof forms;
    return forms[person] ?? "—";
  }

  const activeNodeData = createMemo<DiagramNode | null>(() =>
    activeNode() ? (data.nodes.find(n => n.id === activeNode()) ?? null) : null
  );

  // ── Render ────────────────────────────────────────────────
  return (
    <div class="verb-diagram">
      {/* ── Controls bar ── */}
      <div class="diagram-controls">
        <fieldset class="inline-controls">
          <label>
            Verb
            <select value={selectedVerb()} onInput={e => setSelectedVerb(e.currentTarget.value)}>
              {Object.keys(VERBS).map(v => <option value={v}>{v}</option>)}
            </select>
          </label>
          <label>
            Person
            <select value={selectedPerson()} onInput={e => setSelectedPerson(e.currentTarget.value)}>
              {PERSONS.map(p => <option value={p}>{p}</option>)}
            </select>
          </label>
          <label class="inline-checkbox">
            <input type="checkbox" role="switch"
              checked={showLiterary()}
              onChange={e => setShowLiterary(e.currentTarget.checked)}
            />
            Literary tenses
          </label>
        </fieldset>
      </div>

      {/* ── SVG diagram ── */}
      <div class="diagram-scroll-container">
        <svg
          width={SVG_W}
          height={SVG_H}
          viewBox={`0 0 ${SVG_W} ${SVG_H}`}
          class="grammar-map-svg"
          aria-label="French verb tense diagram"
        >
          {/* Column headers (time labels) */}
          {Object.entries(TIME_LABELS).map(([pos, label]) => {
            const col = TIME_COLUMNS[pos as keyof typeof TIME_COLUMNS];
            const x = colX(col) + CARD_W / 2;
            return (
              <text
                x={x} y={COL_HEADER_H - 6}
                text-anchor="middle" font-size="10"
                fill="var(--pico-muted-color)"
                class="col-label"
              >
                {label}
              </text>
            );
          })}

          {/* Lane header labels */}
          <For each={MOOD_ORDER}>
            {(mood, i) => (
              <text
                x={LANE_HEADER_W - 8}
                y={laneY(i()) + CARD_H / 2 + 4}
                text-anchor="end" font-size="11" font-weight="600"
                fill={`hsl(${MOOD_HUE[mood]} 55% 50%)`}
                class="lane-label"
              >
                {MOOD_LABELS[mood]}
              </text>
            )}
          </For>

          {/* Lane background stripes */}
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

          {/* Edges */}
          <For each={visibleEdges()}>
            {edge => {
              const from = getPosById(edge.from);
              const to = getPosById(edge.to);
              if (!from || !to) return null;
              const isActive = activeEdge() === edge;
              const color = isActive
                ? "hsl(210 80% 55%)"
                : "var(--pico-muted-color)";
              return (
                <g
                  class="diagram-edge"
                  onClick={() => setActiveEdge(isActive ? null : edge)}
                  style={{ cursor: "pointer" }}
                  aria-label={`${edge.from} → ${edge.to}: ${edge.label}`}
                  role="button"
                  tabIndex={0}
                  onKeyDown={e => e.key === "Enter" && setActiveEdge(isActive ? null : edge)}
                >
                  <line
                    x1={from.cx} y1={from.cy}
                    x2={to.cx} y2={to.cy}
                    stroke={color}
                    stroke-width={isActive ? 3 : 2}
                    stroke-dasharray={EDGE_DASH[edge.type]}
                    stroke-linecap="round"
                    opacity={isActive ? 1 : 0.5}
                  />
                  {/* Invisible wider click target */}
                  <line
                    x1={from.cx} y1={from.cy}
                    x2={to.cx} y2={to.cy}
                    stroke="transparent"
                    stroke-width="12"
                  />
                </g>
              );
            }}
          </For>

          {/* Node cards */}
          <For each={nodePositions()}>
            {({ node, x, y }) => {
              const isActive = () => activeNode() === node.id;
              const isHighlighted = () => highlightedTense() === node.id;
              const hue = MOOD_HUE[node.lane as MoodId];
              const conjForm = () => getConjugation(node.id);
              const title = tenseTitles[node.id] ?? node.id;

              return (
                <g
                  class="diagram-node"
                  onClick={() => {
                    setActiveNode(isActive() ? null : node.id);
                    setActiveEdge(null);
                  }}
                  style={{ cursor: "pointer" }}
                  role="button"
                  tabIndex={0}
                  aria-label={`${title}: ${conjForm()}`}
                  onKeyDown={e => e.key === "Enter" && setActiveNode(isActive() ? null : node.id)}
                >
                  {/* Card background */}
                  <rect
                    x={x} y={y}
                    width={CARD_W} height={CARD_H}
                    rx="8"
                    fill={isActive() || isHighlighted()
                      ? `hsl(${hue} 55% 50% / 0.15)`
                      : "var(--pico-card-background-color, #fff)"}
                    stroke={isActive() || isHighlighted()
                      ? `hsl(${hue} 60% 50%)`
                      : `hsl(${hue} 40% 70% / 0.5)`}
                    stroke-width={isActive() || isHighlighted() ? 2 : 1.5}
                  />

                  {/* Tense name */}
                  <text
                    x={x + 8} y={y + 18}
                    font-size="10" font-weight="600"
                    fill={`hsl(${hue} 50% 35%)`}
                  >
                    {title}
                  </text>

                  {/* Conjugated form (reactive) */}
                  <text
                    x={x + CARD_W / 2} y={y + 44}
                    text-anchor="middle"
                    font-size="15" font-weight="700"
                    fill="var(--pico-color)"
                  >
                    {selectedPerson()} {conjForm()}
                  </text>

                  {/* Aspect icon */}
                  <g transform={`translate(${x + CARD_W - 22}, ${y + 6})`}>
                    <AspectIcon aspect={node.aspect} size={14} color={`hsl(${hue} 50% 50%)`} />
                  </g>

                  {/* One-line rule */}
                  <text
                    x={x + CARD_W / 2} y={y + CARD_H - 8}
                    text-anchor="middle"
                    font-size="9"
                    fill="var(--pico-muted-color)"
                  >
                    {(tenseRules[node.id] ?? "").slice(0, 45)}
                  </text>
                </g>
              );
            }}
          </For>
        </svg>
      </div>

      {/* ── Active edge explanation ── */}
      <Show when={activeEdge()}>
        {edge => (
          <aside class="edge-panel" role="complementary" aria-live="polite">
            <button class="close-btn" onClick={() => setActiveEdge(null)} aria-label="Close">✕</button>
            <strong class="edge-type-label">{edge().type.replace("-", " ")}</strong>
            <p>{edge().label}</p>
            <p class="edge-nodes">
              <a href={`/verbs/tenses/${edge().from}`}>{tenseTitles[edge().from] ?? edge().from}</a>
              {" ↔ "}
              <a href={`/verbs/tenses/${edge().to}`}>{tenseTitles[edge().to] ?? edge().to}</a>
            </p>
          </aside>
        )}
      </Show>

      {/* ── Active node detail panel ── */}
      <Show when={activeNodeData()}>
        {(nodeData: () => DiagramNode) => {
          const tenseId = () => nodeData().id;
          const verb = () => VERBS[selectedVerb()];
          const forms = () => verb()?.forms[tenseId() as TenseId];

          return (
            <aside class="node-panel" role="complementary" aria-live="polite">
              <button class="close-btn" onClick={() => setActiveNode(null)} aria-label="Close">✕</button>
              <h3>{tenseTitles[tenseId()] ?? tenseId()}</h3>
              <p class="one-line-rule">{tenseRules[tenseId()] ?? ""}</p>

              {/* Conjugation mini-table */}
              <Show when={forms()}>
                {f => (
                  <table class="conj-table">
                    <tbody>
                      <For each={PERSONS}>
                        {person => {
                          const form = f()[person];
                          if (form === null) return null;
                          const isSelected = () => selectedPerson() === person;
                          return (
                            <tr
                              class={isSelected() ? "highlighted-row" : ""}
                              onClick={() => setSelectedPerson(person)}
                              style={{ cursor: "pointer" }}
                            >
                              <th scope="row">{person}</th>
                              <td><strong>{form}</strong></td>
                            </tr>
                          );
                        }}
                      </For>
                    </tbody>
                  </table>
                )}
              </Show>

              <a href={`/verbs/tenses/${tenseId()}`} class="learn-more">
                Full reference page →
              </a>
            </aside>
          );
        }}
      </Show>

      {/* ── Legend ── */}
      <details class="diagram-legend">
        <summary>Legend</summary>
        <dl>
          <dt><svg width="32" height="8"><line x1="0" y1="4" x2="32" y2="4" stroke="currentColor" stroke-width="2" /></svg></dt>
          <dd>Auxiliary-compound (built from another tense)</dd>
          <dt><svg width="32" height="8"><line x1="0" y1="4" x2="32" y2="4" stroke="currentColor" stroke-width="2" stroke-dasharray={EDGE_DASH["aspect-pair"]} /></svg></dt>
          <dd>Aspect pair (ongoing vs completed)</dd>
          <dt><svg width="32" height="8"><line x1="0" y1="4" x2="32" y2="4" stroke="currentColor" stroke-width="2" stroke-dasharray={EDGE_DASH["stem-share"]} /></svg></dt>
          <dd>Stem share</dd>
          <dt><svg width="32" height="8"><line x1="0" y1="4" x2="32" y2="4" stroke="currentColor" stroke-width="2" stroke-dasharray={EDGE_DASH["mood-swap"]} /></svg></dt>
          <dd>Mood swap</dd>
          <dt><svg width="32" height="8"><line x1="0" y1="4" x2="32" y2="4" stroke="currentColor" stroke-width="2" stroke-dasharray={EDGE_DASH["anteriority"]} /></svg></dt>
          <dd>Anteriority</dd>
        </dl>
      </details>
    </div>
  );
}
