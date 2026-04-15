import type { VerbData } from "./types";
import parler from "./parler";
import etre from "./etre";
import avoir from "./avoir";
import aller from "./aller";
import faire from "./faire";

export type { VerbData };
export type { Person, TenseId, TenseForms } from "./types";
export { PERSONS } from "./types";

export const VERBS: Record<string, VerbData> = {
  parler,
  être: etre,
  avoir,
  aller,
  faire,
};

export const VERB_NAMES = Object.keys(VERBS);
