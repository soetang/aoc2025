# Copilot instructions for this repo

This repository is a Rust Advent of Code workspace based on the `advent-of-code-rust` template. It provides a small CLI and a set of conventions that make solving AoC days fast. Use these rules to be immediately productive.

## Big picture

- Solutions are standalone binaries under `src/bin/` named by two-digit day, e.g. `src/bin/01.rs`.
- Each solution exposes two functions with this exact signature and names:
  - `fn part_one(input: &str) -> Option<T>`
  - `fn part_two(input: &str) -> Option<T>`
  where `T: Display` (commonly `i64`, `u64`, `String`). Returning `None` prints a ✖.
- The `solution!(DAY)` macro in `advent_of_code::template` wires `main` for the bin: reads `data/inputs/<DAY>.txt`, runs parts, prints timed results, and optionally submits via `aoc-cli`.
- Inputs/examples/puzzles live in `data/`:
  - `data/inputs/<DD>.txt` real input used by `cargo solve`.
  - `data/examples/<DD>.txt` used by unit tests inside the bin file.
  - `data/puzzles/<DD>.md` optional puzzle text.
- The CLI entrypoint `src/main.rs` implements subcommands in `src/template/commands/*` to scaffold, run, time, and read.

## Developer workflows

- Scaffold a new day: `cargo scaffold <day>` creates `src/bin/<DD>.rs` and empty data files.
- Run one day: `cargo solve <DD> [--release] [--dhat] [--submit <part>]`
  - `--release` enables bench-like timing; `--dhat` profiles heap (needs `dhat-heap` feature).
  - `--submit` uses `aoc-cli` to submit the selected part if installed and session is configured.
- Run all days: `cargo all [--release]` executes compiled aggregate runner.
- Benchmark: `cargo time <day> [--all] [--store]` prints average timings and can store them into README.
- Tests: `cargo test` or `cargo test --bin <DD> [part_one|part_two]`. Unit tests for each bin are in that bin file.
- Formatting: `cargo fmt`; Linting: `cargo clippy`.
- Optional: `cargo download <day>` and `cargo read <day>` require `aoc-cli` and `~/.adventofcode.session`.

## Code patterns and conventions

- Place solution code in the bin file for that day. At the top, invoke `advent_of_code::solution!(<day>)` or `solution!(<day>, 1|2)` when you want to run only one part.
- Keep pure parsing and solving functions separate for clarity. Example structure inside `src/bin/01.rs`:
  - `fn parse(input: &str) -> ParsedType { ... }`
  - `pub fn part_one(input: &str) -> Option<i64> { let data = parse(input); /* compute */ Some(answer) }`
  - `pub fn part_two(input: &str) -> Option<i64> { let data = parse(input); /* compute */ Some(answer) }`
- Use `advent_of_code::template::read_file("examples", DAY)` in tests, or `read_file_part("examples", DAY, n)` if multiple example files (e.g. `01-2.txt`).
- If a part can’t produce a value yet, return `None` (the runner prints an intermediate ✖ correctly).
- Avoid global state; `runner.rs` benches by re-calling your functions with the same `&str` input. Keep functions deterministic and side-effect free.
- For multi-line outputs, the runner prints a folded header; return a `String` to control formatting.

## Important files

- `src/bin/<DD>.rs`: per-day solutions and tests. Uses `solution!` macro.
- `src/template/mod.rs`: re-exports `day`, `runner`, helper `read_file`, `read_file_part`, ANSI constants.
- `src/template/day.rs`: `Day` type, `day!` macro, `all_days()` iterator.
- `src/template/runner.rs`: timing, printing, optional submission via `aoc-cli`.
- `src/template/commands/*.rs`: cargo-like subcommands (`scaffold`, `solve`, `time`, `all`, `download`, `read`).
- `data/inputs|examples|puzzles`: filesystem contract expected by helpers and commands.
- `Cargo.toml`: features `today`, `dhat-heap`, and `test_lib`. Profile `dhat` used with `--profile dhat`.

## Integration notes

- `aoc-cli` integration: install `cargo install aoc-cli --version 0.12.0` and save session cookie to `~/.adventofcode.session`.
- DHAT heap profiling: run `cargo solve <DD> --dhat` to execute under DHAT and emit `dhat-heap.json`.
- “Today” shorthand: enable `today` feature to use `cargo today` (only valid Dec 1–25).

## Gotchas and tips

- Binary names must be two digits (`01`, `02`, …); commands accept `01` as string and also `1` where `Day` parsing is used.
- The runner detects `--time` when present and performs extra benching; `cargo solve --release` shows averaged timing automatically.
- When storing benchmarks, `cargo time --store` writes into README; don’t hand-edit that table.
- Keep `part_one/part_two` return types consistent across refactors to avoid printing issues in `runner.rs`.
- If you add shared utilities, place them in `src/lib.rs` (module `advent_of_code`) and import from bins.

## Example test snippet

```rust
// Unit test layout inside a bin file
mod tests {
  use advent_of_code::template::{read_file, Day};
  use super::*;

  const DAY: Day = advent_of_code::day!(1);

  // test function
  fn test_part_one() {
    let input = read_file("examples", DAY);
    assert_eq!(part_one(&input), Some(42));
  }
}
```

If any of the above feels off for this repo, point it out and we’ll refine the instructions. 