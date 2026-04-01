# AGENTS.md

This repository is the workspace for **Yongle**.

Yongle is a structured learning and knowledge-processing system.
Its long-term direction is not "just a note app" and not "just an SRS app".
It aims to become a system that combines:

- structured knowledge authoring
- deterministic rendering and extraction
- spaced repetition
- incremental reading
- workflow/state-machine-based learning processes
- a reusable non-GUI core
- multiple frontends and tools built around the core

The codebase is expected to evolve toward a layered architecture:

1. core domain and shared abstractions
2. parsing / compiling / extraction / query logic
3. learning pipeline and scheduling
4. reusable CLI / service / application interfaces
5. GUI and external integration layers

Current naming convention:
- workspace crates should generally use the `yongle_xxx` prefix
- `yongle_core` is the top-level library crate that integrates other crates and serves as the main reusable core library

---

## Primary instructions for the coding agent

When working in this repository, follow these rules strictly.

### 1. Preserve architecture clarity

- Prefer small, explicit modules.
- Keep boundaries clear between domain logic, infrastructure, UI-facing integration, and glue code.
- Do not place GUI-specific concerns into core crates.
- Do not place Electron-specific or frontend-specific assumptions into reusable Rust crates.
- Keep `yongle_core` as an integration-oriented crate, not a dumping ground for unrelated logic.

### 2. Prefer reuse over duplication

- If logic can live in a focused crate, do not bury it inside a higher-level crate.
- If a concept is likely to be reused by CLI, GUI, tests, or future services, place it in a reusable crate/module.
- Avoid ad-hoc utilities when a domain abstraction is more appropriate.

### 3. Optimize for long-term maintainability

- Prefer explicit types over clever shortcuts.
- Prefer readable control flow over compact but opaque code.
- Avoid unnecessary macro-heavy designs unless they clearly improve the architecture.
- Keep public APIs stable, minimal, and intentional.
- Design with future extraction, scheduling, and workflow/state-machine logic in mind.

### 4. Do not introduce accidental complexity

- Do not over-generalize prematurely.
- Do not add configuration, traits, or abstraction layers unless they solve a real problem in this repository.
- Do not introduce hidden global state.
- Minimize cross-crate coupling.

---

## Documentation and comments

### 5. All code comments must be written in English

This applies to:

- line comments
- block comments
- doc comments
- module-level documentation
- public API documentation

Do not write comments in Chinese or mixed Chinese-English.

### 6. Public items must be documented

Every meaningful `pub` item must have documentation comments.

This includes, when applicable:

- `pub struct`
- `pub enum`
- `pub trait`
- `pub fn`
- `pub const`
- `pub static`
- `pub type`
- important `pub mod`

The documentation does not need to be verbose, but it must explain the purpose of the item clearly.

Minimum standard:
- what the item is for
- what role it plays in the system
- important invariants or usage constraints when relevant

For complex public APIs, also document:
- ownership/lifetime expectations
- error semantics
- thread-safety assumptions
- whether the API is stable, low-level, or integration-oriented

### 7. Internal comments should be sparse and meaningful

- Do not comment obvious code.
- Use comments to explain intent, invariants, tradeoffs, and non-obvious decisions.
- Prefer improving names and structure before adding comments.

---

## Code style expectations

### 8. Prefer explicit, idiomatic Rust

- Follow idiomatic Rust naming and module organization.
- Use `Result`/`Option` carefully and intentionally.
- Avoid `unwrap()` / `expect()` in production paths unless failure is truly impossible or explicitly justified.
- Propagate errors with meaningful types where practical.
- Avoid panics in library code unless they represent violated invariants.

### 9. Keep APIs narrow

- Expose the smallest public surface necessary.
- Default to private visibility.
- Only make items `pub` when they are intentionally part of a crate's external API.

### 10. Write code that is easy to test

- Prefer pure logic where possible.
- Separate parsing, transformation, and side effects.
- Keep filesystem, process, network, and platform interaction behind clear boundaries.

---

## Planning guidance

### 11. Favor the long-term product direction

When making local design decisions, prefer choices that fit the long-term Yongle direction:

- structured knowledge instead of loose text blobs
- deterministic and inspectable processing instead of opaque magic
- note content as the source of truth, with derived views/projections
- learning workflows as explicit processes, not vague chat behavior
- reusable non-GUI core first, GUI second

### 12. Anticipate future subsystems

Code should not hardcode assumptions that would block future work in areas such as:

- Typst-first or structured authoring pipelines
- extraction of semantic units from source documents
- flashcard projection from the same semantic source
- incremental reading workflows
- agent/state-machine-based learning orchestration
- cross-platform desktop integration
- CLI and automation workflows

Do not implement these future systems speculatively unless requested, but avoid designs that make them awkward.

---

## Editing rules for the agent

### 13. Before introducing a new crate, module, or abstraction

Ask implicitly through your design whether:

- this belongs in an existing crate/module
- this is reusable enough to justify a new unit
- this keeps the workspace architecture cleaner
- this increases or reduces conceptual load

Prefer fewer, more coherent crates over fragmentation.

### 14. When modifying public APIs

- preserve naming consistency
- preserve conceptual consistency
- update documentation comments
- update tests if behavior changes
- avoid breaking changes unless they clearly improve the design

### 15. When adding TODOs

Write them in English and make them specific.

Bad:
- `TODO: improve this`

Good:
- `TODO: Split scheduler-independent review state from FSRS-specific parameters.`

---

## Output expectations for the agent

When implementing requested changes:

- keep diffs focused
- avoid unrelated refactors
- do not rename broadly without good reason
- document new public items
- maintain English comments/doc comments
- preserve workspace coherence

If a requested change conflicts with these rules, prefer the rule set unless the user explicitly overrides it.

