import { createSignal } from "solid-js";

interface Props {
  term: string;
  definition: string;
  children: string;
}

export default function Term(props: Props) {
  const [show, setShow] = createSignal(false);

  return (
    <span
      class="term"
      onMouseEnter={() => setShow(true)}
      onMouseLeave={() => setShow(false)}
      onFocus={() => setShow(true)}
      onBlur={() => setShow(false)}
      tabIndex={0}
      role="term"
      aria-describedby={`term-${props.term}`}
    >
      {props.children}
      {show() && (
        <span class="term-tooltip" id={`term-${props.term}`} role="tooltip">
          <strong>{props.term}:</strong> {props.definition}
        </span>
      )}
      <style>{`
        .term {
          border-bottom: 1px dotted var(--text-2);
          cursor: help;
          position: relative;
        }
        .term-tooltip {
          position: absolute;
          bottom: calc(100% + 6px);
          left: 50%;
          transform: translateX(-50%);
          background: var(--surface-3);
          border: 1px solid var(--border);
          border-radius: 6px;
          padding: 0.5rem 0.75rem;
          font-size: 0.8rem;
          white-space: nowrap;
          max-width: 280px;
          white-space: normal;
          z-index: 100;
          box-shadow: 0 4px 12px rgba(0,0,0,0.3);
          color: var(--text);
        }
        .term-tooltip::after {
          content: "";
          position: absolute;
          top: 100%;
          left: 50%;
          transform: translateX(-50%);
          border: 5px solid transparent;
          border-top-color: var(--border);
        }
      `}</style>
    </span>
  );
}
