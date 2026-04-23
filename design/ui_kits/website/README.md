# La Coquille — Website UI Kit

A high-fidelity interactive prototype of the La Coquille website. Click through the navigation to explore all screens.

## Screens

| Screen | Route |
|---|---|
| Home | Topic grid with hero section |
| Verbes | Grammar Map diagram preview + tense/choice index |
| Tense page | Full reference page with conjugation sidebar |

## Components

- **Nav.jsx** — Sticky navigation with brand, links, breadcrumb, mobile drawer
- **HomePage.jsx** — Hero with ghost watermark, topic grid (available/coming-soon states)
- **VerbsPage.jsx** — Grammar Map diagram (mood lanes + tense cards), controls bar (verb selector, person pills, literary toggle), tense/choice index
- **TensePage.jsx** — Article layout with sections, sidebar conjugation table with person selector, related tense pills

## Design width
1200px container, responsive to viewport.

## Usage
Open `index.html` directly in a browser. Navigation state is persisted in localStorage.
