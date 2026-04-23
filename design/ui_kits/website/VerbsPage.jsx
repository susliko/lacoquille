// VerbsPage.jsx — bright, colorful verb hub
const { useState } = React;

const MOOD_META = {
  indicatif:    { color:'#2563eb', bg:'#eff6ff', label:'Indicatif'    },
  conditionnel: { color:'#7c3aed', bg:'#f5f3ff', label:'Conditionnel' },
  subjonctif:   { color:'#d97706', bg:'#fffbeb', label:'Subjonctif'   },
  imperatif:    { color:'#059669', bg:'#ecfdf5', label:'Impératif'    },
};

const TENSES = [
  { id:'plus-que-parfait',     name:'Plus-que-parfait',     mood:'indicatif',    rule:'Action completed before another past action.' },
  { id:'imparfait',            name:'Imparfait',            mood:'indicatif',    rule:'Ongoing, habitual, or descriptive past.' },
  { id:'passe-compose',        name:'Passé composé',        mood:'indicatif',    rule:'Completed action at a definite past moment.' },
  { id:'present',              name:'Présent',              mood:'indicatif',    rule:'Current states, habits, universal truths.' },
  { id:'futur-simple',         name:'Futur simple',         mood:'indicatif',    rule:'Future actions and predictions.' },
  { id:'futur-anterieur',      name:'Futur antérieur',      mood:'indicatif',    rule:'Action completed before a future reference.' },
  { id:'conditionnel-present', name:'Conditionnel présent', mood:'conditionnel', rule:'Hypothetical actions, polite requests.' },
  { id:'conditionnel-passe',   name:'Conditionnel passé',   mood:'conditionnel', rule:'Unreal past conditions; regret.' },
  { id:'subjonctif-present',   name:'Subjonctif présent',   mood:'subjonctif',   rule:'Doubt, will, emotion, obligation.' },
  { id:'subjonctif-passe',     name:'Subjonctif passé',     mood:'subjonctif',   rule:'Completed action in subjunctive context.' },
  { id:'imperatif-present',    name:'Impératif présent',    mood:'imperatif',    rule:'Commands, instructions, requests.' },
];

const PERSONS = ['je','tu','il','nous','vous','ils'];
const CHOICES = [
  'Imparfait vs Passé composé',
  'Déclencheurs du subjonctif',
  'Propositions en si',
  'Futur vs Conditionnel',
  'Discours rapporté',
];

const CONJ = {
  parler: {
    'passe-compose':       ['j\'ai parlé','tu as parlé','il a parlé','ns avons parlé','vs avez parlé','ils ont parlé'],
    imparfait:             ['je parlais','tu parlais','il parlait','ns parlions','vs parliez','ils parlaient'],
    present:               ['je parle','tu parles','il parle','ns parlons','vs parlez','ils parlent'],
    'futur-simple':        ['je parlerai','tu parleras','il parlera','ns parlerons','vs parlerez','ils parleront'],
    'plus-que-parfait':    ['j\'avais parlé','tu avais parlé','il avait parlé','ns avions parlé','vs aviez parlé','ils avaient parlé'],
    'futur-anterieur':     ['j\'aurai parlé','tu auras parlé','il aura parlé','ns aurons parlé','vs aurez parlé','ils auront parlé'],
    'conditionnel-present':['je parlerais','tu parlerais','il parlerait','ns parlerions','vs parleriez','ils parleraient'],
    'conditionnel-passe':  ['j\'aurais parlé','tu aurais parlé','il aurait parlé','ns aurions parlé','vs auriez parlé','ils auraient parlé'],
    'subjonctif-present':  ['que je parle','que tu parles','qu\'il parle','que ns parlions','que vs parliez','qu\'ils parlent'],
    'subjonctif-passe':    ['que j\'aie parlé','que tu aies parlé','qu\'il ait parlé','—','—','—'],
    'imperatif-present':   ['—','parle','—','parlons','parlez','—'],
  },
};

function TenseCard({ tense, person, verb, setRoute }) {
  const [hov, setHov] = useState(false);
  const m = MOOD_META[tense.mood];
  const pIdx = PERSONS.indexOf(person);
  const form = CONJ[verb]?.[tense.id]?.[pIdx] || '—';

  return (
    <div
      style={{
        background: hov ? m.bg : 'white',
        border: `1.5px solid ${hov ? m.color : 'var(--border-subtle)'}`,
        borderRadius: 10, padding:'12px 14px',
        cursor:'pointer', transition:'all .15s',
        minWidth: 150, flex:'1 1 150px',
        borderTop: `4px solid ${m.color}`,
      }}
      onMouseEnter={()=>setHov(true)} onMouseLeave={()=>setHov(false)}
      onClick={()=>setRoute({page:'tense', tenseId: tense.id})}
    >
      <div style={{fontSize:'0.875rem', fontWeight:600, fontFamily:'var(--font-display)', color:'var(--text)', marginBottom:4}}>
        {tense.name}
      </div>
      <div style={{fontFamily:'var(--font-mono)', fontSize:'0.78rem', color: m.color, fontWeight:500}}>
        {form}
      </div>
    </div>
  );
}

function VerbsPage({ setRoute }) {
  const [person, setPerson] = useState('il');
  const [verb, setVerb] = useState('parler');
  const moods = ['indicatif','conditionnel','subjonctif','imperatif'];

  return (
    <main style={{padding:'2.5rem 2rem 5rem', maxWidth:1200, margin:'0 auto'}}>
      <div style={{marginBottom:'2rem'}}>
        <div style={{fontSize:'0.75rem', fontWeight:600, letterSpacing:'0.12em', textTransform:'uppercase', color:'var(--primary)', marginBottom:8}}>
          Grammaire · Les Verbes
        </div>
        <h1 style={{fontFamily:'var(--font-display)', fontSize:'clamp(2rem,5vw,3rem)', fontWeight:800, letterSpacing:'-0.03em', lineHeight:1.1, marginBottom:8}}>
          Verbes
        </h1>
        <p style={{fontSize:'1rem', color:'var(--text-2)', maxWidth:'52ch', fontWeight:300}}>
          The Grammar Map — every French tense connected by semantic relations. Pick a subject and watch the conjugations update live.
        </p>
      </div>

      {/* Controls */}
      <div style={{display:'flex', alignItems:'center', gap:16, flexWrap:'wrap', padding:'12px 16px', background:'var(--surface)', border:'1.5px solid var(--border-subtle)', borderRadius:'12px 12px 0 0', borderBottom:'none'}}>
        <div style={{display:'flex', alignItems:'center', gap:8}}>
          <span style={{fontSize:'0.7rem', fontWeight:600, letterSpacing:'0.1em', textTransform:'uppercase', color:'var(--text-muted)'}}>Verbe</span>
          <select value={verb} onChange={e=>setVerb(e.target.value)}
            style={{background:'white', border:'1.5px solid var(--border)', borderRadius:7, color:'var(--text)', fontFamily:'var(--font-mono)', fontSize:'0.85rem', padding:'4px 8px', outline:'none'}}>
            <option value="parler">parler</option>
          </select>
        </div>
        <div style={{display:'flex', alignItems:'center', gap:8}}>
          <span style={{fontSize:'0.7rem', fontWeight:600, letterSpacing:'0.1em', textTransform:'uppercase', color:'var(--text-muted)'}}>Sujet</span>
          <div style={{display:'flex', gap:3}}>
            {PERSONS.map(p => (
              <button key={p} onClick={()=>setPerson(p)} style={{
                fontFamily:'var(--font-mono)', fontSize:'0.75rem', padding:'4px 9px', borderRadius:6,
                border:'1.5px solid', cursor:'pointer', transition:'all .15s',
                background: p===person ? 'var(--primary)' : 'white',
                color: p===person ? 'white' : 'var(--text-2)',
                borderColor: p===person ? 'var(--primary)' : 'var(--border)',
                fontWeight: p===person ? 600 : 400,
              }}>{p}</button>
            ))}
          </div>
        </div>
      </div>

      {/* Mood lanes */}
      <div style={{border:'1.5px solid var(--border-subtle)', borderRadius:'0 0 12px 12px', overflow:'hidden'}}>
        {moods.map((mood,mi) => {
          const m = MOOD_META[mood];
          const moodTenses = TENSES.filter(t => t.mood === mood);
          return (
            <div key={mood} style={{
              borderBottom: mi < moods.length-1 ? '1px solid var(--border-subtle)' : 'none',
              padding:'14px 16px',
              background: mi % 2 === 0 ? 'white' : 'var(--surface)',
            }}>
              <div style={{display:'flex', alignItems:'center', gap:8, marginBottom:10}}>
                <span style={{display:'inline-block', width:10, height:10, borderRadius:'50%', background:m.color}} />
                <span style={{fontFamily:'var(--font-mono)', fontSize:'0.7rem', fontWeight:600, letterSpacing:'0.1em', textTransform:'uppercase', color:m.color}}>
                  {m.label}
                </span>
              </div>
              <div style={{display:'flex', gap:8, flexWrap:'wrap'}}>
                {moodTenses.map(t => (
                  <TenseCard key={t.id} tense={t} person={person} verb={verb} setRoute={setRoute} />
                ))}
              </div>
            </div>
          );
        })}
      </div>

      {/* Index + choices */}
      <div style={{display:'grid', gridTemplateColumns:'1fr 1fr', gap:'3rem', marginTop:'3rem'}}>
        <div>
          <h2 style={{fontFamily:'var(--font-display)', fontSize:'1.5rem', fontWeight:800, letterSpacing:'-0.02em', marginBottom:'1rem'}}>
            Tous les temps
          </h2>
          <div style={{display:'flex', flexDirection:'column'}}>
            {TENSES.map(t => {
              const m = MOOD_META[t.mood];
              return (
                <div key={t.id}
                  onClick={()=>setRoute({page:'tense', tenseId:t.id})}
                  style={{display:'flex', flexDirection:'column', padding:'10px 0', borderBottom:'1px solid var(--border-subtle)', cursor:'pointer', gap:2}}>
                  <div style={{display:'flex', alignItems:'center', gap:7}}>
                    <span style={{width:7, height:7, borderRadius:'50%', background:m.color, flexShrink:0}} />
                    <span style={{fontFamily:'var(--font-display)', fontSize:'1rem', fontWeight:600, color:'var(--text)'}}>{t.name}</span>
                  </div>
                  <span style={{fontSize:'0.78rem', color:'var(--text-2)', paddingLeft:14}}>{t.rule}</span>
                </div>
              );
            })}
          </div>
        </div>
        <div>
          <h2 style={{fontFamily:'var(--font-display)', fontSize:'1.5rem', fontWeight:800, letterSpacing:'-0.02em', marginBottom:'1rem'}}>
            Guides de choix
          </h2>
          <div style={{display:'flex', flexDirection:'column', gap:8}}>
            {CHOICES.map(c => (
              <div key={c} style={{
                padding:'12px 16px', background:'var(--surface)',
                border:'1.5px solid var(--border-subtle)', borderRadius:10,
                fontSize:'0.9rem', color:'var(--text-2)', cursor:'pointer',
                display:'flex', justifyContent:'space-between', alignItems:'center',
              }}>
                {c}
                <span style={{color:'var(--primary)', fontSize:'0.8rem'}}>→</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </main>
  );
}

Object.assign(window, { VerbsPage });
