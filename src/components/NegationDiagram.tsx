import { createSignal, For, onMount, onCleanup } from "solid-js";

interface NegationParticle {
  id: string;
  label: string;
  meaning: string;
}

interface Props {
  particles: NegationParticle[];
}

export default function NegationDiagram(props: Props) {
  const [activeId, setActiveId] = createSignal("pas");

  const CARD_W = 90;
  const CARD_H = 60;
  const GAP = 12;
  const svgW = 500;
  const svgH = 200;

  return (
    <div class="negation-diagram">
      <svg
        width={svgW}
        height={svgH}
        viewBox={`0 0 ${svgW} ${svgH}`}
        role="img"
        aria-label="French negation structure"
      >
        {/* Main bracket visualization */}
        {/* ne box */}
        <rect x={40} y={20} width={50} height={44} rx="6"
          fill="var(--surface-2)" stroke="var(--indicatif)" stroke-width="2" />
        <text x={65} y={48} text-anchor="middle" font-size="16" font-weight="700" fill="var(--indicatif)">
          ne
        </text>

        {/* Arrow */}
        <line x1={95} y1={42} x2={140} y2={42} stroke="var(--border)" stroke-width="2" />
        <polygon points="140,38 148,42 140,46" fill="var(--border)" />

        {/* Verb placeholder */}
        <rect x={150} y={20} width={100} height={44} rx="6"
          fill="var(--surface-3)" stroke="var(--border)" stroke-width="1" />
        <text x={200} y={48} text-anchor="middle" font-size="14" fill="var(--text)">
          verb
        </text>

        {/* Arrow */}
        <line x1={255} y1={42} x2={300} y2={42} stroke="var(--border)" stroke-width="2" />
        <polygon points="300,38 308,42 300,46" fill="var(--border)" />

        {/* Active particle box */}
        <rect x={310} y={20} width={110} height={44} rx="6"
          fill="var(--surface-2)" stroke="var(--gold)" stroke-width="2" />
        <text x={365} y={40} text-anchor="middle" font-size="18" font-weight="700" fill="var(--gold)">
          {props.particles.find(p => p.id === activeId())?.label}
        </text>
        <text x={365} y={56} text-anchor="middle" font-size="9" fill="var(--text-2)">
          {props.particles.find(p => p.id === activeId())?.meaning}
        </text>

        {/* Particle selector row */}
        <For each={props.particles}>
          {(particle, i) => {
            const x = 40 + i() * (CARD_W + GAP);
            const isActive = () => activeId() === particle.id;
            return (
              <g
                class="particle-card"
                onClick={() => setActiveId(particle.id)}
                style={{ cursor: "pointer" }}
                role="button"
                tabIndex={0}
                onKeyDown={(e) => e.key === "Enter" && setActiveId(particle.id)}
              >
                <rect
                  x={x}
                  y={100}
                  width={CARD_W}
                  height={CARD_H}
                  rx="8"
                  fill={isActive() ? "var(--surface-3)" : "var(--surface-2)"}
                  stroke={isActive() ? "var(--gold)" : "var(--border)"}
                  stroke-width={isActive() ? 2 : 1}
                />
                <text x={x + CARD_W/2} y={130} text-anchor="middle" font-size="16" font-weight="600" fill="var(--text)">
                  {particle.label}
                </text>
                <text x={x + CARD_W/2} y={148} text-anchor="middle" font-size="9" fill="var(--text-2)">
                  {particle.meaning}
                </text>
              </g>
            );
          }}
        </For>

        {/* Full example below */}
        <text x={svgW/2} y={185} text-anchor="middle" font-size="11" fill="var(--text-2)">
          je ne comprends <tspan fill="var(--gold)" font-weight="600">{props.particles.find(p => p.id === activeId())?.label}</tspan>
        </text>
      </svg>

      <style>{`
        .negation-diagram {
          display: flex;
          justify-content: center;
          padding: 1rem 0;
          overflow-x: auto;
        }
        .particle-card:hover rect {
          stroke: var(--text-2);
        }
        .particle-card {
          transition: transform 0.15s ease;
        }
        .particle-card:hover {
          transform: translateY(-2px);
        }
      `}</style>
    </div>
  );
}
