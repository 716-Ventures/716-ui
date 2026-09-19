# Roadmap and release policy

## 0. Reference and renderer proof

Select and record a shadcn reference profile. Prototype a button, text field, menu and dialog in a small shortlist of native toolkits. Test appearance, motion, input, IME, selection, clipboard, accessibility and scaling on Linux/Wayland. Record results and select one renderer. No toolkit is selected yet.

## 1. First coherent collection

Implement tokens, typography, buttons, inputs, labels, checkbox/switch, separators, menus, popovers and dialogs with completed parity contracts. Build a standalone native gallery covering variants, themes, states and motion. Publish real source and tests; document gaps.

## 2. Composition and application use

Add tabs, lists/tables, split views, scroll regions, progress and feedback components. Validate stable identity, large data, controlled state and event APIs. Integrate through an adapter in a consuming application while keeping the gallery independent. agentOS is an initial consumer, not a runtime dependency.

## 3. Public packages

Before publishing the first experimental package, choose the actual crate/package names and verify availability; declare the supported Rust version, dependency licenses and tested platform matrix. Establish formatting, tests, documentation and native integration checks in CI. Publish a changelog, reproducible gallery instructions and artifact checksums where binaries are distributed.

Use semantic versioning, with explicit migration notes for breaking changes during 0.x. Pin upstream reference changes and review their effect on native behavior before updating. Release approval and credentials belong to maintainers; do not automatically publish on every merge.

## Release acceptance

A release must identify implemented components, their verification status, known limitations, tested targets and performance evidence. Do not claim universal native support or exact visual/behavioral parity without the corresponding tests. Track input latency, frame timing and memory with realistic contents; performance budgets will be set against the first validated environment.

No package or binary release exists at project creation. The repository currently publishes design documents only.
