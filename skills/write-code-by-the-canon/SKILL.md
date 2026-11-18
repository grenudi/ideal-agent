---
name: write-code-by-the-canon
description: The software canon's core for writing and reviewing code - what the checks enforce, the few rules every change meets, and how to look up any principle by id. Use when you write or review code, or are asked which way a design should go.
---

# Writing code by the canon

The canon is one place, not this file: `~/Projects/creation-guidelines/software-engineering-canon`, a tad base
of principles, practices and architectures, each with an id. This is its core; look the rest up by id, and
cite the id when you choose (decision 2026-11-17-a-programmer-consulted-like-a-researcher).

**What the checks enforce** - meet them before landing, never argue with them:
- Rust: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, run by the repository's
  CI too (`ci-gatekeeper`; a crate whose CI does not run them is a sign for its owner). Shell: `sh -n`,
  shellcheck. No new Python, and no new C where Rust serves (practice 21) - only upstream's own C.
- One invocation surface per program, declared with argenv (`declared-invocation-surface`, practice 43): a
  program still on clap moves when it is next changed, in that change.

**What every change meets**:
- **A test first**, failing for the missing thing (`test-at-the-seams`); real output pinned before a refactor
  (`capture-refactor-compare`).
- **Types that hold the rules**: parse at the edge, carry the parsed type (`parse-dont-validate`,
  `illegal-states-unrepresentable`, `newtypes`); total functions; fail fast and loudly (`fail-fast`).
- **One fact in one place**, derived where it can be (`dry`, `derive-dont-verify`); the person's words as names
  (`names-are-the-persons-words`).
- **A functional core and an imperative shell** (`functional-core-imperative-shell`); the top of each layer
  decides which, not how (`top-layers-decide-which-not-how`).
- **Debts only shrink** (`ratchet`): a change leaves the code it touched better by the canon, not worse.
- **A person's attention only where it must be** (`converge-and-ask-only-on-change`): a tool run again with
  nothing changed changes nothing and asks nothing, and knows what it did.
- **The smallest change that does it** (`kiss`, `yagni`); remove what the change made unused.

**Look up any other** - read-only, by id:
`cd ~/Projects/creation-guidelines/software-engineering-canon && tad sql "SELECT * FROM principles WHERE id = 'ID'"`
(the tables: principles, practices, architectures). How a project is run with an agent - its loop, guards and
CI kit - is `creation-guidelines/ideal-project-playbook` (its `kit/rust`, `kit/ci`).
