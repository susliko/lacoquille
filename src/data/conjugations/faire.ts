import type { VerbData } from "./types";

const faire: VerbData = {
  infinitive: "faire",
  translation: "to do / make",
  auxiliaire: "avoir",
  participe: "fait",
  forms: {
    "present-indicatif": {
      je: "fais", tu: "fais", il: "fait",
      nous: "faisons", vous: "faites", ils: "font",
    },
    "imparfait": {
      je: "faisais", tu: "faisais", il: "faisait",
      nous: "faisions", vous: "faisiez", ils: "faisaient",
    },
    "passe-compose": {
      je: "ai fait", tu: "as fait", il: "a fait",
      nous: "avons fait", vous: "avez fait", ils: "ont fait",
    },
    "plus-que-parfait": {
      je: "avais fait", tu: "avais fait", il: "avait fait",
      nous: "avions fait", vous: "aviez fait", ils: "avaient fait",
    },
    "passe-simple": {
      je: "fis", tu: "fis", il: "fit",
      nous: "fîmes", vous: "fîtes", ils: "firent",
    },
    "passe-anterieur": {
      je: "eus fait", tu: "eus fait", il: "eut fait",
      nous: "eûmes fait", vous: "eûtes fait", ils: "eurent fait",
    },
    "futur-simple": {
      je: "ferai", tu: "feras", il: "fera",
      nous: "ferons", vous: "ferez", ils: "feront",
    },
    "futur-anterieur": {
      je: "aurai fait", tu: "auras fait", il: "aura fait",
      nous: "aurons fait", vous: "aurez fait", ils: "auront fait",
    },
    "conditionnel-present": {
      je: "ferais", tu: "ferais", il: "ferait",
      nous: "ferions", vous: "feriez", ils: "feraient",
    },
    "conditionnel-passe": {
      je: "aurais fait", tu: "aurais fait", il: "aurait fait",
      nous: "aurions fait", vous: "auriez fait", ils: "auraient fait",
    },
    "subjonctif-present": {
      je: "fasse", tu: "fasses", il: "fasse",
      nous: "fassions", vous: "fassiez", ils: "fassent",
    },
    "subjonctif-passe": {
      je: "aie fait", tu: "aies fait", il: "ait fait",
      nous: "ayons fait", vous: "ayez fait", ils: "aient fait",
    },
    "subjonctif-imparfait": {
      je: "fisse", tu: "fisses", il: "fît",
      nous: "fissions", vous: "fissiez", ils: "fissent",
    },
    "subjonctif-plus-que-parfait": {
      je: "eusse fait", tu: "eusses fait", il: "eût fait",
      nous: "eussions fait", vous: "eussiez fait", ils: "eussent fait",
    },
    "imperatif-present": {
      je: null, tu: "fais", il: null,
      nous: "faisons", vous: "faites", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "aie fait", il: null,
      nous: "ayons fait", vous: "ayez fait", ils: null,
    },
  },
};

export default faire;
