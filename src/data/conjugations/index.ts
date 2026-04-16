import type { VerbData } from "./types";
import parler from "./parler";
import finir  from "./finir";
import vendre from "./vendre";
import etre   from "./etre";
import avoir  from "./avoir";
import aller  from "./aller";
import faire  from "./faire";

export type { VerbData };
export type { Person, TenseId, TenseForms } from "./types";
export { PERSONS } from "./types";

export const VERBS: Record<string, VerbData> = {
  parler,
  finir,
  vendre,
  "être": etre,
  avoir,
  aller,
  faire,
};

/** Ordered selector options: group entries first, then irregulars */
export const VERB_OPTIONS: { value: string; label: string }[] = [
  { value: "parler", label: "−er  (parler)" },
  { value: "finir",  label: "−ir  (finir)"  },
  { value: "vendre", label: "−re  (vendre)" },
  { value: "être",   label: "être"           },
  { value: "avoir",  label: "avoir"          },
  { value: "aller",  label: "aller"          },
  { value: "faire",  label: "faire"          },
];
