// Nav.jsx — bright, clean
const { useState } = React;

function SiteNav({ route, setRoute }) {
  const [open, setOpen] = useState(false);
  const links = [
    { label: 'Verbes', page: 'verbs' },
    { label: 'Pronoms', page: null },
    { label: 'Adjectifs', page: null },
    { label: 'Articles', page: null },
  ];
  const crumbs = route.page === 'verbs'
    ? [{ label: 'Verbes', page: 'verbs' }]
    : route.page === 'tense'
      ? [{ label: 'Verbes', page: 'verbs' }, { label: route.tenseId?.replace(/-/g,' ') }]
      : [];

  return (
    <>
      <nav style={S.nav}>
        <span style={S.brand} onClick={() => setRoute({page:'home'})}>La Coquille</span>
        {crumbs.length > 0 && (
          <div style={S.crumbs}>
            {crumbs.map((c,i) => (
              <span key={i} style={{display:'flex',alignItems:'center',gap:6}}>
                {i > 0 && <span style={{color:'var(--border-bright)'}}>›</span>}
                {c.page
                  ? <span onClick={()=>setRoute({page:c.page})} style={S.crumbLink}>{c.label}</span>
                  : <span style={{color:'var(--text)',fontWeight:500,fontSize:'0.85rem'}}>{c.label}</span>}
              </span>
            ))}
          </div>
        )}
        <ul style={S.links}>
          {links.map(l => (
            <li key={l.label} style={{margin:0}}>
              <span onClick={()=>l.page&&setRoute({page:l.page})} style={{
                ...S.link,
                background: route.page===l.page ? 'var(--primary-soft)' : 'transparent',
                color: route.page===l.page ? 'var(--primary)' : (l.page ? 'var(--text-2)' : 'var(--text-muted)'),
                fontWeight: route.page===l.page ? 500 : 400,
                cursor: l.page ? 'pointer' : 'default',
              }}>{l.label}</span>
            </li>
          ))}
        </ul>
        <button style={S.hamburger} onClick={()=>setOpen(o=>!o)}>☰</button>
      </nav>
      {open && (
        <div style={S.drawer}>
          {links.map(l => (
            <span key={l.label} onClick={()=>{if(l.page){setRoute({page:l.page});setOpen(false);}}}
              style={{...S.drawerLink, color: l.page?'var(--text)':'var(--text-muted)', cursor:l.page?'pointer':'default'}}>
              {l.label}
              {l.page && <span style={{color:'var(--primary)'}}>→</span>}
            </span>
          ))}
        </div>
      )}
    </>
  );
}

const S = {
  nav: {
    background:'var(--bg)', borderBottom:'1px solid var(--border-subtle)',
    padding:'0 2rem', height:60,
    display:'flex', alignItems:'center', gap:12,
    position:'sticky', top:0, zIndex:100,
  },
  brand: {
    fontFamily:'var(--font-display)', fontSize:'1.2rem', fontWeight:800,
    color:'var(--primary)', cursor:'pointer', letterSpacing:'-0.02em', flexShrink:0,
  },
  crumbs: { display:'flex', alignItems:'center', gap:6, flex:1, overflow:'hidden', marginLeft:8 },
  crumbLink: { fontSize:'0.85rem', color:'var(--text-2)', cursor:'pointer' },
  links: {
    display:'flex', alignItems:'center', gap:2, listStyle:'none', padding:0, marginLeft:'auto',
  },
  link: {
    fontSize:'0.875rem', padding:'5px 12px', borderRadius:8,
    transition:'background .15s, color .15s',
  },
  hamburger: {
    display:'none', background:'none', border:'1px solid var(--border-subtle)',
    borderRadius:8, padding:'5px 10px', color:'var(--text-2)', marginLeft:'auto',
  },
  drawer: {
    position:'fixed', top:60, left:0, right:0, background:'var(--bg)',
    borderBottom:'1px solid var(--border-subtle)', zIndex:100,
    padding:'0.75rem 2rem 1rem', display:'flex', flexDirection:'column', gap:4,
  },
  drawerLink: {
    display:'flex', alignItems:'center', justifyContent:'space-between',
    padding:'0.65rem 0.5rem', fontSize:'0.95rem',
    borderBottom:'1px solid var(--border-subtle)',
  },
};

Object.assign(window, { SiteNav });
