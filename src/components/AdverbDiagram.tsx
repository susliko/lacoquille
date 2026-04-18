import { createSignal, For, Show, onMount, onCleanup } from "solid-js";

interface AdverbNode {
  id: string;
  label: string;
  examples: string[];
  href?: string;
}

interface Props {
  nodes: AdverbNode[];
  centerLabel?: string;
}

export default function AdverbDiagram(props: Props) {
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

  const NODE_R = 52;
  const CENTER_R = 40;
  const RING_SPACING = 108;
  const svgW = 320;
  const svgH = 320;
  const cx = svgW / 2;
  const cy = svgH / 2;

  const angleStep = (2 * Math.PI) / props.nodes.length;
  const typeColors: Record<string, string> = {
    manner: "#4d8eff",
    time: "#9d6cf0",
    place: "#2dbe96",
    degree: "#f0a030",
    frequency: "#e05c5c",
  };

  function getNodePosition(index: number) {
    const angle = angleStep * index - Math.PI / 2;
    return {
      x: cx + Math.cos(angle) * RING_SPACING,
      y: cy + Math.sin(angle) * RING_SPACING,
    };
  }

  return (
    <div class="adverb-diagram">
      <svg
        width={svgW}
        height={svgH}
        viewBox={`0 0 ${svgW} ${svgH}`}
        role="img"
        aria-label="French adverb types"
        class="adverb-diagram-svg"
      >
        {/* Center */}
        <circle cx={cx} cy={cy} r={CENTER_R} fill="var(--surface-2)" stroke="var(--gold)" stroke-width="2" />
        <text x={cx} y={cy + 4} text-anchor="middle" font-size="10" font-weight="600" fill="var(--gold)">
          {props.centerLabel ?? "Adverbs"}
        </text>

        {/* Connection lines */}
        <For each={props.nodes}>
          {(node, i) => {
            const pos = getNodePosition(i());
            const isActive = () => activeId() === node.id;
            return (
              <line
                x1={cx} y1={cy}
                x2={pos.x} y2={pos.y}
                stroke={isActive() ? typeColors[node.id] ?? "var(--gold)" : "var(--border)"}
                stroke-width={isActive() ? 2 : 1}
                opacity={isActive() ? 0.8 : 0.4}
              />
            );
          }}
        </For>

        {/* Nodes */}
        <For each={props.nodes}>
          {(node, i) => {
            const pos = getNodePosition(i());
            const isActive = () => activeId() === node.id;
            const color = typeColors[node.id] ?? "var(--gold)";

            return (
              <g
                class="adverb-node"
                onClick={() => handleClick(node.id, node.href)}
                style={{ cursor: "pointer" }}
                role="button"
                tabIndex={0}
                onKeyDown={(e) => e.key === "Enter" && handleClick(node.id, node.href)}
              >
                {/* Glow ring */}
                <Show when={isActive()}>
                  <circle cx={pos.x} cy={pos.y} r={NODE_R + 8} fill="none" stroke={color} stroke-width="2" opacity="0.3" />
                </Show>

                {/* Main circle */}
                <circle
                  cx={pos.x} cy={pos.y} r={NODE_R}
                  fill={isActive() ? "var(--surface-3)" : "var(--surface-2)"}
                  stroke={isActive() ? color : "var(--border-bright)"}
                  stroke-width={isActive() ? 2 : 1}
                />

                {/* Type label */}
                <text x={pos.x} y={pos.y - 8} text-anchor="middle" font-size="10" font-weight="600" fill={color}>
                  {node.label}
                </text>

                {/* Examples */}
                <text x={pos.x} y={pos.y + 8} text-anchor="middle" font-size="8" fill="var(--text-2)">
                  {node.examples.slice(0, 2).join(", ")}
                </text>

                {/* Arrow indicator */}
                <Show when={node.href}>
                  <text x={pos.x} y={pos.y + 22} text-anchor="middle" font-size="9" fill={color}>→</text>
                </Show>
              </g>
            );
          }}
        </For>

        {/* Active detail panel */}
        <Show when={activeId()}>
          {() => {
            const node = props.nodes.find(n => n.id === activeId());
            if (!node || !node.href) return null;
            return (
              <g>
                <text x={cx} y={cy + 60} text-anchor="middle" font-size="9" fill="var(--text-2)">
                  Click to explore →
                </text>
              </g>
            );
          }}
        </Show>
      </svg>

      <style>{`
        .adverb-diagram {
          display: flex;
          justify-content: center;
          padding: 1.5rem 0;
        }
        .adverb-diagram-svg {
          max-width: 100%;
          height: auto;
        }
        .adverb-node {
          transition: transform 0.2s ease;
        }
        .adverb-node:hover {
          transform: scale(1.05);
        }
      `}</style>
    </div>
  );
}
