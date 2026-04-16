import type { VerbData } from "./types";

// Regular -ir (2nd group) — -iss- infix in plural présent, imparfait, subjonctif
const finir: VerbData = {
  infinitive: "finir",
  translation: "to finish",
  auxiliaire: "avoir",
  participe: "fini",
  stem: "fin",
  forms: {
    "present-indicatif": {
      je: "finis", tu: "finis", il: "finit",
      nous: "finissons", vous: "finissez", ils: "finissent",
    },
    "imparfait": {
      je: "finissais", tu: "finissais", il: "finissait",
      nous: "finissions", vous: "finissiez", ils: "finissaient",
    },
    "passe-compose": {
      je: "ai fini", tu: "as fini", il: "a fini",
      nous: "avons fini", vous: "avez fini", ils: "ont fini",
    },
    "plus-que-parfait": {
      je: "avais fini", tu: "avais fini", il: "avait fini",
      nous: "avions fini", vous: "aviez fini", ils: "avaient fini",
    },
    "passe-simple": {
      je: "finis", tu: "finis", il: "finit",
      nous: "finîmes", vous: "finîtes", ils: "finirent",
    },
    "passe-anterieur": {
      je: "eus fini", tu: "eus fini", il: "eut fini",
      nous: "eûmes fini", vous: "eûtes fini", ils: "eurent fini",
    },
    "futur-simple": {
      je: "finirai", tu: "finiras", il: "finira",
      nous: "finirons", vous: "finirez", ils: "finiront",
    },
    "futur-anterieur": {
      je: "aurai fini", tu: "auras fini", il: "aura fini",
      nous: "aurons fini", vous: "aurez fini", ils: "auront fini",
    },
    "conditionnel-present": {
      je: "finirais", tu: "finirais", il: "finirait",
      nous: "finirions", vous: "finiriez", ils: "finiraient",
    },
    "conditionnel-passe": {
      je: "aurais fini", tu: "aurais fini", il: "aurait fini",
      nous: "aurions fini", vous: "auriez fini", ils: "auraient fini",
    },
    "subjonctif-present": {
      je: "finisse", tu: "finisses", il: "finisse",
      nous: "finissions", vous: "finissiez", ils: "finissent",
    },
    "subjonctif-passe": {
      je: "aie fini", tu: "aies fini", il: "ait fini",
      nous: "ayons fini", vous: "ayez fini", ils: "aient fini",
    },
    "subjonctif-imparfait": {
      je: "finisse", tu: "finisses", il: "finît",
      nous: "finissions", vous: "finissiez", ils: "finissent",
    },
    "subjonctif-plus-que-parfait": {
      je: "eusse fini", tu: "eusses fini", il: "eût fini",
      nous: "eussions fini", vous: "eussiez fini", ils: "eussent fini",
    },
    "imperatif-present": {
      je: null, tu: "finis", il: null,
      nous: "finissons", vous: "finissez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "aie fini", il: null,
      nous: "ayons fini", vous: "ayez fini", ils: null,
    },
  },
};

export default finir;
