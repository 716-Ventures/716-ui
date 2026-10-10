# Changelog

## 0.1.0-alpha.4

Adds read-only `List@1` and `KeyValue@1` schemas and `DataList`, with stable keyed selection, cleared selection after removal and native virtualized rows. Accessible collection labels are required. The shadcn Item source is pinned alongside the existing profile. ARM64 Linux CI 38030789105 passed pure/native checks and mapped 4,000-row gallery scenarios across both appearances, scales 1/2 and reduced motion. Physical assistive input and full reference appearance/motion parity remain unqualified.

## 0.1.0-alpha.3

Adds typed `Table@1` schemas and native `DataTable` with bounded unique row keys, resizable columns and selection preserved across keyed updates/reorder. ARM64 Linux CI 38027478239 verified allocated virtualized cells across 4,000 rows and both appearances/scales. Full reference and physical input parity remain unqualified.

## 0.1.0-alpha.2

Adds a typed, versioned component catalog usable without the native feature or GTK. Adds labeled multiline text editing with local undo, Unicode selection/IME support supplied by GTK, Ctrl+Enter submission, and protection against external reconciliation while focused. Long unbroken text wraps within its reading surface.

ARM64 Linux CI run 38017369881 passed metadata-only tests, native tests and light/dark scale 1/2 scenarios including multiline draft preservation. Screen-reader journeys, physical IME/clipboard workflows and full shadcn reference parity remain unqualified. Rust 1.85.1 is the verified toolchain.

## 0.1.0-alpha.1

First experimental GTK 4 release: semantic light/dark tokens, buttons, selectable Unicode text, status, progress, safe links and a labeled single-line text field. Editing, focus, selection and typed events remain local to the application. Focused field reconciliation preserves in-progress edits.

Verified with Rust 1.85.1 on ARM64 Ubuntu 24.04, GTK 4.14.5 and a private headless Weston display. CI checks native events, selection, disabled controls, invalid links, focused drafts, Japanese glyph coverage, text scale 1/2, reduced motion and both appearances. Captures are published as CI artifacts; run 38015203443 passed. Reading and control contrast have deterministic tests.

The shadcn source/style/theme profile is pinned in `reference/profile.json`. This release does not claim full shadcn parity. Physical pointer/IME/clipboard workflows, screen-reader navigation, motion timing, other platforms, multiline inputs and a broader component catalog remain unqualified or unimplemented. Applications must treat this API as experimental.
