export type Person = "je" | "tu" | "il" | "elle" | "on" | "nous" | "vous" | "ils";

export const PERSONS: Person[] = ["je", "tu", "il", "elle", "on", "nous", "vous", "ils"];

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
  | "imperatif-passe"
  // Verbal periphrases — computed dynamically, no stored forms needed
  | "futur-proche"
  | "passe-recent"
  | "present-progressif";

/** null = this person has no imperative form */
export type TenseForms = Partial<Record<Person, string | null>>;

export interface VerbData {
  infinitive: string;
  translation: string; // brief English gloss, e.g. "to speak"
  auxiliaire: "avoir" | "être";
  participe: string;  // past participle (masc. sg. base form shown; être-verbs get agreement note elsewhere)
  stem?: string;      // override stem for stem/ending highlighting in tables (defaults to infinitive minus last 2 chars)
  forms: Partial<Record<TenseId, TenseForms>>;
}
