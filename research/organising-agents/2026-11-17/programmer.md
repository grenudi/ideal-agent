# A programmer for the canon (2026-11-17)

## The canon, where it is
- creation-guidelines/software-engineering-canon: a tad base, 20 principles, 14 practices, 10 architectures,
  each with an id (parse-dont-validate, ci-gatekeeper, declared-invocation-surface...); no named queries -
  read by `tad sql` on its tables. Cited by docs (5 files), by ideal-agent nowhere.
- creation-guidelines/ideal-project-playbook: 37 concepts with ids plus prose chapters and an agent kit
  (kit/rust, kit/guards, kit/ci): how a project is run with an agent.
- grenudi/design-canon: one README (5.9 KB), the same description as software-engineering-canon, last pushed
  2026-09-21 - likely its earlier version.
- Overlaps across the three and ideal-agent: about ten rules (every-layer-is-a-top, ratchet, ci-gatekeeper,
  names, one fact one place, capture-refactor-compare, declared-invocation-surface = practice 43, look around
  first = practice 22).

## Today's code (2026-11-17)
- Slips: lab-tools devlab and tone changed today (d40cc56, f904196, d510232, 1b35fc6) and still on clap
  against practice 43; google-walleye-lab #44 added a 222-line C helper, replaced by Rust 28 minutes later
  (#52) but the C file stays in the tree; google-walleye-lab's CI runs no cargo, so the Rust helper has no
  clippy or test in CI.
- Good: lab-tools CI runs clippy -D warnings and tests on the workspace; today's lab-tools changes ship tests.

## Others
Claude Code subagents: skills preloaded into one agent, a tool allowlist, a fresh context - a clean reviewer.
Google's reviewer standard: approve what improves the code's health; the style guide decides style. ChatDev:
programmer, reviewer and tester roles beat a bare model; a 2025 ablation: the tester mattered most. No study
isolates a reviewer with a guide against one agent with the same guide. The usual split: tools enforce what
they can, the writer carries a short core, a separate reviewer judges the rest.
