# 716 UI

Native components with shadcn as their visual and interaction reference.

716 UI is an independent, open-source component library intended for native applications, including agentOS. It aims to reproduce the appearance, interaction states and motion of selected shadcn components through native implementations. Applications can use it without an agent, model provider, account or network connection.

**Status: experimental GTK 4 alpha.** Version `v0.1.0-alpha.7` provides working native controls and a standalone gallery verified on ARM64 Ubuntu 24.04 / GTK 4.14 / Wayland. Full shadcn visual, interaction, accessibility, and motion parity is not yet qualified.

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

Development: `RichText@1` uses up to 256 inert runs (plain/strong/emphasis/code, 64 KiB total) with escaped native text and preserved selection. `Chart@1` uses 1–128 uniquely keyed, finite bounded values, a native bar plot with a zero baseline, and an exact accessible data table. Invalid updates preserve content. Physical assistive input and reference chart/typography parity remain unqualified.
