# ATP instruction design

Use this reference when adding an instruction or changing how an existing token is parsed, registered, serialized, or tested.

1. Inspect neighboring tokens with matching parameter shapes before choosing the mnemonic, struct fields, validation, or opcode.
2. Implement the actual `InstructionMethods` contract from `src/tokens/mod.rs`. `transform` returns `Result<String, AtpError>` and `from_params` validates `Vec<AtpParamTypes>`.
3. Preserve the text representation (`to_atp_line`), short identifier (`get_string_repr`), parsed parameter order, and bytecode parameter order as one coherent contract.
4. Add the module export under `src/tokens/transforms/mod.rs` or `src/tokens/instructions/mod.rs`, and add/update the entry in `src/globals/table.rs`. The table defines identifier, opcode, constructor, and syntax, so it is the authoritative registry.
5. Add focused tests in the adjacent `test.rs`: normal behavior, input/parameter boundary cases, Unicode behavior when indexing characters, and bytecode layout/round-trip behavior under `#[cfg(feature = "bytecode")]`.
6. For expected-success serialization in tests, call `to_bytecode().unwrap()` because the trait returns `Result<Vec<u8>, AtpError>`.

Mnemonic names generally use lowercase abbreviations. Confirm uniqueness in `TOKEN_TABLE` before selecting one. Prefer existing `AtpParamTypes` and validation helpers over creating a parallel parser.
