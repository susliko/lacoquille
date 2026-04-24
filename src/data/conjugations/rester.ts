import type { VerbData } from "./types";

const rester: VerbData = {
  infinitive: "rester",
  translation: "to stay",
  auxiliaire: "être",
  participe: "resté",
  forms: {
    "present-indicatif": {
      je: "reste", tu: "restes", il: "reste",
      nous: "restons", vous: "restez", ils: "restent",
    },
    "imparfait": {
      je: "restais", tu: "restais", il: "restait",
      nous: "restions", vous: "restiez", ils: "restaient",
    },
    "passe-compose": {
      je: "suis resté", tu: "es resté", il: "est resté",
      nous: "sommes restés", vous: "êtes restés", ils: "sont restés",
    },
    "plus-que-parfait": {
      je: "étais resté", tu: "étais resté", il: "était resté",
      nous: "étions restés", vous: "étiez restés", ils: "étaient restés",
    },
    "passe-simple": {
      je: "restai", tu: "restas", il: "resta",
      nous: "restâmes", vous: "restâtes", ils: "restèrent",
    },
    "passe-anterieur": {
      je: "fus resté", tu: "fus resté", il: "fut resté",
      nous: "fûmes restés", vous: "fûtes restés", ils: "furent restés",
    },
    "futur-simple": {
      je: "resterai", tu: "restera", il: "restera",
      nous: "resterons", vous: "resterez", ils: "resteront",
    },
    "futur-anterieur": {
      je: "serai resté", tu: "seras resté", il: "sera resté",
      nous: "serons restés", vous: "serez restés", ils: "seront restés",
    },
    "conditionnel-present": {
      je: "resterais", tu: "resterais", il: "resterait",
      nous: "resterions", vous: "resteriez", ils: "resteraient",
    },
    "conditionnel-passe": {
      je: "serais resté", tu: "serais resté", il: "serait resté",
      nous: "serions restés", vous: "seriez restés", ils: "seraient restés",
    },
    "subjonctif-present": {
      je: "reste", tu: "restes", il: "reste",
      nous: "restions", vous: "restiez", ils: "restent",
    },
    "subjonctif-passe": {
      je: "sois resté", tu: "sois resté", il: "soit resté",
      nous: "soyons restés", vous: "soyez restés", ils: "soient restés",
    },
    "subjonctif-imparfait": {
      je: "restasse", tu: "restasses", il: "restât",
      nous: "restassions", vous: "restassiez", ils: "restassent",
    },
    "subjonctif-plus-que-parfait": {
      je: "fusse resté", tu: "fusses resté", il: "fût resté",
      nous: "fussions restés", vous: "fussiez restés", ils: "fussent restés",
    },
    "imperatif-present": {
      je: null, tu: "reste", il: null,
      nous: "restons", vous: "restez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "sois resté", il: null,
      nous: "soyons restés", vous: "soyez restés", ils: null,
    },
  },
};

export default rester;
