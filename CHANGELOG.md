# Changelog

## 0.1.0-alpha.1

First experimental GTK 4 release: semantic light/dark tokens, buttons, selectable Unicode text, status, progress, safe links and a labeled single-line text field. Editing, focus, selection and typed events remain local to the application. Focused field reconciliation preserves in-progress edits.

Verified with Rust 1.85.1 on ARM64 Ubuntu 24.04, GTK 4.14.5 and a private headless Weston display. CI checks native events, selection, disabled controls, invalid links, focused drafts, Japanese glyph coverage, text scale 1/2, reduced motion and both appearances. Captures are published as CI artifacts; run 38015203443 passed. Reading and control contrast have deterministic tests.

The shadcn source/style/theme profile is pinned in `reference/profile.json`. This release does not claim full shadcn parity. Physical pointer/IME/clipboard workflows, screen-reader navigation, motion timing, other platforms, multiline inputs and a broader component catalog remain unqualified or unimplemented. Applications must treat this API as experimental.
