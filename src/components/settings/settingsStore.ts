/**
 * App settings — lacoquille.settings.v1 (handoff 2026-11, Settings screen).
 *
 * Plain localStorage, no Solid dependency, so any store or island can
 * read it. Defaults follow the handoff: typing on, accents lenient
 * (strict stays available for DELF candidates).
 */

export interface AppSettings {
  /** Off removes cloze + transform — most of the map. The settings row says so. */
  typing: boolean;
  /** Lenient accepts a missing accent with a nudge. */
  accents: "lenient" | "strict";
  sound: boolean;
}

const KEY = "lacoquille.settings.v1";

function defaults(): AppSettings {
  return { typing: true, accents: "lenient", sound: true };
}

export function readSettings(): AppSettings {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return defaults();
    return { ...defaults(), ...JSON.parse(raw) };
  } catch {
    return defaults();
  }
}

export function readSettingsOnce(): AppSettings {
  return readSettings();
}

export function writeSettings(next: AppSettings): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(next));
  } catch {
    /* private mode: settings live for the session only */
  }
}
