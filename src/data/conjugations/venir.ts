import type { VerbData } from "./types";

const venir: VerbData = {
  infinitive: "venir",
  translation: "to come",
  auxiliaire: "être",
  participe: "venu",
  forms: {
    "present-indicatif": {
      je: "viens", tu: "viens", il: "vient",
      nous: "venons", vous: "venez", ils: "viennent",
    },
    "imparfait": {
      je: "venais", tu: "venais", il: "venait",
      nous: "venions", vous: "veniez", ils: "venaient",
    },
    "passe-compose": {
      je: "suis venu", tu: "es venu", il: "est venu",
      nous: "sommes venus", vous: "êtes venus", ils: "sont venus",
    },
    "plus-que-parfait": {
      je: "étais venu", tu: "étais venu", il: "était venu",
      nous: "étions venus", vous: "étiez venus", ils: "étaient venus",
    },
    "passe-simple": {
      je: "vins", tu: "vins", il: "vint",
      nous: "vînmes", vous: "vîntes", ils: "vinrent",
    },
    "passe-anterieur": {
      je: "fus venu", tu: "fus venu", il: "fut venu",
      nous: "fûmes venus", vous: "fûtes venus", ils: "furent venus",
    },
    "futur-simple": {
      je: "viendrai", tu: "viendras", il: "viendra",
      nous: "viendrons", vous: "viendrez", ils: "viendront",
    },
    "futur-anterieur": {
      je: "serai venu", tu: "seras venu", il: "sera venu",
      nous: "serons venus", vous: "serez venus", ils: "seront venus",
    },
    "conditionnel-present": {
      je: "viendrais", tu: "viendrais", il: "viendrait",
      nous: "viendrions", vous: "viendriez", ils: "viendraient",
    },
    "conditionnel-passe": {
      je: "serais venu", tu: "serais venu", il: "serait venu",
      nous: "serions venus", vous: "seriez venus", ils: "seraient venus",
    },
    "subjonctif-present": {
      je: "vienne", tu: "viennes", il: "vienne",
      nous: "venions", vous: "veniez", ils: "viennent",
    },
    "subjonctif-passe": {
      je: "sois venu", tu: "sois venu", il: "soit venu",
      nous: "soyons venus", vous: "soyez venus", ils: "soient venus",
    },
    "subjonctif-imparfait": {
      je: "vinsse", tu: "vinsses", il: "vînt",
      nous: "vinssions", vous: "vinssiez", ils: "vinssent",
    },
    "subjonctif-plus-que-parfait": {
      je: "fusse venu", tu: "fusses venu", il: "fût venu",
      nous: "fussions venus", vous: "fussiez venus", ils: "fussent venus",
    },
    "imperatif-present": {
      je: null, tu: "viens", il: null,
      nous: "venons", vous: "venez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "sois venu", il: null,
      nous: "soyons venus", vous: "soyez venus", ils: null,
    },
  },
};

export default venir;
