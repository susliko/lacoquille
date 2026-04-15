import type { VerbData } from "./types";

// être is its own auxiliary in compound tenses; past participle agrees with subject
// Forms shown here are masc. sg. — agreement note is handled in the UI component
const etre: VerbData = {
  infinitive: "être",
  translation: "to be",
  auxiliaire: "être",
  participe: "été",
  forms: {
    "present-indicatif": {
      je: "suis", tu: "es", il: "est",
      nous: "sommes", vous: "êtes", ils: "sont",
    },
    "imparfait": {
      je: "étais", tu: "étais", il: "était",
      nous: "étions", vous: "étiez", ils: "étaient",
    },
    "passe-compose": {
      je: "ai été", tu: "as été", il: "a été",
      nous: "avons été", vous: "avez été", ils: "ont été",
    },
    "plus-que-parfait": {
      je: "avais été", tu: "avais été", il: "avait été",
      nous: "avions été", vous: "aviez été", ils: "avaient été",
    },
    "passe-simple": {
      je: "fus", tu: "fus", il: "fut",
      nous: "fûmes", vous: "fûtes", ils: "furent",
    },
    "passe-anterieur": {
      je: "eus été", tu: "eus été", il: "eut été",
      nous: "eûmes été", vous: "eûtes été", ils: "eurent été",
    },
    "futur-simple": {
      je: "serai", tu: "seras", il: "sera",
      nous: "serons", vous: "serez", ils: "seront",
    },
    "futur-anterieur": {
      je: "aurai été", tu: "auras été", il: "aura été",
      nous: "aurons été", vous: "aurez été", ils: "auront été",
    },
    "conditionnel-present": {
      je: "serais", tu: "serais", il: "serait",
      nous: "serions", vous: "seriez", ils: "seraient",
    },
    "conditionnel-passe": {
      je: "aurais été", tu: "aurais été", il: "aurait été",
      nous: "aurions été", vous: "auriez été", ils: "auraient été",
    },
    "subjonctif-present": {
      je: "sois", tu: "sois", il: "soit",
      nous: "soyons", vous: "soyez", ils: "soient",
    },
    "subjonctif-passe": {
      je: "aie été", tu: "aies été", il: "ait été",
      nous: "ayons été", vous: "ayez été", ils: "aient été",
    },
    "subjonctif-imparfait": {
      je: "fusse", tu: "fusses", il: "fût",
      nous: "fussions", vous: "fussiez", ils: "fussent",
    },
    "subjonctif-plus-que-parfait": {
      je: "eusse été", tu: "eusses été", il: "eût été",
      nous: "eussions été", vous: "eussiez été", ils: "eussent été",
    },
    "imperatif-present": {
      je: null, tu: "sois", il: null,
      nous: "soyons", vous: "soyez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "aie été", il: null,
      nous: "ayons été", vous: "ayez été", ils: null,
    },
  },
};

export default etre;
