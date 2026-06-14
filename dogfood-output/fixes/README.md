# Fix Plan: lacoquille UI/UX Audit

Source audit: `dogfood-output/report.md` (20 issues, severity breakdown below).
Each round picks up related issues, dispatches scout → planner → worker → reviewer, and lands PRs that the user can review.

| Severity | Count |
|----------|-------|
| Critical | 4 |
| High | 6 |
| Medium | 5 |
| Low | 5 |

## Round Plan

| Round | Focus | Issues | Status |
|-------|-------|--------|--------|
| **1** | Practice hub CSS + shadowing backend | ISSUE-001, ISSUE-002 | 🟡 in progress |
| **2** | Article translation/tokenization | ISSUE-003, ISSUE-004 | ⬜ pending |
| **3** | Navigation & missing pages | ISSUE-006, ISSUE-007, ISSUE-011, ISSUE-013, ISSUE-019 | ⬜ pending |
| **4** | Typing Race UX | ISSUE-009, ISSUE-017 | ⬜ pending |
| **5** | Cosmetic & a11y | ISSUE-005, ISSUE-008, ISSUE-012, ISSUE-014, ISSUE-015, ISSUE-016, ISSUE-018, ISSUE-020 | ⬜ pending |

## Conventions

- One branch per issue, named `fix/issue-NNN-slug`.
- Each issue has a plan file `fixes/issue-NNN/PLAN.md`, evidence in `fixes/issue-NNN/EVIDENCE.md`, and the worker's progress in `fixes/issue-NNN/PROGRESS.md`.
- PRs are opened via `github_create_pull_request`; CI must pass before merging.
- After every fix: re-run the relevant browser tests from `dogfood-output/` and attach before/after screenshots.
- Do not bundle unrelated changes.

## Progress

See `PROGRESS.md` for the live log.
