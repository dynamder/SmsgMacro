# Implementation Plan: SoulSmsg Package Parsing

**Branch**: `002-smsg-package-parsing` | **Date**: 2026-03-07 | **Spec**: spec.md

**Input**: Feature specification from `/specs/002-smsg-package-parsing/spec.md`

## Summary

Add attribute support to the `#[smsg]` macro to support both single file parsing (`type = file`) and package parsing (`type = package`). Package parsing includes: package.toml metadata/dependencies, import statements in .smsg files, and **generating Rust modules that mirror the package folder structure** (FR-013).

## Technical Context

**Language/Version**: Rust 1.75+ (proc macros)  
**Primary Dependencies**: winnow (parsing), quote, syn  
**Storage**: File-based (.smsg files, package.toml)  
**Testing**: cargo test  
**Target Platform**: Any Rust project  
**Project Type**: library/proc-macro  
**Performance Goals**: <100ms parsing per file  
**Constraints**: Must maintain backward compatibility with existing string-only attribute syntax  
**Scale/Scope**: Single crate, supports multiple packages

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Gate | Status | Notes |
|------|--------|-------|
| Code Quality: Functions <80 lines, Files <600 lines | PASS | Macro parsing will be in small functions |
| Code Quality: Cyclomatic complexity <10 | PASS | Simple attribute parsing logic |
| Testable Units: 80% coverage core, 70% overall | PASS | Parser + macro tests required |
| MVP First: Independent deployable increments | PASS | File type first, package type second |
| Technology: Uses winnow for parsing | PASS | Using winnow per user request |
| Technology: Uses clippy | PASS | Will run clippy on changes |

## Project Structure

### Documentation (this feature)

```text
specs/002-smsg-package-parsing/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Main macro entry point - needs attribute parsing update
├── parser/
│   └── mod.rs           # Existing .smsg parser with winnow, add package.toml parser
├── codegen/
│   ├── mod.rs
│   ├── struct_gen.rs    # Add module structure generation (FR-013)
│   └── derive_gen.rs
├── ir.rs                # IR definitions
└── error.rs             # Error types

tests/
├── integration_test.rs  # Existing integration tests
└── fixtures/            # Test .smsg files and packages
```

**Structure Decision**: Single proc-macro crate. Update `lib.rs` for new attribute syntax, add package parsing, and update `struct_gen.rs` to generate nested modules mirroring folder structure.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
