# math-eval

An arithmetic expression evaluator in Rust (edition 2024, no dependencies): a
hand-written lexer, a recursive-descent parser that builds an AST, and a
tree-walking evaluator over `f64`. The binary evaluates one fixed expression,
`(2 + 4) / 2`, and prints the tokens and the result; it reads no input yet. What
matters most: correct precedence and associativity, and errors returned as
values instead of panics.

This repository uses the engineering harness. Harness configuration, project
overrides, and project memory live in `.engineering/`. Decisions live in the
ADR directory declared in `.engineering/harness.yaml` (`adr_dir`).

## Commands

Verified on 2026-11-06 with cargo and rustc 1.98.0 on `PATH`. The project's
toolchain comes from the Nix flake dev shell (`nix develop`, or direnv through
`.envrc`, which says `use flake`): cargo, rustc, clippy, rustfmt, rust-analyzer
from nixpkgs locked on 2026-07-08. That shell was not built here (its store
paths had been garbage-collected and `nix develop --offline` could not rebuild
them), so the commands below ran with a Rust 1.98.0 toolchain from the Nix
store instead. Run from the repository root, inside the dev shell.

| Purpose | Command |
|---|---|
| Restore | none (no dependencies); toolchain: `nix develop` (unverified here, see above) |
| Build | `cargo build` |
| Typecheck | `cargo check` (under 1 s) |
| Fast tests | `cargo test` (30 unit tests, under 1 s) |
| Full tests | `cargo test` (there are no integration tests) |
| Run locally | `cargo run` (prints the token list, then `3`) |
| Lint / format check | `cargo clippy --all-targets` and `cargo fmt --check` |

On `main` (`6736ab9`) clippy reports one warning (`unnecessary_cast`,
`src/parser.rs:130`) and `cargo fmt --check` fails on a trailing blank line at
the end of `src/parser.rs`.

## Architecture

A single binary crate; `src/main.rs` declares the modules and runs the pipeline
`tokenize` → `Parser::parse` → `eval`. There is no library target.

- `src/lexer.rs`: `tokenize(&str) -> Result<Vec<Token>, String>`. Numbers are
  `f64` with at most one decimal point; identifiers are runs of letters;
  operators `+ - * / ^`, parentheses, and commas; spaces and tabs are skipped,
  anything else is an error.
- `src/parser.rs`: `Parser` turns tokens into `Expr` (`Number`, `BinaryOp`
  with the operator `Token`, `FunctionCall`). The grammar is in the comments
  above each method:
  - `expr := term (('+' | '-') term)*`
  - `term := power (('*' | '/') power)*`
  - `power := unary ('^' power)?` (right-associative)
  - `unary := '-' unary | primary` (encoded as `0 - x`, so `-2^2` is `4`)
  - `primary := NUMBER | IDENT | IDENT '(' args ')' | '(' expr ')'`, with the
    constants `pi` and `e`
- `src/eval.rs`: `eval(&Expr) -> Result<f64, String>`. Division by zero is an
  error. Functions: `sqrt`, `abs`, `sin`, `cos` (one argument), `max`, `min`,
  `pow` (two arguments); any other name or arity is an error.

## Conventions

- Every fallible step returns `Result<_, String>` and propagates with `?`; no
  `unwrap` outside tests. Message case is not consistent (`division by zero`,
  `Expected ')'`).
- Tests live in a `#[cfg(test)] mod tests` at the bottom of the module they
  test.
- Commits follow Conventional Commits, in English, one line. The local
  `commit-msg` hook (`check-commit-msg --summary-only` from the harness)
  enforces it where installed.

## Reference implementations

Copy these when adding something similar:

- Grammar rule: `Parser::parse_power` in `src/parser.rs`
- Evaluator case: the `Expr::FunctionCall` arm of `eval` in `src/eval.rs`
- Lexer test: `tokenizes_plus` in `src/lexer.rs`
- End-to-end test through lexer, parser, and evaluator: the `parse_and_eval`
  helper and `respects_operator_precedence` in `src/parser.rs`

## Testing

Rust's built-in test harness, unit tests only: 6 in `src/lexer.rs`, 24 in
`src/parser.rs` (these go through `tokenize`, `Parser`, and `eval`). Run one
test with `cargo test respects_operator_precedence`, one module with
`cargo test parser::`. No CI runs them.

## Observability

None. The binary prints to stdout; the dev shell sets `RUST_BACKTRACE=1`.

## Delivery

| Item | Value |
|---|---|
| CI on every change | missing |
| Merge gate (required checks and reviewers, resolved threads, stale approvals dismissed) | missing: `main` has no branch protection, ruleset, or CODEOWNERS (checked 2026-11-13) |
| Hosting / provider | none (a local binary; no release workflow or tags) |
| Environments | none |
| Staging gate | missing |
| Deploy trigger | none |
| Rollback | missing |
| Feature flags | missing |
| Database migrations | none |

## Security notes

No authentication, secrets, network, or file access. The only input today is a
literal in `src/main.rs`. Before the evaluator reads input from a user: the
parser recurses once per nesting level and per unary minus with no depth limit,
so deeply nested input can overflow the stack; and arithmetic overflow
yields `inf` rather than an error.
