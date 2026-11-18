# Grammar rule sheets, A1 + A2 — keyed by exact taxonomy name.
# Style: humanizer (state it, plain words, one concrete example). Heuristics over completeness.
# em marks the pattern inside the French example.

R = {
# ── A1 · Tense ──────────────────────────────────────────────
"Present: être & avoir": {
  "rules": [
    {"h": "Two irregular workhorses", "p": "Être and avoir follow no pattern; learn their six forms by heart. They build most of the rest of the language.", "ex": {"fr": "Tu <em>es</em> fatigué ? Oui, j'<em>ai</em> sommeil.", "en": "Are you tired? Yes, I'm sleepy."}},
    {"h": "je becomes j'", "p": "Before a vowel, je drops its e: je suis → j'ai, j'étais, j'aurai.", "ex": {"fr": "J'<em>ai</em> une question.", "en": "I have a question."}},
  ]},
"Present: -er verbs": {
  "rules": [
    {"h": "Drop -er, add the endings", "p": "parler → je parle, tu parles, il parle, nous parlons, vous parlez, ils parlent. About 90% of verbs work this way.", "ex": {"fr": "Nous <em>parlons</em> français à la maison.", "en": "We speak French at home."}},
    {"h": "The -ent you never hear", "p": "il parle and ils parlent sound identical. The endings are spelling, not sound; listen for nous and vous instead.", "ex": {"fr": "Ils <em>habitent</em> à Lyon.", "en": "They live in Lyon."}},
  ]},
"Present: common irregular verbs": {
  "rules": [
    {"h": "Learn the top ten as words", "p": "aller, faire, prendre, venir, pouvoir, vouloir, devoir, savoir, voir, dire are irregular and everywhere. Memorize each as a whole word, not a pattern.", "ex": {"fr": "Je <em>vais</em> au marché et je <em>prends</em> du pain.", "en": "I'm going to the market and getting bread."}},
    {"h": "The boot shape", "p": "In many irregulars, nous and vous revert to the infinitive stem: je viens, tu viens, il vient, but nous venons, vous venez.", "ex": {"fr": "Ils <em>viennent</em> ce soir ; nous, nous <em>venons</em> demain.", "en": "They're coming tonight; we're coming tomorrow."}},
  ]},
"Present: pronominal verbs": {
  "rules": [
    {"h": "The pronoun copies the subject", "p": "se lever → je me lève, tu te lèves, il se lève, nous nous levons, vous vous levez, ils se lèvent.", "ex": {"fr": "Je <em>me</em> lève à sept heures.", "en": "I get up at seven."}},
    {"h": "Daily routine is pronominal", "p": "Many verbs take the pronoun even though English doesn't: s'appeler, se lever, se coucher, se rappeler.", "ex": {"fr": "Elle <em>s'</em>appelle Léa.", "en": "Her name is Léa."}},
    {"h": "Reciprocal and passive se", "p": "With a plural subject the same form reads 'each other': nous nous aimons, ils s'écrivent. With a thing as subject it reads passive: ce livre se vend bien.", "ex": {"fr": "Nous <em>nous</em> aimons ; ce livre <em>se vend</em> bien.", "en": "We love each other; this book sells well."}},
  ],
  "exc": "Some verbs exist only with se: se souvenir, s'en aller, il se peut que = maybe."},
"Être en train de": {
  "rules": [
    {"h": "Happening right now", "p": "être en train de + infinitive points at this exact moment. The plain present covers both now and habitually; this pins it to now.", "ex": {"fr": "Je <em>suis en train de</em> cuisiner, je te rappelle.", "en": "I'm cooking right now, I'll call you back."}},
  ]},
"Futur proche": {
  "rules": [
    {"h": "aller + infinitive", "p": "The near future works like English 'going to'. aller conjugates, the main verb stays an infinitive.", "ex": {"fr": "On <em>va voir</em> un film ce soir.", "en": "We're going to watch a film tonight."}},
    {"h": "Decided vs predicted", "p": "Futur proche sounds decided or imminent; futur simple sounds predicted or distant. Tonight's plans take futur proche.", "ex": {"fr": "Il <em>va pleuvoir</em>, prends ton parapluie.", "en": "It's going to rain, take your umbrella."}},
    {"h": "The past near future", "p": "Aller in the imparfait + infinitive = was about to. The plan was set, then something interrupted it.", "ex": {"fr": "Il <em>allait</em> partir quand elle est arrivée.", "en": "He was about to leave when she arrived."}},
  ]},
"Passé récent": {
  "rules": [
    {"h": "venir de + infinitive", "p": "'Just did it'. venir conjugates, de stays, the action stays an infinitive.", "ex": {"fr": "Je <em>viens de</em> finir mes devoirs.", "en": "I've just finished my homework."}},
    {"h": "The past of just", "p": "For 'had just', venir moves to the imparfait: je venais de finir. Never the passé composé of venir for this meaning.", "ex": {"fr": "Il <em>venait de</em> partir quand elle est arrivée.", "en": "He had just left when she arrived."}},
  ]},
"Passé composé: first steps": {
  "rules": [
    {"h": "Two pieces", "p": "Passé composé = avoir or être in the present + the participle. Regular participles: parler → parlé, finir → fini, vendre → vendu.", "ex": {"fr": "J'<em>ai mangé</em> et elle <em>est partie</em>.", "en": "I ate and she left."}},
    {"h": "A finished event", "p": "It says what happened and moved on: what you did, not how things were.", "ex": {"fr": "Hier, nous <em>avons visité</em> le musée.", "en": "Yesterday we visited the museum."}},
  ]},
# ── A1 · Mood ───────────────────────────────────────────────
"Imperative": {
  "rules": [
    {"h": "Present tense, no subject", "p": "Take the tu, nous or vous present form and drop the pronoun: parle ! parlons ! parlez !", "ex": {"fr": "<em>Écoute</em> bien et <em>répète</em>.", "en": "Listen carefully and repeat."}},
    {"h": "Three rebels", "p": "être → sois, soyons, soyez. avoir → aie, ayons, ayez. savoir → sache, sachons, sachez.", "ex": {"fr": "<em>Sois</em> gentil avec le chat !", "en": "Be nice to the cat!"}},
    {"h": "The tu -s drops, then returns", "p": "-er verbs and aller drop the tu -s: parle ! va ! But it comes back before y and en: vas-y, parles-en.", "ex": {"fr": "N'hésite pas, <em>vas-y</em> !", "en": "Don't hesitate, go on!"}},
    {"h": "Reflexive commands", "p": "Attach the reflexive pronoun after the verb: lève-toi, levons-nous, levez-vous. The negative wraps normally: ne te lève pas.", "ex": {"fr": "<em>Lève-toi</em> !", "en": "Get up!"}},
    {"h": "Past imperative: aie fini, sois parti", "p": "avoir or être imperative + participle orders an action done by a deadline. Formal register only — speech uses the present imperative plus a deadline.", "ex": {"fr": "Soyez <em>rentrés</em> avant minuit.", "en": "Be back before midnight."}},
  ]},
"Polite conditionnel": {
  "rules": [
    {"h": "je voudrais, not je veux", "p": "The conditionnel of vouloir, pouvoir and aimer turns requests into polite ones.", "ex": {"fr": "Je <em>voudrais</em> un café, s'il vous plaît.", "en": "I'd like a coffee, please."}},
    {"h": "Pourriez-vous", "p": "pouvoir in the conditionnel makes questions softer than pouvez-vous.", "ex": {"fr": "<em>Pourriez-vous</em> m'aider, s'il vous plaît ?", "en": "Could you help me, please?"}},
  ]},
"Pouvoir, vouloir, devoir + infinitive": {
  "rules": [
    {"h": "Can, want, must", "p": "pouvoir = can, vouloir = want, devoir = must. Each sits straight before an infinitive.", "ex": {"fr": "Je <em>veux apprendre</em>, mais je <em>dois travailler</em>.", "en": "I want to learn, but I have to work."}},
    {"h": "High-frequency irregulars", "p": "je peux, je veux, je dois; ils peuvent, ils veulent, ils doivent. You will use these more than most regular verbs.", "ex": {"fr": "Ils <em>peuvent venir</em> à huit heures.", "en": "They can come at eight."}},
  ]},
"Il faut + infinitive": {
  "rules": [
    {"h": "The impersonal must", "p": "il faut + infinitive = it's necessary to. The il never changes and nothing agrees with anything.", "ex": {"fr": "<em>Il faut</em> réserver à l'avance.", "en": "You have to book ahead."}},
    {"h": "il me faut + noun", "p": "With a person and a noun, il faut becomes 'need': il me faut du temps.", "ex": {"fr": "<em>Il me faut</em> cinq minutes.", "en": "I need five minutes."}},
  ]},
"Pour + infinitive": {
  "rules": [
    {"h": "Answers 'what for?'", "p": "pour + infinitive = in order to. It explains a purpose.", "ex": {"fr": "Je suis là <em>pour apprendre</em>.", "en": "I'm here to learn."}},
    {"h": "Before a noun it's plain 'for'", "p": "pour + noun gives a beneficiary; pour + infinitive gives a goal.", "ex": {"fr": "Un cadeau <em>pour</em> Marie.", "en": "A gift for Marie."}},
  ]},
# ── A1 · Nouns & articles ───────────────────────────────────
"Gender of nouns": {
  "rules": [
    {"h": "Guess by ending, then memorize", "p": "-tion, -té, -ure, -ette lean feminine; -age, -ment, -eau, -isme lean masculine. The rest is memory.", "ex": {"fr": "la na<em>tion</em>, le from<em>age</em>.", "en": "the nation, the cheese."}},
    {"h": "Learn the article as part of the word", "p": "Memorize 'une table', never 'table'. You can't use the noun without its gender anyway.", "ex": {"fr": "C'est <em>une</em> idée, c'est <em>un</em> problème.", "en": "It's an idea, it's a problem."}},
  ],
  "exc": "-age is masculine except la page, la plage, la cage, l'image, la nage, la rage."},
"Plural of nouns": {
  "rules": [
    {"h": "Add -s and don't say it", "p": "Most plurals add a silent -s; singular and plural sound the same. -al goes to -aux: journal → journaux.", "ex": {"fr": "un cheval, des <em>chevaux</em>.", "en": "a horse, horses."}},
    {"h": "The seven -oux words", "p": "bijou, caillou, chou, genou, hibou, joujou, pou take -x. Every other -ou word takes -s.", "ex": {"fr": "des <em>cailloux</em> et des <em>choux</em>.", "en": "pebbles and cabbages."}},
  ]},
"Definite & indefinite articles": {
  "rules": [
    {"h": "Known vs new", "p": "le, la, les point at something known; un, une, des introduce something new.", "ex": {"fr": "J'ai <em>un</em> chien. <em>Le</em> chien s'appelle Rex.", "en": "I have a dog. The dog is called Rex."}},
    {"h": "Melting articles", "p": "le and la become l' before a vowel, but les never elides. de + le = du, à + le = au, à + les = aux, de + les = des.", "ex": {"fr": "C'est l'ami <em>du</em> voisin, le chien <em>des</em> voisins.", "en": "That's the neighbour's friend, the neighbours' dog."}},
    {"h": "General statements keep the article", "p": "Abstract and mass nouns take le/la where English uses nothing: le bonheur, le lait, la musique.", "ex": {"fr": "<em>La</em> musique est importante.", "en": "Music is important."}},
  ]},
"Partitive articles": {
  "rules": [
    {"h": "du, de la, des = some", "p": "For stuff you don't count: du pain, de la confiture, des pâtes. Before a vowel it's de l': de l'eau. English often skips the article; French never does.", "ex": {"fr": "Je prends <em>du</em> café, elle boit <em>de l'</em>eau.", "en": "I have coffee, she drinks water."}},
    {"h": "Negative drains it to de", "p": "In the negative, du, de la and des all collapse to de.", "ex": {"fr": "Il n'y a plus <em>de</em> lait.", "en": "There's no milk left."}},
    {"h": "Liking keeps its article", "p": "aimer, préférer and adorer keep the definite article, even in the negative: je n'aime pas le café.", "ex": {"fr": "Je n'aime pas <em>le</em> café noir.", "en": "I don't like black coffee."}},
  ],
  "exc": "Quantity words also take de: beaucoup de pain, une tasse de café.",
  "see": ["Ne… pas, ne… jamais"]},
# ── A1 · Adjectives & adverbs ───────────────────────────────
"Adjective agreement": {
  "rules": [
    {"h": "Copy the noun", "p": "Feminine adds -e, plural adds -s: un petit garçon, une petite fille, des petites filles.", "ex": {"fr": "Elle est <em>contente</em>, ils sont <em>contents</em>.", "en": "She's happy, they're happy."}},
    {"h": "Mixed groups read masculine", "p": "One male in a group and the whole group goes masculine plural.", "ex": {"fr": "Marc et Léa sont <em>petits</em>.", "en": "Marc and Léa are short."}},
    {"h": "Irregular feminines", "p": "bon → bonne, nouveau → nouvelle, vieux → vieille, beau → belle, blanc → blanche, frais → fraîche. A masculine already ending in -e doesn't move: rouge stays rouge.", "ex": {"fr": "une <em>belle</em> journée, une table <em>rouge</em>.", "en": "a beautiful day, a red table."}},
  ],
  "exc": "Noun and compound colours never agree: des robes marron, orange, bleu marine — but rose agrees: des robes roses. beau, nouveau, vieux take bel, nouvel, vieil before a vowel: un bel homme, un vieil ami."},
"Adjective position": {
  "rules": [
    {"h": "Most follow, a short band leads", "p": "Most adjectives follow the noun. Beauty, age, goodness and size go before: beau, joli, jeune, vieux, nouveau, bon, mauvais, meilleur, grand, petit, gros, long.", "ex": {"fr": "une <em>grande</em> maison, une voiture <em>rouge</em>.", "en": "a big house, a red car."}},
    {"h": "Position can flip meaning", "p": "un grand homme = a great man; un homme grand = a tall man. ancien, cher and propre behave the same way.", "ex": {"fr": "mon <em>ancien</em> prof (ex-) / un meuble <em>ancien</em> (antique)", "en": "my former teacher / an antique piece of furniture."}},
    {"h": "dernier and seul flip too", "p": "la semaine dernière = last week, la dernière semaine = the final one of a series. seul before the noun = only, after it = alone. Two trailing adjectives join with et; des shrinks to de before a pre-placed adjective (de grandes maisons).", "ex": {"fr": "la <em>semaine dernière</em> / la <em>dernière</em> semaine ; une robe <em>verte et</em> bleue.", "en": "last week / the final week; a green and blue dress."}},
  ]},
"Possessive adjectives": {
  "rules": [
    {"h": "Match the thing, not the owner", "p": "mon, ma, mes agree with the noun that follows. son livre can be his book or her book.", "ex": {"fr": "<em>ma</em> sœur et <em>ses</em> amis.", "en": "my sister and her friends."}},
    {"h": "Vowels go masculine", "p": "Before a vowel, feminine nouns borrow mon, ton, son: mon amie, ton école.", "ex": {"fr": "<em>Mon</em> école est grande.", "en": "My school is big."}},
  ]},
"Demonstrative adjectives": {
  "rules": [
    {"h": "ce, cet, cette, ces", "p": "ce before a consonant, cet before a vowel, cette for feminine, ces for plural. One set for near and far.", "ex": {"fr": "<em>Cet</em> appartement, <em>cette</em> cuisine.", "en": "this apartment, this kitchen."}},
  ]},
"Quantity & intensity adverbs": {
  "rules": [
    {"h": "très boosts adjectives, beaucoup boosts verbs", "p": "très grand works, beaucoup grand does not. j'aime beaucoup works, j'aime très does not.", "ex": {"fr": "Ce gâteau est <em>très</em> bon, j'aime <em>beaucoup</em>.", "en": "This cake is very good, I like it a lot."}},
    {"h": "trop = too", "p": "trop + adjective, trop de + noun.", "ex": {"fr": "C'est <em>trop</em> cher et il y a <em>trop de</em> monde.", "en": "It's too expensive and there are too many people."}},
    {"h": "assez, peu, autant", "p": "assez = quite, peu = not very, before an adjective. Verbs compare with autant que, not aussi: il travaille autant que moi.", "ex": {"fr": "C'est <em>assez</em> bon, et il travaille <em>autant que</em> moi.", "en": "It's quite good, and he works as much as I do."}},
    {"h": "Making adverbs: feminine + -ment", "p": "lent → lente → lentement; an adjective ending in a mute -e adds -ment directly (vraiment); -ant and -ent give -amment and -emment (évidemment). Irregulars: bon → bien, mauvais → mal.", "ex": {"fr": "Elle écoute <em>attentivement</em>, <em>évidemment</em>.", "en": "She listens attentively, obviously."}},
    {"h": "Where adverbs sit", "p": "After the verb they modify (elle écoute attentivement), before an adjective or another adverb (très intelligent), and between auxiliary and participle in compound tenses (elle a bien mangé). Several adverbs stack manner, place, time.", "ex": {"fr": "Elle a <em>bien</em> mangé ; il court <em>rapidement</em> dans le parc <em>le matin</em>.", "en": "She ate well; he runs quickly in the park in the morning."}},
  ]},
# ── A1 · Pronouns ───────────────────────────────────────────
"Subject pronouns & on": {
  "rules": [
    {"h": "on does everything", "p": "on replaces nous in everyday speech and conjugates like il: on va, on est, on mange.", "ex": {"fr": "<em>On y va</em> ?", "en": "Shall we go?"}},
    {"h": "tu or vous", "p": "tu for friends, family and children; vous for strangers, teachers and groups. Unsure? vous.", "ex": {"fr": "Comment <em>vous</em> appelez-vous ?", "en": "What's your name? (formal)"}},
    {"h": "il is often nobody", "p": "Weather, time and general statements use a dummy il: il pleut, il est trois heures, il faut. ils covers any group with one male; elles only all-female groups.", "ex": {"fr": "<em>Il</em> pleut, <em>il</em> est trois heures.", "en": "It's raining, it's three o'clock."}},
  ]},
"Stressed pronouns": {
  "rules": [
    {"h": "After prepositions and for emphasis", "p": "moi, toi, lui, elle, nous, vous, eux, elles stand alone, follow c'est, or follow a preposition.", "ex": {"fr": "C'est <em>moi</em>. Viens avec <em>nous</em> !", "en": "It's me. Come with us!"}},
  ]},
# ── A1 · Sentence ───────────────────────────────────────────
"Ne… pas, ne… jamais": {
  "rules": [
    {"h": "Wrap the verb", "p": "ne before the conjugated verb, pas after. In casual speech the ne often falls silent.", "ex": {"fr": "Je <em>ne</em> comprends <em>pas</em>.", "en": "I don't understand."}},
    {"h": "pas swaps out", "p": "jamais = never, plus = anymore. They take pas's slot: je ne mange jamais de viande.", "ex": {"fr": "Elle ne sort <em>plus</em> le soir.", "en": "She doesn't go out in the evenings anymore."}},
    {"h": "Negating an infinitive", "p": "ne pas wraps the infinitive itself, not the governing verb: je préfère ne pas partir.", "ex": {"fr": "Je préfère <em>ne pas</em> partir.", "en": "I prefer not to leave."}},
  ],
  "see": ["Partitive articles", "Ne… plus, rien, personne"]},
"Three question forms": {
  "rules": [
    {"h": "Three roads, one meaning", "p": "Intonation: tu viens ? est-ce que: est-ce que tu viens ? inversion: viens-tu ?", "ex": {"fr": "<em>Est-ce que</em> tu viens avec nous ?", "en": "Are you coming with us?"}},
    {"h": "The formality ladder", "p": "Intonation is casual, inversion is formal, est-ce que sits safely in the middle.", "ex": {"fr": "Où <em>habites-tu</em> ?", "en": "Where do you live? (formal)"}},
    {"h": "Inversion fine print", "p": "A vowel-final verb takes a -t- bridge: va-t-il, parle-t-elle. Noun subjects stay in front: Marie vient-elle ? And je never inverts — use est-ce que je.", "ex": {"fr": "Où <em>habite-t</em>-elle ? <em>Marie vient-elle</em> ? <em>Est-ce que je</em> parle bien ?", "en": "Where does she live? Is Marie coming? Do I speak well?"}},
  ]},
"Question words": {
  "rules": [
    {"h": "The seven essentials", "p": "qui = who, que or quoi = what, où = where, quand = when, comment = how, pourquoi = why, combien = how much.", "ex": {"fr": "<em>Quand</em> est-ce qu'on part ? Demain.", "en": "When are we leaving? Tomorrow."}},
    {"h": "que vs quoi", "p": "que opens the question and shrinks to qu' before a vowel; quoi takes over after a preposition and stands alone in casual speech.", "ex": {"fr": "<em>Qu'</em>est-ce que tu veux ? Tu fais <em>quoi</em> ?", "en": "What do you want? What are you doing?"}},
    {"h": "qu'est-ce qui and combien de", "p": "When 'what' is the subject of its own question, it's qu'est-ce qui. combien takes de before a noun: combien de temps.", "ex": {"fr": "<em>Qu'est-ce qui</em> se passe ? <em>Combien de</em> temps ça prend ?", "en": "What's happening? How long does it take?"}},
    {"h": "Embedded questions keep statement order", "p": "After savoir, demander, dire, no inversion — and que becomes ce que: je ne sais pas ce que tu veux.", "ex": {"fr": "Dis-moi <em>où tu habites</em>.", "en": "Tell me where you live."}},
  ]},
"Il y a, c'est, voilà": {
  "rules": [
    {"h": "il y a = there is / there are", "p": "One form for singular and plural, and il y avait for the past.", "ex": {"fr": "<em>Il y a</em> du pain ?", "en": "Is there any bread?"}},
    {"h": "voilà points, c'est identifies", "p": "voilà presents something you can see arriving; c'est names or labels it.", "ex": {"fr": "<em>Voilà</em> le bus ! <em>C'est</em> le nôtre.", "en": "There's the bus! It's ours."}},
  ],
  "see": ["C'est vs il/elle est"]},
"C'est vs il/elle est": {
  "rules": [
    {"h": "c'est + noun, il est + adjective", "p": "With a noun or a modified noun, c'est. With a bare adjective about a person, il est.", "ex": {"fr": "<em>C'est</em> un bon film. Il <em>est</em> talentueux.", "en": "It's a good film. He's talented."}},
    {"h": "il and elle refer back", "p": "Use il or elle when the subject is already in play; c'est when you label or announce.", "ex": {"fr": "Qui est-ce ? <em>C'est</em> Marie. Elle est là.", "en": "Who is it? It's Marie. She's here."}},
  ]},
"Dans, depuis, il y a": {
  "rules": [
    {"h": "Three clocks", "p": "dans = in (from now), depuis = for or since (still going), il y a = ago.", "ex": {"fr": "J'habite ici <em>depuis</em> trois ans.", "en": "I've lived here for three years."}},
    {"h": "depuis keeps the present", "p": "French stays in the present with depuis; English switches to have + participle.", "ex": {"fr": "Il <em>travaille</em> depuis deux heures.", "en": "He's been working for two hours."}},
  ]},
"Prepositions of place": {
  "rules": [
    {"h": "à = at, de = from, dans = inside", "p": "à Paris, de Lyon, dans la chambre. Countries: en with feminine or vowel-initial names (en France, en Iran), au with masculine consonant ones (au Portugal, au Canada), aux with plurals (aux États-Unis).", "ex": {"fr": "Je rentre <em>dans</em> la maison, je viens <em>de</em> la boulangerie.", "en": "I'm going back into the house, I'm coming from the bakery."}},
    {"h": "Coming from, heading toward", "p": "Coming from a country mirrors the à-side: de France (feminine or vowel), du Canada, des États-Unis. vers = toward, direction without arrival.", "ex": {"fr": "Je reviens <em>de</em> France et je marche <em>vers</em> la gare.", "en": "I'm coming back from France and walking toward the station."}},
  ]},
"Prepositions of time": {
  "rules": [
    {"h": "à for clocks, en for periods", "p": "à huit heures, en mars, en 2024, le lundi for every Monday, du lundi au vendredi.", "ex": {"fr": "On part <em>en</em> juin, <em>à</em> six heures.", "en": "We're leaving in June, at six."}},
  ]},
"Et, ou, mais, alors": {
  "rules": [
    {"h": "The four basic connectors", "p": "et = and, ou = or, mais = but, alors = so or then. They join clauses without commas.", "ex": {"fr": "Je voudrais un thé <em>ou</em> un café, <em>mais</em> sans lait.", "en": "I'd like tea or coffee, but no milk."}},
  ]},
# ── A2 · Tense ──────────────────────────────────────────────
"Passé composé with avoir": {
  "rules": [
    {"h": "The default auxiliary", "p": "Every verb takes avoir except the être band and pronominals.", "ex": {"fr": "Elle <em>a acheté</em> une voiture.", "en": "She bought a car."}},
    {"h": "-ir and -re participles", "p": "finir → fini. Regular -re verbs drop the re and add -u: attendre → attendu, vendre → vendu, rendre → rendu. Two irregular families: prendre → pris (apprendre, comprendre, reprendre), mettre → mis (permettre, promettre).", "ex": {"fr": "J'<em>ai attendu</em>, puis j'<em>ai pris</em> le bus.", "en": "I waited, then took the bus."}},
    {"h": "The irregular participles to know", "p": "eu, été, fait, pris, mis, vu, dit, su, voulu, pu, dû, bu, lu — most compound tenses lean on these. Used as an adjective, the participle agrees like one: une table cassée.", "ex": {"fr": "Il a <em>pris</em> son temps et a <em>bu</em> son café ; une lettre <em>écrite</em> à la main.", "en": "He took his time and drank his coffee; a handwritten letter."}},
  ]},
"Passé composé with être": {
  "rules": [
    {"h": "The house verbs", "p": "Seventeen verbs of coming, going and staying take être: aller, arriver, descendre, devenir, entrer, monter, mourir, naître, partir, passer, rentrer, rester, retourner, revenir, sortir, tomber, venir. Every prefixed cousin follows its parent: redevenir, repartir, ressortir.", "ex": {"fr": "Ils <em>sont partis</em> à l'aube.", "en": "They left at dawn."}},
    {"h": "The participle agrees", "p": "With être the participle works like an adjective: elle est allée, elles sont allées.", "ex": {"fr": "Marie <em>est montée</em> avec ses valises.", "en": "Marie went up with her suitcases."}},
    {"h": "Movement fine print", "p": "Each verb drags its preposition: aller à Paris, venir de Lyon, entrer dans la maison, sortir du cinéma. aller moves away from the speaker, venir toward. sortir crosses a boundary out; partir is the departure itself.", "ex": {"fr": "Je <em>vais</em> au marché, tu <em>viens</em> ? Il <em>sort de</em> la voiture et <em>part</em> pour Lyon.", "en": "I'm going to the market, coming? He gets out of the car and leaves for Lyon."}},
  ],
  "exc": "monter, descendre, rentrer, retourner, sortir and passer take avoir when a direct object follows: j'ai monté les valises, j'ai retourné les crêpes, j'ai passé une heure là-bas."},
"Imparfait": {
  "rules": [
    {"h": "One stem for everyone", "p": "Take the nous form of the present, drop -ons, add -ais, -ais, -ait, -ions, -iez, -aient. Only être misbehaves: ét-.", "ex": {"fr": "Nous <em>chantions</em>, vous <em>chantiez</em>.", "en": "We sang, you sang."}},
    {"h": "What it's for", "p": "Descriptions, habits, weather, feelings, background. Anything that lasted without a deadline.", "ex": {"fr": "Quand j'étais petit, il y <em>avait</em> un parc ici.", "en": "When I was little, there was a park here."}},
  ]},
"Passé composé vs imparfait": {
  "rules": [
    {"h": "Scene vs action", "p": "Imparfait paints the scene: weather, age, mood. Passé composé moves the plot: what happened next.", "ex": {"fr": "Il <em>pleuvait</em> quand je <em>suis sorti</em>.", "en": "It was raining when I went out."}},
    {"h": "Habit vs event", "p": "An uncounted habit with toujours or chaque été takes imparfait; a counted series of events takes passé composé.", "ex": {"fr": "Chaque été nous <em>allions</em> à la mer ; un jour, nous <em>avons pris</em> le train.", "en": "Every summer we went to the sea; one day we took the train."}},
    {"h": "Stative verbs stay imparfait", "p": "savoir, croire, vouloir, pouvoir, aimer describe states and default to imparfait. The passé composé marks the moment the state begins: j'ai su = I found out.", "ex": {"fr": "Je <em>savais</em> la réponse ; quand j'<em>ai su</em> la vérité, j'ai pleuré.", "en": "I knew the answer; when I learned the truth, I cried."}},
    {"h": "A closed span takes passé composé", "p": "pendant + a measured, finished span → passé composé: j'ai habité à Paris pendant deux ans (it's over). Open description with no endpoint → imparfait: j'habitais à Paris.", "ex": {"fr": "J'<em>ai habité</em> à Paris pendant deux ans.", "en": "I lived in Paris for two years."}},
  ],
  "see": ["Imparfait", "Passé composé with avoir", "Passé composé with être"]},
"Futur simple": {
  "rules": [
    {"h": "Infinitive + avoir's endings", "p": "parler → je parlerai, finir → je finirai. Endings: -ai, -as, -a, -ons, -ez, -ont.", "ex": {"fr": "Tu <em>verras</em>, tout <em>ira</em> bien.", "en": "You'll see, everything will be fine."}},
    {"h": "The irregular stems", "p": "être → ser-, avoir → aur-, aller → ir-, faire → fer-, venir → viendr-, tenir → tiendr-, voir → verr-, savoir → saur-, pouvoir → pourr-, vouloir → voudr-, devoir → devr-, recevoir → recevr-, envoyer → enverr-, courir → courr-, mourir → mourr-, valoir → vaudr-, acquérir → acquerr-; falloir → il faudra, pleuvoir → il pleuvra.", "ex": {"fr": "J'<em>aurai</em> le temps, je <em>ferai</em> ça demain.", "en": "I'll have time, I'll do that tomorrow."}},
    {"h": "quand takes the futur", "p": "After quand, lorsque, dès que pointing at the future, French uses the futur where English keeps the present.", "ex": {"fr": "Quand tu <em>arriveras</em>, appelle-moi.", "en": "When you arrive, call me."}},
  ],
  "exc": "appeler and jeter double their consonant: j'appellerai, je jetterai."},
# ── A2 · Mood ───────────────────────────────────────────────
"Verbs with prepositions": {
  "rules": [
    {"h": "Some verbs need a bridge", "p": "décider de, penser à, rêver de, s'intéresser à, avoir besoin de. The preposition is part of the verb; learn the pair.", "ex": {"fr": "Elle <em>rêve de</em> voyager.", "en": "She dreams of travelling."}},
    {"h": "Second verb stays an infinitive", "p": "je commence à travailler, j'ai peur de voler, il continue à manger.", "ex": {"fr": "Il a commencé <em>à pleuvoir</em>.", "en": "It started to rain."}},
    {"h": "A few verbs bridge with en", "p": "croire en (believe in) and avoir confiance en. And jouer splits: jouer à = games and sports, jouer de = instruments.", "ex": {"fr": "Je <em>crois en</em> toi ; je joue <em>au</em> foot et <em>du</em> piano.", "en": "I believe in you; I play football and the piano."}},
  ],
  "exc": "Same verb, different bridge, different meaning: penser à quelqu'un (think of someone) vs penser de (have an opinion about)."},
# ── A2 · Adjectives & adverbs ───────────────────────────────
"Comparative": {
  "rules": [
    {"h": "plus, aussi, moins", "p": "plus rapide que, aussi rapide que, moins rapide que. The adjective sits in the middle, que does the comparing.", "ex": {"fr": "Le train est <em>plus rapide que</em> le bus.", "en": "The train is faster than the bus."}},
    {"h": "bon and mauvais go irregular", "p": "meilleur = better, pire = worse. Plus bon doesn't exist; plus mauvais exists but pire is the usual choice. bien compares with mieux, never plus bien.", "ex": {"fr": "Il chante <em>mieux</em> que moi.", "en": "He sings better than me."}},
    {"h": "de plus en plus", "p": "de plus en plus = more and more, de moins en moins = less and less, fixed order, before the adjective.", "ex": {"fr": "C'est de <em>plus en plus</em> difficile.", "en": "It's more and more difficult."}},
  ]},
"Superlative": {
  "rules": [
    {"h": "le plus, le moins", "p": "le plus grand, la plus petite, les moins chers. The article turns the comparative into a superlative.", "ex": {"fr": "C'est <em>la meilleure</em> pâtisserie de la ville.", "en": "It's the best bakery in town."}},
    {"h": "bon → le meilleur, mauvais → le pire", "p": "The irregular pair carries into the superlative.", "ex": {"fr": "C'est <em>le pire</em> jour de ma vie.", "en": "It's the worst day of my life."}},
  ]},
# ── A2 · Pronouns ───────────────────────────────────────────
"Direct object pronouns": {
  "rules": [
    {"h": "Replace the thing", "p": "me, te, le, la, nous, vous, les stand before the conjugated verb: je vois Marie → je la vois.", "ex": {"fr": "Tu connais Léa ? Oui, je <em>la</em> connais.", "en": "Do you know Léa? Yes, I know her."}},
    {"h": "me te nous vous come first", "p": "Before le la les: il me le donne. In the passé composé the pronoun climbs before the auxiliary.", "ex": {"fr": "Elle <em>me l'</em>a donné hier.", "en": "She gave it to me yesterday."}},
  ]},
"Indirect object pronouns": {
  "rules": [
    {"h": "Replace the person after à", "p": "à Paul → lui, à mes parents → leur. lui covers him and her alike.", "ex": {"fr": "Je téléphone à Paul → je <em>lui</em> téléphone.", "en": "I'm calling Paul → I'm calling him."}},
    {"h": "Only with à-verbs", "p": "Téléphoner à, parler à, écrire à, offrir à. If there's no à, you need a direct pronoun.", "ex": {"fr": "Je <em>leur</em> ai écrit une carte.", "en": "I wrote them a card."}},
    {"h": "me, te, nous, vous go both ways", "p": "First and second persons use one pronoun for direct and indirect: il me voit, il me parle. lui and leur exist only in the third person.", "ex": {"fr": "Il <em>me</em> voit et il <em>me</em> parle.", "en": "He sees me and he speaks to me."}},
  ],
  "see": ["Direct object pronouns"]},
"En": {
  "rules": [
    {"h": "en replaces de + noun", "p": "du pain → j'en prends, de l'espoir → j'en ai. If the phrase runs on de, en can stand in for the noun.", "ex": {"fr": "Tu veux du café ? J'<em>en</em> veux bien.", "en": "Want some coffee? I'd love some."}},
    {"h": "en with numbers", "p": "After a number or quantity, en holds the counted noun's place: j'en veux deux, il y en a beaucoup.", "ex": {"fr": "J'<em>en</em> ai acheté trois.", "en": "I bought three."}},
  ],
  "see": ["Y", "Partitive articles"]},
"Y": {
  "rules": [
    {"h": "y = there, for places", "p": "y replaces any location phrase: à + place (je vais à Paris → j'y vais), but also dans, sur, chez (chez le médecin → j'y vais). It also covers verbs that take à + thing, like penser à and croire à.", "ex": {"fr": "Tu vas au marché ? J'<em>y</em> vais maintenant.", "en": "Are you going to the market? I'm on my way there."}},
    {"h": "Never for people", "p": "For à + person, keep à with a stressed pronoun: je pense à toi stays.", "ex": {"fr": "Je vais à la banque, puis j'<em>y</em> retire de l'argent.", "en": "I'm going to the bank, then I'll withdraw money there."}},
  ]},
"Uses of on": {
  "rules": [
    {"h": "we, people, you", "p": "on replaces nous in speech, makes generic statements (on ne sait jamais), and even means 'you' in recipes and instructions.", "ex": {"fr": "<em>On</em> se retrouve à huit heures ?", "en": "Shall we meet at eight?"}},
    {"h": "on shrinks before vowels", "p": "on a, on est, and in the negative on n'a pas, on n'est pas.", "ex": {"fr": "<em>On n'a</em> pas le choix.", "en": "We have no choice."}},
  ]},
"Qui, que, où": {
  "rules": [
    {"h": "Doer, done-to, place", "p": "qui is the subject (l'homme qui parle), que is the object (le film que j'ai vu), où is place or time (la ville où je suis né).", "ex": {"fr": "la femme <em>qui</em> chante, la chanson <em>que</em> j'aime.", "en": "the woman who sings, the song I love."}},
    {"h": "que becomes qu'", "p": "Before a vowel, drop the e: le film qu'on a vu.", "ex": {"fr": "le livre <em>qu'</em>elle lit.", "en": "the book she's reading."}},
  ]},
"Possessive pronouns": {
  "rules": [
    {"h": "le mien, la tienne, les siens", "p": "The standalone possessive agrees with the thing owned, not the owner: ma voiture → la mienne.", "ex": {"fr": "Tu prends ta voiture ou <em>la mienne</em> ?", "en": "Are you taking your car or mine?"}},
    {"h": "The circumflex three", "p": "notre → le nôtre, votre → le vôtre, leur → le leur. Everyone else is plain: le mien, la tienne, les siens.", "ex": {"fr": "Notre maison est plus grande que <em>la vôtre</em>.", "en": "Our house is bigger than yours."}},
  ]},
"Tout, toute, tous": {
  "rules": [
    {"h": "Four shapes", "p": "tout, toute, tous, toutes modify a noun and agree with it: tous les enfants, toute la journée.", "ex": {"fr": "<em>tous</em> les enfants, <em>toute</em> la journée.", "en": "all the children, the whole day."}},
    {"h": "The adverb agrees by sound", "p": "Before a feminine adjective starting with a consonant, tout agrees: toute contente, toutes contentes. Before a vowel it stays put: tout étonnée.", "ex": {"fr": "Elle est <em>toute</em> contente mais <em>tout</em> étonnée.", "en": "She's quite happy but quite surprised."}},
    {"h": "tous the pronoun sounds 'tooss'", "p": "Ils sont venus tous keeps the s sound; tous les jours drops it.", "ex": {"fr": "Ils sont venus <em>tous</em> (tooss) ; <em>tous</em> les jours (tou).", "en": "They all came; every day."}},
  ]},
"Ça, cela, ceci": {
  "rules": [
    {"h": "ça does the daily work", "p": "ça is the spoken all-purpose 'it / that'. cela is the same word dressed up for writing; ceci points at the closer thing.", "ex": {"fr": "<em>Ça</em> marche pour demain ?", "en": "Does that work for tomorrow?"}},
    {"h": "celui, celle, ceux, celles = the one(s)", "p": "Standalone 'the one / the ones', agreeing with the noun it replaces. celui-ci = this one, celui-là = that one. Never point at a person with ceci or cela — use the celui series.", "ex": {"fr": "Mes clés ? Prends <em>celles</em> de la cuisine.", "en": "My keys? Take the ones from the kitchen."}},
    {"h": "celui qui = the one who", "p": "Free relative, people or things: celui qui travaille gagne. With a plural predicate, c'est becomes ce sont: ce sont eux.", "ex": {"fr": "<em>Celui qui</em> travaille gagne. <em>Ce sont</em> eux.", "en": "The one who works wins. It's them."}},
  ]},
# ── A2 · Sentence ───────────────────────────────────────────
"Ne… plus, rien, personne": {
  "rules": [
    {"h": "pas swaps out", "p": "plus = anymore, rien = nothing, personne = no one. They take pas's slot after the verb.", "ex": {"fr": "Je n'ai <em>plus</em> d'argent.", "en": "I have no money left."}},
    {"h": "rien and personne can lead", "p": "As subjects they come first, and ne still guards the verb: personne ne sait, rien n'a changé.", "ex": {"fr": "<em>Personne ne</em> comprend.", "en": "Nobody understands."}},
    {"h": "Negatives stack, never cancel", "p": "Two negatives in one sentence intensify: je ne comprends jamais rien, personne n'a plus rien. Unlike English, they never make a positive.", "ex": {"fr": "Je ne comprends <em>jamais rien</em>.", "en": "I never understand anything."}},
  ]},
"Exclamations": {
  "rules": [
    {"h": "que and comme = how", "p": "Que c'est beau ! Comme il pleut ! Both boost a whole clause.", "ex": {"fr": "<em>Qu'</em>elle est belle, cette ville !", "en": "How beautiful this city is!"}},
    {"h": "quel + noun", "p": "quel agrees with the noun it boosts: quelle idée ! quelles vacances !", "ex": {"fr": "<em>Quelle</em> surprise !", "en": "What a surprise!"}},
  ]},
"Cause & consequence": {
  "rules": [
    {"h": "parce que answers why", "p": "The cause clause follows the result: je reste parce qu'il pleut.", "ex": {"fr": "Je reste <em>parce qu'</em>il pleut.", "en": "I'm staying because it's raining."}},
    {"h": "donc points forward", "p": "donc announces the consequence: il pleut, donc je reste. car is a softer cause, tacked on after a comma.", "ex": {"fr": "Il pleut, <em>donc</em> je reste.", "en": "It's raining, so I'm staying."}},
  ]},
"Purpose & opposition": {
  "rules": [
    {"h": "pour + infinitive = aim", "p": "It answers 'what for?' When a new person enters the aim, graduate to pour que + subjunctive (B1).", "ex": {"fr": "Je suis venue <em>pour</em> t'<em>aider</em>.", "en": "I came to help you."}},
    {"h": "mais vs pourtant", "p": "mais joins opposites; pourtant concedes one: c'est loin, pourtant on y va.", "ex": {"fr": "C'est loin, <em>pourtant</em> on y va.", "en": "It's far, yet we're going."}},
  ]},
"Sequencing connectors": {
  "rules": [
    {"h": "The story ladder", "p": "d'abord = first, puis or ensuite = then, enfin = finally.", "ex": {"fr": "<em>D'abord</em> on mange, <em>ensuite</em> on joue, <em>enfin</em> on dort.", "en": "First we eat, then we play, finally we sleep."}},
  ]},
}
