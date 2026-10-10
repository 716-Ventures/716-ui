# 716 UI

Native components with shadcn as their visual and interaction reference.

716 UI is an independent, open-source component library intended for native applications, including agentOS. It aims to reproduce the appearance, interaction states and motion of selected shadcn components through native implementations. Applications can use it without an agent, model provider, account or network connection.

**Status: experimental GTK 4 alpha.** Version `v0.1.0-alpha.18` provides working native controls and a standalone gallery verified on ARM64 Ubuntu 24.04 / GTK 4.14 / Wayland. Full shadcn visual, interaction, accessibility, and motion parity is not yet qualified.

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

Document transfers are bounded before editing: clipboard and primary selection reads have a three-second deadline and a 64 KiB cap. Text drops copy at the drop location; moving text by drop is deliberately unsupported so rejected drops cannot delete their source. Rejected synchronous selected IME replacements restore the selection through native undo while retaining earlier history. Physical IME engines, host middle-click delivery and complete drag-and-drop journeys still require qualification.
