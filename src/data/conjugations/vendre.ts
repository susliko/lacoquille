import type { VerbData } from "./types";

// Regular -re (3rd group)
const vendre: VerbData = {
  infinitive: "vendre",
  translation: "to sell",
  auxiliaire: "avoir",
  participe: "vendu",
  stem: "vend",
  forms: {
    "present-indicatif": {
      je: "vends", tu: "vends", il: "vend",
      nous: "vendons", vous: "vendez", ils: "vendent",
    },
    "imparfait": {
      je: "vendais", tu: "vendais", il: "vendait",
      nous: "vendions", vous: "vendiez", ils: "vendaient",
    },
    "passe-compose": {
      je: "ai vendu", tu: "as vendu", il: "a vendu",
      nous: "avons vendu", vous: "avez vendu", ils: "ont vendu",
    },
    "plus-que-parfait": {
      je: "avais vendu", tu: "avais vendu", il: "avait vendu",
      nous: "avions vendu", vous: "aviez vendu", ils: "avaient vendu",
    },
    "passe-simple": {
      je: "vendis", tu: "vendis", il: "vendit",
      nous: "vendîmes", vous: "vendîtes", ils: "vendirent",
    },
    "passe-anterieur": {
      je: "eus vendu", tu: "eus vendu", il: "eut vendu",
      nous: "eûmes vendu", vous: "eûtes vendu", ils: "eurent vendu",
    },
    "futur-simple": {
      je: "vendrai", tu: "vendras", il: "vendra",
      nous: "vendrons", vous: "vendrez", ils: "vendront",
    },
    "futur-anterieur": {
      je: "aurai vendu", tu: "auras vendu", il: "aura vendu",
      nous: "aurons vendu", vous: "aurez vendu", ils: "auront vendu",
    },
    "conditionnel-present": {
      je: "vendrais", tu: "vendrais", il: "vendrait",
      nous: "vendrions", vous: "vendriez", ils: "vendraient",
    },
    "conditionnel-passe": {
      je: "aurais vendu", tu: "aurais vendu", il: "aurait vendu",
      nous: "aurions vendu", vous: "auriez vendu", ils: "auraient vendu",
    },
    "subjonctif-present": {
      je: "vende", tu: "vendes", il: "vende",
      nous: "vendions", vous: "vendiez", ils: "vendent",
    },
    "subjonctif-passe": {
      je: "aie vendu", tu: "aies vendu", il: "ait vendu",
      nous: "ayons vendu", vous: "ayez vendu", ils: "aient vendu",
    },
    "subjonctif-imparfait": {
      je: "vendisse", tu: "vendisses", il: "vendît",
      nous: "vendissions", vous: "vendissiez", ils: "vendissent",
    },
    "subjonctif-plus-que-parfait": {
      je: "eusse vendu", tu: "eusses vendu", il: "eût vendu",
      nous: "eussions vendu", vous: "eussiez vendu", ils: "eussent vendu",
    },
    "imperatif-present": {
      je: null, tu: "vends", il: null,
      nous: "vendons", vous: "vendez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "aie vendu", il: null,
      nous: "ayons vendu", vous: "ayez vendu", ils: null,
    },
  },
};

export default vendre;
