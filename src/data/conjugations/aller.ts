import type { VerbData } from "./types";

// aller uses être as auxiliary; past participle allé agrees with subject
const aller: VerbData = {
  infinitive: "aller",
  translation: "to go",
  auxiliaire: "être",
  participe: "allé",
  forms: {
    "present-indicatif": {
      je: "vais", tu: "vas", il: "va",
      nous: "allons", vous: "allez", ils: "vont",
    },
    "imparfait": {
      je: "allais", tu: "allais", il: "allait",
      nous: "allions", vous: "alliez", ils: "allaient",
    },
    "passe-compose": {
      je: "suis allé", tu: "es allé", il: "est allé",
      nous: "sommes allés", vous: "êtes allés", ils: "sont allés",
    },
    "plus-que-parfait": {
      je: "étais allé", tu: "étais allé", il: "était allé",
      nous: "étions allés", vous: "étiez allés", ils: "étaient allés",
    },
    "passe-simple": {
      je: "allai", tu: "allas", il: "alla",
      nous: "allâmes", vous: "allâtes", ils: "allèrent",
    },
    "passe-anterieur": {
      je: "fus allé", tu: "fus allé", il: "fut allé",
      nous: "fûmes allés", vous: "fûtes allés", ils: "furent allés",
    },
    "futur-simple": {
      je: "irai", tu: "iras", il: "ira",
      nous: "irons", vous: "irez", ils: "iront",
    },
    "futur-anterieur": {
      je: "serai allé", tu: "seras allé", il: "sera allé",
      nous: "serons allés", vous: "serez allés", ils: "seront allés",
    },
    "conditionnel-present": {
      je: "irais", tu: "irais", il: "irait",
      nous: "irions", vous: "iriez", ils: "iraient",
    },
    "conditionnel-passe": {
      je: "serais allé", tu: "serais allé", il: "serait allé",
      nous: "serions allés", vous: "seriez allés", ils: "seraient allés",
    },
    "subjonctif-present": {
      je: "aille", tu: "ailles", il: "aille",
      nous: "allions", vous: "alliez", ils: "aillent",
    },
    "subjonctif-passe": {
      je: "sois allé", tu: "sois allé", il: "soit allé",
      nous: "soyons allés", vous: "soyez allés", ils: "soient allés",
    },
    "subjonctif-imparfait": {
      je: "allasse", tu: "allasses", il: "allât",
      nous: "allassions", vous: "allassiez", ils: "allassent",
    },
    "subjonctif-plus-que-parfait": {
      je: "fusse allé", tu: "fusses allé", il: "fût allé",
      nous: "fussions allés", vous: "fussiez allés", ils: "fussent allés",
    },
    "imperatif-present": {
      je: null, tu: "va", il: null,
      nous: "allons", vous: "allez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "sois allé", il: null,
      nous: "soyons allés", vous: "soyez allés", ils: null,
    },
  },
};

export default aller;
