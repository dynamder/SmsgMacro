# Implementation Plan: Message Hash Verification

**Branch**: `003-message-hash` | **Date**: 2026-03-07 | **Spec**: `spec.md`
**Input**: Feature specification from `/specs/003-message-hash/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Generate blake3 hash for each message definition in an smsg package. Hash is hardcoded in MessageMeta trait via procedural macro. SmsgEnvelope<T> wrapper includes version_hash field initialized at construction time via MessageMeta::version_hash().

## Technical Context

**Language/Version**: Rust 2024 (edition)  
**Primary Dependencies**: blake3 (for hash), syn, quote, winnow, toml, proc-macro2  
**Storage**: N/A  
**Testing**: cargo test, cargo clippy  
**Target Platform**: Cross-platform (Windows/Linux)  
**Project Type**: library (Rust proc-macro crate)  
**Performance Goals**: Under 1 second for packages with up to 100 messages  
**Constraints**: Error messages within 500ms  
**Scale/Scope**: 100 messages per package

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
|------|--------|-------|
| Technology Standards (clippy) | PASS | Will use cargo clippy |
| Technology Standards (winnow) | PASS | Already in use |
| Dependencies active maintenance | PASS | blake3 is well-maintained |
| Code Quality (<80 lines/function) | PASS | Design follows single responsibility |
| Testable Units | PASS | Unit tests required for all functions |

**Violations**: None

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: Existing Rust proc-macro crate structure. No new directories needed. Feature adds:
- Hash computation in codegen module
- MessageMeta trait generation via proc-macro
- SmsgEnvelope wrapper type in generated code

```text
src/
├── lib.rs                  # Main entry, smsg attribute macro
├── codegen/
│   ├── mod.rs
│   ├── struct_gen.rs       # Struct generation
│   └── derive_gen.rs       # Derive macro generation (add MessageMeta here)
├── parser/
│   └── package_parser.rs
├── ir.rs                   # Intermediate representation
└── error.rs                # Error types
```

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
