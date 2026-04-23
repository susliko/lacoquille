// TensePage.jsx — bright, bold tense reference
const { useState } = React;

const MOOD_META = {
  indicatif:    { color:'#2563eb', bg:'#eff6ff', label:'Indicatif'    },
  conditionnel: { color:'#7c3aed', bg:'#f5f3ff', label:'Conditionnel' },
  subjonctif:   { color:'#d97706', bg:'#fffbeb', label:'Subjonctif'   },
  imperatif:    { color:'#059669', bg:'#ecfdf5', label:'Impératif'    },
};

const PERSONS = ['je','tu','il','nous','vous','ils'];

const TENSES_DATA = {
  'passe-compose': {
    name:'Passé composé', mood:'indicatif',
    rule:'Completed action at a specific point in the past.',
    what:'The passé composé describes actions that were fully completed at a definite moment. It is the primary narrative past tense in spoken French.',
    when:['A completed action: Il a mangé une pomme.','A series of sequential events: Elle est entrée, a souri, a parlé.','Actions with explicit time markers: hier, à midi, soudain.','An action that interrupted an ongoing state (imparfait).'],
    how:'Auxiliary avoir or être (conjugated in présent) + past participle. Most verbs use avoir. Être verbs: motion verbs and all pronominal verbs.',
    pitfalls:['Auxiliary choice: être vs avoir changes meaning for some verbs (monter, sortir, etc.).','Agreement: with être the past participle agrees in gender/number with the subject.'],
    examples:[{fr:'Il a parlé doucement.',en:'He spoke softly.'},{fr:'Elle est partie tôt.',en:'She left early.'},{fr:'Nous avons fini hier.',en:'We finished yesterday.'}],
    related:['imparfait','plus-que-parfait','present'],
  },
  imparfait: {
    name:'Imparfait', mood:'indicatif',
    rule:'Ongoing, habitual, or descriptive past actions.',
    what:'The imparfait paints the background of a narrative — ongoing states, habits, and descriptions in the past, without specifying when they started or ended.',
    when:['Background description: La maison était grande.','Habitual past actions: Chaque soir, il lisait.','Ongoing action interrupted by PC: Je dormais quand il est entré.','Emotional states and mental processes.'],
    how:'Take the nous form of présent, remove -ons, add: -ais, -ais, -ait, -ions, -iez, -aient.',
    pitfalls:['Être is irregular: j\'étais, tu étais, il était…','Do not confuse ongoing state (imparfait) with completed event (passé composé).'],
    examples:[{fr:'Il parlait doucement.',en:'He was speaking softly.'},{fr:'Chaque été, nous partions en vacances.',en:'Every summer, we went on vacation.'},{fr:'Il faisait beau.',en:'The weather was nice.'}],
    related:['passe-compose','plus-que-parfait'],
  },
  present: {
    name:'Présent', mood:'indicatif',
    rule:'Current states, habits, and universal truths.',
    what:'The présent de l\'indicatif is the foundation of the tense system — it expresses what is happening now, what happens regularly, and established facts.',
    when:['Actions happening now: Je lis ce livre.','Habitual actions: Elle travaille tous les jours.','Universal truths: La Terre tourne autour du Soleil.','Near-future (informal): On part demain.'],
    how:'-er verbs: stem + e,es,e,ons,ez,ent. -ir verbs: stem + is,is,it,issons,issez,issent. Many common verbs are irregular.',
    pitfalls:['Être and avoir are completely irregular — memorise them.','Stem changes in many -er verbs (acheter, appeler…).'],
    examples:[{fr:'Je parle français.',en:'I speak French.'},{fr:'Nous travaillons ensemble.',en:'We work together.'},{fr:'Ils arrivent demain.',en:'They\'re arriving tomorrow.'}],
    related:['passe-compose','imparfait','futur-simple'],
  },
};

const CONJ = {
  parler: {
    'passe-compose':       ['j\'ai parlé','tu as parlé','il a parlé','nous avons parlé','vous avez parlé','ils ont parlé'],
    imparfait:             ['je parlais','tu parlais','il parlait','nous parlions','vous parliez','ils parlaient'],
    present:               ['je parle','tu parles','il parle','nous parlons','vous parlez','ils parlent'],
    'futur-simple':        ['je parlerai','tu parleras','il parlera','nous parlerons','vous parlerez','ils parleront'],
    'plus-que-parfait':    ['j\'avais parlé','tu avais parlé','il avait parlé','nous avions parlé','vous aviez parlé','ils avaient parlé'],
  },
};

const TENSE_LIST = [
  {id:'passe-compose',name:'Passé composé'},{id:'imparfait',name:'Imparfait'},
  {id:'present',name:'Présent'},{id:'futur-simple',name:'Futur simple'},
  {id:'plus-que-parfait',name:'Plus-que-parfait'},{id:'conditionnel-present',name:'Conditionnel présent'},
];

function TensePage({ setRoute, tenseId }) {
  const [activePerson, setActivePerson] = useState('il');
  const data = TENSES_DATA[tenseId] || TENSES_DATA['passe-compose'];
  const m = MOOD_META[data.mood];
  const forms = CONJ.parler[tenseId] || CONJ.parler.present;

  return (
    <main style={{padding:'2.5rem 2rem 5rem', maxWidth:1200, margin:'0 auto'}}>
      <div style={{display:'grid', gridTemplateColumns:'1fr 300px', gap:'3.5rem', alignItems:'start'}}>

        {/* Article */}
        <article>
          {/* Header */}
          <div style={{marginBottom:'2rem', paddingBottom:'1.5rem', borderBottom:'1px solid var(--border-subtle)'}}>
            <span style={{
              display:'inline-flex', alignItems:'center', padding:'3px 12px',
              background:m.bg, color:m.color,
              border:`1.5px solid ${m.color}30`,
              borderRadius:100, fontSize:'0.72rem', fontWeight:600,
              letterSpacing:'0.08em', textTransform:'uppercase', marginBottom:14,
            }}>{m.label}</span>
            <h1 style={{fontFamily:'var(--font-display)', fontSize:'clamp(2rem,5vw,3rem)', fontWeight:800, letterSpacing:'-0.03em', lineHeight:1.1, marginBottom:8}}>
              {data.name}
            </h1>
            <p style={{fontFamily:'var(--font-display)', fontSize:'1.15rem', color:'var(--text-2)', fontStyle:'italic'}}>
              {data.rule}
            </p>
          </div>

          <Section title="What it is">
            <p style={A.p}>{data.what}</p>
          </Section>

          <Section title="When to use">
            <ul style={A.ul}>{data.when.map((w,i) => <li key={i} style={A.li}>{w}</li>)}</ul>
          </Section>

          <Section title="How to form">
            <p style={A.p}>{data.how}</p>
          </Section>

          {data.pitfalls?.length > 0 && (
            <Section title="Common pitfalls">
              <div style={{display:'flex', flexDirection:'column', gap:8}}>
                {data.pitfalls.map((p,i) => (
                  <div key={i} style={{display:'flex', gap:10, padding:'10px 14px', background:m.bg, border:`1px solid ${m.color}30`, borderRadius:8, fontSize:'0.875rem', color:'var(--text)'}}>
                    <span style={{color:m.color, flexShrink:0, fontWeight:700}}>!</span>
                    {p}
                  </div>
                ))}
              </div>
            </Section>
          )}

          <Section title="Examples">
            <div style={{display:'flex', flexDirection:'column'}}>
              {data.examples.map((ex,i) => (
                <div key={i} style={{padding:'10px 0', borderBottom:'1px solid var(--border-subtle)'}}>
                  <div style={{fontFamily:'var(--font-mono)', fontSize:'0.9rem', color:'var(--text)', marginBottom:3}}>{ex.fr}</div>
                  <div style={{fontSize:'0.85rem', color:'var(--text-2)'}}>{ex.en}</div>
                </div>
              ))}
            </div>
          </Section>

          {data.related?.length > 0 && (
            <Section title="Related tenses">
              <div style={{display:'flex', flexWrap:'wrap', gap:8}}>
                {data.related.map(r => {
                  const t = TENSE_LIST.find(t=>t.id===r);
                  return t ? (
                    <span key={r} onClick={()=>setRoute({page:'tense',tenseId:r})}
                      style={{display:'inline-block', padding:'5px 14px', border:'1.5px solid var(--border)', borderRadius:100, fontSize:'0.82rem', color:'var(--text-2)', cursor:'pointer'}}>
                      {t.name}
                    </span>
                  ) : null;
                })}
              </div>
            </Section>
          )}

          <span onClick={()=>setRoute({page:'verbs'})} style={{
            display:'inline-flex', alignItems:'center', gap:6, marginTop:'2.5rem',
            fontSize:'0.875rem', color:'var(--text-2)', cursor:'pointer',
          }}>
            ← Retour aux Verbes
          </span>
        </article>

        {/* Sidebar */}
        <aside style={{position:'sticky', top:76}}>
          <div style={{background:'white', border:'1.5px solid var(--border-subtle)', borderRadius:12, overflow:'hidden'}}>
            <div style={{padding:'14px 16px 12px', borderBottom:'1px solid var(--border-subtle)', background:m.bg}}>
              <div style={{fontSize:'0.65rem', fontWeight:600, letterSpacing:'0.12em', textTransform:'uppercase', color:m.color, marginBottom:6}}>
                Conjugaison
              </div>
              <div style={{fontFamily:'var(--font-mono)', fontSize:'0.875rem', color:m.color, fontWeight:500}}>
                parler
              </div>
            </div>
            <div style={{padding:'10px 12px 8px', display:'flex', gap:3, flexWrap:'wrap', borderBottom:'1px solid var(--border-subtle)'}}>
              {PERSONS.map(p => (
                <button key={p} onClick={()=>setActivePerson(p)} style={{
                  fontFamily:'var(--font-mono)', fontSize:'0.7rem', padding:'3px 8px', borderRadius:6, border:'1.5px solid',
                  background: p===activePerson ? m.color : 'white',
                  color: p===activePerson ? 'white' : 'var(--text-2)',
                  borderColor: p===activePerson ? m.color : 'var(--border)',
                  cursor:'pointer',
                }}>{p}</button>
              ))}
            </div>
            <table style={{width:'100%', borderCollapse:'collapse'}}>
              <tbody>
                {PERSONS.map((p,i) => (
                  <tr key={p} onClick={()=>setActivePerson(p)} style={{background: p===activePerson ? m.bg : 'transparent', cursor:'pointer'}}>
                    <th style={{fontFamily:'var(--font-mono)', fontSize:'0.72rem', fontWeight:400, textAlign:'right', padding:'5px 8px', borderBottom:'1px solid var(--border-subtle)', width:40, color: p===activePerson ? m.color : 'var(--text-muted)'}}>
                      {p}
                    </th>
                    <td style={{fontFamily:'var(--font-mono)', fontSize:'0.875rem', padding:'5px 10px', borderBottom:'1px solid var(--border-subtle)', color: p===activePerson ? m.color : 'var(--text)', fontWeight: p===activePerson ? 500 : 400}}>
                      {forms?.[i] || '—'}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </aside>
      </div>
    </main>
  );
}

function Section({ title, children }) {
  return (
    <section style={{marginTop:'1.75rem'}}>
      <h2 style={{fontFamily:'var(--font-display)', fontSize:'1.3rem', fontWeight:800, letterSpacing:'-0.02em', marginBottom:'0.75rem'}}>
        {title}
      </h2>
      {children}
    </section>
  );
}

const A = {
  p: {fontSize:'0.95rem', color:'var(--text)', lineHeight:1.75, fontWeight:300},
  ul: {paddingLeft:'1.4rem', margin:0},
  li: {fontSize:'0.9rem', color:'var(--text)', lineHeight:1.65, fontWeight:300, marginBottom:'0.3rem'},
};

Object.assign(window, { TensePage });
