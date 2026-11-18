import { defineCollection } from "astro:content";
import { z } from "zod";
import { glob } from "astro/loaders";

// ── Mood / field enums ────────────────────────────────────────
const MoodEnum = z.enum(["indicatif", "conditionnel", "subjonctif", "imperatif", "nonfinite"]);
const DifficultyEnum = z.enum(["beginner", "intermediate", "advanced"]).default("beginner");
const AspectEnum = z.enum(["completed", "ongoing", "anterior", "habitual", "none"]);
const TimePositionEnum = z.enum(["far-past", "past", "near-past", "present", "near-future", "future", "far-future"]);

// ── Tense reference pages (the conjugation-table surface) ─────
const tenses = defineCollection({
  loader: glob({ pattern: "**/*.md", base: "./src/content/verbs/tenses" }),
  schema: z.object({
    title: z.string(),              // "Présent de l'indicatif"
    mood: MoodEnum,
    timePosition: TimePositionEnum,
    aspect: AspectEnum,
    literary: z.boolean().default(false),
    difficulty: DifficultyEnum,
    oneLineRule: z.string(),        // shown in diagram node card
    pitfalls: z.array(z.string()).optional(),
    relatedTenses: z.array(z.string()).optional(),
    relatedChoicePages: z.array(z.string()).optional(),
    examples: z.array(z.object({
      fr: z.string(),
      en: z.string(),
    })).max(5),
  }),
});

// Grammar reference and verb-guide collections were folded into the
// knowledge sheets (src/data/knowledge/) — see design/prototypes/knowledge/.

export const collections = { tenses };
