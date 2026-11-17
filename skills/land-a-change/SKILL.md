---
name: land-a-change
description: How a change reaches main in the open-commons ecosystem - its own branch, checks, a record, land, main read back. Use whenever you are about to edit a repository under ~/Projects or land a change, including data-only changes.
---

# Landing a change

- **Branch first, then write.** `fresh OWNER/REPO BRANCH` resets the clone and cleans untracked files, so
  anything written before it is lost. Where another session may work in the same clone, use a `git worktree`
  of your own instead (`land` and `landit` handle worktrees). Work only in places your brief says you own.
- **Change what you mean.** `change FILE` with the exact old text, `--- becomes ---`, the new text; rows
  through the place's own tool - `tad r NAME ...` / `tad set` in a tad place, `ocoeco note` in an ocoeco
  place. Never copy a whole file from an older copy.
- **Check like CI.** Rust: `rust_checked` (fmt, clippy -D warnings, tests). A tad place: after the last data
  change, `tad canon && tad render && make -s verify` (CI compares the rendered pages; a row entered after
  rendering fails it). Shell: `sh -n` and shellcheck.
- **Record in the same change.** Decisions (with what was rejected), the backlog item's next step and the
  session log go in the owning place's own data (record-the-work). Where a place keeps no glimpses (a tad
  place such as linux-phone-oco/docs or ideal-agent), the commit and its pull request are the record, signed by
  your OCOECO_BY.
- **Land.** `landit "SUMMARY" KIND` (summary at most 88 characters; KIND feat, fix, docs, ...), or plain
  `land`. One commit per branch, with a conventional title. It waits for every check and refuses on a
  failing one. If it stops, the branch is often already pushed: read `gh pr checks BRANCH` and the failing
  run's log (`gh run view ID --log-failed`), fix on that branch, push, then `land --no-manual`. The plugin's
  hook refuses any `git push` that reaches main.
- **Read main back.** After landing, check that main holds the work (`git show origin/main:FILE`, the rows),
  not only its record.
