# Component parity contract

Every implemented component needs a completed contract and a gallery scenario for each supported state. This is a specification template, not evidence of implemented behavior.

## Reference provenance

Record the upstream repository and exact commit, component source paths, style/theme, underlying primitive and version, fonts/icons and licenses, motion definitions and reference environment. Record display scale and text rendering settings for comparisons. The reference revision and theme are pinned in `reference/profile.json`; no component is currently declared fully conformant.

## Required component specification

| Section | Required information |
|---|---|
| API | Name/version, typed props, defaults, variants, slots and emitted events |
| Appearance | Dimensions, spacing, type metrics, colors, border, radius, shadow and icon geometry |
| States | Default, hover, pressed, focus, selection, disabled, loading and error, where applicable |
| Input | Keyboard map, pointer actions, text/IME handling, clipboard and touch if supported |
| Focus | Entry target, traversal, roving focus where appropriate, modal containment and restoration |
| Overlays | Anchor geometry, clipping avoidance, collision handling, outside interaction and dismissal |
| Motion | Trigger, property, duration, easing, delay, transform origin and interruption behavior |
| Accessibility | Role, name, value, state, relationships, announcements and actions |
| Adaptation | Light/dark themes, text scaling, display scale, RTL, localization and reduced motion |
| Data | Controlled/uncontrolled behavior, stable identity, draft handling and validation |
| Limits | Large/empty content, resource limits, failure states and degraded behavior |

Preserve upstream behavior where defined. If a required case is unspecified, record the library's deliberate extension; do not invent an upstream guarantee. For each motion, specify what happens when reversed or interrupted before completion. Reduced motion must retain immediate, understandable state feedback.

## Verification

Maintain reference scenarios for all supported variants and states, with native captures at matching sizes, fonts and scale. Compare geometry, colors and typography with documented tolerances; differences from font rasterization require explicit assessment rather than a blanket pixel-perfect claim.

Run the same interaction sequences against the reference and native implementation: pointer, keyboard, focus restoration, selection, dismissal and editing. Compare outcomes and accessibility semantics rather than browser-specific implementation details. Capture motion over time to compare duration, easing and continuity, not just endpoint screenshots.

The native gallery must exercise these cases without an agent or external account. Mark unimplemented, partial and verified coverage honestly. A component is not complete merely because its default screenshot matches.

## Table@1 / DataTable

Required properties are `label`, 1–32 `columns` (nonempty labels, ≤256 UTF-8 bytes each) and `rows` (≤10,000). Each row has exactly an opaque `id` (1–128 ASCII letters, digits, hyphen, underscore or colon) and `cells` matching the column count (≤4,096 UTF-8 bytes each). Duplicate keys and NULs are rejected before model replacement. The table is read-only; it grants no host operation. Empty rows retain headers. No sorting, filtering, expansion, editing or inferred status is provided.

GTK ColumnView supplies native column headers, selection and column resizing. The model preserves row objects by key and restores selection by key after reordering; removal clears that selection. Only visible cells are constructed. The table has its own bounded scroll region, with horizontal overflow available for wide content. Cell text is inert.

The ARM64 Linux gallery verifies visible allocated cells for 4,000 rows, bounded cell construction, key-based selection across reordered/updated data, duplicate rejection without mutation, Unicode, light/dark, text scales 1/2 and reduced motion. Programmatic selection is tested; physical keyboard/pointer, screen-reader relationships and reference appearance/motion require further qualification. The pinned shadcn table source is a comparison reference, retained under MIT.

## Button / native variants

`button(label, variant, disabled)` provides Primary, Secondary, Outline, Ghost and Destructive native GTK buttons. Labels are literal; activation emits GTK's local clicked event. Disabled buttons use GTK sensitivity and 50% whole-widget opacity. Enter/Space and accessibility activation follow GTK button semantics. Icon slots, reference size variants, loading and invalid states are not provided by this API.

The pinned `reference/button.tsx` uses primary/90 and secondary/80 background hover, accent hover for outline/ghost, and no separate pressed opacity. Native hover now applies those background alpha values while keeping labels opaque; active+hover retains that endpoint. Dark outline uses input/30 normally and input/50 on hover; dark ghost hover uses accent/50. The existing deliberate destructive color (`#b91c1c`) and stronger outline border remain. Native focus rings differ from Radix; focus geometry and full browser parity are unqualified. Native background transitions use the observed reference default of 150 ms with cubic-bezier(0.4, 0, 0.2, 1), disabled under reduced motion. A local browser observer captured actual pointer enter/leave computed-style trajectories in the pinned component. Frame-for-frame browser/native timing and physical presentation parity remain unqualified.

`button-states` renders actual GTK CSS state flags and samples painted pixels against the pinned Zinc endpoints: five variants × five states × two appearances, with tolerance of two 8-bit channel values. The native gallery runs it in its private Wayland display and retains per-state PNGs. It catches the former pressed opacity and missing ghost/outline accent feedback. This is an endpoint regression, not a browser screenshot comparison or a physical pointer/motion qualification.

`button-motion` samples painted primary/ghost colors through hover, reversal and interrupted hover in light/dark appearances, both with and without reduced motion. It verifies endpoint completion, bounded color excursions, intermediate primary and ghost values when animated and immediate reduced-motion feedback after style processing. The gallery retains four motion traces. These timer-driven GTK captures do not establish hardware presentation timing or exact browser curve parity.

`button-metrics` compares the natural border boxes of five text-only variants against measured browser widths in `reference/button-metrics.json`, using the pinned Inter Medium 14px font. Both appearances pass a 1px width tolerance and exact 36px height. Non-outline variants no longer reserve an invisible 1px border. State and motion captures now use GTK border-box bounds, with state PNGs checked at 190×40; GTK content dimensions previously scaled those captures. This qualifies these default dimensions only, not complete typography or component parity.

## Input / native TextField

The checksum-pinned browser harness includes the actual `reference/input.tsx` component. A 1280px viewport inspection recorded Inter 14px, 36px height, 6px radius, 4px/12px padding, dark input/30 background and 50% disabled opacity. Controlled Unicode editing, Tab focus with focus-visible ring, disabled semantics and light/dark rendering were exercised. Measurements and scope are retained in `reference/input-observation.json`; browser engine version was not collected.

The native Entry now uses the dark input/30 tint and 50% disabled opacity. `input-states` samples ten painted background endpoints at 190×36, with two-channel-value tolerance, and retains PNGs. The previous native dark field painted `[9,9,11]` instead of the reference composite `[18,18,20]`; the regression rejects that baseline. Existing gallery and assistive journeys cover Unicode editing, focus-preserved drafts and disabled state. Placeholder style colors now match the pinned muted foreground, with explicit opacity; the prior GTK placeholder exposed ordinary foreground instead. The probe reads the real GTK placeholder widget color and retains normal/disabled placeholder captures. GTK’s [documented text CSS nodes](https://docs.gtk.org/gtk4/class.Text.html#css-nodes) include the placeholder node. Native stronger borders, focus presentation, text metrics and padding differ; full typography, responsive/mobile, invalid/file-input and motion parity remain unqualified.

The actual Entry focus probe now grabs focus through the native control and checks the painted ring outside its border in a parent snapshot. Entry delegates focus to GtkText, so the theme uses `:focus-within`; the former `:focus-visible` parent selector missed real delegated focus even though forced CSS flags passed. The native ring remains 3px with 2px offset and muted color, deliberately different from the browser ring. Light/dark parent captures and pixel checks qualify this native ring, not physical keyboard delivery.

## Tabs / default horizontal presentation

`Tabs::new(title)` and `set_pages` provide 1–32 uniquely keyed labeled native Notebook pages. Selection follows stable keys across reorder; removing the selected key chooses the first remaining page. Rendering does not dispatch application actions. The current API does not expose per-page disabled state, line style or vertical tabs.

The pinned default horizontal browser fixture measures a 36px strip, 3px padding, 8px strip radius and 29px triggers with 4px/8px padding, 6px radius and Inter Medium 14px. Native `tabs-states` checks both themes against recorded palette/geometry, natural Account width within 2px (73px on Ubuntu/GTK 4.14 and 75px on Debian/GTK 4.18, against the 74.047px browser reference), and computed active/inactive label typography and colors. The previous unscoped Notebook inherited beige GTK chrome and 37px triggers. Scoped styling now uses muted strip colors, background/input active paint and native Notebook bottom spacing to retain the measured trigger border box. Native header width follows the allocated content rather than the reference fit-content strip.

Browser ArrowRight, disabled-skip/wrap, Home and End behavior was observed separately. The native control retains GTK Notebook keyboard/focus semantics; reference keyboard parity, focus paint, transition trajectories, assistive announcements, RTL and other variants remain unqualified. Stable page selection and removal/reorder behavior retain the existing gallery checks. An independent AT-SPI journey selects each page through the tab list’s Selection interface, verifies exactly one selected child and checks the selected tab roles; it does not establish screen-reader announcements.

## Bounded Chart / numeric domain

Native charts use a zero-inclusive domain over all bounded values. Positive, negative and mixed fractional values retain the full available scale; only all-zero data uses a synthetic 0–1 domain. The former minimum span of one compressed fractions and hid very small values. Ratios are computed before multiplying by plot height so finite subnormal values remain visible. Exact values and labels remain available in the accessible data table.

`chart-states` samples actual painted bars in both themes for positive/negative/mixed fractions, all-zero data, tiny and subnormal values, and integer data. Pure tests cover domain boundaries and invalid values. These are correctness regressions for the native bounded chart, not Recharts or shadcn Chart appearance parity.
