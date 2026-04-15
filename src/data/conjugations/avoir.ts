import type { VerbData } from "./types";

const avoir: VerbData = {
  infinitive: "avoir",
  translation: "to have",
  auxiliaire: "avoir",
  participe: "eu",
  forms: {
    "present-indicatif": {
      je: "ai", tu: "as", il: "a",
      nous: "avons", vous: "avez", ils: "ont",
    },
    "imparfait": {
      je: "avais", tu: "avais", il: "avait",
      nous: "avions", vous: "aviez", ils: "avaient",
    },
    "passe-compose": {
      je: "ai eu", tu: "as eu", il: "a eu",
      nous: "avons eu", vous: "avez eu", ils: "ont eu",
    },
    "plus-que-parfait": {
      je: "avais eu", tu: "avais eu", il: "avait eu",
      nous: "avions eu", vous: "aviez eu", ils: "avaient eu",
    },
    "passe-simple": {
      je: "eus", tu: "eus", il: "eut",
      nous: "eûmes", vous: "eûtes", ils: "eurent",
    },
    "passe-anterieur": {
      je: "eus eu", tu: "eus eu", il: "eut eu",
      nous: "eûmes eu", vous: "eûtes eu", ils: "eurent eu",
    },
    "futur-simple": {
      je: "aurai", tu: "auras", il: "aura",
      nous: "aurons", vous: "aurez", ils: "auront",
    },
    "futur-anterieur": {
      je: "aurai eu", tu: "auras eu", il: "aura eu",
      nous: "aurons eu", vous: "aurez eu", ils: "auront eu",
    },
    "conditionnel-present": {
      je: "aurais", tu: "aurais", il: "aurait",
      nous: "aurions", vous: "auriez", ils: "auraient",
    },
    "conditionnel-passe": {
      je: "aurais eu", tu: "aurais eu", il: "aurait eu",
      nous: "aurions eu", vous: "auriez eu", ils: "auraient eu",
    },
    "subjonctif-present": {
      je: "aie", tu: "aies", il: "ait",
      nous: "ayons", vous: "ayez", ils: "aient",
    },
    "subjonctif-passe": {
      je: "aie eu", tu: "aies eu", il: "ait eu",
      nous: "ayons eu", vous: "ayez eu", ils: "aient eu",
    },
    "subjonctif-imparfait": {
      je: "eusse", tu: "eusses", il: "eût",
      nous: "eussions", vous: "eussiez", ils: "eussent",
    },
    "subjonctif-plus-que-parfait": {
      je: "eusse eu", tu: "eusses eu", il: "eût eu",
      nous: "eussions eu", vous: "eussiez eu", ils: "eussent eu",
    },
    "imperatif-present": {
      je: null, tu: "aie", il: null,
      nous: "ayons", vous: "ayez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "aie eu", il: null,
      nous: "ayons eu", vous: "ayez eu", ils: null,
    },
  },
};

export default avoir;
