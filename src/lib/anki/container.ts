/**
 * Anki package containers — the outside of the .apkg / .colpkg.
 *
 * Three shapes exist in the wild (handoff docs/04-vocab-and-anki.md):
 *
 *   · a zip archive holding `collection.anki2` / `.anki21` /
 *     `.anki21b` plus a `media` map — every .apkg, and .colpkg up to
 *     Anki 23.10;
 *   · a single zstd frame wrapping a tar, whose entries are the same
 *     files — .colpkg from Anki 23.10 on;
 *   · a plain zstd frame that IS the collection (rare legacy .colpkg
 *     of exactly one file) — handled by the same magic check.
 *
 * The zip is read by slicing the Blob (central directory → only the
 * entries we need), so a media-heavy deck never enters memory: on a
 * 500 MB package we read kilobytes of directory and one database.
 *
 * Only the parts that matter for import are decoded — media bytes are
 * never extracted (text cards don't need them; the map is counted so
 * the "What's in it" screen can say what the package carries).
 */

import { inflateSync } from "fflate";

// zstd appears in .anki21b and modern .colpkg only — keep the
// decoder out of the first-load bundle and pull it on first use
let zstdDecompress: ((d: Uint8Array) => Uint8Array) | null = null;
async function zstdSync(bytes: Uint8Array): Promise<Uint8Array> {
  if (!zstdDecompress) {
    zstdDecompress = (await import("fzstd")).decompress;
  }
  return zstdDecompress(bytes);
}

export type CollectionVariant = "anki2" | "anki21" | "anki21b";
export type ContainerKind = "zip" | "tar+zstd" | "zstd";

export class AnkiError extends Error {
  kind: "badfile" | "unsupported" | "toolarge";
  constructor(kind: AnkiError["kind"], message: string) {
    super(message);
    this.kind = kind;
  }
}

export interface AnkiContainer {
  collection: Uint8Array;
  variant: CollectionVariant;
  kind: ContainerKind;
  /** Media files the package declares; null when the map is unreadable. */
  mediaCount: number | null;
}

const ZSTD_MAGIC = [0x28, 0xb5, 0x2f, 0xfd];
const SQLITE_MAGIC = "SQLite format 3\x00";

/** Whole-file decompression ceiling — reading works by slice, so this
    only guards the tar+zstd and zstd paths, which must materialise. */
const MAX_DECOMPRESSED = 512 * 1024 * 1024;

const isZstd = (b: Uint8Array, at = 0): boolean =>
  b.length >= at + 4 &&
  b[at] === ZSTD_MAGIC[0] &&
  b[at + 1] === ZSTD_MAGIC[1] &&
  b[at + 2] === ZSTD_MAGIC[2] &&
  b[at + 3] === ZSTD_MAGIC[3];

async function zstd(bytes: Uint8Array, what: string): Promise<Uint8Array> {
  try {
    return await zstdSync(bytes);
  } catch {
    throw new AnkiError("badfile", `${what} is compressed but couldn't be unpacked — the file may be truncated.`);
  }
}

const decoder = new TextDecoder();
const isSqlite = (b: Uint8Array): boolean =>
  b.length > 16 && decoder.decode(b.slice(0, 16)) === SQLITE_MAGIC;

// ─── zip ────────────────────────────────────────────────────────────

interface Eocd {
  count: number;
  cdOff: number;
  cdSize: number;
}

interface ZipEntry {
  name: string;
  method: number;
  /** Compressed size from the central directory. */
  csize: number;
  /** Local-header offset. */
  lho: number;
}

/** Locate the end-of-central-directory record (scanning backwards past
    any trailing bytes — some exporters append data). */
async function readEocd(file: Blob): Promise<Eocd> {
  const tail = new Uint8Array(await file.slice(Math.max(0, file.size - 66000)).arrayBuffer());
  for (let i = tail.length - 22; i >= 0; i--) {
    if (tail[i] === 0x50 && tail[i + 1] === 0x4b && tail[i + 2] === 0x05 && tail[i + 3] === 0x06) {
      const dv = new DataView(tail.buffer, tail.byteOffset);
      const count = dv.getUint16(i + 10, true);
      const cdSize = dv.getUint32(i + 12, true);
      const cdOff = dv.getUint32(i + 16, true);
      if (cdOff === 0xffffffff || cdSize === 0xffffffff) {
        throw new AnkiError("unsupported", "This package uses ZIP64 — too large to open here.");
      }
      return { count, cdOff, cdSize };
    }
  }
  throw new AnkiError("badfile", "This doesn't look like an Anki package (no zip directory).");
}

async function readDirectory(file: Blob, eocd: Eocd): Promise<ZipEntry[]> {
  const cd = new Uint8Array(await file.slice(eocd.cdOff, eocd.cdOff + eocd.cdSize).arrayBuffer());
  const dv = new DataView(cd.buffer, cd.byteOffset);
  const entries: ZipEntry[] = [];
  let p = 0;
  for (let n = 0; n < eocd.count; n++) {
    if (p + 46 > cd.length || cd[p] !== 0x50 || cd[p + 1] !== 0x4b || cd[p + 2] !== 0x01 || cd[p + 3] !== 0x02) break;
    const method = dv.getUint16(p + 10, true);
    const csize = dv.getUint32(p + 20, true);
    const nameLen = dv.getUint16(p + 28, true);
    const extraLen = dv.getUint16(p + 30, true);
    const commentLen = dv.getUint16(p + 32, true);
    const lho = dv.getUint32(p + 42, true);
    entries.push({ name: decoder.decode(cd.slice(p + 46, p + 46 + nameLen)), method, csize, lho });
    p += 46 + nameLen + extraLen + commentLen;
  }
  return entries;
}

/** Fetch one entry's bytes: local header (names can differ from the
    central directory's extra field, so re-read the local lengths). */
async function readEntry(file: Blob, e: ZipEntry): Promise<Uint8Array> {
  const head = new Uint8Array(await file.slice(e.lho, e.lho + 30).arrayBuffer());
  if (head[0] !== 0x50 || head[1] !== 0x4b || head[2] !== 0x03 || head[3] !== 0x04) {
    throw new AnkiError("badfile", "This package's zip is damaged.");
  }
  const dv = new DataView(head.buffer, head.byteOffset);
  const nameLen = dv.getUint16(26, true);
  const extraLen = dv.getUint16(28, true);
  const start = e.lho + 30 + nameLen + extraLen;
  const raw = new Uint8Array(await file.slice(start, start + e.csize).arrayBuffer());
  switch (e.method) {
    case 0:
      return raw;
    case 8:
      try {
        return inflateSync(raw);
      } catch {
        throw new AnkiError("badfile", "This package's zip is damaged.");
      }
    case 93:
      return await zstd(raw, `zip entry ${e.name}`);
    default:
      throw new AnkiError("unsupported", `This package uses a compression this app can't unpack (${e.method}).`);
  }
}

// ─── tar (inside zstd) ──────────────────────────────────────────────

function tarFind(tar: Uint8Array, names: RegExp): Uint8Array | null {
  let p = 0;
  while (p + 512 <= tar.length) {
    // two zero bytes lead every end-of-archive block; real headers
    // start with a name, so this can't false-positive mid-archive
    if (tar[p] === 0 && tar[p + 1] === 0) break;
    const name = decoder.decode(tar.slice(p, p + 100)).replace(/\0.*$/, "");
    const sizeOct = decoder.decode(tar.slice(p + 124, p + 136)).replace(/\0/g, "").trim();
    const size = parseInt(sizeOct || "0", 8);
    if (!Number.isFinite(size) || size < 0) break; // corrupt header — stop, don't loop
    const type = String.fromCharCode(tar[p + 156] || 48);
    const dataStart = p + 512;
    if (type === "0" || type === "\0") {
      // regular file — the ones we want have short names; pax/GNU
      // long-name headers ('x'/'L') are skipped with their payloads
      if (names.test(name)) return tar.slice(dataStart, dataStart + size);
    }
    p = dataStart + Math.ceil(size / 512) * 512;
  }
  return null;
}

// ─── media map (count only) ─────────────────────────────────────────

/** Old packages: JSON `{ "0": "img.jpg", … }`. New (schema 18): a
    protobuf of `repeated { name, index }` — walk top-level records
    and count those that decode like entries. Never throws. */
function countMedia(map: Uint8Array | null): number | null {
  if (!map) return null;
  try {
    const text = decoder.decode(map);
    if (text.trimStart().startsWith("{")) {
      const j = JSON.parse(text) as Record<string, unknown>;
      return Object.keys(j).length;
    }
  } catch {
    /* fall through to the protobuf walk */
  }
  try {
    let n = 0;
    let p = 0;
    while (p < map.length) {
      const tag = readVarint(map, p);
      if (!tag) break;
      p = tag.next;
      if ((tag.value & 0x7) !== 2) break; // top level is packed records
      const len = readVarint(map, p);
      if (!len) break;
      p = len.next;
      p += len.value;
      n += 1; // shape drift tolerated — the display is informational
    }
    return n;
  } catch {
    return null;
  }
}

function readVarint(b: Uint8Array, at: number): { value: number; next: number } | null {
  let v = 0;
  let shift = 0;
  for (let i = at; i < b.length; i++) {
    v += (b[i] & 0x7f) * Math.pow(2, shift);
    if ((b[i] & 0x80) === 0) return { value: v, next: i + 1 };
    shift += 7;
    if (shift > 42) return null;
  }
  return null;
}

// ─── entry point ────────────────────────────────────────────────────

const VARIANT_ORDER: Array<[CollectionVariant, RegExp]> = [
  ["anki21b", /(?:^|\/)collection\.anki21b$/],
  ["anki21", /(?:^|\/)collection\.anki21$/],
  ["anki2", /(?:^|\/)collection\.anki2$/],
];

export async function readAnkiContainer(file: Blob): Promise<AnkiContainer> {
  const head = new Uint8Array(await file.slice(0, 8).arrayBuffer());

  // whole-file zstd: a .colpkg tarball, or a bare compressed collection
  if (isZstd(head)) {
    if (file.size > MAX_DECOMPRESSED) {
      throw new AnkiError("toolarge", "This package is too large to open on this device.");
    }
    const raw = await zstd(new Uint8Array(await file.arrayBuffer()), "The package");
    // the tar case: an entry per file, media alongside the collection
    if (decoder.decode(raw.slice(257, 262)) === "ustar") {
      for (const [variant, re] of VARIANT_ORDER) {
        const hit = tarFind(raw, re);
        if (hit && isSqlite(hit)) {
          return { collection: hit, variant, kind: "tar+zstd", mediaCount: null };
        }
      }
      throw new AnkiError("badfile", "No Anki collection inside this package.");
    }
    if (isSqlite(raw)) {
      return { collection: raw, variant: "anki21b", kind: "zstd", mediaCount: null };
    }
    throw new AnkiError("badfile", "This package unpacked to something this app can't read.");
  }

  // the zip case — every .apkg
  if (head[0] !== 0x50 || head[1] !== 0x4b) {
    throw new AnkiError("badfile", "This isn't an Anki package (.apkg or .colpkg).");
  }
  const eocd = await readEocd(file);
  const entries = await readDirectory(file, eocd);
  const byName = (re: RegExp) => entries.find((e) => re.test(e.name));

  let variant: CollectionVariant | null = null;
  let collection: Uint8Array | null = null;
  for (const [v, re] of VARIANT_ORDER) {
    const hit = byName(re);
    if (!hit) continue;
    let bytes = await readEntry(file, hit);
    // .anki21b is itself a zstd frame in some exports, deflate in
    // others — decide by magic, not by the zip method byte
    if (isZstd(bytes)) bytes = await zstd(bytes, "The collection");
    if (isSqlite(bytes)) {
      variant = v;
      collection = bytes;
      break;
    }
  }
  if (!collection || !variant) {
    throw new AnkiError("badfile", "No Anki collection inside this package.");
  }

  const mediaEntry = byName(/(?:^|\/)media$/);
  const mediaCount = mediaEntry ? countMedia(await readEntry(file, mediaEntry)) : null;
  return { collection, variant, kind: "zip", mediaCount };
}
