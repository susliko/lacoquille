import type { VerbData } from "./types";

const parler: VerbData = {
  infinitive: "parler",
  translation: "to speak",
  auxiliaire: "avoir",
  participe: "parlé",
  forms: {
    "present-indicatif": {
      je: "parle", tu: "parles", il: "parle",
      nous: "parlons", vous: "parlez", ils: "parlent",
    },
    "imparfait": {
      je: "parlais", tu: "parlais", il: "parlait",
      nous: "parlions", vous: "parliez", ils: "parlaient",
    },
    "passe-compose": {
      je: "ai parlé", tu: "as parlé", il: "a parlé",
      nous: "avons parlé", vous: "avez parlé", ils: "ont parlé",
    },
    "plus-que-parfait": {
      je: "avais parlé", tu: "avais parlé", il: "avait parlé",
      nous: "avions parlé", vous: "aviez parlé", ils: "avaient parlé",
    },
    "passe-simple": {
      je: "parlai", tu: "parlas", il: "parla",
      nous: "parlâmes", vous: "parlâtes", ils: "parlèrent",
    },
    "passe-anterieur": {
      je: "eus parlé", tu: "eus parlé", il: "eut parlé",
      nous: "eûmes parlé", vous: "eûtes parlé", ils: "eurent parlé",
    },
    "futur-simple": {
      je: "parlerai", tu: "parleras", il: "parlera",
      nous: "parlerons", vous: "parlerez", ils: "parleront",
    },
    "futur-anterieur": {
      je: "aurai parlé", tu: "auras parlé", il: "aura parlé",
      nous: "aurons parlé", vous: "aurez parlé", ils: "auront parlé",
    },
    "conditionnel-present": {
      je: "parlerais", tu: "parlerais", il: "parlerait",
      nous: "parlerions", vous: "parleriez", ils: "parleraient",
    },
    "conditionnel-passe": {
      je: "aurais parlé", tu: "aurais parlé", il: "aurait parlé",
      nous: "aurions parlé", vous: "auriez parlé", ils: "auraient parlé",
    },
    "subjonctif-present": {
      je: "parle", tu: "parles", il: "parle",
      nous: "parlions", vous: "parliez", ils: "parlent",
    },
    "subjonctif-passe": {
      je: "aie parlé", tu: "aies parlé", il: "ait parlé",
      nous: "ayons parlé", vous: "ayez parlé", ils: "aient parlé",
    },
    "subjonctif-imparfait": {
      je: "parlasse", tu: "parlasses", il: "parlât",
      nous: "parlassions", vous: "parlassiez", ils: "parlassent",
    },
    "subjonctif-plus-que-parfait": {
      je: "eusse parlé", tu: "eusses parlé", il: "eût parlé",
      nous: "eussions parlé", vous: "eussiez parlé", ils: "eussent parlé",
    },
    // Imperative only has 3 persons: 2sg, 1pl, 2pl
    "imperatif-present": {
      je: null, tu: "parle", il: null,
      nous: "parlons", vous: "parlez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "aie parlé", il: null,
      nous: "ayons parlé", vous: "ayez parlé", ils: null,
    },
  },
};

export default parler;
