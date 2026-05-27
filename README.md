# ATP — Advanced Text Processor

> A sequential text-transformation DSL built in Rust. Write pipelines once, run them anywhere.

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
![Status: Pre-release](https://img.shields.io/badge/status-pre--release-orange)

---

## What is ATP?

ATP is a text-processing DSL that executes **sequential pipelines of single-purpose instructions**. Each instruction performs exactly one transformation on the input text, and instructions chain one after another — the output of each step becomes the input of the next.

```
"  hello world  " → tbs → raw world Rust → tua → "HELLO RUST"
```

Pipelines can be written in human-readable `.atp` files, compiled to optimized `.atb` binary files, composed directly in Rust via the `AtpBuilder` API, or — soon — called from JavaScript and Python through native bindings.

---

## Why ATP?

- **Portable pipelines** — write a pipeline once, share it as a `.atp` file, version it with Git, and audit it instruction by instruction.
- **Optimized binary format** — `.atb` eliminates parsing overhead for production use cases.
- **Extensible by design** — adding a new instruction touches exactly 4 files and never modifies the core. The instruction set can scale to thousands without architectural changes.
- **Fluent Rust API** — compose pipelines directly in code via `AtpBuilder` without writing `.atp` files.

---

## File Formats

| Format | Extension | Use case |
|--------|-----------|----------|
| Text pipeline | `.atp` | Human-readable, editable, versionable |
| Binary pipeline | `.atb` | Optimized for performance and distribution |

### .atp Syntax

```atp
// Comments start with //
// One instruction per line, ending with ;

tbs;
raw world Rust;
tua;
```

---

## Quick Start

### As a Rust library

Add to your `Cargo.toml`:

```toml
[dependencies]
atp = "0.1.0"
```

Use the builder API:

```rust
use atp::builder::atp_builder::AtpBuilder;
use atp::builder::atp_processor::AtpProcessorMethods;

let (mut processor, id) = AtpBuilder::new()
    .trim_both_sides()
    .replace_all_with("world", "Rust")
    .to_uppercase_all()
    .build();

let result = processor.process_all(&id, "  hello world  ");
// Ok("HELLO RUST")
```

### From a .atp file (CLI)

```bash
atp run pipeline.atp --input "  hello world  "
```

> **Note:** The CLI is currently under development and not yet available.

---

## Instruction Reference

### String Manipulation

| Mnemonic | Description | Args | Example |
|----------|-------------|------|---------|
| `atb` | Add to beginning | `<text>` | `"world"` → `"helloworld"` |
| `ate` | Add to end | `<text>` | `"hello"` → `"hello!"` |
| `ins` | Insert at index | `<index> <text>` | Insert after nth char |
| `rev` | Reverse | — | `"abc"` → `"cba"` |
| `rpt` | Repeat | `<times>` | `"hi"` × 3 → `"hihihi"` |
| `rmws` | Remove whitespace | — | Removes all whitespace |
| `splc` | Split characters | — | `"abc"` → `"a b c"` |

### Delete Operations

| Mnemonic | Description | Args |
|----------|-------------|------|
| `dlf` | Delete first char | — |
| `dll` | Delete last char | — |
| `dls` | Delete single char | `<index>` |
| `dlc` | Delete chunk | `<start> <end>` |
| `dla` | Delete after index | `<index>` |
| `dlb` | Delete before index | `<index>` |

### Replace Operations

| Mnemonic | Description | Args |
|----------|-------------|------|
| `raw` | Replace all occurrences | `<pattern> <replacement>` |
| `rfw` | Replace first occurrence | `<pattern> <replacement>` |
| `rlw` | Replace last occurrence | `<pattern> <replacement>` |
| `rnw` | Replace nth occurrence | `<pattern> <replacement> <n>` |
| `rcw` | Replace up to N occurrences | `<pattern> <replacement> <count>` |

### Trim & Select

| Mnemonic | Description | Args |
|----------|-------------|------|
| `tbs` | Trim both sides | — |
| `tls` | Trim left side | — |
| `trs` | Trim right side | — |
| `slt` | Select substring | `<start> <end>` |
| `sslt` | Split and select | `<pattern> <index>` |

### Case Conversion

| Mnemonic | Description | Args |
|----------|-------------|------|
| `tua` | Uppercase all | — |
| `tla` | Lowercase all | — |
| `tucs` | Uppercase single char | `<index>` |
| `tlcs` | Lowercase single char | `<index>` |
| `tucc` | Uppercase chunk | `<start> <end>` |
| `tlcc` | Lowercase chunk | `<start> <end>` |
| `tucw` | Uppercase word | `<index>` |
| `tlcw` | Lowercase word | `<index>` |

### Capitalize

| Mnemonic | Description | Args |
|----------|-------------|------|
| `cfw` | Capitalize first word | — |
| `clw` | Capitalize last word | — |
| `cts` | Capitalize single word | `<index>` |
| `ctc` | Capitalize chunk | `<start> <end>` |
| `ctr` | Capitalize range | `<start> <end>` |

### Rotate & Pad

| Mnemonic | Description | Args |
|----------|-------------|------|
| `rtl` | Rotate left | `<times>` |
| `rtr` | Rotate right | `<times>` |
| `padl` | Pad left | `<text> <max_len>` |
| `padr` | Pad right | `<text> <max_len>` |

### Case Formatting (Join)

| Mnemonic | Description | Example |
|----------|-------------|---------|
| `jkbc` | kebab-case | `"hello world"` → `"hello-world"` |
| `jsnc` | snake_case | `"hello world"` → `"hello_world"` |
| `jcmc` | camelCase | `"hello world"` → `"helloWorld"` |
| `jpsc` | PascalCase | `"hello world"` → `"HelloWorld"` |

### Encoding & Escaping

| Mnemonic | Description |
|----------|-------------|
| `urle` | URL encode |
| `urld` | URL decode |
| `htmle` | HTML escape |
| `htmlu` | HTML unescape |
| `jsone` | JSON escape |
| `jsonu` | JSON unescape |

---

## Feature Flags

| Flag | Description |
|------|-------------|
| `default` | Core library, no CLI, no bytecode |
| `bytecode` | Enables `.atb` binary protocol and CLI binary |
| `test_access` | Enables test helpers (`rand`, `random-string`, `tempfile`) |

---

## Adding a New Instruction

ATP is designed so that adding a new instruction never modifies the core. Only 4 files are involved:

- Create `src/tokens/transforms/<mnemonic>/mod.rs` — instruction struct + `InstructionMethods` impl
- Create `src/tokens/transforms/<mnemonic>/test.rs` — unit tests
- Register the mnemonic in `src/globals/table.rs`
- Add the builder method in the `AtpBuilder`

See [`references/instruction-design.md`](references/instruction-design.md) for the full guide with code templates.

---

## Project Structure

```
src/
├── main.rs                    — CLI entrypoint (requires bytecode feature)
├── api/                       — Public API surface
├── bytecode/                  — Binary .atb protocol (feature-gated)
├── context/
│   ├── execution_context.rs   — Runtime execution state
│   └── static_context.rs      — Static/compile-time context
├── globals/
│   ├── table.rs               — Instruction registry
│   └── var.rs                 — Variable definitions
├── macros/                    — Internal Rust macros
├── text/
│   ├── reader.rs              — .atp file parsing
│   └── writer.rs              — Output writing
├── tokens/
│   ├── transforms/            — One subdirectory per transform instruction
│   └── instructions/          — Control-flow tokens (blk, cblk, ifdc, val)
└── utils/                     — Shared utilities
```

---

## Roadmap

**Done**
- [x] Core library with 40+ instructions
- [x] `.atp` text format
- [x] `.atb` binary protocol
- [x] `AtpBuilder` Rust API
- [x] ~80% test coverage

**In progress**
- [ ] Complete CLI
- [ ] Pipeline chaining (`pipeline1.atp | pipeline2.atp`)

**Planned**
- [ ] REPL (interactive shell)
- [ ] Batch processing with parallel pipeline execution
- [ ] C FFI (`cdylib`)
- [ ] JavaScript bindings (WASM / NAPI)
- [ ] Python bindings (PyO3)
- [ ] Public pipeline repository
- [ ] `atp-game` — learn ATP by playing
- [ ] Project website
- [ ] Publish to crates.io

---

## Contributing

Contributions are welcome. Before opening a PR, please read [`CONTRIBUTING.md`](CONTRIBUTING.md) for the instruction design pattern, commit conventions, and review process.

All commits, code, and documentation should be in **English**.

Commit format: `type(scope): description` ([Conventional Commits](https://www.conventionalcommits.org/))

---

## License

[GPL-3.0](LICENSE) — forks must remain open source.