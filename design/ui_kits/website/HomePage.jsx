// HomePage.jsx — polygon hero nav (Aphyr-inspired, no hover needed)
const { useState, useCallback } = React;

const REGIONS = [
  { id:'syntaxe',   name:'Syntaxe',   sub:'word order',          color:'#6366f1', points:'0,0 300,0 300,100 200,100 0,100',            cx:130, cy:45,  fs:16, anim:'pr-0', available:false },
  { id:'verbs',     name:'Verbes',    sub:'tenses & conjugation', color:'#2563eb', points:'300,0 680,0 680,90 300,100',                   cx:488, cy:37,  fs:22, anim:'pr-1', available:true  },
  { id:'pronoms',   name:'Pronoms',   sub:'personal · relative',  color:'#7c3aed', points:'680,0 1200,0 1200,270 820,270 680,90',         cx:916, cy:110, fs:18, anim:'pr-2', available:false },
  { id:'adjectifs', name:'Adjectifs', sub:'accord · ordre',       color:'#d97706', points:'820,270 1200,270 1200,420 660,420',            cx:952, cy:334, fs:18, anim:'pr-3', available:false },
  { id:'articles',  name:'Articles',  sub:'défini · partitif',    color:'#059669', points:'660,420 1200,420 1200,500 650,500',            cx:912, cy:451, fs:15, anim:'pr-4', available:false },
  { id:'negation',  name:'Négation',  sub:'ne…pas · jamais',      color:'#0891b2', points:'310,430 660,420 650,500 0,500 0,430',          cx:325, cy:463, fs:15, anim:'pr-5', available:false },
  { id:'preps',     name:'Prép.',     sub:'à · de · en',          color:'#0d9488', points:'0,260 130,260 310,430 0,430',                  cx:100, cy:344, fs:12, anim:'pr-6', available:false },
  { id:'adverbes',  name:'Adverbes',  sub:'formation',            color:'#16a34a', points:'0,100 200,100 130,260 0,260',                  cx:76,  cy:175, fs:13, anim:'pr-7', available:false },
];

function toast(msg) {
  const el = document.createElement('div');
  el.className = 'toast';
  el.textContent = msg;
  document.body.appendChild(el);
  setTimeout(() => el.remove(), 2000);
}

function PolygonHero({ setRoute }) {
  const handle = useCallback((r) => {
    if (r.available) setRoute({ page: r.id });
    else toast(r.name + ' — coming soon');
  }, [setRoute]);

  return (
    <div style={{ width:'100%', background:'var(--bg)', overflow:'hidden' }}>
      <svg viewBox="0 0 1200 500" style={{ width:'100%', height:'auto', display:'block' }}
           xmlns="http://www.w3.org/2000/svg">

        {/* Outer regions */}
        {REGIONS.map(r => (
          <g key={r.id} className={`poly-region ${r.anim}${r.available?' clickable':''}`}
             onClick={() => handle(r)}>
            <polygon points={r.points} fill={r.color} stroke="white" strokeWidth="6" strokeLinejoin="round"/>
            <text x={r.cx} y={r.cy} textAnchor="middle"
                  fontFamily="'Syne'" fontSize={r.fs} fontWeight="700" fill="white"
                  style={{pointerEvents:'none'}}>
              {r.name}
            </text>
            {r.sub && r.fs >= 15 && (
              <text x={r.cx} y={r.cy + r.fs * 0.95} textAnchor="middle"
                    fontFamily="'DM Sans'" fontSize={Math.max(10, r.fs * 0.62)} fill="rgba(255,255,255,0.7)"
                    style={{pointerEvents:'none'}}>
                {r.sub}
              </text>
            )}
            {r.available && (
              <text x={r.cx} y={r.cy + r.fs * 1.9} textAnchor="middle"
                    fontFamily="'DM Sans'" fontSize="12" fill="rgba(255,255,255,0.95)" fontWeight="600"
                    style={{pointerEvents:'none'}}>
                ● explore now →
              </text>
            )}
          </g>
        ))}

        {/* Brand center */}
        <g className="poly-region pr-brand" style={{cursor:'default'}}>
          <polygon points="200,100 300,100 680,90 820,270 660,420 310,430 130,260"
                   fill="#f0efff" stroke="white" strokeWidth="6" strokeLinejoin="round"/>
          {/* subtle grid pattern inside brand */}
          <text x="448" y="204" textAnchor="middle"
                fontFamily="'Syne'" fontSize="54" fontWeight="800" fill="#3730a3"
                letterSpacing="-1" style={{pointerEvents:'none'}}>
            La Coquille
          </text>
          <text x="448" y="250" textAnchor="middle"
                fontFamily="'DM Sans'" fontSize="17" fill="#6b7280" fontWeight="400"
                style={{pointerEvents:'none'}}>
            Grammaire française interactive
          </text>
          <text x="448" y="290" textAnchor="middle"
                fontFamily="'DM Sans'" fontSize="13" fill="#9ca3af"
                style={{pointerEvents:'none'}}>
            Click any topic to explore
          </text>
        </g>
      </svg>
    </div>
  );
}

function HomePage({ setRoute }) {
  return (
    <div style={{ background:'var(--bg)', minHeight:'calc(100vh - 60px)' }}>
      <PolygonHero setRoute={setRoute} />

      {/* Strip below polygon */}
      <div style={{ maxWidth:1200, margin:'0 auto', padding:'2.5rem 2rem 4rem' }}>
        <div style={strip.row}>
          <div style={strip.card}>
            <div style={strip.num}>18</div>
            <div style={strip.label}>tense pages</div>
          </div>
          <div style={strip.card}>
            <div style={strip.num}>5</div>
            <div style={strip.label}>choice guides</div>
          </div>
          <div style={strip.card}>
            <div style={strip.num}>∞</div>
            <div style={strip.label}>always free</div>
          </div>
          <div style={{...strip.card, flex:3, alignItems:'flex-start', background:'var(--primary)', borderColor:'var(--primary)'}}>
            <div style={{fontFamily:'var(--font-display)', fontSize:'1.1rem', fontWeight:700, color:'white', marginBottom:6}}>
              Start with Verbes →
            </div>
            <div style={{fontSize:'0.875rem', color:'rgba(255,255,255,0.75)', fontWeight:300}}>
              The Grammar Map connects all 18 tenses by their semantic relations. It's the only one of its kind.
            </div>
            <button onClick={()=>setRoute({page:'verbs'})} style={strip.btn}>
              Explore the diagram
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

const strip = {
  row: { display:'grid', gridTemplateColumns:'1fr 1fr 1fr 3fr', gap:12 },
  card: {
    background:'var(--surface)', border:'1px solid var(--border-subtle)',
    borderRadius:12, padding:'1.25rem 1.5rem',
    display:'flex', flexDirection:'column', gap:4,
  },
  num: { fontFamily:'var(--font-display)', fontSize:'2.2rem', fontWeight:800, color:'var(--primary)', lineHeight:1 },
  label: { fontSize:'0.8rem', color:'var(--text-2)', fontWeight:400 },
  btn: {
    marginTop:16, alignSelf:'flex-start',
    background:'white', color:'var(--primary)', border:'none',
    padding:'8px 18px', borderRadius:8, fontWeight:600, fontSize:'0.875rem',
    cursor:'pointer',
  },
};

Object.assign(window, { HomePage });
