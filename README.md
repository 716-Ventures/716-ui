# 716 UI

Native components with shadcn as their visual and interaction reference.

716 UI is an independent, open-source component library intended for native applications, including agentOS. It aims to reproduce the appearance, interaction states and motion of selected shadcn components through native implementations. Applications can use it without an agent, model provider, account or network connection.

**Status: experimental GTK 4 alpha.** Version `v0.1.0-alpha.1` provides working native controls and a standalone gallery verified on ARM64 Ubuntu 24.04 / GTK 4.14 / Wayland. Full shadcn visual, interaction, accessibility, and motion parity is not yet qualified.

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

The next alpha adds renderer-independent, typed component metadata (`default-features = false`) and native multiline text editing. The host may consume the exact same catalog without linking GTK. These changes remain under CI verification until tagged.
