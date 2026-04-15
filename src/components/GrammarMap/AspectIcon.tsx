import type { AspectId } from "./types";

interface Props {
  aspect: AspectId;
  size?: number;
  color?: string;
}

/**
 * Shared glyph library — same symbol means the same thing across all Grammar Map topics.
 * completed = filled dot | ongoing = wave | anterior = dot·dot | habitual = repeat arrows | none = empty
 */
export function AspectIcon({ aspect, size = 16, color = "currentColor" }: Props) {
  const s = size;
  const c = color;

  switch (aspect) {
    case "completed":
      return (
        <svg width={s} height={s} viewBox="0 0 16 16" aria-label="completed" role="img">
          <circle cx="8" cy="8" r="5" fill={c} />
        </svg>
      );
    case "ongoing":
    case "habitual":
      return (
        <svg width={s} height={s} viewBox="0 0 16 16" aria-label={aspect} role="img">
          <path
            d="M1 8 Q4 4 8 8 Q12 12 15 8"
            stroke={c} stroke-width="2" fill="none" stroke-linecap="round"
          />
        </svg>
      );
    case "anterior":
      return (
        <svg width={s} height={s} viewBox="0 0 16 16" aria-label="anterior" role="img">
          <circle cx="5" cy="8" r="3.5" fill={c} />
          <circle cx="13" cy="8" r="3.5" fill={c} />
          <line x1="8.5" y1="8" x2="9.5" y2="8" stroke={c} stroke-width="1.5" />
        </svg>
      );
    default:
      return <span style={{ display: "inline-block", width: `${s}px`, height: `${s}px` }} />;
  }
}
