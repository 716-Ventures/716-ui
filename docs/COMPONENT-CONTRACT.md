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
