# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Rewrite the current .smsg file parser to use the nom parser combinator library. The original parser used manual string splitting; the new parser uses nom's combinators for better error handling and maintainability.

## Constitution Check

*Gate: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Constitution Compliance Analysis

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Code Quality | ✅ PASS | Code follows SOLID, self-documenting |
| II. User Experience First | ✅ PASS | Clear error messages with line/col |
| III. Testable Units | ✅ PASS | Parser has unit tests |
| IV. Good Maintainability | ✅ PASS | Modular parser with nom combinators |
| V. Simple and Concise | ✅ PASS | Using nom combinators keeps code clean |
| VI. MVP First | ✅ PASS | Core parsing complete |
| Technology Standards: nom | ✅ PASS | Parser now uses nom |
| Security: Input validation | ✅ PASS | Parser validates all input |

### Violations (if any)
None - all resolved

## Project Structure

### Documentation (this feature)

```text
specs/001-smsg-proc-macro/
├── plan.md              # This file
├── spec.md              # Feature specification
└── research.md          # Phase 0 output
```

### Source Code

```text
src/
├── lib.rs               # Main library entry
├── error.rs             # Error types
├── ir.rs                # IR definitions
├── parser/
│   └── mod.rs           # Nom-based parser (NEW)
├── codegen/
│   ├── mod.rs           # Code generation
│   └── struct_gen.rs    # Struct generation
```

**Structure Decision**: Single proc-macro crate library

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
