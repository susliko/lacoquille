/**
 * Settings — a pushed screen (handoff 2026-11): opened from the gear
 * in the Practice header, no tab bar, back arrow in the header.
 *
 * Four groups. The Anki group arrives with Anki import (deferred).
 * "Your data" exports every lacoquille.* store as one versioned JSON
 * file — progress lives on this device, and clearing browser data
 * destroys it with no recovery.
 */

import { createSignal, onMount, Show } from "solid-js";
import {
  readSettings,
  writeSettings,
  type AppSettings,
} from "./settings/settingsStore";

const Switch = (props: { on: boolean }) => (
  <span class={`set-switch${props.on ? " on" : ""}`} role="presentation" />
);

const Back = () => (
  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.6" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
    <path d="M15 6l-6 6 6 6" />
  </svg>
);

const DOC_KEYS = [
  "lacoquille.feed.v1",
  "lacoquille.exercises.v1",
  "lacoquille.vocab.v1",
  "lacoquille.settings.v1",
] as const;

export default function SettingsPanel(props: { version: string }) {
  const [s, setS] = createSignal<AppSettings | null>(null);
  onMount(() => setS(readSettings()));

  const update = <K extends keyof AppSettings>(k: K, v: AppSettings[K]) => {
    const next = { ...s()!, [k]: v };
    setS(next);
    writeSettings(next);
  };

  const exportAll = () => {
    const doc: Record<string, unknown> = { version: 1, exportedAt: new Date().toISOString() };
    for (const k of DOC_KEYS) {
      const v = localStorage.getItem(k);
      if (v) doc[k] = JSON.parse(v);
    }
    const blob = new Blob([JSON.stringify(doc, null, 2)], { type: "application/json" });
    const a = document.createElement("a");
    a.href = URL.createObjectURL(blob);
    a.download = "lacoquille-export.json";
    a.click();
    URL.revokeObjectURL(a.href);
  };

  const resetAll = () => {
    if (!window.confirm("Reset everything? Progress on this device will be deleted.")) return;
    for (const k of DOC_KEYS) localStorage.removeItem(k);
    location.href = "/";
  };

  return (
    <div class="set-root">
      <header class="set-bar">
        <a class="set-back" href="/" aria-label="Back">
          <Back />
        </a>
        <div class="set-title">Settings</div>
      </header>

      <Show when={s()} fallback={<div class="set-loading" />}>
        <section class="set-sec">
          <div class="set-sechead"><h2>Practice</h2></div>
          <div class="set-rows">
            <button type="button" class="set-row" onClick={() => update("typing", !s()!.typing)}>
              <span class="set-lb">
                Typing items
                <small>Off leaves only tap-based items, and reaches fewer topics</small>
              </span>
              <Switch on={s()!.typing} />
            </button>
            <button
              type="button"
              class="set-row"
              onClick={() => update("accents", s()!.accents === "lenient" ? "strict" : "lenient")}
            >
              <span class="set-lb">
                Accents
                <small>Lenient accepts a missing accent with a nudge</small>
              </span>
              <span class="set-val">{s()!.accents === "lenient" ? "Lenient" : "Strict"}</span>
            </button>
            <button type="button" class="set-row" onClick={() => update("sound", !s()!.sound)}>
              <span class="set-lb">Sound</span>
              <Switch on={s()!.sound} />
            </button>
          </div>
        </section>

        <section class="set-sec">
          <div class="set-sechead"><h2>Your data</h2></div>
          <div class="set-rows">
            <button type="button" class="set-row" onClick={exportAll}>
              <span class="set-lb">
                Export everything
                <small>Grammar progress too, as one JSON file</small>
              </span>
            </button>
            <button type="button" class="set-row set-danger" onClick={resetAll}>
              <span class="set-lb">Reset</span>
            </button>
          </div>
          <p class="set-foot">
            Progress lives on this device. Export it before you clear your browser data or
            move to another phone.
          </p>
        </section>

        <section class="set-sec">
          <div class="set-sechead"><h2>About</h2></div>
          <div class="set-rows">
            <div class="set-row">
              <span class="set-lb">Version</span>
              <span class="set-val">{props.version}</span>
            </div>
            <div class="set-row">
              <span class="set-lb">Source</span>
              <span class="set-val">github.com/susliko/lacoquille</span>
            </div>
          </div>
        </section>
      </Show>
    </div>
  );
}
