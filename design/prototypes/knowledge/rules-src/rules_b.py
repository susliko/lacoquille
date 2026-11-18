# Grammar rule sheets, B1 + B2 — keyed by exact taxonomy name.

R = {
# ── B1 · Tense ──────────────────────────────────────────────
"Plus-que-parfait": {
  "rules": [
    {"h": "The past before the past", "p": "Imparfait of avoir or être + participle: j'avais mangé, j'étais parti. For an action already done before another past one.", "ex": {"fr": "Quand je suis arrivé, ils <em>avaient déjà</em> mangé.", "en": "When I arrived, they had already eaten."}},
  ],
  "see": ["Imparfait"]},
"Past tenses in a story": {
  "rules": [
    {"h": "Imparfait sets, passé composé acts", "p": "Descriptions and states take imparfait; events take passé composé, or passé simple in writing.", "ex": {"fr": "La nuit <em>tombait</em> quand le téléphone <em>a sonné</em>.", "en": "Night was falling when the phone rang."}},
    {"h": "Plus-que-parfait for backstory", "p": "Anything that happened before your story's timeline gets plus-que-parfait.", "ex": {"fr": "Elle <em>avait oublié</em> ses clés, donc elle attendait dehors.", "en": "She had forgotten her keys, so she waited outside."}},
  ]},
"Agreement with preceding object": {
  "rules": [
    {"h": "avoir agrees when the object goes first", "p": "The participle agrees with a direct object placed before the verb: les fleurs que j'ai achetées, je les ai vues.", "ex": {"fr": "les lettres qu'elle <em>a écrites</em>.", "en": "the letters she wrote."}},
    {"h": "Object after = no agreement", "p": "j'ai écrit des lettres stays put. Only a preposed object (relative que or an object pronoun) triggers agreement.", "ex": {"fr": "Je les <em>ai vues</em> hier.", "en": "I saw them (them = women) yesterday."}},
  ]},
# ── B1 · Mood ───────────────────────────────────────────────
"Conditionnel présent: forms": {
  "rules": [
    {"h": "Futur stem + imparfait endings", "p": "je parlerais, je finirais. Take the futur stem and hang the imparfait endings on it: -ais, -ais, -ait, -ions, -iez, -aient.", "ex": {"fr": "J'<em>aimerais</em> un verre d'eau.", "en": "I'd like a glass of water."}},
    {"h": "The stems copy the futur simple", "p": "Every irregular futur stem carries over: je serais, j'aurais, j'irais, je ferais, je viendrais, je verrais, je saurais, je pourrais, je voudrais, je devrais.", "ex": {"fr": "À ta place, je <em>ferais</em> attention.", "en": "In your place, I'd be careful."}},
  ]},
"Conditionnel présent: uses": {
  "rules": [
    {"h": "Wish, request, hypothesis", "p": "je voudrais softens a request, j'aimerais states a wish, and the conditionnel completes an unreal si-clause.", "ex": {"fr": "Je <em>viendrais</em> bien, mais je travaille.", "en": "I'd love to come, but I'm working."}},
    {"h": "The future of yesterday", "p": "In reported speech and news, a past future becomes conditionnel: il a dit qu'il viendrait.", "ex": {"fr": "Elle a promis qu'elle <em>appellerait</em>.", "en": "She promised she would call."}},
    {"h": "The journalist's conditionnel", "p": "The conditionnel marks unconfirmed, second-hand information — the speaker isn't vouching for it. Constant in news.", "ex": {"fr": "Le suspect <em>serait</em> en fuite.", "en": "The suspect is reportedly on the run."}},
  ]},
"Subjonctif présent: forms": {
  "rules": [
    {"h": "Mostly looks like the present", "p": "que je parle, que je finisse, que je vende. nous and vous borrow the imparfait stem: que je vienne, que nous venions.", "ex": {"fr": "Il faut que tu <em>viennes</em>.", "en": "You have to come."}},
    {"h": "The irregulars", "p": "être → soit, avoir → ait, aller → aille, faire → fasse, pouvoir → puisse, savoir → sache, vouloir → veuille, valoir → vaille, falloir → qu'il faille, pleuvoir → qu'il pleuve.", "ex": {"fr": "pour que tu <em>sois</em> là.", "en": "so that you're there."}},
  ]},
"Subjonctif after feelings & il faut": {
  "rules": [
    {"h": "Feelings and wants trigger it", "p": "After je veux que, je suis content que, il est dommage que, c'est triste que, the next verb goes subjunctive.", "ex": {"fr": "Je suis ravi que tu <em>sois</em> là.", "en": "I'm delighted you're here."}},
    {"h": "il faut que + subjunctive", "p": "il faut que tu fasses. With no second subject, keep the infinitive: il faut partir.", "ex": {"fr": "Il faut qu'on <em>parte</em>.", "en": "We have to leave."}},
  ],
  "exc": "Certainty verbs go the other way: je sais que, il est clair que take the indicative.",
  "see": ["Subjonctif présent: forms", "Subjonctif vs indicatif"]},
"Subjonctif vs indicatif": {
  "rules": [
    {"h": "Doubt vs certainty", "p": "Subjective or uncertain triggers (il faut que, bien que, je doute que) take subjunctive. Stated as fact (je pense que, il est certain que) take indicative.", "ex": {"fr": "Je pense qu'il <em>est</em> là, mais je doute qu'il <em>reste</em>.", "en": "I think he's there, but I doubt he'll stay."}},
    {"h": "Opinion flips with negation", "p": "je pense que → indicative; je ne pense pas que → subjunctive. Negating an opinion plants doubt.", "ex": {"fr": "Je ne pense pas qu'elle <em>vienne</em>.", "en": "I don't think she's coming."}},
  ]},
"Passive voice": {
  "rules": [
    {"h": "être + participle", "p": "The thing becomes the subject: ce tableau a été peint par Picasso. par introduces the doer.", "ex": {"fr": "La souris <em>a été mangée</em> par le chat.", "en": "The mouse was eaten by the cat."}},
    {"h": "Spoken French avoids it", "p": "On + active verb does the same job and sounds normal: on a volé ma voiture.", "ex": {"fr": "<em>On m'a volé</em> mon vélo.", "en": "Someone stole my bike."}},
  ]},
"Gérondif": {
  "rules": [
    {"h": "en + -ant", "p": "Means 'by doing' or 'while doing'. The en-form shares its subject with the main verb, and the -ant never agrees.", "ex": {"fr": "Il apprend le français <em>en écoutant</em> la radio ; elles chantaient <em>en marchant</em>.", "en": "He learns French by listening to the radio; they sang while walking."}},
    {"h": "Form it from the nous form", "p": "Drop -ons, add -ant: parler → en parlant, finir → en finissant. The odd ones: avoir → ayant, savoir → sachant.", "ex": {"fr": "C'est en <em>sachant</em> écouter qu'on apprend.", "en": "You learn by knowing how to listen."}},
  ]},
# ── B1 · Nouns & articles ───────────────────────────────────
"Nominalization": {
  "rules": [
    {"h": "Verb becomes noun", "p": "arriver → l'arrivée, développer → le développement. Formal writing prefers the noun version of an action.", "ex": {"fr": "<em>La construction</em> du pont a pris deux ans.", "en": "The construction of the bridge took two years."}},
    {"h": "Adjective becomes noun", "p": "utile → l'utilité, possible → la possibilité. -ité and -(t)ion do most of the work.", "ex": {"fr": "C'est d'une grande <em>utilité</em>.", "en": "It's of great use."}},
    {"h": "The infinitive as a noun", "p": "The infinitive itself can be the subject or object where English uses the -ing form: mentir est mal, j'aime dormir.", "ex": {"fr": "<em>Mentir</em> est mal.", "en": "Lying is wrong."}},
  ]},
# ── B1 · Pronouns ───────────────────────────────────────────
"Double object pronouns": {
  "rules": [
    {"h": "The order line", "p": "me te nous vous → le la les → lui leur → y → en. Il me le donne fits all three first slots.", "ex": {"fr": "Elle <em>me l'</em>a offert pour mon anniversaire.", "en": "She gave it to me for my birthday."}},
    {"h": "The imperative exception", "p": "Positive commands attach the pronouns after the verb: donne-le-moi. Negative ones wrap as usual: ne me le donne pas.", "ex": {"fr": "Donnez-<em>le-moi</em> !", "en": "Give it to me!"}},
    {"h": "Elide when they touch", "p": "me, te, le, la shrink before y and en: m'y, t'en, l'y, l'en. And lui/leur never combine with y — one or the other.", "ex": {"fr": "Il <em>m'y</em> envoie demain.", "en": "He's sending me there tomorrow."}},
  ]},
"Dont": {
  "rules": [
    {"h": "dont = de + which", "p": "For anything the verb links with de: le livre dont je parle (parler de), la chanson dont il se souvient (se souvenir de).", "ex": {"fr": "le film <em>dont</em> tout le monde parle.", "en": "the film everyone's talking about."}},
    {"h": "dont also owns", "p": "Possession with de: la voisine dont le chien aboie. Whose, when the sentence runs on de.", "ex": {"fr": "un auteur <em>dont</em> les livres se vendent bien.", "en": "an author whose books sell well."}},
  ]},
"Compound relatives": {
  "rules": [
    {"h": "Preposition + lequel", "p": "avec lequel, à laquelle, dans lesquels. lequel agrees in gender and number with what it replaces.", "ex": {"fr": "la table <em>sur laquelle</em> je travaille.", "en": "the table I work on."}},
    {"h": "People still get qui", "p": "After a preposition naming a person, qui does the job: l'ami avec qui je voyage.", "ex": {"fr": "un ami <em>avec qui</em> je voyage souvent.", "en": "a friend I often travel with."}},
  ]},
"Indefinite pronouns": {
  "rules": [
    {"h": "someone, everyone, nobody", "p": "quelqu'un, tout le monde, personne ne, rien ne, quelque chose, tout. personne and rien need ne.", "ex": {"fr": "<em>Tout le monde</em> est là, <em>personne ne</em> manque.", "en": "Everyone's here, nobody's missing."}},
    {"h": "de before an adjective", "p": "quelque chose, quelqu'un, rien take de + adjective: quelque chose de beau, quelqu'un de gentil, rien d'intéressant.", "ex": {"fr": "C'est <em>quelqu'un de</em> très gentil.", "en": "He/she is someone very kind."}},
    {"h": "The rest of the band", "p": "chacun = each one, quelques-unes = some (agrees), plusieurs = several, certains = some, d'autres = others.", "ex": {"fr": "J'ai lu <em>quelques-uns</em> de ses livres.", "en": "I've read a few of his books."}},
  ]},
"Ce qui, ce que, ce dont": {
  "rules": [
    {"h": "When the whole clause is the object", "p": "ce qui = what (subject of its clause), ce que = what (object), ce dont = what (after de).", "ex": {"fr": "Je sais <em>ce que</em> tu veux ; <em>ce qui</em> compte, c'est l'effort.", "en": "I know what you want; what matters is effort."}},
  ]},
# ── B1 · Sentence ───────────────────────────────────────────
"Si + présent": {
  "rules": [
    {"h": "Real condition", "p": "si + present in the if-half, future or present in the other half. Never a future after si itself.", "ex": {"fr": "<em>Si</em> tu veux, on <em>ira</em> à la plage.", "en": "If you want, we'll go to the beach."}},
  ]},
"Si + imparfait": {
  "rules": [
    {"h": "The daydream si", "p": "si + imparfait → conditionnel in the other half. Unreal today, but still daydreamable.", "ex": {"fr": "<em>Si j'avais</em> le temps, j'<em>achèterais</em> une maison.", "en": "If I had the time, I'd buy a house."}},
    {"h": "The conditionnel never follows si", "p": "The mistake everyone makes: si j'aurais. The if-half takes imparfait, only the other half gets the conditionnel.", "ex": {"fr": "<em>Si j'étais</em> toi, je <em>ferais</em> attention.", "en": "If I were you, I'd be careful."}},
  ],
  "see": ["Conditionnel présent: uses", "Si + plus-que-parfait"]},
"Reported speech (present)": {
  "rules": [
    {"h": "One step back in time", "p": "Present → imparfait, passé composé → plus-que-parfait, futur → conditionnel. 'Je pars' becomes il a dit qu'il partait.", "ex": {"fr": "« J'arrive » → Elle a dit qu'elle <em>arrivait</em>.", "en": "'I'm arriving.' → She said she was arriving."}},
    {"h": "Reporting a question", "p": "Yes/no questions report with si. Wh-questions keep their word in statement order, no inversion.", "ex": {"fr": "Elle m'a demandé <em>si</em> je venais ; il a demandé <em>où j'allais</em>.", "en": "She asked if I was coming; he asked where I was going."}},
    {"h": "Reporting a command", "p": "A command becomes de + infinitive after dire or demander. Negatives keep ne pas before the infinitive: elle nous a demandé de ne pas partir.", "ex": {"fr": "Il m'a dit <em>de fermer</em> la porte.", "en": "He told me to close the door."}},
  ],
  "see": ["Sequence of tenses"]},
"Ne… que, aucun, ni… ni": {
  "rules": [
    {"h": "ne… que = only", "p": "que lands after the verb and limits it: je n'ai que cinq euros. Keep the ne in writing.", "ex": {"fr": "Il ne reste <em>que</em> deux places.", "en": "Only two tickets are left."}},
    {"h": "aucun and ni… ni", "p": "aucun + ne = not a single one; ni… ni = neither nor, with a single ne.", "ex": {"fr": "Je n'ai <em>ni</em> le temps <em>ni</em> l'envie.", "en": "I have neither the time nor the desire."}},
  ]},
"Cause & consequence (extended)": {
  "rules": [
    {"h": "comme, puisque, car", "p": "comme opens the sentence (Comme il pleut, restons), puisque cites a known cause, car explains mid-sentence.", "ex": {"fr": "<em>Puisque</em> tu insistes, je reste.", "en": "Since you insist, I'll stay."}},
    {"h": "Announcing the result", "p": "donc, c'est pourquoi, c'est pour ça que all point forward to the consequence.", "ex": {"fr": "Il pleuvait ; <em>c'est pourquoi</em> on a annulé.", "en": "It was raining; that's why we cancelled."}},
  ]},
"Pour que, afin de": {
  "rules": [
    {"h": "Same aim, two grammars", "p": "pour que + subjunctive when a new subject acts; pour or afin de + infinitive when the subject stays. The same split runs through avant: avant de partir, avant qu'il parte.", "ex": {"fr": "Je parle lentement <em>pour que tu comprennes</em> ; <em>avant de partir</em>, ferme la porte.", "en": "I speak slowly so that you understand; before leaving, close the door."}},
  ]},
"Concession & opposition": {
  "rules": [
    {"h": "bien que + subjunctive", "p": "Concede, then counter. bien que takes the subjunctive; même si takes the indicative.", "ex": {"fr": "<em>Bien qu'il soit</em> tard, on continue.", "en": "Although it's late, we carry on."}},
  ]},
"Ce qui…, c'est": {
  "rules": [
    {"h": "Spotlight construction", "p": "Ce qui + verb + c'est + the star of the sentence. The star usually comes last, where the emphasis lives.", "ex": {"fr": "<em>Ce qui m'énerve</em>, <em>c'est</em> le bruit.", "en": "What annoys me is the noise."}},
  ]},
"Il est / c'est + adjective + de": {
  "rules": [
    {"h": "il est + adjective + de", "p": "For general statements about things: il est important de venir. Switch to c'est when you point at a specific one with a noun.", "ex": {"fr": "<em>Il est</em> important <em>de</em> venir à l'heure.", "en": "It's important to arrive on time."}},
  ]},
"Narrative time markers": {
  "rules": [
    {"h": "The storyteller's toolkit", "p": "un matin, soudain, puis, enfin, le lendemain. They pace the story; imparfait runs underneath, events land in passé composé.", "ex": {"fr": "<em>Soudain</em>, la lumière <em>s'est éteinte</em>.", "en": "Suddenly, the light went out."}},
  ]},
# ── B2 · Tense ──────────────────────────────────────────────
"Futur antérieur": {
  "rules": [
    {"h": "Future before future", "p": "Futur of avoir or être + participle: j'aurai fini, je serai rentré. For an action done before another future one.", "ex": {"fr": "Quand tu arriveras, j'<em>aurai déjà mangé</em>.", "en": "When you arrive, I'll have already eaten."}},
    {"h": "After quand, dès que", "p": "Inside a time clause, a future action that must complete first lands in the futur antérieur.", "ex": {"fr": "Quand tu <em>auras fini</em>, appelle-moi.", "en": "When you've finished, call me."}},
  ],
  "see": ["Futur simple"]},
"Past infinitive": {
  "rules": [
    {"h": "avoir or être + participle, as an infinitive", "p": "après avoir mangé, après être rentré. Keeps the infinitive after a preposition while pointing at the past.", "ex": {"fr": "Après <em>avoir travaillé</em>, il dort.", "en": "After working, he sleeps."}},
    {"h": "être past infinitives agree", "p": "après être partis, après être partie. Same agreement rule as the passé composé.", "ex": {"fr": "après <em>être rentrés</em> du voyage.", "en": "after coming back from the trip."}},
    {"h": "ayant fini, étant parti", "p": "The participle has its own past: ayant + participle, étant + participle. The action finished before the main one.", "ex": {"fr": "<em>Ayant fini</em> son travail, il est sorti.", "en": "Having finished his work, he went out."}},
  ]},
"Sequence of tenses": {
  "rules": [
    {"h": "The main clause sets the floor", "p": "il dit qu'il viendra (present → future), il a dit qu'il viendrait (past → conditionnel). Each step back shifts the subordinate one step too.", "ex": {"fr": "Il a dit qu'il <em>viendrait</em> le lendemain.", "en": "He said he would come the next day."}},
    {"h": "Time words shift with tenses", "p": "demain → le lendemain, hier → la veille, aujourd'hui → ce jour-là.", "ex": {"fr": "« Demain » → <em>le lendemain</em> ; « hier » → <em>la veille</em>.", "en": "'tomorrow' → 'the next day'; 'yesterday' → 'the day before'."}},
  ]},
"Exceptional participle agreement": {
  "rules": [
    {"h": "faire never agrees", "p": "With faire + infinitive, the participle stays frozen: les robes qu'elle a fait coudre.", "ex": {"fr": "les meubles qu'elle <em>a fait</em> livrer.", "en": "the furniture she had delivered."}},
    {"h": "Pronominals with indirect objects don't agree", "p": "elles se sont parlé, elles se sont téléphoné: parler à and téléphoner à make the object indirect, and agreement dies.", "ex": {"fr": "Elles se <em>sont téléphoné</em> hier soir.", "en": "They called each other last night."}},
  ]},
# ── B2 · Mood ───────────────────────────────────────────────
"Conditionnel passé": {
  "rules": [
    {"h": "The regret machine", "p": "aurais or serais + participle: j'aurais dit, je serais venu. Pairs with si + plus-que-parfait for the past that never happened.", "ex": {"fr": "<em>Si j'avais su</em>, je <em>serais venu</em>.", "en": "If I'd known, I'd have come."}},
    {"h": "Standalone reproach", "p": "With no si-clause at all, the conditionnel passé scolds or regrets — the hypothesis stays unspoken.", "ex": {"fr": "Tu <em>aurais pu</em> me prévenir !", "en": "You could have warned me!"}},
  ]},
"Subjonctif after verbs & impersonals": {
  "rules": [
    {"h": "Want, doubt, deny", "p": "vouloir, douter, nier, craindre, éviter que + subjunctive. Impersonals follow: il est possible que, il vaut mieux que.", "ex": {"fr": "Je doute qu'il <em>vienne</em>.", "en": "I doubt he'll come."}},
    {"h": "Certainty and stating go indicative", "p": "je suis sûr que, il est évident que, il dit que. But nier and douter stay subjunctive even though they sound certain.", "ex": {"fr": "Je nie qu'il <em>ait menti</em>.", "en": "I deny that he lied."}},
    {"h": "Superlatives and uniqueness", "p": "After le meilleur, le seul, le premier, le dernier, the subordinate verb often goes subjunctive.", "ex": {"fr": "C'est le meilleur film que j'<em>aie</em> vu.", "en": "It's the best film I've seen."}},
  ],
  "exc": "Affirmative espérer que takes the indicative: j'espère qu'il vient. Only negated or questioned espérer leans subjunctive."},
"Subjonctif after conjunctions": {
  "rules": [
    {"h": "Learn the gang as one block", "p": "pour que, afin que, avant que, bien que, quoique, jusqu'à ce que, à condition que, à moins que, pourvu que, sans que, de peur que always take the subjunctive. No exceptions to weigh.", "ex": {"fr": "Reste ici <em>jusqu'à ce qu'il revienne</em>.", "en": "Stay here until he comes back."}},
    {"h": "The holdout: après que", "p": "après que describes something that already happened, so it takes the indicative: après qu'il est parti.", "ex": {"fr": "après qu'elle <em>est rentrée</em>.", "en": "after she got home."}},
  ]},
"Subjonctif passé": {
  "rules": [
    {"h": "avoir or être subjunctive + participle", "p": "qu'il ait fini, qu'elle soit partie. The subjunctive action finished before the main one.", "ex": {"fr": "Je regrette qu'elle <em>ait refusé</em>.", "en": "I regret that she refused."}},
  ]},
"Participle vs verbal adjective": {
  "rules": [
    {"h": "Two spellings, two jobs", "p": "The adjective describes (un professeur négligent), the participle acts (en négligeant sa santé).", "ex": {"fr": "un professeur <em>négligent</em> / en <em>négligeant</em> sa santé.", "en": "a careless teacher / by neglecting his health."}},
    {"h": "The common pairs", "p": "excellent / excellant, précédent / précédant, convaincant / convainquant, différent / différant.", "ex": {"fr": "un argument <em>convaincant</em>.", "en": "a convincing argument."}},
  ]},
# ── B2 · Pronouns ───────────────────────────────────────────
"Interrogative lequel": {
  "rules": [
    {"h": "Which one, agreeing", "p": "lequel, laquelle, lesquels, lesquelles choose between known options. With à and de they fuse: auquel, duquel.", "ex": {"fr": "<em>Lequel</em> de ces deux pulls préfères-tu ?", "en": "Which of these two sweaters do you prefer?"}},
  ]},
# ── B2 · Sentence ───────────────────────────────────────────
"Si + plus-que-parfait": {
  "rules": [
    {"h": "The missed-train si", "p": "si + plus-que-parfait → conditionnel passé. Regret about the past that can't be redone.", "ex": {"fr": "<em>Si nous étions partis</em> plus tôt, nous <em>aurions vu</em> le coucher de soleil.", "en": "If we'd left earlier, we'd have seen the sunset."}},
    {"h": "Mixed conditionals", "p": "Past condition, present result: si + plus-que-parfait → conditionnel présent. The missed chance explains now.", "ex": {"fr": "Si tu <em>avais étudié</em>, tu <em>comprendrais</em> maintenant.", "en": "If you had studied, you would understand now."}},
    {"h": "si beyond if", "p": "si also introduces indirect questions (je me demande si elle viendra), suggestions (si on allait au cinéma ?) and wishes (si seulement il était là !).", "ex": {"fr": "<em>Si on allait</em> au cinéma ?", "en": "What if we went to the cinema?"}},
  ],
  "see": ["Conditionnel passé", "Si + imparfait"]},
"Au cas où, à condition que": {
  "rules": [
    {"h": "Just in case vs only if", "p": "au cas où takes the conditionnel (au cas où il pleuvrait); à condition que takes the subjunctive (à condition que tu viennes).", "ex": {"fr": "Prends un parapluie <em>au cas où</em> il pleuvrait.", "en": "Take an umbrella in case it rains."}},
  ]},
"Advanced opposition & concession": {
  "rules": [
    {"h": "quoique vs quoi que", "p": "quoique = although (quoique ce soit difficile). quoi que = whatever (quoi que tu fasses).", "ex": {"fr": "<em>Quoi que</em> tu fasses, sois prudent.", "en": "Whatever you do, be careful."}},
    {"h": "quand bien même", "p": "Even if, with a conditionnel in both halves: quand bien même tu refuserais, je comprendrais.", "ex": {"fr": "Quand bien même tu <em>refuserais</em>, je comprendrais.", "en": "Even if you refused, I'd understand."}},
  ]},
"Advanced purpose & cause": {
  "rules": [
    {"h": "de peur que, non que", "p": "de peur que + subjunctive = for fear that; non que + subjunctive = not that. Both formal.", "ex": {"fr": "Je me tais <em>de peur qu'il</em> se fâche.", "en": "I stay quiet for fear he'll get angry."}},
    {"h": "faute de, sous prétexte que", "p": "faute de + noun = for lack of; sous prétexte que = on the pretext that, usually suspicious.", "ex": {"fr": "<em>Faute de</em> temps, j'ai abandonné.", "en": "For lack of time, I gave up."}},
  ]},
}
