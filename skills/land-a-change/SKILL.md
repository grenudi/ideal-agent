---
name: land-a-change
description: How a change reaches main: its own branch, checks, a record, land, main read back. Use before editing a repository under ~/Projects or landing a change, data-only ones too.
---

# Landing a change

- **Branch first, in a worktree of your own.** `branch ROLE/WHAT` (from the session's start, scripts/agent-env)
  makes a worktree from origin/main in the worktree home (~/.local/share/agent-worktrees/REPO/ROLE/WHAT) and
  moves you there; the clone under ~/Projects stays at main for everyone who reads it, and the guard refuses a
  commit, reset or checkout in it, and a worktree anywhere else. To read main without writing, `branch --read
  CLONE` (detached, same home). Remove your worktree after `land`. Work only in places your brief says you own.
- **Name the branch `ROLE/WHAT`** - your role (`keeper/`, `builder/`, `research/`, `coord/`, `orch/`,
  `steward/`) and what the one change does (`builder/structure-homes`). It holds one change and is deleted when it
  lands. A repository with a scheme of its own says so in its README (the kernel's `topic/`, `wip/`, `int/`).
- **A git flow the project gave to git-rhizoid runs through it**, where the repository's README or your brief
  names its graph or lists - never the same flow by hand beside it. A test tree from topic branches is
  `rhizoid assemble LIST` (its own locked worktree and tag; `--done TAG` once what was built from it is
  recorded); a pin the graph names moves only by `rhizoid bump` - never edited by hand (its
  `rhizoid/bump/...` branch lands with `land` from a worktree on that branch, which reuses the pull request
  `--pr` opened); a dependency to patch is a cutting (`rhizoid add` or `import`, followed by `update`);
  `rhizoid status` says where each edge stands. A verb that changes anything outside the machine prints its
  plan (`outward=yes|no` per step) and acts only with `now`: read the plan, then run it with `now` - the first
  outward step for a module (a fork, a push, a merge from upstream, a cut tag) waits on the author's go.
  Elsewhere `branch` and `land` are the whole flow. Which graph, lists and tag shapes a project uses is its
  brief's (linux-phone-oco: each lane's command for each git job, `ideal-agent-linux-phone-oco/rhizoid.md`).
- **Change what you mean.** Edit the exact text that changes; rows
  through the base's named records (`tad r NAME ...`). Never copy a whole file from an older copy.
- **Check like CI.** Rust: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. A tad place: after the last data
  change, `tad canon && tad render && make -s verify` (CI compares the rendered pages; a row entered after
  rendering fails it). Shell: `sh -n` and shellcheck.
- **Record in the same change.** Decisions (with what was rejected), the backlog item's next step and the
  session log go in the owning place's own data (record-the-work). The commit and its pull request are the
  record of the work, signed by your AGENT_NAME (the session's start sets it as the commits' author).
- **Read your diff against main before landing.** `git fetch && git rebase origin/main`, then
  `git diff --stat origin/main...HEAD`: every file in it is one you meant to change. A branch cut before
  another landing, carrying whole files from its base, puts that landing back when it merges (docs #226 undid
  #230's eight briefs on 2026-11-17); land refuses such a change and names the files (LAND_REVERT_OK=1 when
  the revert is meant).
- **Land.** Commit (one commit per branch, a conventional title), then `land` (`land --help` for the rest). It waits for every check and refuses on a
  failing one. If it stops, the branch is often already pushed: read `gh pr checks BRANCH` and the failing
  run's log (`gh run view ID --log-failed`), fix on that branch, push, then `land --no-manual`. The plugin's
  hook refuses any `git push` that reaches main.
- **Read main back.** After landing, check that main holds the work (`git show origin/main:FILE`, the rows),
  not only its record.
