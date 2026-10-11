# Changelog

## 0.1.0-alpha.35

- Add `icon_button` and `button_with_icon` with required bounded accessible names, leading/trailing slots and square icon-only sizes. GTK owns local activation and disabled behavior.
- Ship only Lucide Plus and ArrowRight from pinned 0.474.0, with ISC and inherited Feather notices. Cairo adapters use live native theme foreground; no host icon-theme lookup is required.
- Compare 192 independent browser samples against 384 mapped native controls across light/dark and text scales 1/2. Verify exact heights, bounded widths, icon viewports, spacing, theme foreground and visible painted strokes. Retain 12 native capture profiles.
- Exercise named icon actions and disabled state through independent AT-SPI, plus live light-1/dark-2/light-1 font/icon reflow.
- Match the pinned Secondary foreground and disable transitions while measuring browser theme endpoints.
- Full glyph rasterization, physical pointer/keyboard, RTL and motion parity remain unqualified. Declarative `Button@1` metadata is unchanged.

## 0.1.0-alpha.34

- Preserve fractional label positions after GTK 4.14 mapping replaces the constructor context policy, and settle them once after live theme changes. A mapped light-1/dark-2/light-1 regression checks exact geometry, fractional positions and no action dispatch; weak label tracking does not retain closed buttons.

- Add the native Link action-button variant with primary text, transparent backgrounds and a painted hover underline. Keep URI navigation in the separate `link()` control.
- Expand pinned browser/native size checks to six variants (96 mapped samples across appearances/scales), and rendered state checks to 60 endpoints. Verify the antialiased underline stroke and primary foreground in both appearances.
- Retain GTK underline positioning as a documented difference from the browser four-pixel offset; full appearance/motion parity and icons remain unqualified.

## 0.1.0-alpha.33

- Explicitly retain fractional button glyph positions across GTK versions. Ubuntu GTK 4.14 rounded 12px text advances and failed the unchanged 1px reference width tolerance; native Pango diagnostics reproduce the 39px rounded versus 40.13px fractional text width.

- Add native text-only ExtraSmall, Small and Large button sizes alongside Default, preserving existing `button()` calls.
- Measure the pinned browser size matrix with an isolated headless browser; compare 80 mapped native buttons across light/dark and text scales 1/2, including typography, border-box dimensions, disabled Unicode labels and captures.
- Keep declarative `Button@1` metadata unchanged until its hosts support size properties. Icons, Link and full appearance/motion parity remain unqualified.

## 0.1.0-alpha.32

- Keep current Result/Error values visible while the user selects an older message; reconciliation previously left only the obsolete outcome visible.
- Retain the selected snapshot, update a separate native status notice, and reconcile the main message after deselection. Add mapped Unicode and invalid-update regressions in both appearances.

## 0.1.0-alpha.31

- Break the multiline submit-controller reference cycle so closing the final TextArea owner releases its view, buffer and host callbacks.
- Add a real GTK callback/owner regression covering exact Unicode change/submit values, plain Return and retained host clones.

## 0.1.0-alpha.20

Primary paste and text drops retain GTK’s nearest insertion iterator when the pointer is in blank space below or right of the text. Previously the high-level wrapper discarded that iterator whenever the point was not directly over a glyph. A mapped gallery regression reproduces the lost blank-area paste on alpha.19, then verifies exact Unicode append and grouped undo across appearance/text-scale profiles.

## 0.1.0-alpha.19

Button hover changes variant backgrounds without fading labels. Pressed buttons retain the pinned reference hover endpoint rather than introducing a whole-widget opacity change. Dark outline backgrounds use the pinned input alpha. A rendered GTK regression samples all five variants in default, hover, pressed, disabled and focus states in both appearances (50 endpoints, two-channel-value tolerance). The old pressed-button behavior fails this regression. These endpoint checks do not establish physical pointer delivery, motion or full reference parity.

## 0.1.0-alpha.18

Uses horizontal scrolling for plain-text documents instead of soft wrapping. This avoids expensive reflow of a maximum-sized single line during composition. The agentOS GTK text-input-v3 integration exercised a 64 KiB single-line document, observed preedit processing before commit, rejected overflow, and preserved exact text, selection, durable draft and saved value.

## 0.1.0-alpha.17

Keeps the selected-deletion receipt alive across IME preedit main-loop turns. A rejected or cancelled composition restores the original selection through native undo; accepted commits and intervening edits invalidate restoration. Native regression checks reproduce GTK 4.14 preedit deletion followed by delayed rejection, cancellation, acceptance and intervening edits.

## 0.1.0-alpha.16

Preserves selected text when a synchronous GTK IME replacement exceeds 64 KiB, retaining earlier undo and removing the rejected deletion from redo. Primary paste and text drops now use bounded asynchronous preflight. Text drops copy at the insertion point; MOVE drops are no longer accepted. ARM64 Debian guest tests cover native replacement ordering, prior undo/redo, oversized and valid primary paste through the controller, bounded Unicode streams and mapped theme/scale/assistive journeys. Physical IME engines and complete drag-source/destination handoffs remain unqualified.

## 0.1.0-alpha.15

Preflights asynchronous clipboard paste before deleting a document selection, with bounded reads, timeout, focus and revision guards.

## 0.1.0-alpha.14

Rejects oversized document insertions at the native buffer boundary.

## 0.1.0-alpha.13

Adds `DocumentEditor@1` and shared plain-text editing with monospace native text, bounded literal Unicode find/replace, selection replacement and grouped undo. Replacement growth is preflighted before allocation. ARM64 Linux CI 38036067038 passed all-feature builds, mapped Unicode editing/undo scenarios across themes/scales and the independent assistive probe. Persistence, save/discard, revisions and file import/export belong to the host; physical IME/clipboard and full visual/motion parity remain unqualified.

## 0.1.0-alpha.12

Adds optional `terminal` feature, `TerminalView` and `PtySession@1` schema using VTE 0.8 GTK4 bindings. Native terminal rendering accepts 16 KiB host chunks and bounded scrollback, emits local input, exposes grid/interaction state and starts with input disabled. ARM64 Linux CI 38035243208 passed all-feature builds, mapped theme/scale terminal scenarios and the assistive probe. Attachment, transport, authority and child lifecycle belong to the host; physical terminal input, IME and full reference parity remain unqualified.

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
