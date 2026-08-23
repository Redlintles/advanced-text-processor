---
name: atp-project
description: Work on the ATP Rust text-processing DSL, including token implementations, `.atp` pipelines, bytecode, tests, public API, and planned FFI. Use when the task mentions ATP, its instruction mnemonics, `InstructionMethods`, `AtpBuilder`, or `.atp`/bytecode files; do not use for unrelated Rust projects.
---

# ATP project

ATP (`atp`) is a Rust text-processing DSL. Pipelines contain single-purpose tokens and can be used through `.atp` source files or the Rust API. The crate is edition 2024; bytecode support and the CLI are gated behind the `bytecode` feature.

## Work from the codebase

- Treat the repository as the source of truth for instruction names, signatures, opcodes, and behavior. `instructions.txt` is useful for orientation, but confirm details in the relevant module.
- Tokens live in `src/tokens/transforms/<mnemonic>/` and `src/tokens/instructions/<mnemonic>/`; each normally has `mod.rs` and `test.rs`.
- The common contract is `src/tokens/mod.rs::InstructionMethods`. Token lookup, syntax, and opcode registration are in `src/globals/table.rs`.
- Use the local instruction's `mod.rs` and neighboring tokens as the implementation template. Preserve Unicode-safe behavior where it is relevant.

## Token and bytecode changes

- Keep `to_atp_line`, `transform`, `get_string_repr`, `from_params`, and, when enabled, `get_opcode` and `to_bytecode` consistent with the token's syntax and registration.
- `to_bytecode` is feature-gated and returns `Result<Vec<u8>, AtpError>`. In tests that expect serialization to succeed, unwrap it: `let bytecode = token.to_bytecode().unwrap();`. Propagate or assert errors in production code instead of discarding them.
- Run bytecode-related tests with `cargo test --features bytecode` (or `test_access` when its helpers are needed). Do not change opcode values or bytecode layout without checking reader/writer compatibility.
- When adding or altering a token, update its module, its tests, `src/tokens/transforms/mod.rs` or `src/tokens/instructions/mod.rs` if needed, and the registration/syntax in `src/globals/table.rs`. Search for all call sites and documentation that name the mnemonic.

## Public API and conventions

- The Rust fluent API is defined by `AtpBuilderMethods` in `src/api/mod.rs`, implemented by `AtpBuilder` in `src/api/atp_builder.rs`, and run by `AtpProcessor`.
- Builder operations return `Result`; examples and tests should unwrap only when success is part of the test precondition.
- Maintain existing public behavior unless the request explicitly changes it. Use `AtpError` and its error codes for validation failures rather than introducing ad-hoc error strings.
- Write new commits, code, and user-facing documentation in English unless the user requests otherwise.

## References

- For designing or registering an instruction, read [instruction design](references/instruction-design.md).
- For a requested C-compatible binding layer, read [FFI planning](references/ffi-guide.md). The FFI is planned; do not implement it unless asked.
