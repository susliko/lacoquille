import type { VerbData } from "./types";

const partir: VerbData = {
  infinitive: "partir",
  translation: "to leave",
  auxiliaire: "être",
  participe: "parti",
  forms: {
    "present-indicatif": {
      je: "pars", tu: "pars", il: "part",
      nous: "partons", vous: "partez", ils: "partent",
    },
    "imparfait": {
      je: "partais", tu: "partais", il: "partait",
      nous: "partions", vous: "partiez", ils: "partaient",
    },
    "passe-compose": {
      je: "suis parti", tu: "es parti", il: "est parti",
      nous: "sommes partis", vous: "êtes partis", ils: "sont partis",
    },
    "plus-que-parfait": {
      je: "étais parti", tu: "étais parti", il: "était parti",
      nous: "étions partis", vous: "étiez partis", ils: "étaient partis",
    },
    "passe-simple": {
      je: "partis", tu: "partis", il: "partit",
      nous: "partîmes", vous: "partîtes", ils: "partirent",
    },
    "passe-anterieur": {
      je: "fus parti", tu: "fus parti", il: "fut parti",
      nous: "fûmes partis", vous: "fûtes partis", ils: "furent partis",
    },
    "futur-simple": {
      je: "partira", tu: "partiras", il: "partira",
      nous: "partirons", vous: "partirez", ils: "partiront",
    },
    "futur-anterieur": {
      je: "serai parti", tu: "seras parti", il: "sera parti",
      nous: "serons partis", vous: "serez partis", ils: "seront partis",
    },
    "conditionnel-present": {
      je: "partirais", tu: "partirais", il: "partirait",
      nous: "partirions", vous: "partiriez", ils: "partiraient",
    },
    "conditionnel-passe": {
      je: "serais parti", tu: "serais parti", il: "serait parti",
      nous: "serions partis", vous: "seriez partis", ils: "seraient partis",
    },
    "subjonctif-present": {
      je: "parte", tu: "partes", il: "parte",
      nous: "partions", vous: "partiez", ils: "partent",
    },
    "subjonctif-passe": {
      je: "sois parti", tu: "sois parti", il: "soit parti",
      nous: "soyons partis", vous: "soyez partis", ils: "soient partis",
    },
    "subjonctif-imparfait": {
      je: "partisse", tu: "partisses", il: "partît",
      nous: "partissions", vous: "partissiez", ils: "partissent",
    },
    "subjonctif-plus-que-parfait": {
      je: "fusse parti", tu: "fusses parti", il: "fût parti",
      nous: "fussions partis", vous: "fussiez partis", ils: "fussent partis",
    },
    "imperatif-present": {
      je: null, tu: "pars", il: null,
      nous: "partons", vous: "partez", ils: null,
    },
    "imperatif-passe": {
      je: null, tu: "sois parti", il: null,
      nous: "soyons partis", vous: "soyez partis", ils: null,
    },
  },
};

export default partir;
