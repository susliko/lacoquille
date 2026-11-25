/**
 * The estimate itself — stratified frequency sampling.
 *
 * Vocabulary size can't be counted (nobody has a list of the words
 * they know), but it can be estimated: split the 20,000 most common
 * French lemmas into frequency bands, sample a few words per band,
 * ask "do you know this one?", and the yes-fractions are binomial
 * estimates of P(know a word of that band). Total estimate:
 *
 *   V = deck term (a census — every deck word counted with a
 *       passive-known proxy)  +  Σ band (N_b − deckInBand_b) · p̂_b
 *
 * …with deck words excluded from the sampled population so nothing
 * is counted twice. Invented French-looking words are mixed into the
 * questions; claiming to know them is a false alarm, and the
 * yes-fractions are corrected for that optimism (the high-threshold
 * correction: p_adj = (p − F)/(1 − F), clamped at 0).
 *
 * Uncertainty: a Wilson score interval per band (binomial noise in
 * p̂), mapped through the same correction, summed band-by-band into a
 * conservative interval for the total. The deck term is a census and
 * carries no counted variance.
 *
 * Frequency data: src/data/vocab/frequency.json, built from Lexique383
 * by tools/vocab-frequency.py — lemma-ranked, so a verb and its forty
 * conjugations are one word. Pseudowords are trigram-generated,
 * filtered against every real form Lexique knows plus the top English
 * forms (this learner knows English — "space" would be a dirty trap).
 */

import freq from "../../data/vocab/frequency.json";
import { wordSplit } from "./words";
import type { Calibration, CalibrationBand } from "./calibration";

/** Words in rank order — index + 1 = rank. */
export const WORDS: string[] = freq.words;
/** Cumulative band edges: [1000, 3000, 6000, 12000, 20000]. */
export const EDGES: number[] = freq.meta.edges;
export const PSEUDOWORDS: string[] = freq.pseudowords;

export const SAMPLES_PER_BAND = 12;
export const PSEUDOS_PER_TEST = 10;

const RANK = new Map<string, number>();
WORDS.forEach((w, i) => {
  if (!RANK.has(w)) RANK.set(w, i + 1);
});

/** Human label for a band (0-based). */
export function bandLabel(b: number): string {
  const from = b === 0 ? 1 : EDGES[b - 1] + 1;
  const to = EDGES[b];
  return from === 1 ? `top ${to.toLocaleString("en")}` : `${fmtK(from)}–${fmtK(to)}`;
}

const fmtK = (n: number) => (n >= 1000 ? `${Math.round(n / 1000)}k` : String(n));

export interface SampleItem {
  word: string;
  /** 0-based band index; -1 for an invented word. */
  band: number;
  pseudo: boolean;
}

/** Draw one test: SAMPLES_PER_BAND words per band (deck words
 *  excluded — they're counted by the census, not sampled) plus
 *  PSEUDOS_PER_TEST invented words, everything shuffled together. */
export function drawSample(deckKeys: Set<string>): SampleItem[] {
  const items: SampleItem[] = [];
  for (let b = 0; b < EDGES.length; b++) {
    const from = b === 0 ? 0 : EDGES[b - 1];
    const to = EDGES[b];
    const pool: string[] = [];
    for (let i = from; i < to; i++) {
      if (!deckKeys.has(WORDS[i])) pool.push(WORDS[i]);
    }
    shuffle(pool);
    for (const w of pool.slice(0, SAMPLES_PER_BAND)) {
      items.push({ word: w, band: b, pseudo: false });
    }
  }
  for (const w of shuffle([...PSEUDOWORDS]).slice(0, PSEUDOS_PER_TEST)) {
    items.push({ word: w, band: -1, pseudo: true });
  }
  return shuffle(items);
}

function shuffle<T>(a: T[]): T[] {
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}

// ─── the math ────────────────────────────────────────────────────────

export interface BandStat {
  from: number;
  to: number;
  yes: number;
  no: number;
  deckInBand: number;
}

export interface EstimateResult {
  estimate: number;
  low: number;
  high: number;
  /** Optimism rate on invented words (0…1). */
  falseAlarmRate: number;
  /** Census term: deck words counted directly. */
  deckKnown: number;
  bands: Array<{
    label: string;
    /** Corrected P(know | word of band). */
    p: number;
    pLow: number;
    pHigh: number;
    /** Sampled population: band size minus deck words in it. */
    pop: number;
    yes: number;
    asked: number;
  }>;
}

/** Wilson score interval for a binomial proportion. */
function wilson(k: number, n: number, z = 1.96): [number, number] {
  if (n === 0) return [0, 1];
  const p = k / n;
  const z2 = z * z;
  const d = 1 + z2 / n;
  const c = p + z2 / (2 * n);
  const h = z * Math.sqrt((p * (1 - p)) / n + z2 / (4 * n * n));
  return [Math.max(0, (c - h) / d), Math.min(1, (c + h) / d)];
}

/** High-threshold correction: map observed yes-rates down by the
 *  false-alarm rate. Monotone, so it commutes with the interval
 *  bounds. F = 0 is the identity; F = 1 (every invented word
 *  claimed) discounts every self-report to zero — no division by
 *  zero, no NaN. */
function corrected(x: number, f: number): number {
  if (f <= 0) return x;
  if (f >= 1) return 0;
  return Math.max(0, (x - f) / (1 - f));
}

/** Full estimate from a finished spot-check. */
export function computeEstimate(
  stats: BandStat[],
  falseAlarms: number,
  pseudoAsked: number,
  deckKnown: number,
): EstimateResult {
  const f = pseudoAsked > 0 ? falseAlarms / pseudoAsked : 0;
  let estimate = deckKnown;
  let low = deckKnown;
  let high = deckKnown;
  const bands: EstimateResult["bands"] = [];
  for (const b of stats) {
    const asked = b.yes + b.no;
    const pop = Math.max(0, b.to - b.from + 1 - b.deckInBand);
    if (asked === 0 || pop === 0) continue;
    const [wLo, wHi] = wilson(b.yes, asked);
    const p = corrected(b.yes / asked, f);
    const pLow = corrected(wLo, f);
    const pHigh = corrected(wHi, f);
    estimate += pop * p;
    low += pop * pLow;
    high += pop * pHigh;
    bands.push({
      label: bandLabelFrom(b.from, b.to),
      p,
      pLow,
      pHigh,
      pop,
      yes: b.yes,
      asked,
    });
  }
  return { estimate, low, high, falseAlarmRate: f, deckKnown, bands };
}

function bandLabelFrom(from: number, to: number): string {
  return from === 1 ? `top ${to.toLocaleString("en")}` : `${fmtK(from)}–${fmtK(to)}`;
}

// ─── deck census for the estimate ───────────────────────────────────

export interface DeckCensus {
  /** Passive-known words counted in the decks. */
  known: number;
  /** Deck words per band (0-based index into EDGES). */
  perBand: number[];
  /** Deck words not inside the 20k list (or unmatched). */
  offList: number;
  keys: Set<string>;
}

/** Deck words by frequency band. Keys come from words.ts; ranks are
 *  looked up here because only this module holds the list. */
export function deckCensus(): DeckCensus {
  const split = wordSplit();
  const perBand = new Array(EDGES.length).fill(0);
  let offList = 0;
  for (const key of split.keys) {
    const r = RANK.get(key);
    if (r == null) {
      offList += 1;
      continue;
    }
    const b = bandOf(r);
    if (b >= 0) perBand[b] += 1;
    else offList += 1;
  }
  return { known: split.known, perBand, offList, keys: split.keys };
}

/** 0-based band of a 1-based rank; -1 beyond the list. */
export function bandOf(rank: number): number {
  for (let b = 0; b < EDGES.length; b++) {
    if (rank <= EDGES[b]) return b;
  }
  return -1;
}

// ─── finishing a run ────────────────────────────────────────────────

/** Assemble the calibration record from a completed run and return
 *  it (the caller persists it). */
export function buildCalibration(
  answers: Array<{ band: number; pseudo: boolean; known: boolean }>,
): Calibration | null {
  const census = deckCensus();
  const yesByBand = new Map<number, number>();
  const noByBand = new Map<number, number>();
  let falseAlarms = 0;
  let pseudoAsked = 0;
  let wordsAsked = 0;
  for (const a of answers) {
    if (a.pseudo) {
      pseudoAsked += 1;
      if (a.known) falseAlarms += 1;
      continue;
    }
    wordsAsked += 1;
    const yes = (yesByBand.get(a.band) ?? 0) + (a.known ? 1 : 0);
    const no = (noByBand.get(a.band) ?? 0) + (a.known ? 0 : 1);
    yesByBand.set(a.band, yes);
    noByBand.set(a.band, no);
  }
  if (wordsAsked === 0) return null;

  const stats: BandStat[] = EDGES.map((to, b) => ({
    from: b === 0 ? 1 : EDGES[b - 1] + 1,
    to,
    yes: yesByBand.get(b) ?? 0,
    no: noByBand.get(b) ?? 0,
    deckInBand: census.perBand[b] ?? 0,
  }));

  const result = computeEstimate(stats, falseAlarms, pseudoAsked, census.known);
  const bands: CalibrationBand[] = stats.map((s) => ({
    from: s.from,
    to: s.to,
    yes: s.yes,
    no: s.no,
    deckInBand: s.deckInBand,
  }));
  return {
    version: 1,
    at: Math.floor(Date.now() / 1000),
    wordsAsked,
    pseudoAsked,
    falseAlarms,
    deckKnown: census.known,
    bands,
    estimate: result.estimate,
    low: result.low,
    high: result.high,
  };
}
