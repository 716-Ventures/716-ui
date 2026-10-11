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

GTK ColumnView supplies native column headers, selection and column resizing. The model retains unchanged row objects and replaces objects whose cell contents changed, triggering GTK factory rebinding. It restores selection by key after reordering; removal clears that selection. Only visible cells are constructed. The table has its own bounded scroll region, with horizontal overflow available for wide content. Cell text is inert. Ctrl+C and an explicit “Copy selected row” context menu copy the current row as plain UTF-8 text, joining cells with tabs and preserving embedded tabs/newlines literally. The maximum copied row is bounded by the existing 32×4,096-byte cell limits plus 31 separators. No escaping, spreadsheet formula evaluation, paste, host action or clipboard history is provided. Menu/Shift+F10 and secondary click open the selected-row menu; the label makes its existing-selection scope explicit. Copy closes the menu and restores local focus. Removal disables copying; reordering/updates use the selected key and current cells.

The ARM64 Linux gallery verifies visible allocated cells for 4,000 rows, bounded cell construction, key-based selection across reordered/updated data, duplicate rejection without mutation, Unicode, light/dark, text scales 1/2 and reduced motion. Programmatic selection is tested; physical keyboard/pointer, screen-reader relationships and reference appearance/motion require further qualification. The pinned shadcn table source is a comparison reference, retained under MIT.

Alpha.27 replaces inherited GTK gray/blue states and duplicated cell padding with scoped Zinc Table paint. At native scale one, the real GTK probe checks 40px headers, 37px rows (36px final row), normal/hover/selected backgrounds within two channel values and bounded horizontal overflow for a 4,096-byte spaced cell. Cell labels explicitly disable wrapping. The unchanged pinned browser Table and its developer-only selection fixture are recorded in `reference/table-observation.json`; upstream global border styling is absent from that harness. Column widths/resizing, selection semantics, physical input, clipboard and motion trajectories are not qualified as reference parity.

## Button / native variants

`button(label, variant, disabled)` provides Primary, Secondary, Outline, Ghost, Destructive and Link native GTK buttons. Labels are literal; activation emits GTK's local clicked event. Disabled buttons use GTK sensitivity and 50% whole-widget opacity. Enter/Space and accessibility activation follow GTK button semantics. `button_sized(label, variant, size, disabled)` adds text-only Default, ExtraSmall, Small and Large sizes. Existing `button()` calls retain Default. Icon slots use the separate adapters below; loading and invalid states are not provided by this API. Link is an ordinary local action button; URI navigation remains the separate `link()` API. `Button@1` advertises bounded `variant` and `size` choices through `ButtonProps`; named `icon` and `icon_position` choices use the icon adapters. Host applications retain action ownership.

`icon_button(label, icon, variant, size, disabled)` provides a square local action with a required accessible name and tooltip. `button_with_icon(label, icon, position, variant, size, disabled)` retains literal visible text with a leading or trailing decorative icon. Both return `Result`, rejecting blank names, control characters and names over 256 UTF-8 bytes. Only `Icon::{Plus, ArrowRight}` from pinned Lucide 0.474.0 is shipped, with ISC/Feather notices retained. Icons use live native foreground, have Presentation roles and expose no independent action. No host icon-theme lookup is used. A button's host still owns its local clicked event.

`button-icon-metrics` compares mapped dimensions, icon viewports and spacing against `reference/button-icon-metrics.json`, then checks native foreground and visible painted strokes. The matrix covers leading, trailing and icon-only controls across six variants, four sizes, both themes and text scales 1/2. The independent AT-SPI gallery exercises named actions and disabled state; the live font reflow scenario includes icon layout and foreground changes. Exact browser/native glyph rasterization, physical input, RTL, focus appearance and motion parity remain qualification work. Alpha.35 originally exposed only label/disabled in the declarative catalog; alpha.36 adds the typed variant/size/icon choices described above.

The pinned `reference/button.tsx` uses primary/90 and secondary/80 background hover, accent hover for outline/ghost, and no separate pressed opacity. Native hover now applies those background alpha values while keeping labels opaque; active+hover retains that endpoint. Dark outline uses input/30 normally and input/50 on hover; dark ghost hover uses accent/50. The existing deliberate destructive color (`#b91c1c`) and stronger outline border remain. Native focus rings differ from Radix; focus geometry and full browser parity are unqualified. Native background transitions use the observed reference default of 150 ms with cubic-bezier(0.4, 0, 0.2, 1), disabled under reduced motion. A local browser observer captured actual pointer enter/leave computed-style trajectories in the pinned component. Frame-for-frame browser/native timing and physical presentation parity remain unqualified.

`button-states` renders actual GTK CSS state flags and samples painted pixels against the pinned Zinc endpoints: six variants × five states × two appearances, with tolerance of two 8-bit channel values. The native gallery runs it in its private Wayland display and retains per-state PNGs. It catches the former pressed opacity and missing ghost/outline accent feedback. This is an endpoint regression, not a browser screenshot comparison or a physical pointer/motion qualification.

`button-motion` samples painted primary/ghost colors through hover, reversal and interrupted hover in light/dark appearances, both with and without reduced motion. It verifies endpoint completion, bounded color excursions, intermediate primary and ghost values when animated and immediate reduced-motion feedback after style processing. The gallery retains four motion traces. These timer-driven GTK captures do not establish hardware presentation timing or exact browser curve parity.

`button-metrics` compares the natural border boxes of five text-only variants against measured browser widths in `reference/button-metrics.json`, using the pinned Inter Medium 14px font. Both appearances pass a 1px width tolerance and exact 36px height. Non-outline variants no longer reserve an invisible 1px border. State and motion captures now use GTK border-box bounds, with state PNGs checked at 190×40; GTK content dimensions previously scaled those captures. This qualifies these default dimensions only, not complete typography or component parity.

`button-size-metrics` compares all six variants at xs/sm/default/lg against the measured pinned browser matrix in `reference/button-size-metrics.json`. Its native gallery scenarios cover both appearances and text scales 1/2, exact scaled 24/32/36/40px heights, bounded width differences, Inter Medium 12px for xs and 14px for other sizes, native Button roles and disabled Unicode labels. The outline retains a one-pixel border during native text scaling. Captures retain all size rows; complete appearance, physical input and motion parity remain unqualified. Reproduce browser measurements after building `tools/reference` with `node tools/reference/measure-button-sizes.mjs reference/button-size-metrics.json`; it uses an isolated headless Chrome (override with `REFERENCE_CHROME`) and closes its server and browser profile.

The Link variant uses primary-colored text on a transparent background and a real painted hover underline, retained during active+hover. The endpoint fixture checks the foreground, transparent background and added underline stroke in both appearances. GTK uses its native underline metric; the pinned browser four-pixel underline offset is not reproduced. Button labels explicitly use fractional Pango glyph positions so older GTK does not accumulate integral glyph-advance error at 12px. The width tolerance stays 1px at scale one.

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

## Toggle / native Checkbox endpoints

The boolean Toggle remains a native GTK CheckButton with an accessible label, typed boolean changes and focus-preserved reconciliation. Scoped styling replaces inherited blue GTK paint with pinned Zinc primary/on-primary colors and dark input/30. The native indicator uses the measured 16px square and 4px radius at text scale one, scaling with the native typography. Disabled controls use 50% opacity; native borders stay strengthened for contrast, and focus uses the native 3px outline with 2px offset. GTK owns the check glyph and keyboard/focus semantics; Lucide is a developer browser reference dependency only.

`toggle-states` checks actual checked/unchecked/disabled painted backgrounds, geometry and explicit native action behavior in both themes; the independent AT-SPI gallery still exercises role/state/activation. Alpha.38 adds literal validation feedback below. Indeterminate presentation remains host-owned; exact glyph/label geometry, focus paint, motion trajectories and full reference parity remain unqualified.

## Choice / native Select trigger

`Choice::new(title, options, value)` accepts 1–32 distinct bounded labels and an exact selected option. Local typed changes carry no application authority; focused controls preserve local values during reconciliation. Per-option disabled, placeholder and size variants are not exposed by this API. Alpha.38 adds host-owned invalid feedback below.

The checksum-pinned Select harness records the unchanged upstream trigger at 190×36 with regular Inter 14px, 6px radius, 8px/12px padding, transparent light background and dark input/30 (input/50 on hover), plus 50% disabled opacity. Headless Chrome 154.0.8037.93 exercised Space opening, Home/End enabled-option selection with Enter, Escape cancellation and restored trigger focus. `reference/select-observation.json` retains provenance and scoped observations. The harness does not import upstream website global styles; its popup borders are therefore a separate qualification boundary.

`choice-states` checks actual GTK trigger geometry, normal/hover/disabled backgrounds and regular text in both themes, plus all four edges of the actual native focus ring. The previous trigger inherited GTK gray gradients and Medium text. Native borders remain strengthened, focus uses a 3px muted outline with 2px offset, and GTK retains its arrow and popup/keyboard semantics. The native trigger removes inherited background transitions; reference focus-color/shadow trajectories, popup placement/appearance, exact arrow geometry, RTL and physical input remain unqualified. Full Select conformance is not claimed.

## Collection content reconciliation

Table, List and KeyValue use stable row keys for selection and reconciliation. Their private boxed model objects are reused only when row contents are unchanged; changed objects must be replaced because mutating boxed Rust data emits no native property notification to existing cell bindings. `collection-updates` checks actual mapped Unicode labels in all three controls, confirms old text disappears, and verifies selected identity across reorder. Clipboard-model checks alone cannot establish visible content freshness.

List and KeyValue share Table’s deliberate selected-row copying, keyboard/context-menu controls and current-selection guards. A List copies its one cell verbatim; KeyValue joins its two cells with a tab. The bounded local clipboard regression also checks removal guards and owner release for both controls. Native column/list navigation stays with GTK.

### Multiline callback ownership

The TextArea Ctrl+Enter controller holds its view weakly. Closing the final host owner releases the native view, text buffer and registered callback captures; retaining a host clone keeps the editor alive. `text-area-events` invokes the installed application controller and checks exact Unicode Changed/Submitted values, plain Return without submission and final weak-owner release. This is callback/lifetime evidence, not physical keyboard or IME qualification.

### Outcome selection and current values

Selecting a Result/Error heading or message preserves that text snapshot. A changed incoming label/message also appears in a separate nonselectable Status notice labeled “Current value”; later observations update this notice without touching the selection. Reconciliation returns false while the main text stays protected. Once selection clears, a successful reconciliation updates the main text and hides the notice. Invalid updates alter neither representation. Recovery remains a separate local intent with host-owned authority. `outcome-updates` verifies mapped Unicode notice text and retained selection in both appearances; live announcement timing and full screen-reader journeys remain unqualified.

`ActionButton::reconcile` validates a complete `ButtonProps` before changing the existing GTK button. Identical declarations retain their children; content changes preserve the button, focus and host click connections. Disabled-only updates retain content. Invalid declarations return an error without changing sensitivity or content. `ButtonProps::from_value` enforces catalog-compatible enum values, a nonempty label of at most 256 UTF-8 bytes, and an icon whenever an icon position is supplied. Icon labels reject control characters; ordinary text labels preserve the existing multiline behavior. No colors, host icon names, fonts or executable commands are accepted as properties.

## RTL button qualification

Additional alpha.36 qualification covers logical leading/trailing icon order in RTL. The independently measured pinned browser matrix has 192 RTL samples; 384 mapped native samples cover all six variants, four sizes, both appearances and text scales 1/2, with four retained-button update journeys. LTR and RTL AT-SPI clients verify named icon activation and disabled state. The literal ArrowRight glyph retains its named direction; logical placement follows GTK reading direction. Library implementation and API are unchanged. Full bidirectional language shaping, physical input, RTL speech and full appearance/motion parity remain unqualified.

See `reference/button-icon-rtl-metrics.json` and `test-output/button-rtl-qualified/verification.json`. Host applications select GTK reading direction; the fixtures use the public GTK default-direction API before constructing controls. Updates retain focus, host classes and the existing clicked owner in both directions.

## Validation feedback

Alpha.37 adds literal inline validation feedback to TextField and DocumentEditor. `set_validation_message(Some(message))` sets a wrapped readable error, native invalid state and its ErrorMessage accessibility relation (DescribedBy on GTK 4.14 because its upstream error relation has a type mismatch); `None` clears them. Nonempty messages are bounded to 1,024 UTF-8 bytes, with invalid updates rejected before mutation. Updates preserve text, GTK logical focus, selection and host event ownership; identical feedback preserves the error selection. The declarative catalog adds optional `validation_message` strings to both controls; hosts must call the shared bound validator. Four mapped theme/text-scale profiles and independent LTR/RTL AT-SPI state/relation/clear journeys pass in the complete Debian gallery. Caption colors meet 4.5:1 contrast and retain the native contrast policy; full shadcn error paint/motion, physical input and screen-reader error announcements remain unqualified.

Validation is host-owned; this API neither evaluates a value nor commits or discards an edit. Feedback uses literal label text, never markup. TextField and DocumentEditor expose their feedback helper for native composition; the document caption sits outside its scrolling text buffer. See `test-output/input-validation-qualified/verification.json`.

## Choice and Toggle validation

Alpha.38 extends the same bounded literal `validation_message` contract to Choice and Toggle. Their `set_validation_message` methods expose wrapped error captions, invalid state and a related readable error (DescribedBy on GTK 4.14; ErrorMessage on newer GTK). Feedback does not change the selected option, boolean/inconsistent state, focus, sensitivity or change-event ownership. Removing the message clears only feedback. Native indicator/trigger borders and focus retain the existing contrast policy; full Select/Checkbox appearance, popup and motion parity remain unqualified. The consuming host owns validation decisions, drafts, Save/Discard and action authority.

Hosts constructing `Toggle` with a struct literal must use `Toggle::new` or supply the new feedback owner; constructors initialize the helper. `Choice` already requires its constructor because options remain private. Identical messages retain caption selection; invalid messages fail before mutation. The independent gallery verifies related errors, clear and unchanged checkbox state in LTR and RTL. Native mapped fixtures cover both appearances/text scales, selected-option retention, checked/unchecked and inconsistent-state retention, logical focus and zero change events during feedback.

Choice accessibility names now retain both the field label and the selected option after mapping and native selection changes, using literal `label: option` text. GTK previously exposed only the selected item. Independent LTR/RTL clients verify the initial name, explicit option change and preserved related error.

## Choice field-label updates

Alpha.39 adds `Choice::set_label(title) -> Result<bool, String>` and exposes the selectable `heading`. It validates the same bounded label contract before mutation and keeps the visible heading and `label: option` accessibility name synchronized, including later native option changes. Identical labels return `Ok(true)` without rewriting selected text. A selected or focused heading returns `Ok(false)`; hosts retry after that selection/focus ends. Successful updates retain the option, control focus, validation relation and host change-event ownership. Hosts should use this method instead of independently rewriting the heading or accessible label. Four mapped theme/text-scale profiles and independent LTR/RTL rename-then-option-change/error/clear journeys pass in the complete Debian gallery; native tests verify selected-heading deferral, atomic invalid labels and unchanged focus/value/events. Evidence is retained in `test-output/choice-label-qualified/`. GTK 4.14 CI is the separate release check.

Alpha.40 requests a low-priority “Entry valid.” announcement when a focused control’s existing validation message clears. It does not announce repeated empty feedback or clear feedback on background controls. The GTK 4.14 announcement symbol is resolved from the loaded Unix toolkit, preserving the GTK 4.12 ABI floor; an unavailable symbol leaves the existing state/relation behavior intact. Values, selection, callbacks and literal error captions retain their existing contracts. The standalone gallery qualifies ordinary state/relation behavior; consuming-host screen-reader speech verification is a separate check. This does not qualify physical audio, braille or all assistive workflows.
