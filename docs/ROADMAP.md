# Roadmap and release policy

## 0. Reference and renderer proof

GTK 4 is the selected native renderer. The concrete shadcn source/style/theme profile is pinned in `reference/profile.json`. The standalone gallery and ARM64 Linux CI exercise implemented controls, Unicode editing, composition guards, clipboard bounds, stable identities, accessibility and appearance/text-scale profiles. The renderer choice is implemented; comparative toolkit evaluation and full reference appearance/motion conformance are not claimed.

## 1. First coherent collection

Implement tokens, typography, buttons, inputs, labels, checkbox/switch, separators, menus, popovers and dialogs with completed parity contracts. Build a standalone native gallery covering variants, themes, states and motion. Publish real source and tests; document gaps.

## 2. Composition and application use

Add tabs, lists/tables, split views, scroll regions, progress and feedback components. Validate stable identity, large data, controlled state and event APIs. Integrate through an adapter in a consuming application while keeping the gallery independent. agentOS is an initial consumer, not a runtime dependency.

## 3. Public packages

Before publishing the first experimental package, choose the actual crate/package names and verify availability; declare the supported Rust version, dependency licenses and tested platform matrix. Establish formatting, tests, documentation and native integration checks in CI. Publish a changelog, reproducible gallery instructions and artifact checksums where binaries are distributed.

Use semantic versioning, with explicit migration notes for breaking changes during 0.x. Pin upstream reference changes and review their effect on native behavior before updating. Release approval and credentials belong to maintainers; do not automatically publish on every merge.

## Release acceptance

A release must identify implemented components, their verification status, known limitations, tested targets and performance evidence. Do not claim universal native support or exact visual/behavioral parity without the corresponding tests. Track input latency, frame timing and memory with realistic contents; performance budgets will be set against the first validated environment.

Versioned experimental source tags and a changelog are published. The repository contains the GTK implementation, typed metadata, independent gallery and native CI. No registry package or binary distribution is published; source tags do not establish full reference parity.
