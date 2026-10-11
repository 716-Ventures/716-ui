# 716 UI

Native components with shadcn as their visual and interaction reference.

716 UI is an independent, open-source component library intended for native applications, including agentOS. It aims to reproduce the appearance, interaction states and motion of selected shadcn components through native implementations. Applications can use it without an agent, model provider, account or network connection.

**Status: experimental GTK 4 alpha.** Version `v0.1.0-alpha.37` provides working native controls and a standalone gallery verified on ARM64 Ubuntu 24.04 / GTK 4.14 / Wayland. Full shadcn visual, interaction, accessibility, and motion parity is not yet qualified. The private Wayland gallery additionally samples 60 button-state background endpoints; see [the component contract](docs/COMPONENT-CONTRACT.md#button--native-variants) for its scope.

## Design commitments

- Pin a concrete shadcn source revision, style and theme before implementing corresponding components.
- Match appearance, keyboard/pointer behavior, focus, selection and animation against reproducible reference scenarios.
- Make text editing, accessibility, scaling, clipboard support and reduced motion part of component correctness.
- Keep behavior deterministic and local. Typed events let the host application decide what to do.
- Ship reusable controls and design tokens with a standalone gallery, documentation and regression tests.
- Preserve native responsiveness under application load.

## Project documents

- [Architecture and project boundaries](docs/ARCHITECTURE.md)
- [Component parity specification](docs/COMPONENT-CONTRACT.md)
- [Implementation and release roadmap](docs/ROADMAP.md)
- [Contributing](CONTRIBUTING.md)
- [Upstream references and notices](THIRD_PARTY.md)

## License

Original work is licensed under [Apache-2.0](LICENSE). Upstream code, assets and adapted material retain their applicable notices. This project is independent and is not an official shadcn project.

Build with Rust 1.85.1 and GTK ≥4.12: `cargo build --locked`. Run `cargo run --bin gallery`; the gallery has no agent, accounts or network dependency. `dbus-run-session -- python3 tests/native_gallery.py` runs native interaction scenarios and captures both appearances in a private Weston display. See `reference/profile.json` for the pinned reference and declared differences.

Version 0.1.0-alpha.2 adds renderer-independent, typed component metadata (`default-features = false`) and native multiline text editing. The host may consume the same catalog without linking GTK. ARM64 Linux CI verifies both metadata-only builds and native single/multiline edit preservation.

Version 0.1.0-alpha.3 adds `Table@1` metadata and `DataTable`, a GTK ColumnView with stable row keys, native column headers, keyboard selection and virtualized cells. Columns and cells are bounded, duplicate keys are rejected, and selection follows the row key across updates and reordering. ARM64 Linux gallery verification covers 4,000 rows with allocated visible cells, selection across reordering, duplicate-key rejection, both themes, both tested text scales and reduced motion. Physical assistive-input and shadcn visual/motion parity remain unqualified.

Version 0.1.0-alpha.4: `List@1` and `KeyValue@1` use bounded, stable keyed rows in GTK ListView. `DataList::new(label, false)` accepts one string cell per row; `true` accepts key/value pairs. Rows are read-only, updates preserve selection by key, and deleted keys clear selection. The pinned shadcn Item reference supplies the visual/semantic comparison baseline; item actions/media and full physical input/motion parity are not qualified. ARM64 Linux CI run 38030789105 passed 4,000-row virtualization, keyed reorder/removal and native accessibility roles in both themes, text scales 1/2 and reduced motion.

Version 0.1.0-alpha.5: `Section@1` and `Scroll@1` add labeled grouping and bounded native vertical scrolling. `Section` exposes a native accessible heading and content box; `ScrollRegion` exposes its content box within a 160–480 pixel natural-height viewport. Headings now expose the native Heading role and level. The gallery checks group/heading roles, actual scroll ranges and label updates; ARM64 Linux CI 38031215921 passed these checks across both themes, scales 1/2 and reduced motion. Complete screen-reader and reference motion journeys remain unqualified.

Version 0.1.0-alpha.6: `Split@1` uses two native resizable panes; `Tabs@1` uses 1–32 uniquely keyed pages with bounded labels. Native tab selection follows keys across reorder and falls back to the first surviving page after removal. Invalid updates preserve the current pages. Divider positions remain local to the host. Physical input and reference appearance/motion parity remain unqualified.

ARM64 Linux CI 38031633481 verified keyed tab reorder/removal, label reconciliation, invalid-update preservation and retained split positions across both themes, scales 1/2 and reduced motion.

Version 0.1.0-alpha.7: `Choice@1` and `Toggle@1` provide labeled native form values. Choices have 1–32 distinct bounded options and an exact selected value; toggles emit booleans. Focused controls retain local values during reconciliation. These controls carry no commit or system-action authority; hosts own draft persistence and deliberate submission. Physical input and reference appearance/motion parity remain unqualified.

ARM64 Linux CI 38032103678 passed typed choice/toggle changes, invalid option rejection and focused value preservation across both themes, scales 1/2 and reduced motion.

Version 0.1.0-alpha.15: `RichText@1` uses up to 256 inert runs (plain/strong/emphasis/code, 64 KiB total) with escaped native text and preserved selection. `Chart@1` uses 1–128 uniquely keyed, finite bounded values, a native bar plot with a zero baseline, and an exact accessible data table. Invalid updates preserve content. Physical assistive input and reference chart/typography parity remain unqualified.

ARM64 Linux CI 38032311971 passed escaped rich-text rendering/selection, native chart allocation, exact data rows and invalid-update preservation across both themes, scales 1/2 and reduced motion.

Development verification additionally exercises a separate AT-SPI client over the accessibility bus: exposed names/roles, Unicode text editing, disabled buttons and native toggle activation. GTK 4.14 explicitly does not implement the AT-SPI GrabFocus request, so assistive focus remains a known platform limitation; native keyboard focus has separate in-process tests. This automated interface check does not establish subjective screen-reader usability or physical input qualification.

Development: `Image@1` declares a host-owned resource reference and bounded alternative label. `ImageView` accepts complete CRC-checked PNG bytes up to 256 KiB and four million pixels, never opens paths or URLs, and bounds its native viewport to 240 pixels. Host resource authorization and delivery belong to the application. Other formats and reference appearance parity remain unqualified.

`ReferenceView` emits local navigation intent for `DocumentReference@1` and `ApplicationReference@1`. The host resolves the target identity, activity scope, revision and current availability; the library does not launch applications, open files or interpret the target as a URL. Missing views have a disabled control and explicit tooltip.

`OutcomeView` supplies literal selectable `Result@1` status and `Error@1` alert content, preserves selection during reconciliation and emits local recovery intent. The host supplies source bindings, callback availability, revision guards and assessment. `reference/alert.tsx` pins the corresponding reference primitive; full appearance, live-region announcement timing and physical assistive recovery remain unqualified.

The optional `terminal` feature provides `TerminalView` for `PtySession@1` using VTE 0.8 GTK4 bindings (MIT) and the system VTE library (LGPL). It accepts bounded 16 KiB display chunks, keeps 1,000 scrollback lines and disables input until the host explicitly attaches. It never spawns a child or resolves a source. Attachment, input ownership, retries, resize and cleanup belong to the host. Enable this feature with the system `libvte-2.91-gtk4-dev` development package.

`DocumentEditor@1` and `DocumentEditor` edit a bounded shared plain-text document with horizontally scrolling monospace native text, literal Unicode find/replace and grouped undo. Replacement growth is checked before allocation. Editing is local; persistence, save/discard, revisions and optional file import/export are host responsibilities. File paths and commands are never inferred from its text.

Document transfers are bounded before editing: clipboard and primary selection reads have a three-second deadline and a 64 KiB cap. Text drops copy at the drop location; moving text by drop is deliberately unsupported so rejected drops cannot delete their source. Rejected synchronous selected IME replacements restore the selection through native undo while retaining earlier history. Blank-area paste/drop positions retain GTK’s nearest iterator instead of discarding non-glyph hits. Physical IME engines and host input delivery require host qualification; the standalone gallery does not establish complete compositor/client drag handoffs.

Button background motion follows the pinned reference default of 150 ms cubic-bezier(0.4, 0, 0.2, 1), with immediate reduced-motion feedback. Four painted native traces cover light/dark hover, reversal and interruption; a separate browser observer captures actual reference pointer trajectories. These checks do not establish frame-for-frame or physical presentation parity.

Default text-only Button dimensions are checked against measured pinned browser widths within 1px, with exact 36px height and Inter Medium 14px in both themes. State captures use verified 190×40 border boxes. Text-only xs/sm/lg sizes and leading/trailing/icon-only Plus/ArrowRight buttons additionally have measured browser/native geometry checks; full typography and icon rasterization parity remain unqualified.

Input placeholders use the pinned muted foreground in both themes, with explicit opacity rather than a theme-dependent fade. Input backgrounds now follow the pinned dark input/30 tint and 50% disabled opacity. Ten painted endpoints check default/disabled/focused backgrounds across both themes; the browser harness exercises Unicode editing, keyboard focus and disabled semantics. Stronger native borders and native focus presentation remain deliberate differences.

Native Entry focus uses `:focus-within` to follow its text delegate. A real control-focus probe verifies the painted native ring outside the border in both themes; forced CSS flags alone no longer count as focus-ring evidence.

The default horizontal Tabs strip now uses the pinned muted palette and measured 36px/29px strip/trigger heights. The native probe compares actual paint, natural Account width, typography and active/inactive text in both appearances; its scope is documented in the component contract. The GTK Notebook still fills its allocation and retains native keyboard/focus behavior; full Tabs parity remains unqualified.

Alpha.24 corrects fractional and very small Chart values: the zero-inclusive domain now uses the measured range rather than a minimum span of one. Both themes have actual paint regressions, including signed, zero and subnormal data; the exact accessible table remains available.

Alpha.25 aligns native Toggle checkbox state backgrounds and 16px geometry with the pinned Checkbox reference, with both-theme painted regressions and disabled action checks. Stronger borders, native focus and GTK check glyphs remain documented differences.

Alpha.26 aligns Choice trigger backgrounds, regular-weight text and 36px height with the pinned Select reference. Both-theme paint checks cover normal, hover and disabled states; actual native focus verifies all four painted ring edges. Native strengthened borders, focus treatment, GTK arrow, popup and keyboard behavior remain distinct; full Select appearance/interaction/motion parity is unqualified.

Alpha.27 scopes Table styling to the pinned Zinc reference: 40px headers, 37px body rows, page/hover/selected backgrounds and regular body text. A 4 KiB spaced cell stays on one line and exposes horizontal scrolling instead of wrapping. Native column resizing, key-based selection and virtualization remain intact. Both-theme endpoint and overflow regressions join the complete gallery; full Table geometry, clipboard, physical input and reference motion parity remain unqualified.

Alpha.28 adds deliberate selected-row copying: Ctrl+C and the accessible “Copy selected row” context menu export current cell text separated by tabs. Menu/Shift+F10 open the menu; copying restores local Table focus. Selection updates/removal are checked at invocation, and callbacks do not retain closed tables. A separate Wayland client verifies exact Unicode delivery in the agentOS test compositor; the standalone headless gallery checks local clipboard/action behavior. Synthetic callback checks do not qualify physical keyboard or pointer delivery.

Alpha.29 fixes stale visible Table, List and KeyValue labels after same-key content updates. Changed rows replace their private model object so GTK rebinds the visible cells; unchanged row objects and stable-key selection survive reconciliation and reorder. The native regression checks actual mapped label text in all three collections, rather than only their stored data.

Alpha.30 extends the guarded selected-row Copy action and accessible context menu to List and KeyValue. Both share the Table clipboard implementation, preserve exact Unicode and tab-separated cells, disable Copy after selection removal and release their owners on closure. Native regressions check the installed Ctrl+C callback and exact local clipboard text; physical input and full reference parity remain unqualified.

Alpha.31 releases multiline TextArea views, buffers and host event callbacks when their final owner closes. The Ctrl+Enter submit controller holds a weak view reference; native regression checks preserve Unicode Changed/Submitted payloads, plain Return behavior and remaining host clones before final release.

Alpha.32 preserves selected Result/Error text while showing a separate current-value notice when reconciliation observes a changed label or message. The notice continues updating and disappears after selection clears and the main message reconciles. Both-theme native checks verify mapped current text, Unicode selection retention and invalid-update rejection. Live screen-reader announcement timing remains unqualified.

Alpha.33 adds `ButtonSize::{Default, ExtraSmall, Small, Large}` and `button_sized`, preserving existing `button()` behavior. A checksum-pinned browser size matrix supplies measured widths and 24/32/36/40px heights. The complete ARM64 Linux/Wayland gallery passes all five variants in both appearances and text scales 1/2, with mapped font/geometry checks and retained captures. Native outline borders remain one pixel during text scaling. Icons, the Link variant and complete appearance/motion parity remain open; the declarative `Button@1` metadata remains unchanged.

Alpha.34 adds the native Link action-button variant, with primary text, transparent backgrounds and a painted hover underline. The complete Debian ARM64 gallery passes all six variants at four text-only sizes, both appearances and text scales 1/2 (96 mapped samples), plus 60 painted state endpoints. GTK underline positioning remains a documented difference from the browser four-pixel offset. Link activation is a local action; `link()` remains the separate URI-navigation control. Icons and full appearance/motion parity remain unqualified.

Alpha.35 adds named icon-only and leading/trailing action buttons with pinned Lucide Plus/ArrowRight, native theme foreground and local GTK activation. Independent browser measurements cover both themes, six variants and four sizes; native gallery checks add text scales 1/2, painted strokes, live theme reflow and independent AT-SPI actions. Secondary foreground now follows the pinned theme. Only two icons ship; declarative Button metadata and full reference conformance remain unchanged.

Alpha.36 adds bounded `ButtonProps` for six variants, four sizes and the two named icons, with optional leading/trailing/only positioning in `Button@1`. `ActionButton` validates before mutation and retains the GTK button, focus, host classes and connected action owner through updates. Missing optional values retain defaults; explicit nulls, unknown choices, unnamed icons and positions without an icon are rejected. The complete Debian ARM64 / GTK 4.18 Wayland gallery passes, including four mapped reconciliation journeys across both appearances and text scales 1/2. Host applications must validate declarations and retain their own action authorization; the library never executes a command. Full reference parity remains unqualified.

Additional alpha.36 qualification covers logical leading/trailing icon order in RTL. The independently measured pinned browser matrix has 192 RTL samples; 384 mapped native samples cover all six variants, four sizes, both appearances and text scales 1/2, with four retained-button update journeys. LTR and RTL AT-SPI clients verify named icon activation and disabled state. The literal ArrowRight glyph retains its named direction; logical placement follows GTK reading direction. Library implementation and API are unchanged. Full bidirectional language shaping, physical input, RTL speech and full appearance/motion parity remain unqualified.

Alpha.37 adds literal inline validation feedback to TextField and DocumentEditor. `set_validation_message(Some(message))` sets a wrapped readable error, native invalid state and its ErrorMessage accessibility relation (DescribedBy on GTK 4.14, whose error relation has an upstream type mismatch); `None` clears them. Nonempty messages are bounded to 1,024 UTF-8 bytes, with invalid updates rejected before mutation. Updates preserve text, GTK logical focus, selection and host event ownership; identical feedback preserves the error selection. The declarative catalog adds optional `validation_message` strings to both controls; hosts must call the shared bound validator. Four mapped theme/text-scale profiles and independent LTR/RTL AT-SPI state/relation/clear journeys pass in the complete Debian gallery. Caption colors meet 4.5:1 contrast and retain the native contrast policy; full shadcn error paint/motion, physical input and screen-reader error announcements remain unqualified.

Alpha.38 extends the same bounded literal `validation_message` contract to Choice and Toggle. Their `set_validation_message` methods expose wrapped error captions, invalid state and a related readable error (DescribedBy on GTK 4.14; ErrorMessage on newer GTK). Feedback does not change the selected option, boolean/inconsistent state, focus, sensitivity or change-event ownership. Removing the message clears only feedback. Native indicator/trigger borders and focus retain the existing contrast policy; full Select/Checkbox appearance, popup and motion parity remain unqualified. The consuming host owns validation decisions, drafts, Save/Discard and action authority.

Choice accessibility names now retain both the field label and the selected option after mapping and native selection changes, using literal `label: option` text. GTK previously exposed only the selected item. Independent LTR/RTL clients verify the initial name, explicit option change and preserved related error.
