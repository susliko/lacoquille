/**
 * Export — "get your words out", the cheap one-way direction the
 * handoff settled on (docs/04-vocab-and-anki.md left .apkg round-
 * tripping as an open question; writing a valid package without a
 * real Anki to test against is the expensive path, and Anki's own
 * text import is first-class).
 *
 * One .txt per deck (tab-separated, Anki's import headers, plain
 * text — #html:false), named for its deck via #deck:. Several decks
 * zip into one file so the download stays a single tap. Scheduling
 * does not travel; the UI says so next to the button.
 */

import { strToU8, zipSync } from "fflate";

export interface ExportDeck {
  name: string;
  cards: Array<{ front: string; back: string }>;
}

export interface ExportFile {
  filename: string;
  mime: string;
  data: Uint8Array;
}

/** Tab and newline are structure in a text import — flatten them. */
function tsvCell(s: string): string {
  return s.replace(/[\t\r\n]+/g, " ").trim();
}

function deckFilename(name: string): string {
  const base = name
    .replace(/::/g, " — ")
    .replace(/[\\/:*?"<>|]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();
  return `${base || "deck"}.txt`;
}

export function buildAnkiExport(decks: ExportDeck[]): ExportFile {
  const stamp = new Date().toISOString().slice(0, 10);
  const files: Record<string, Uint8Array> = {};

  for (const d of decks) {
    if (d.cards.length === 0) continue;
    const lines = [`#separator:tab`, `#html:false`, `#deck:${d.name}`];
    for (const c of d.cards) {
      const front = tsvCell(c.front);
      const back = tsvCell(c.back);
      if (front && back) lines.push(`${front}\t${back}`);
    }
    if (lines.length > 3) files[deckFilename(d.name)] = strToU8(lines.join("\n") + "\n");
  }

  const names = Object.keys(files);
  if (names.length === 0) {
    throw new Error("nothing to export");
  }
  if (names.length === 1) {
    const name = names[0];
    return { filename: `lacoquille ${name}`, mime: "text/tab-separated-values", data: files[name] };
  }
  return {
    filename: `lacoquille decks ${stamp}.zip`,
    mime: "application/zip",
    data: zipSync(files),
  };
}

/** Trigger the download (browser only; a plain anchor download keeps
    it one tap on mobile — no blob URL juggling visible to the user). */
export function downloadExport(f: ExportFile): void {
  const url = URL.createObjectURL(new Blob([f.data as BlobPart], { type: f.mime }));
  const a = document.createElement("a");
  a.href = url;
  a.download = f.filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  setTimeout(() => URL.revokeObjectURL(url), 5000);
}
