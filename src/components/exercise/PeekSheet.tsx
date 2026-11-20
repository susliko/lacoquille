/**
 * The peek — a sheet over the feed, never a navigation (handoff
 * 2026-11 §5). The feed stays mounted and visible behind a dim scrim,
 * so it is obvious nothing was left.
 *
 * Four ways out, all cheap: drag the sheet down, tap the scrim, tap
 * the X, press Escape. The primary button "Got it — next" closes the
 * sheet *and* advances in one tap, so reading the rule costs exactly
 * one tap more than not reading it.
 *
 * Implementation notes from the prototype: the sheet is 80% max
 * height so the exercise stays visible behind it; drag-to-dismiss is
 * ignored while the sheet body is scrolled; prefers-reduced-motion
 * disables the rise animation (tokens.css media query).
 */

import { createEffect, createSignal, For, Show } from "solid-js";
import { peekFor, type PeekContent } from "./peek";

export default function PeekSheet(props: {
  slug: string | undefined;
  onClose: () => void;
  /** "Got it — next": close *and* advance. */
  onNext: () => void;
}) {
  const [body, setBody] = createSignal<PeekContent | null>(peekFor(props.slug));
  const [dragY, setDragY] = createSignal(0);
  const [scrolled, setScrolled] = createSignal(false);
  let startY = 0;
  let dragging = false;
  let primary: HTMLButtonElement | undefined;
  let sheetEl: HTMLDivElement | undefined;

  createEffect(() => {
    if (props.slug) setBody(peekFor(props.slug));
    if (primary) primary.focus();
  });

  const onKey = (e: KeyboardEvent) => {
    if (e.key === "Escape") {
      e.preventDefault();
      props.onClose();
    }
  };
  createEffect(() => {
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  });

  const down = (y: number) => {
    // ignore the gesture when the sheet body is scrolled
    if (scrolled()) return;
    startY = y;
    dragging = true;
  };
  const move = (y: number) => {
    if (!dragging) return;
    setDragY(Math.max(0, y - startY));
  };
  const up = () => {
    if (!dragging) return;
    dragging = false;
    if (dragY() > 90) props.onClose();
    else setDragY(0);
  };

  return (
    <>
      <button type="button" class="peek-scrim" aria-label="Close" tabindex="-1" onClick={props.onClose} />
      <div
        class="peek-sheet"
        ref={sheetEl}
        role="dialog"
        aria-modal="true"
        aria-labelledby="peek-title"
        style={{ transform: dragY() ? `translateY(${dragY()}px)` : "" }}
      >
        <div
          class="peek-grab"
          onTouchStart={(e) => down(e.touches[0].clientY)}
          onTouchMove={(e) => move(e.touches[0].clientY)}
          onTouchEnd={up}
          onMouseDown={(e) => {
            e.preventDefault();
            down(e.clientY);
            const mm = (ev: MouseEvent) => move(ev.clientY);
            const mu = () => {
              up();
              window.removeEventListener("mousemove", mm);
              window.removeEventListener("mouseup", mu);
            };
            window.addEventListener("mousemove", mm);
            window.addEventListener("mouseup", mu);
          }}
        >
          <span class="peek-handle" aria-hidden="true" />
          <button type="button" class="peek-x" aria-label="Close" onClick={props.onClose}>
            ×
          </button>
        </div>

        <div class="peek-body" onScroll={(e) => setScrolled(e.currentTarget.scrollTop > 0)}>
          <Show when={body()} fallback={<p class="peek-missing">No rule sheet for this topic yet.</p>}>
            {(b) => (
              <>
                <div class="peek-eyebrow">The rule</div>
                <h3 id="peek-title" class="peek-title">{b().title}</h3>
                <For each={b().blocks}>
                  {(blk) => (
                    <div class="peek-block">
                      <Show when={blk.h}>
                        <b class="peek-block-h">{blk.h}</b>
                      </Show>
                      <p innerHTML={blk.p} />
                    </div>
                  )}
                </For>
                <Show when={b().examples.length > 0}>
                  <table class="peek-ex">
                    <tbody>
                      <For each={b().examples}>
                        {([fr, en]) => (
                          <tr>
                            <td class="peek-ex-fr" innerHTML={fr} />
                            <td class="peek-ex-en">{en}</td>
                          </tr>
                        )}
                      </For>
                    </tbody>
                  </table>
                </Show>
              </>
            )}
          </Show>
        </div>

        <div class="peek-foot">
          <Show when={body()}>
            {(b) => (
              <a class="peek-ghost" href={`/knowledge/t/${b().entryId}`}>
                Full page
              </a>
            )}
          </Show>
          <button type="button" class="peek-cta" ref={primary} onClick={props.onNext}>
            Got it — next
          </button>
        </div>
      </div>
    </>
  );
}
