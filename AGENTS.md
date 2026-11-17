# AGENTS.md - working in ideal-agent

This repository is the one home of general agent know-how in the open commons (decision
2026-11-17-one-home-for-agent-know-how). Put here only what every project shares; a project's domain stays
in its own data. Something general found in a project - a practice, a procedure, a guard, a role's
method - moves here, and the project keeps a pointer.

- **Data through tad, landing through land.** `data/*.json` and `docs/` are written by tad only: rows with
  the named records (`tad statements` lists them: practice, decision, research, source, landscape, backlog,
  backlog-status, session-log), then `tad canon && tad render && make verify`. No ocoeco notes, no
  glimpses: the commit and its pull request are the record, signed by your OCOECO_BY.
- **The plugin**: `.claude-plugin/` (plugin.json, marketplace.json), `skills/NAME/SKILL.md`,
  `hooks/hooks.json`, `agents/NAME.md`. Executables live in `scripts/` (never a top-level `bin/`); shell
  or Rust only, never Python. Bump `version` in plugin.json and Cargo.toml together (CI checks it): the
  plugin's program is rebuilt per version.
- **The program** (`src/`): ideal-agent's guard, start and practices commands, declared with argenv
  (`src/contract.rs`). `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` before
  landing. A guard is proven against its own event before it is trusted: a refused case and an allowed one,
  in its tests and live through Claude Code.
- **Branch first, then write**; one commit per branch with a conventional title; land with `land`.
