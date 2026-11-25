/**
 * Calibration store — lacoquille.vocabestimate.v1.
 *
 * One snapshot per spot-check: per-band yes/no counts, the invented
 * words claimed as known (false alarms), and the deck census at
 * calibration time. The point estimate and its interval are frozen
 * into the record so the Progress card can show them without
 * importing the 20k-word frequency list (that lives only in the
 * spot-check island's chunk).
 */

export interface CalibrationBand {
  /** 1-based rank range, inclusive. */
  from: number;
  to: number;
  yes: number;
  no: number;
  /** Deck words inside the band — excluded from the sampled
      population (they're counted directly by the census). */
  deckInBand: number;
}

export interface Calibration {
  version: 1;
  /** Epoch seconds of completion. */
  at: number;
  wordsAsked: number;
  pseudoAsked: number;
  /** Invented words claimed as known. */
  falseAlarms: number;
  /** Passive-known words counted in the decks at calibration time. */
  deckKnown: number;
  bands: CalibrationBand[];
  estimate: number;
  low: number;
  high: number;
}

const KEY = "lacoquille.vocabestimate.v1";

export function readCalibration(): Calibration | null {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return null;
    const c = JSON.parse(raw) as Calibration;
    if (c.version !== 1 || !Array.isArray(c.bands)) return null;
    return c;
  } catch {
    return null;
  }
}

export function writeCalibration(c: Calibration): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(c));
  } catch {
    /* private mode: the estimate lives only for this sitting */
  }
}

export function clearCalibration(): void {
  try {
    localStorage.removeItem(KEY);
  } catch {
    /* nothing to clear */
  }
}
