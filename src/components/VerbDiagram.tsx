import { createSignal, createMemo, For, Show } from "solid-js";
import type { DiagramData, DiagramNode, DiagramEdge, MoodId } from "./GrammarMap/types";
import {
  TIME_COLUMNS, TIME_LABELS, MOOD_LABELS, MOOD_ORDER, MOOD_HUE, EDGE_DASH,
} from "./GrammarMap/types";
import { AspectIcon } from "./GrammarMap/AspectIcon";
import { VERBS, PERSONS, VERB_OPTIONS } from "../data/conjugations/index";
import type { TenseId } from "../data/conjugations/types";

const CARD_W = 140;
const CARD_H = 72;
const COL_GAP = 16;
const LANE_GAP = 12;
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
  tenseRules: Record<string, string>;
}

function edgePath(from: NodePos, to: NodePos, edge: DiagramEdge): string {
  const dx = to.cx - from.cx;
  const dy = to.cy - from.cy;

  if (edge.type === "mood-swap" || edge.type === "stem-share") {
    const midX = from.cx;
    return `M ${from.cx} ${from.y + CARD_H} C ${midX} ${from.cy + dy * 0.5}, ${midX} ${from.cy + dy * 0.5}, ${to.cx} ${to.y}`;
  }

  if (Math.abs(dy) > LANE_H * 0.5 && Math.abs(dx) > COL_W * 0.5) {
    const cp1x = from.cx;
    const cp1y = from.cy + dy * 0.4;
    const cp2x = to.cx;
    const cp2y = to.cy - dy * 0.4;
    return `M ${from.cx} ${from.cy} C ${cp1x} ${cp1y}, ${cp2x} ${cp2y}, ${to.cx} ${to.cy}`;
  }

  if (Math.abs(dy) <= 1) {
    const y = from.cy + (dy > 0 ? 4 : -4);
    return `M ${from.x + CARD_W} ${y} L ${to.x} ${y}`;
  }

  return `M ${from.cx} ${from.cy} L ${to.cx} ${to.cy}`;
}

export default function VerbDiagram({ data, tenseTitles, tenseRules }: Props) {
  const [selectedPerson, setSelectedPerson] = createSignal<string>("je");
  const [selectedVerb, setSelectedVerb] = createSignal<string>("parler");
  const [showLiterary, setShowLiterary] = createSignal(false);
  const [activeNode, setActiveNode] = createSignal<string | null>(null);
  const [activeEdge, setActiveEdge] = createSignal<DiagramEdge | null>(null);
  // Tier-1 linking: populated when user taps a verb token in an example sentence
  const [highlightedTense, _setHighlightedTense] = createSignal<string | null>(null);

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

  function nodeTitle(node: DiagramNode): string {
    return tenseTitles[node.id] ?? (node as any).title ?? node.id;
  }

  return (
    <div class="verb-diagram">
      <div class="diagram-controls">
        <div class="control-group">
          <span class="control-label">Verb</span>
          <select
            class="verb-select"
            value={selectedVerb()}
            onChange={e => setSelectedVerb(e.currentTarget.value)}
          >
            {VERB_OPTIONS.map(opt => (
              <option value={opt.value}>{opt.label}</option>
            ))}
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
          <input
            type="checkbox"
            checked={showLiterary()}
            onChange={e => setShowLiterary(e.currentTarget.checked)}
          />
          <span class="literary-toggle-label">Literary tenses</span>
        </label>
      </div>

      <div class="diagram-scroll-wrap">
        <svg
          width={SVG_W}
          height={SVG_H}
          viewBox={`0 0 ${SVG_W} ${SVG_H}`}
          class="grammar-map-svg"
          role="img"
          aria-label="French verb tense diagram"
        >
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

          <For each={Object.entries(TIME_LABELS)}>
            {([pos, label]) => {
              const col = TIME_COLUMNS[pos as keyof typeof TIME_COLUMNS];
              const x = colX(col) + CARD_W / 2;
              return (
                <text
                  x={x} y={COL_HEADER_H - 6}
                  text-anchor="middle" font-size="9"
                  fill="var(--text-2)"
                  class="col-label"
                >
                  {label}
                </text>
              );
            }}
          </For>

          <For each={MOOD_ORDER}>
            {(mood, i) => (
              <text
                x={LANE_HEADER_W - 6}
                y={laneY(i()) + CARD_H / 2 + 4}
                text-anchor="end" font-size="10" font-weight="600"
                fill={`hsl(${MOOD_HUE[mood]} 55% 50%)`}
                class="lane-label"
              >
                {MOOD_LABELS[mood]}
              </text>
            )}
          </For>

          <For each={visibleEdges()}>
            {edge => {
              const from = posMap().get(edge.from);
              const to = posMap().get(edge.to);
              if (!from || !to) return null;
              const isActive = () => activeEdge() === edge;
              const color = () => isActive() ? "hsl(210 80% 55%)" : "var(--text-2)";
              const d = edgePath(from, to, edge);
              return (
                <g
                  class="diagram-edge"
                  onClick={() => setActiveEdge(isActive() ? null : edge)}
                  style={{ cursor: "pointer" }}
                  aria-label={`${edge.from} to ${edge.to}: ${edge.label}`}
                  role="button"
                  tabIndex={0}
                  onKeyDown={e => e.key === "Enter" && setActiveEdge(isActive() ? null : edge)}
                >
                  <path
                    d={d}
                    fill="none"
                    stroke={color()}
                    stroke-width={isActive() ? 2.5 : 1.5}
                    stroke-dasharray={EDGE_DASH[edge.type]}
                    stroke-linecap="round"
                    opacity={isActive() ? 1 : 0.4}
                  />
                  <path
                    d={d}
                    fill="none"
                    stroke="transparent"
                    stroke-width="14"
                  />
                </g>
              );
            }}
          </For>

          <For each={nodePositions()}>
            {({ node, x, y }) => {
              const isActive = () => activeNode() === node.id;
              const isHighlighted = () => highlightedTense() === node.id;
              const hue = MOOD_HUE[node.lane as MoodId];
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
                  aria-label={`${nodeTitle(node)}: ${getConjugation(node.id)}`}
                  onKeyDown={e => e.key === "Enter" && setActiveNode(isActive() ? null : node.id)}
                >
                  <rect
                    x={x} y={y}
                    width={CARD_W} height={CARD_H}
                    rx="6"
                    fill={isActive() || isHighlighted() ? `hsl(${hue} 55% 50% / 0.12)` : "var(--surface-2)"}
                    stroke={isActive() || isHighlighted()
                      ? `hsl(${hue} 60% 50%)`
                      : `hsl(${hue} 40% 70% / 0.5)`}
                    stroke-width={isActive() || isHighlighted() ? 2 : 1}
                  />

                  <text
                    x={x + CARD_W / 2} y={y + 16}
                    text-anchor="middle" font-size="10" font-weight="600"
                    fill={`hsl(${hue} 50% 40%)`}
                  >
                    {nodeTitle(node)}
                  </text>

                  <text
                    x={x + CARD_W / 2} y={y + 42}
                    text-anchor="middle"
                    font-size="14" font-weight="700"
                    fill="var(--text)"
                    class="conj-form-text"
                  >
                    {getConjugation(node.id)}
                  </text>

                  <g transform={`translate(${x + CARD_W - 20}, ${y + 5})`}>
                    <AspectIcon aspect={node.aspect} size={12} color={`hsl(${hue} 50% 50%)`} />
                  </g>

                  <text
                    x={x + CARD_W / 2} y={y + CARD_H - 6}
                    text-anchor="middle"
                    font-size="8"
                    fill="var(--text-2)"
                  >
                    {(tenseRules[node.id] ?? "").slice(0, 38)}
                  </text>
                </g>
              );
            }}
          </For>
        </svg>
      </div>

      <Show when={activeEdge()}>
        {edge => (
          <aside class="edge-panel" role="complementary" aria-live="polite">
            <button class="panel-close" onClick={() => setActiveEdge(null)} aria-label="Close">&times;</button>
            <strong class="panel-type-tag">{edge().type.replace(/-/g, " ")}</strong>
            <p>{edge().label}</p>
            <p class="panel-edge-nodes">
              <a href={`/verbs/tenses/${edge().from}`}>{nodeTitle(data.nodes.find(n => n.id === edge().from)!)} </a>
              {" ↔ "}
              <a href={`/verbs/tenses/${edge().to}`}>{nodeTitle(data.nodes.find(n => n.id === edge().to)!)}</a>
            </p>
          </aside>
        )}
      </Show>

      <Show when={activeNodeData()}>
        {(nodeData: () => DiagramNode) => {
          const tenseId = () => nodeData().id;
          const verb = () => VERBS[selectedVerb()];
          const forms = () => verb()?.forms[tenseId() as TenseId];

          return (
            <aside class="node-panel" role="complementary" aria-live="polite">
              <button class="panel-close" onClick={() => setActiveNode(null)} aria-label="Close">&times;</button>
              <h3>{nodeTitle(nodeData())}</h3>
              <p class="panel-rule">{tenseRules[tenseId()] ?? ""}</p>

              <Show when={forms()}>
                {f => (
                  <table class="panel-conj-table">
                    <tbody>
                      <For each={PERSONS}>
                        {person => {
                          const form = f()[person];
                          if (form === null) return null;
                          const isSelected = () => selectedPerson() === person;
                          return (
                            <tr
                              class={isSelected() ? "active-row" : ""}
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

              <a href={`/verbs/tenses/${tenseId()}`} class="panel-learn-more">
                Full reference page &rarr;
              </a>
            </aside>
          );
        }}
      </Show>

      <details class="diagram-legend">
        <summary>Legend</summary>
        <dl>
          <For each={[
            { type: "auxiliary-compound" as const, label: "Auxiliary-compound (built from another tense)" },
            { type: "aspect-pair" as const, label: "Aspect pair (ongoing vs completed)" },
            { type: "stem-share" as const, label: "Stem share" },
            { type: "mood-swap" as const, label: "Mood swap" },
            { type: "anteriority" as const, label: "Anteriority" },
          ]}>
            {item => (
              <>
                <dt>
                  <svg width="32" height="8"><line x1="0" y1="4" x2="32" y2="4" stroke="currentColor" stroke-width="2" stroke-dasharray={EDGE_DASH[item.type]} /></svg>
                </dt>
                <dd>{item.label}</dd>
              </>
            )}
          </For>
        </dl>
      </details>
    </div>
  );
}
