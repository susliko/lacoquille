/**
 * wordKey — normalize a card face to a bare word.
 *
 * The word-identity primitive shared by the vocab store (deduping
 * cards into words), the Anki import (stamping a note-level word on
 * every direction of a note) and the estimate (matching deck words
 * against the frequency list). Zero dependencies on purpose: lib and
 * components both import it.
 *
 * Lowercase, articles and elisions stripped ("La souris" → "souris",
 * "j'ai" → "ai"). Expressions and typos stay as-is — they never
 * match the frequency list and count as off-list deck words.
 */

export function wordKey(face: string): string {
  let w = face.toLowerCase().trim().replace(/[’`]/g, "'");
  w = w.replace(/^(?:le|la|les|l'|un|une|des|du|d'|de|au|aux|à la|à l')\s+/, "");
  w = w.replace(/^[a-z]'/, "");
  w = w.replace(/[.!?…,;:"«»()\[\]]+$/g, "");
  return w.trim();
}
