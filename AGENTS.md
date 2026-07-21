# AGENTS.md

## Project

RoxxBuilder — Rust (actix-web 4) API that generates optimized Dofus builds for a given attack.
Data comes from the dofus-db API (https://api.dofusdb.fr) and is cached under `resource/data/` (gitignored).

## Build / Run / Test

- Build & check: `cargo check --all-targets`
- Run (from repo root, fetches dofus-db data on version change): `cargo run --release` → serves on `localhost:8008`
- CLI args are parsed with clap in `src/cli.rs` (parsed once in `main`, globally available via `cli::args()`); `-v` / `--verbose` enables verbose output — use the `vprintln!` macro instead of `println!` for verbose-only messages
- Test: `cargo test --release` (search-quality tests are time-limited and need release speed; two of them — `estimator_and_prio_test::full_crap_find_right_comb` and `roxx_builder_graph_test::real_item_coherent_result_test` — assert heuristic convergence thresholds and may fail without indicating a regression)
- `Cargo.lock` is committed: keep it up to date.

## Git policy for agents

- Once changes are done, tested, and the tests pass (if any), commit them.
- Commit only files the agent edited, with a short yet explicit and complete message describing the changes.
- Never push unless explicitly asked for.
