import { createSignal, createMemo, For, Show } from "solid-js";

interface TaxonomyNode {
  id: string;
  label: string;
  sublabel?: string;
  href?: string;
  children?: TaxonomyNode[];
}

interface Props {
  nodes: TaxonomyNode[];
  centerLabel?: string;
  activeId?: string;
  onNodeClick?: (id: string) => void;
}

const NODE_R = 52;
const CENTER_R = 44;
const RING_SPACING = 110;

export default function GrammarDiagram(props: Props) {
  const [activeId, setActiveId] = createSignal<string | null>(props.activeId ?? null);

  function handleNodeClick(id: string, href?: string) {
    if (href) {
      window.location.href = href;
    } else {
      const newActive = activeId() === id ? null : id;
      setActiveId(newActive);
      props.onNodeClick?.(newActive ?? id);
    }
  }

  function getNodePositions(nodes: TaxonomyNode[], centerX: number, centerY: number) {
    const positions: { node: TaxonomyNode; x: number; y: number }[] = [];
    const angleStep = (2 * Math.PI) / nodes.length;
    nodes.forEach((node, i) => {
      const angle = angleStep * i - Math.PI / 2;
      const x = centerX + Math.cos(angle) * RING_SPACING;
      const y = centerY + Math.sin(angle) * RING_SPACING;
      positions.push({ node, x, y });
    });
    return positions;
  }

  function getChildPositions(
    parent: TaxonomyNode,
    parentX: number,
    parentY: number,
    startAngle: number,
    totalAngle: number
  ) {
    if (!parent.children) return [];
    const positions: { node: TaxonomyNode; x: number; y: number }[] = [];
    const angleStep = totalAngle / (parent.children.length + 1);
    parent.children.forEach((child, i) => {
      const angle = startAngle + angleStep * (i + 1);
      const x = parentX + Math.cos(angle) * 80;
      const y = parentY + Math.sin(angle) * 80;
      positions.push({ node: child, x, y });
    });
    return positions;
  }

  const svgW = 340;
  const svgH = 340;
  const cx = svgW / 2;
  const cy = svgH / 2;

  const nodePositions = createMemo(() => getNodePositions(props.nodes, cx, cy));

  return (
    <div class="grammar-diagram">
      <svg
        width={svgW}
        height={svgH}
        viewBox={`0 0 ${svgW} ${svgH}`}
        role="img"
        aria-label="French grammar taxonomy diagram"
        class="grammar-diagram-svg"
      >
        {/* Connection lines from center to outer nodes */}
        <For each={nodePositions()}>
          {({ node, x, y }) => {
            const isActive = () => activeId() === node.id;
            return (
              <line
                x1={cx} y1={cy}
                x2={x} y2={y}
                stroke={isActive() ? "var(--gold)" : "var(--border)"}
                stroke-width={isActive() ? 2 : 1}
                opacity={isActive() ? 0.8 : 0.4}
              />
            );
          }}
        </For>

        {/* Center node */}
        <g class="diagram-center">
          <circle cx={cx} cy={cy} r={CENTER_R} fill="var(--surface-2)" stroke="var(--gold)" stroke-width="2" />
          <text
            x={cx} y={cy + 5}
            text-anchor="middle"
            font-size="11"
            font-weight="600"
            fill="var(--gold)"
          >
            {props.centerLabel ?? "Pronouns"}
          </text>
        </g>

        {/* Outer nodes */}
        <For each={nodePositions()}>
          {({ node, x, y }) => {
            const isActive = () => activeId() === node.id;
            const hasChildren = () => node.children && node.children.length > 0;
            const childCount = () => node.children?.length ?? 0;
            const startAngle = Math.atan2(y - cy, x - cx) - (childCount() > 1 ? 0.4 : 0);

            return (
              <g
                class={`taxonomy-node${isActive() ? " active" : ""}`}
                onClick={() => handleNodeClick(node.id, node.href)}
                style={{ cursor: "pointer" }}
                role="button"
                tabIndex={0}
                aria-label={node.label}
                onKeyDown={e => e.key === "Enter" && handleNodeClick(node.id, node.href)}
              >
                {/* Outer ring for active */}
                <circle
                  cx={x} cy={y} r={NODE_R + 6}
                  fill="none"
                  stroke={isActive() ? "var(--gold)" : "none"}
                  stroke-width="2"
                  opacity={isActive() ? 0.4 : 0}
                />

                {/* Main circle */}
                <circle
                  cx={x} cy={y} r={NODE_R}
                  fill={isActive() ? "var(--surface-3)" : "var(--surface-2)"}
                  stroke={isActive() ? "var(--gold)" : "var(--border-bright)"}
                  stroke-width={isActive() ? 2 : 1}
                />

                {/* Label */}
                <text x={x} y={y - 4} text-anchor="middle" font-size="10" font-weight="500" fill="var(--text)">
                  {node.label.split(" ").map((word, i) => (
                    <tspan x={x} dy={i === 0 ? 0 : 12}>{word}</tspan>
                  ))}
                </text>

                {/* Sublabel */}
                <Show when={node.sublabel}>
                  <text x={x} y={y + 18} text-anchor="middle" font-size="8" fill="var(--text-2)">
                    {node.sublabel}
                  </text>
                </Show>

                {/* Child nodes */}
                <Show when={hasChildren() && isActive()}>
                  {(function() {
                    const childPositions = getChildPositions(node, x, y, startAngle, 1.2);
                    return (
                      <For each={childPositions}>
                        {({ node: child, x: cx, y: cy }) => (
                          <g
                            class="taxonomy-child"
                            onClick={e => {
                              e.stopPropagation();
                              if (child.href) window.location.href = child.href;
                            }}
                            style={{ cursor: child.href ? "pointer" : "default" }}
                            role="button"
                            tabIndex={0}
                            aria-label={child.label}
                          >
                            <circle
                              cx={cx} cy={cy} r={28}
                              fill="var(--surface)"
                              stroke="var(--border)"
                              stroke-width="1"
                            />
                            <text x={cx} y={cy - 2} text-anchor="middle" font-size="8" fill="var(--text)">
                              {child.label}
                            </text>
                            <Show when={child.sublabel}>
                              <text x={cx} y={cy + 10} text-anchor="middle" font-size="7" fill="var(--text-2)">
                                {child.sublabel}
                              </text>
                            </Show>
                          </g>
                        )}
                      </For>
                    );
                  })()}
                </Show>
              </g>
            );
          }}
        </For>
      </svg>

      <style>{`
        .grammar-diagram {
          display: flex;
          justify-content: center;
          padding: 1.5rem 0;
        }
        .grammar-diagram-svg {
          max-width: 100%;
          height: auto;
        }
        .taxonomy-node {
          transition: transform 0.2s ease;
        }
        .taxonomy-node:hover {
          transform: scale(1.05);
        }
        .taxonomy-node.active circle:first-of-type {
          filter: drop-shadow(0 0 12px var(--gold-glow));
        }
      `}</style>
    </div>
  );
}
