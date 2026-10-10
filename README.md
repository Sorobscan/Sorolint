# Sorolint

[![CI](https://img.shields.io/github/actions/workflow/status/Sorobscan/Sorolint/ci.yml?branch=main&label=CI)](https://github.com/Sorobscan/Sorolint/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/Sorobscan/Sorolint)
[![Rust Edition 2021](https://img.shields.io/badge/edition-2021-orange.svg)](Cargo.toml)

> Static analysis engine and linter for [Soroban](https://soroban.stellar.org/) smart contracts — securing the Stellar blockchain, one contract at a time.

Sorolint is a command-line static analyzer and linter for Rust smart contracts deployed on the **Stellar network** via the Soroban smart contract platform. It detects security vulnerabilities and code quality issues before your code ever touches the chain.

---

## Why Sorolint?

Soroban is Stellar's smart contract platform — a WebAssembly-based execution environment designed for speed, low cost, and predictability. But like any smart contract platform, **bugs in Soroban contracts can be exploited on-chain and are irreversible**.

Sorolint catches common vulnerability classes at the source level, before `stellar contract deploy` ever runs.

---

## How does this compare to `clippy` and `cargo-audit`?

These tools solve different problems and are complementary — running all three gives broader coverage than any one alone.

| | `sorolint` | `clippy` | `cargo-audit` |
|---|---|---|---|
| **What it analyzes** | Soroban contract source (`#[contractimpl]` methods, storage calls, cross-contract invocations) | Any Rust source, general-purpose | `Cargo.lock` dependency versions |
| **What it looks for** | Soroban/Stellar-specific contract security bugs (missing `require_auth`, reentrancy, unprotected admin, hardcoded addresses, unsafe storage, etc.) | General Rust correctness, style, and performance lints | Known-vulnerable crate versions published in the [RustSec advisory database](https://rustsec.org/) |
| **Domain awareness** | Yes — understands Soroban SDK patterns (`env.storage()`, `invoke_contract`, `#[contract]`/`#[contractimpl]`) | No — has no concept of Soroban, contracts, or on-chain state | No — operates purely on dependency metadata, not source code |
| **Catches supply-chain CVEs** | No | No | Yes |
| **Catches contract-logic vulnerabilities** | Yes | No | No |
| **Catches general Rust bugs/style issues** | No | Yes | No |

In short: `cargo-audit` checks *what you depend on*, `clippy` checks *how you write Rust in general*, and `sorolint` checks *whether your contract logic is safe to deploy on Stellar*. None of them substitute for the others — a contract can pass `clippy` and `cargo-audit` cleanly while still having an exploitable reentrancy or missing-auth bug, which is the gap `sorolint` is built to close.

---

## Stellar / Soroban Context

Soroban contracts are Rust crates compiled to WASM and deployed to the Stellar network. Key security concerns this tool addresses:

| Concern | Stellar/Soroban Impact |
|---|---|
| Missing `require_auth` | Any caller can invoke privileged contract functions |
| Unchecked arithmetic | Integer overflow/underflow in token balances or ledger math |
| Unprotected admin | Admin keys can be overwritten without authorization |
| Unsafe storage patterns | Persistent/temporary ledger storage misuse |

---

## Requirements

- Rust 1.74+ (2021 edition)
- No Stellar SDK or network connection required — analysis is purely static

## Build

```bash
cargo build --release
```

The binary is `target/release/sorolint` (package `sorolint`).

---

## Usage

Scan a Soroban contract crate before deploying to Stellar:

```bash
cargo run -p sorolint -- scan ./path/to/contract-crate
```

Output as JSON (useful for CI pipelines or web integration):

```bash
cargo run -p sorolint -- scan ./path/to/contract-crate --json
```

Write JSON to a file instead of stdout:

```bash
cargo run -p sorolint -- scan ./path/to/contract-crate --json --output findings.json
```

Emit SARIF 2.1.0 for GitHub Code Scanning:

```bash
cargo run -p sorolint -- scan ./path/to/contract-crate --sarif > findings.sarif
```

List the checks that run by default:

```bash
cargo run -p sorolint -- list-checks
cargo run -p sorolint -- list-checks --json
```

Explain a specific check:

```bash
cargo run -p sorolint -- explain missing-require-auth
```

---

## Configuration (`sorolint.toml`)

Sorolint can be configured per-project using a `sorolint.toml` file in your repository root or scan target directory:

```toml
[scan]
min_severity = "medium"

[checks]
disabled = ["missing-event-emission"]

[checks.sensitive_names]
extra = ["custom_admin_func"]
```

---

## Authors & Maintenance

Created and maintained by **Owolabenjade** (<owolabenjade@gmail.com>).

---

## License

Dual-licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
