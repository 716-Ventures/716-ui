# Upstream references and licensing

shadcn/ui is the visual and interaction reference: https://github.com/shadcn-ui/ui. Its source is MIT licensed: https://github.com/shadcn-ui/ui/blob/main/LICENSE.md.

Copied and adapted upstream material retains its applicable copyright/license notices and exact source revisions below. Inspect the licenses of underlying primitives, animation helpers, icons and fonts separately.

Apache-2.0 applies to original 716 UI work; it does not replace third-party licenses. Referencing shadcn does not imply affiliation or endorsement.

The reference directory now includes pinned shadcn button/input/table source and Zinc theme data from revision `2d3f1cd436b18ea12f24130de4df781355925b08`, retaining MIT notices in `reference/LICENSE.shadcn`. These are parity references, not a browser runtime. Native Rust wrappers and token translation are original Apache-2.0 work. GTK and gtk-rs retain their upstream LGPL/MIT licenses; Cargo.lock records dependency versions. Inter is supplied by the system font package under its upstream SIL Open Font License; no font asset is redistributed here. The licensed native icon subset below is redistributed.

Native Plus and ArrowRight drawing coordinates are adapted from `lucide-react` 0.474.0, retaining its ISC/Feather copyright notices in `reference/LICENSE.lucide` and exact upstream modules in `reference/lucide/`. The original GTK/Cairo adapters remain Apache-2.0. Only these two icons are shipped; no system icon-theme substitution is used.
