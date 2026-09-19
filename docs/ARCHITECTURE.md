# Architecture

## Library responsibilities

716 UI owns semantic design tokens, reusable native components, local interaction state machines, animations, accessible semantics, typed component metadata and a standalone gallery. Its public contracts should support ordinary imperative application integration and declarative composition without requiring either an AI model or a particular application protocol.

The dependency direction is application → library. agentOS may adapt the library to its surface documents and action broker; the library does not depend on agentOS services or identifiers. Other applications can use the same controls with their own state and actions.

| In the library | In the host application |
|---|---|
| Buttons, fields, lists, menus, dialogs, split views | Tasks, activities, document ownership and application navigation |
| Focus behavior within controls, selection and editing | Desktop/window placement, application focus policy |
| Typed events and value validation | File changes, commands, business logic and authorization |
| Theme tokens, component motion and accessibility | Brand selection, data access and policy |
| Loading/error visual states | Whether an operation is actually running or failed |

## Implementation direction

Use Rust for native implementation. Select the rendering toolkit after a comparative prototype validates text fidelity, IME, clipboard, accessibility, GPU/CPU behavior, scaling and motion. Do not create a universal backend abstraction before one backend works well. Linux/Wayland is the first validation target; other platforms remain unverified until listed with test evidence.

Separate reusable design/interaction contracts from renderer-specific code. The precise crate layout and public API follow the prototype, rather than committing to empty crates as a promise. Models may describe component composition in a consuming application; rendering and input never depend on inference.

## State and events

Define state transitions and typed events per component. Distinguish externally controlled values from temporary input state. Preserve drafts, cursor, IME composition and selection during unrelated updates. Specify reconciliation when an application replaces an edited value. UI state never proves a remote or system operation succeeded.

Use stable element/item identities. Lists must support bounded or virtualized data. Layout uses logical units with explicit scaling behavior. Native accessibility adapters expose semantic roles, names, states and actions, rather than relying on visual similarity.

## Visual reference

shadcn is the source of truth for corresponding controls after a specific reference profile is pinned. Record source commit, style, theme, fonts, icons, primitive implementation and motion dependencies. Translate the complete experience into native rendering; the selected source is the reference, not a moving website.

OS-specific interactions have separate specifications. A split view is reusable; desktop tiling, process supervision and global voice orchestration belong to the host. Deliberate differences needed for native accessibility or platform conventions must be documented and tested, never silently counted as exact parity.
