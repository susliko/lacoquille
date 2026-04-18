import { createSignal, For, Show, onMount, onCleanup } from "solid-js";

interface ArticleNode {
  id: string;
  label: string;
  forms: string[];
  description: string;
  href?: string;
}

interface Props {
  nodes: ArticleNode[];
}

export default function ArticleDiagram(props: Props) {
  const [activeId, setActiveId] = createSignal<string | null>(null);
  const [isMobile, setIsMobile] = createSignal(
    typeof window !== "undefined" ? window.innerWidth < 640 : false
  );

  onMount(() => {
    const check = () => setIsMobile(window.innerWidth < 640);
    window.addEventListener("resize", check);
    onCleanup(() => window.removeEventListener("resize", check));
  });

  function handleClick(id: string, href?: string) {
    if (href) {
      window.location.href = href;
    } else {
      setActiveId(activeId() === id ? null : id);
    }
  }

  const NODE_W = 140;
  const NODE_H = 90;
  const GAP = 24;
  const PADDING = 20;

  const totalWidth = PADDING * 2 + props.nodes.length * NODE_W + (props.nodes.length - 1) * GAP;
  const svgH = NODE_H + PADDING * 2 + (activeId() ? 60 : 0);

  return (
    <div class="article-diagram">
      <svg
        width={totalWidth}
        height={svgH}
        viewBox={`0 0 ${totalWidth} ${svgH}`}
        role="img"
        aria-label="French article types"
        class="article-diagram-svg"
      >
        {/* Connection arrows between nodes */}
        <For each={props.nodes.slice(0, -1)}>
          {(_, i) => {
            const fromX = PADDING + i() * (NODE_W + GAP) + NODE_W;
            const toX = PADDING + (i() + 1) * (NODE_W + GAP);
            const y = PADDING + NODE_H / 2;
            return (
              <>
                <line
                  x1={fromX} y1={y}
                  x2={toX - 8} y2={y}
                  stroke="var(--border)"
                  stroke-width="1.5"
                />
                <polygon
                  points={`${toX},${y} ${toX - 10},${y - 5} ${toX - 10},${y + 5}`}
                  fill="var(--border)"
                />
              </>
            );
          }}
        </For>

        {/* Nodes */}
        <For each={props.nodes}>
          {(node) => {
            const x = PADDING + props.nodes.indexOf(node) * (NODE_W + GAP);
            const y = PADDING;
            const isActive = () => activeId() === node.id;
            const colHue = node.id === "definite" ? 200 : node.id === "indefinite" ? 280 : 40;

            return (
              <g
                class="article-node"
                onClick={() => handleClick(node.id, node.href)}
                style={{ cursor: "pointer" }}
                role="button"
                tabIndex={0}
                onKeyDown={(e) => e.key === "Enter" && handleClick(node.id, node.href)}
              >
                {/* Background */}
                <rect
                  x={x} y={y}
                  width={NODE_W} height={NODE_H}
                  rx="8"
                  fill={isActive() ? "var(--surface-3)" : "var(--surface-2)"}
                  stroke={isActive() ? "var(--gold)" : "var(--border)"}
                  stroke-width={isActive() ? 2 : 1}
                />

                {/* Type label */}
                <text
                  x={x + NODE_W / 2} y={y + 18}
                  text-anchor="middle"
                  font-size="11"
                  font-weight="600"
                  fill={`hsl(${colHue} 60% 60%)`}
                >
                  {node.label}
                </text>

                {/* Forms */}
                <text
                  x={x + NODE_W / 2} y={y + 38}
                  text-anchor="middle"
                  font-size="10"
                  fill="var(--text)"
                >
                  {node.forms.join(" · ")}
                </text>

                {/* Description */}
                <text
                  x={x + NODE_W / 2} y={y + 58}
                  text-anchor="middle"
                  font-size="9"
                  fill="var(--text-2)"
                >
                  {node.description}
                </text>

                {/* Link indicator */}
                <Show when={node.href}>
                  <text
                    x={x + NODE_W / 2} y={y + 76}
                    text-anchor="middle"
                    font-size="8"
                    fill="var(--gold)"
                  >
                    →
                  </text>
                </Show>

                {/* Hover glow */}
                <Show when={isActive()}>
                  <rect
                    x={x - 4} y={y - 4}
                    width={NODE_W + 8} height={NODE_H + 8}
                    rx="10"
                    fill="none"
                    stroke="var(--gold)"
                    stroke-width="1"
                    opacity="0.3"
                  />
                </Show>
              </g>
            );
          }}
        </For>

        {/* Detail panel for active node */}
        <Show when={activeId()}>
          {() => {
            const node = props.nodes.find(n => n.id === activeId());
            if (!node) return null;
            const panelY = NODE_H + PADDING + 16;
            return (
              <g class="detail-panel">
                <text
                  x={PADDING}
                  y={panelY}
                  font-size="10"
                  fill="var(--text-2)"
                >
                  Click any article type to learn more →
                </text>
              </g>
            );
          }}
        </Show>
      </svg>

      <style>{`
        .article-diagram {
          display: flex;
          justify-content: center;
          padding: 1.5rem 0;
          overflow-x: auto;
        }
        .article-diagram-svg {
          max-width: 100%;
          height: auto;
        }
        .article-node {
          transition: transform 0.2s ease;
        }
        .article-node:hover {
          transform: translateY(-2px);
        }
      `}</style>
    </div>
  );
}
