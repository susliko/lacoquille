# Implementation Plan

## Goal
Make the "durée" label visible on the mood-swap dashed edge between Présent (indicatif) and Subjonctif présent, and add a "Labels" section to the diagram Key explaining what it means.

## Verification of Current State

**`VerbDiagram.tsx` — edge rendering (lines ~582–609 desktop, ~479–508 mobile):**
- The `edge.label` field from `diagram.json` is used **only** in the `aria-label` attribute
- There is **no** `<text>` SVG element that renders edge labels visually
- The mood-swap edge between `present-indicatif` and `subjonctif-present` currently has label `"same time, different mood"` in `diagram.json`

**`diagram.json` — mood-swap edge (line ~200):**
```json
{
  "type": "mood-swap",
  "from": "present-indicatif",
  "to": "subjonctif-present",
  "label": "same time, different mood"
}
```

**`Legend` component (lines ~319–358):**
- Has two sections: "Difficulty" and "Connections"
- No "Labels" section exists

## Tasks

### Task 1: Update the mood-swap edge label in `diagram.json`
- **File**: `src/content/verbs/diagram.json`
- **Change**: Replace `"label": "same time, different mood"` with `"label": "durée"` on the mood-swap edge between `present-indicatif` and `subjonctif-present`
- **Rationale**: "durée" is short enough to render legibly at font-size 11, and matches the French terminology used in the original issue

### Task 2: Add visible SVG `<text>` element for edge labels (desktop)
- **File**: `src/components/VerbDiagram.tsx`
- **Change**: In the desktop edge `<g>` block (inside the `<For each={visibleEdges()}>` callback, after the transparent hit-test `<path>`), add a `<text>` element:
  ```tsx
  {/* Edge label */}
  <text
    x={(from.cx + to.cx) / 2}
    y={(from.cy + to.cy) / 2 - 6}
    text-anchor="middle"
    font-size="11"
    font-style="italic"
    fill={edgeColor}
    opacity={isActive() ? 1 : 0.85}
    style={{ "pointer-events": "none" }}
  >
    {edge.label}
  </text>
  ```
- **Placement**: Midpoint of the straight-line segment between card centers, with a 6px upward offset so the text sits above the edge line
- **Style**: Italic 11px, same color as the edge (via `fill={edgeColor}`), reduced opacity when not active to avoid visual noise

### Task 3: Add visible SVG `<text>` element for edge labels (mobile)
- **File**: `src/components/VerbDiagram.tsx`
- **Change**: In the mobile edge `<g>` block, add the same `<text>` element pattern:
  ```tsx
  {/* Edge label */}
  <text
    x={(from.cx + to.cx) / 2}
    y={(from.cy + to.cy) / 2 - 5}
    text-anchor="middle"
    font-size="9"
    font-style="italic"
    fill={edgeColor}
    opacity={isActive() ? 1 : 0.85}
    style={{ "pointer-events": "none" }}
  >
    {edge.label}
  </text>
  ```
- **Note**: Mobile uses font-size 9 (vs 11 on desktop) due to the smaller SVG viewport. 9px is still legible at 375px screen width.

### Task 4: Add "Labels" section to the Legend
- **File**: `src/components/VerbDiagram.tsx`
- **Change**: After the "Connections" `<div class="legend-section">` block, add a new section:
  ```tsx
  <div class="legend-section">
    <p class="legend-section-title">Labels</p>
    <div class="legend-grid">
      <div class="legend-item">
        <svg width="32" height="10" aria-hidden="true">
          <line x1="0" y1="5" x2="32" y2="5"
            stroke="var(--edge-mood)" stroke-width="2.5"
            stroke-dasharray="11 5" stroke-linecap="round" />
        </svg>
        <span><em>durée</em> — ongoing action; same form across moods</span>
      </div>
    </div>
  </div>
  ```
- **Note**: Uses `var(--edge-mood)` CSS variable and the `11 5` dash pattern from `EDGE_DASH["mood-swap"]` to render a representative mood-swap line

## Files to Modify

- `src/content/verbs/diagram.json` — update mood-swap edge label from "same time, different mood" to "durée"
- `src/components/VerbDiagram.tsx` — add SVG `<text>` element in both desktop and mobile edge blocks; add "Labels" section to Legend

## New Files
None.

## Dependencies
- Task 2 and Task 3 are independent of each other but both depend on Task 1 (label must exist in JSON before rendering it)
- Task 4 is independent of Tasks 1–3

## Risks

| Risk | Mitigation |
|------|------------|
| The `<text>` element overlaps with node card text on busy parts of the diagram | Use `pointer-events: none` so it doesn't block clicks; text is positioned at the edge midpoint which is typically between cards, not on top of them |
| The mood-swap edge is a bezier curve (cross-lane), so the straight-line midpoint may be slightly off from the visual curve midpoint | Acceptable: the bezier control point is at `from.cx` (same X), so the visual midpoint is close to the straight-line midpoint. For cross-lane edges, the text will appear slightly above the curve center — this is acceptable for edge labels. |
| Mobile font-size 9 may still be too small on low-DPI screens | Test on 375px viewport; 9px italic on a 375px wide SVG (scale factor ~1.5–2x) renders at ~13–18px effective size, which meets WCAG 4.5:1 contrast requirements |
| The `--edge-mood` CSS variable may not be defined | Check `src/styles/global.css` or component CSS — the code uses `EDGE_COLOR["mood-swap"]` which maps to a hex color. Use the hex value directly (`#a78bfa` for violet) or ensure the CSS variable is defined |

## Acceptance Criteria

1. `npm run build` passes with no TypeScript or Astro errors
2. The "durée" label is visible on the mood-swap dashed edge between Présent and Subjonctif présent (font size ≥ 9px on mobile, ≥ 11px on desktop)
3. The label has sufficient contrast (fill color matches edge color; opacity 0.85 when not active, 1.0 when active)
4. The Key (Legend) has a "Labels" section that explains "durée" as indicating ongoing action across moods
5. No regression: all other edges, nodes, and interactive elements render and function correctly
6. The label does not overlap with any node card text (verified by visual inspection)

## Verification Commands

```sh
# Build check
cd /home/susliko/programming/lacoquille && npm run build

# Type check
npx astro check
```

Browser verification (manual, in main session):
1. Open `/reference/verbs/` in agent-browser
2. Take a screenshot; verify "durée" label is visible on the dashed edge between Présent and Subjonctif présent
3. Expand the Key; verify the new "Labels" section appears with the explanation

## Branch and PR

- **Branch**: `fix/issue-005-verb-diagram-label`
- **PR title**: `fix(verbs): make durée label visible and add Key entry (ISSUE-005, ISSUE-014)`
