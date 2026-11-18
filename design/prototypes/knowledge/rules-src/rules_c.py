# Grammar rule sheets, C1 + C2 — keyed by exact taxonomy name.

R = {
# ── C1 · Tense ──────────────────────────────────────────────
"Passé simple (recognize)": {
  "rules": [
    {"h": "The novel tense", "p": "il parla, il finit, il vendit, il fut, il eut. Lives in literature and formal history. Recognize it; you rarely need to write it.", "ex": {"fr": "Il <em>fut</em> une fois une princesse qui <em>vécut</em> seule.", "en": "Once upon a time there was a princess who lived alone."}},
    {"h": "Map it to passé composé", "p": "il alla = il est allé, elle dit = elle a dit. Same meanings, older clothes.", "ex": {"fr": "Elle <em>arriva</em> à l'aube. (= elle est arrivée)", "en": "She arrived at dawn."}},
  ]},
"Passé antérieur (recognize)": {
  "rules": [
    {"h": "eut fini, fut parti", "p": "The passé simple of avoir or être + participle: the past right before a passé simple event. Purely literary, and almost always after dès que or quand.", "ex": {"fr": "Dès qu'il <em>eut terminé</em>, il partit.", "en": "As soon as he had finished, he left."}},
  ]},
"Agreement of pronominal verbs": {
  "rules": [
    {"h": "Agree unless the object is indirect", "p": "elles se sont levées agrees. But se parler, se téléphoner, se plaire run on à, which makes the object indirect and kills the agreement: elles se sont parlé.", "ex": {"fr": "Elles se <em>sont souri</em>.", "en": "They smiled at each other."}},
    {"h": "A following direct object also blocks it", "p": "elle s'est lavé les mains: les mains is the real object, the participle ignores it.", "ex": {"fr": "Elle s'est lavé <em>les mains</em>.", "en": "She washed her hands."}},
  ]},
# ── C1 · Mood ───────────────────────────────────────────────
"Subtle subjonctif cases": {
  "rules": [
    {"h": "Negation plants doubt", "p": "je pense que goes indicative, but je ne pense pas que flips to subjunctive. Same for il me semble que.", "ex": {"fr": "Je ne crois pas qu'il <em>ait</em> raison.", "en": "I don't think he's right."}},
    {"h": "Questions open the door", "p": "crois-tu que, est-il possible que lean uncertain, so the subjunctive fits.", "ex": {"fr": "Penses-tu qu'elle <em>vienne</em> ?", "en": "Do you think she's coming?"}},
  ]},
"Subjonctif vs indicatif meaning": {
  "rules": [
    {"h": "The same verb, two worlds", "p": "il est possible que (subjunctive) hedges; il est certain que (indicative) commits. Choose the mood first, the verb second.", "ex": {"fr": "Il est possible qu'il <em>pleuve</em> ; il est certain qu'il <em>fait</em> gris.", "en": "It may rain; it's certainly grey."}},
    {"h": "avant que vs après que", "p": "Before it happens, doubt: avant qu'il parte (subjunctive). After it happens, it's history: après qu'il est parti (indicative).", "ex": {"fr": "<em>avant qu'il ne parte</em> / <em>après qu'il est parti</em>", "en": "before he leaves / after he left"}},
  ]},
"Attenuating conditionnel": {
  "rules": [
    {"h": "Softening without si", "p": "j'aurais une question, ce serait peut-être mieux, je voudrais comprendre. The conditionnel hedges an opinion or request in polished speech.", "ex": {"fr": "J'<em>aurais</em> une question, si vous permettez.", "en": "I'd have a question, if you allow."}},
  ]},
"Infinitive clauses": {
  "rules": [
    {"h": "Tight formal prose", "p": "croire savoir, prétendre avoir fini, sans hésiter. Formal French prefers an infinitive clause where speech would use que.", "ex": {"fr": "Il prétend <em>avoir tout vu</em>.", "en": "He claims to have seen everything."}},
    {"h": "Perception verbs keep the infinitive", "p": "After voir, entendre, regarder, écouter, sentir, the second verb stays an infinitive, never a conjugated form or a que-clause.", "ex": {"fr": "Je l'<em>entends chanter</em>.", "en": "I hear her singing."}},
  ]},
"Literary hypotheticals": {
  "rules": [
    {"h": "The inverted si", "p": "Eût-il su, il serait venu: an old subjunctive, inverted, replaces si. High literature only.", "ex": {"fr": "<em>Eût-il su</em>, il <em>serait venu</em>.", "en": "Had he known, he would have come."}},
    {"h": "fût-ce", "p": "A fossil meaning 'even if it were': je le finirai, fût-ce tard.", "ex": {"fr": "Je le finirai, <em>fût-ce</em> tard.", "en": "I'll finish it, even if it's late."}},
  ]},
# ── C1 · Adjectives & adverbs ───────────────────────────────
"Quelque, quel que": {
  "rules": [
    {"h": "Two words, three jobs", "p": "quelque + noun = a few (quelque temps); quelque + adjective = however (quelque difficile); quel que + être = whatever (quel que soit le prix).", "ex": {"fr": "<em>Quel que soit</em> le prix, je paie.", "en": "Whatever the price, I'll pay."}},
  ]},
# ── C1 · Pronouns ───────────────────────────────────────────
"Auquel, duquel & co.": {
  "rules": [
    {"h": "à and de fused into lequel", "p": "auquel, duquel, auxquels, desquelles. For things, after verbs that need à or de: penser à, hériter de.", "ex": {"fr": "le projet <em>auquel</em> je pense.", "en": "the project I'm thinking about."}},
    {"h": "People keep qui", "p": "l'ami avec qui reads warmer than l'ami avec lequel, though both exist.", "ex": {"fr": "un collègue <em>avec qui</em> je m'entends bien.", "en": "a colleague I get along with."}},
  ]},
"Ellipsis & avoiding repetition": {
  "rules": [
    {"h": "celui de, celle qui, le sien", "p": "Replace a repeated noun with a demonstrative or possessive pronoun; it keeps the noun's gender.", "ex": {"fr": "Ma voiture ? C'est celle <em>de</em> ma sœur.", "en": "My car? It's my sister's."}},
  ]},
# ── C1 · Sentence ───────────────────────────────────────────
"Avoir beau, quand bien même": {
  "rules": [
    {"h": "avoir beau = in vain", "p": "avoir beau + infinitive: no matter how hard one tries, it fails. avoir conjugates like avoir.", "ex": {"fr": "Il <em>a beau</em> crier, personne ne l'entend.", "en": "He can shout all he likes, nobody hears him."}},
    {"h": "quand bien même = even if", "p": "Both halves usually in conditionnel: quand bien même il pleuvrait, j'irais.", "ex": {"fr": "<em>Quand bien même</em> il pleuvrait, j'irais.", "en": "Even if it rained, I'd go."}},
  ]},
"Formal connectors": {
  "rules": [
    {"h": "ainsi, en effet, or", "p": "ainsi = thus, en effet = indeed, or = yet (formal, nothing to do with 'or').", "ex": {"fr": "<em>Or</em>, la situation avait changé.", "en": "Yet the situation had changed."}},
    {"h": "The formal mais and donc", "p": "toutefois, néanmoins (however); par conséquent, dès lors (therefore).", "ex": {"fr": "Le budget est faible ; <em>néanmoins</em>, on continue.", "en": "The budget is small; nevertheless, we continue."}},
  ]},
"Formal inversion": {
  "rules": [
    {"h": "The subject-verb flip", "p": "Rarement ai-je vu, à peine est-il arrivé. Formal writing inverts after fronted adverbs like rarement, à peine, peut-être.", "ex": {"fr": "<em>Peut-être viendra-t-</em>il demain.", "en": "Perhaps he'll come tomorrow."}},
  ]},
"Expletive ne": {
  "rules": [
    {"h": "A ne that means nothing", "p": "avant que tu ne partes, de peur qu'il ne pleuve. After certain subjunctive triggers an extra ne appears and adds no negation.", "ex": {"fr": "Je reste jusqu'à ce qu'il <em>ne</em> revienne.", "en": "I'm staying until he comes back."}},
  ]},
"Impersonal formal style": {
  "rules": [
    {"h": "Hide the actor", "p": "il s'agit de = it's about, il convient de = it is fitting to, il importe de = it matters to. Formal texts dodge je and nous.", "ex": {"fr": "<em>Il convient de</em> signaler que le délai expire.", "en": "It should be noted that the deadline expires."}},
  ]},
"Spoken vs written French": {
  "rules": [
    {"h": "Two registers, one grammar", "p": "Spoken: on, ça, pas without ne, futur proche. Written: nous, cela, full negation, futur simple.", "ex": {"fr": "<em>On n'y va pas</em> ↔ Nous n'y allons pas.", "en": "We're not going (spoken vs written)."}},
  ]},
# ── C2 · Tense ──────────────────────────────────────────────
"Passé simple, all persons": {
  "rules": [
    {"h": "The full -er paradigm", "p": "je parlai, tu parlas, il parla, nous parlâmes, vous parlâtes, ils parlèrent. The accented endings carry the tense.", "ex": {"fr": "Ils <em>parlèrent</em> longuement jusqu'à l'aube.", "en": "They spoke at length until dawn."}},
    {"h": "-ir and -re", "p": "je finis, nous finîmes, ils finirent; je vendis, nous vendîmes, ils vendirent. A circumflex marks the nous and vous forms.", "ex": {"fr": "Nous <em>vendîmes</em> la maison l'année suivante.", "en": "We sold the house the following year."}},
  ]},
"Fine participle agreement": {
  "rules": [
    {"h": "laisser now agrees", "p": "With laisser + infinitive, agreement with a preceding object is now standard: les enfants qu'elle a laissés sortir.", "ex": {"fr": "les enfants qu'elle <em>a laissés</em> sortir.", "en": "the children she let go out."}},
    {"h": "courir, vivre, mourir", "p": "These agree with a preceding object too: les risques qu'il a courus, la vie qu'ils ont vécue.", "ex": {"fr": "les risques qu'il <em>a courus</em>.", "en": "the risks he ran."}},
  ]},
# ── C2 · Mood ───────────────────────────────────────────────
"Subjonctif imparfait": {
  "rules": [
    {"h": "qu'il parlât", "p": "Imperfect subjunctive: stem + -asse, -asses, -ât, -assions, -assiez, -assent. Literature only; it maps one-to-one onto the present subjunctive.", "ex": {"fr": "bien qu'il <em>fût</em> tard, ils continuaient.", "en": "although it was late, they carried on."}},
  ],
  "exc": "Its past twin, the subjonctif plus-que-parfait (qu'il eût parlé), is effectively obsolete — the subjonctif passé does the job in modern French."},
"Conditionnel passé 2e forme": {
  "rules": [
    {"h": "j'eusse parlé", "p": "Same meaning as j'aurais parlé. Identical in shape to the literary pluperfect subjunctive; very formal writing only.", "ex": {"fr": "Si j'<em>eusse su</em>, je <em>fusse venu</em>.", "en": "Had I known, I would have come."}},
  ]},
# ── C2 · Adjectives & adverbs ───────────────────────────────
"Adjective position & meaning": {
  "rules": [
    {"h": "A different word per slot", "p": "un grand homme = a great man, un homme grand = a tall man. Before the noun the adjective goes figurative; after it, literal.", "ex": {"fr": "un <em>grand</em> homme ≠ un homme <em>grand</em>.", "en": "a great man ≠ a tall man."}},
    {"h": "The classic pairs", "p": "ancien (former vs antique), cher (dear vs expensive), propre (own vs clean), pauvre (poor soul vs penniless).", "ex": {"fr": "mon <em>pauvre</em> ami (regrettable) / un homme <em>pauvre</em> (sans argent)", "en": "my poor friend (alas) / a poor man (broke)"}},
  ]},
# ── C2 · Sentence ───────────────────────────────────────────
"Literary negations": {
  "rules": [
    {"h": "nul, point, guère", "p": "nul ne = nobody (formal), ne… point = not at all (dated), ne… guère = hardly.", "ex": {"fr": "<em>Nul n'est</em> prophète en son pays.", "en": "No one is a prophet in their own land."}},
  ]},
"Stylistic word order": {
  "rules": [
    {"h": "Front it for focus", "p": "Ce jardin, mon grand-père l'avait planté. Left-dislocation lifts the important thing to the head of the sentence.", "ex": {"fr": "<em>Ce jardin</em>, mon grand-père l'avait planté.", "en": "That garden, my grandfather had planted it."}},
    {"h": "Slow-motion adjectives", "p": "In description, trailing adjectives with commas weigh each one: une robe, verte, éclatante, impossible.", "ex": {"fr": "une robe, <em>verte</em>, <em>éclatante</em>, impossible.", "en": "a dress, green, dazzling, impossible."}},
  ]},
"Archaic fixed phrases": {
  "rules": [
    {"h": "Old grammar frozen into idiom", "p": "n'en déplaise à, somme toute, quitte à, il n'empêche que. You can't rebuild them from modern rules; learn them whole.", "ex": {"fr": "<em>N'en déplaise à</em> ses critiques, son œuvre demeure.", "en": "Much to his critics' displeasure, his work remains."}},
  ]},
"Register shifts": {
  "rules": [
    {"h": "Same idea, three outfits", "p": "bagnole (casual), voiture (neutral), véhicule (formal). Pick one register per text and stay in it.", "ex": {"fr": "<em>bagnole</em> / <em>voiture</em> / <em>véhicule</em>", "en": "jalopy / car / vehicle."}},
  ]},
}
