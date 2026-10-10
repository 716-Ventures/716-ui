# Changelog

## 0.1.0-alpha.11

Adds `Result@1`, `Error@1` and `OutcomeView` with selectable literal content, status/alert roles, preserved selections and deliberate local recovery intent. ARM64 Linux CI 38034749537 passed four mapped theme/scale scenarios and the independent assistive probe. Recovery callbacks, assessment and source revisions belong to the host. Alert reference source is pinned; announcement timing, physical assistive recovery and full visual/motion parity remain unqualified.

## 0.1.0-alpha.10

Adds `DocumentReference@1`, `ApplicationReference@1` and `ReferenceView` with bounded accessible labels, local navigation intent and explicit unavailable state. ARM64 Linux CI 38034354273 passed mapped gallery intent/state checks across themes/scales and the independent assistive probe. Resolving identities, authorization, target revisions, placement and application lifecycle belong to the host; references do not launch processes or open URLs. Full reference appearance/motion and physical assistive navigation remain unqualified.

## 0.1.0-alpha.9

Adds `Image@1` and bounded host-supplied PNG rendering with required alternative descriptions, strict encoded-byte/chunk/dimension limits and native decoding. ARM64 Linux CI 38034098961 passed four theme/scale gallery scenarios and independent AT-SPI Unicode editing, roles, disabled state and checkbox activation. Checkboxes expose a local assistive activation action. GTK 4.14 AT-SPI GrabFocus remains unsupported; physical screen-reader navigation and full reference appearance/motion parity remain unqualified.

## 0.1.0-alpha.8

Adds `RichText@1` with bounded escaped plain/strong/emphasis/code runs and preserved selection. Adds `Chart@1` with 1–128 uniquely keyed finite values, a native bar plot and exact accessible data table. ARM64 Linux CI 38032311971 passed gallery checks across both themes, scales 1/2 and reduced motion. Full reference chart/typography, tooltips, physical assistive input and motion parity remain unqualified.

## 0.1.0-alpha.7

Adds `Choice@1` and `Toggle@1`, labeled native DropDown/CheckButton controls with typed local changes and focus-preserved values. Choices require 1–32 distinct bounded options and an exact selected value. ARM64 Linux CI 38032103678 passed gallery checks across both themes, scales 1/2 and reduced motion, including internal toggle focus ownership. Host submission, physical input and full reference appearance/motion parity remain application or qualification work.

## 0.1.0-alpha.6

Adds `Split@1` and `Tabs@1`, native two-pane resizing and 1–32 keyed tab pages. Local tab choice survives reorder and label updates; removal selects the first surviving page. Invalid updates preserve the current content. ARM64 Linux CI 38031633481 passed mapped gallery checks across both themes, scales 1/2 and reduced motion. Physical input and full reference appearance/motion parity remain unqualified.

## 0.1.0-alpha.5

Adds `Section@1` and `Scroll@1` schemas, labeled native sections and bounded vertical scroll regions. Text headings expose the Heading role and level. ARM64 Linux CI 38031215921 passed native allocation, actual scroll-range, group/heading role and label reconciliation checks across both appearances, scales 1/2 and reduced motion. Complete screen-reader journeys and reference appearance/motion parity remain unqualified.

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
