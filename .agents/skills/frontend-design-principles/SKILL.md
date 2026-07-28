---
name: frontend-design-principles
description: Core visual principles for frontend work - subtract text, prefer SVG over prose, detail behind summary, one CTA. Use when building or editing UI, React components, pages, dashboards, docs or marketing sections, modals, or reviewing a frontend diff.
---

# Frontend design principles

Apply to every UI change.

**1. Subtract.** "Perfection is when there is nothing left to subtract." Before
adding anything, delete something: helper text, headings, borders, badges,
duplicated labels.

**2. A picture beats a thousand words.** If an icon or diagram can say it, don't
write the paragraph. Reuse below first, then https://svgl.app or
https://eito.me/icons. Inline it.

**3. Detail → summary.** Show the summary only; reveal detail on hover, click,
or expand. Never dump the full record upfront.

**4. Keep the main thing the main thing.** Exactly one CTA per view, with the
only high-contrast treatment on screen. Everything else stays dull, so contrast
itself points the eye. Same CTA color, shape, and placement app-wide.

## Reuse before creating

**Desktop UI (React + Vite), `src/`**

- Components: `src/components/Chat.tsx`, `SettingsPanel.tsx`,
  `UpdateNotification.tsx`. `Chat.tsx` already inlines its gear and send marks
  as JSX with `stroke="currentColor"` and `aria-hidden="true"`. Follow that
  pattern for new icons; do not add an icon package for a handful of marks.
- Every rule and animation lives in `src/App.css`, including the keyframes
  `message-in`, `typing-bounce`, `update-banner-slide-in`, and
  `update-indeterminate`. Reuse an existing keyframe before writing a new one,
  and keep new rules in that file rather than inline styles.
- Static marks: `src/assets/react.svg`, `public/vite.svg`, `public/tauri.svg`.
  Replace these when branding the app rather than adding placeholders beside
  them.
- Anything the UI needs from the backend comes through a Tauri command, not a
  direct import from `src-tauri/` or `crates/`. See the `update-backend` skill
  before reaching for new data.

**Docs (Next.js + Fumadocs), `docs/`**

- Shared page chrome: `components/ai/page-actions.tsx`,
  `lib/layout.shared.tsx`.
- There is no icon registry yet. To add sidebar icons, create
  `components/icons.tsx` with inline SVG components and resolve them from an
  `icon()` handler in `lib/source.ts`, keyed off each page's `icon:`
  frontmatter. Ship the mark in the bundle; never hot-link an external asset.
