/**
 * App settings — lacoquille.settings.v1 (handoff 2026-11, Settings screen).
 *
 * Plain localStorage, no Solid dependency, so any store or island can
 * read it. Defaults follow the handoff: typing on, accents lenient
 * (strict stays available for DELF candidates), levels shown default
 * to one above the learner's current level — six rows of 0% read as
 * failure to a beginner.
 */

import { currentLevel, LEVELS, type Level } from "../exercise/feedStore";

export interface AppSettings {
  /** Off removes cloze + transform — most of the map. The settings row says so. */
  typing: boolean;
  /** Lenient accepts a missing accent with a nudge. */
  accents: "lenient" | "strict";
  /** Highest level shown in the grammar grid. */
  levelsMax: Level;
  sound: boolean;
}

const KEY = "lacoquille.settings.v1";

function defaults(): AppSettings {
  const idx = LEVELS.indexOf(currentLevel());
  const levelsMax = LEVELS[Math.min(idx + 1, LEVELS.length - 1)];
  return { typing: true, accents: "lenient", levelsMax, sound: true };
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

/** Levels shown in the grid: everything up to levelsMax. */
export function levelsShown(): string[] {
  const max = readSettings().levelsMax;
  return LEVELS.slice(0, LEVELS.indexOf(max) + 1);
}
