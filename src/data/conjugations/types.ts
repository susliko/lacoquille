export type Person = "je" | "tu" | "il" | "nous" | "vous" | "ils";

export const PERSONS: Person[] = ["je", "tu", "il", "nous", "vous", "ils"];

export type TenseId =
  | "present-indicatif"
  | "imparfait"
  | "passe-compose"
  | "plus-que-parfait"
  | "passe-simple"
  | "passe-anterieur"
  | "futur-simple"
  | "futur-anterieur"
  | "conditionnel-present"
  | "conditionnel-passe"
  | "subjonctif-present"
  | "subjonctif-passe"
  | "subjonctif-imparfait"
  | "subjonctif-plus-que-parfait"
  | "imperatif-present"
  | "imperatif-passe";

/** null = this person has no imperative form */
export type TenseForms = Record<Person, string | null>;

export interface VerbData {
  infinitive: string;
  translation: string; // brief English gloss, e.g. "to speak"
  auxiliaire: "avoir" | "être";
  participe: string;  // past participle (masc. sg. base form shown; être-verbs get agreement note elsewhere)
  forms: Partial<Record<TenseId, TenseForms>>;
}
