---
name: lab-session-start
description: Start of every session in the open-commons ecosystem (on the lab machine or anywhere) - read the practices, sign your work, read the inbox and the lenses before choosing anything. Use at the very beginning of a session, and again after a long pause.
---

# Starting a session

1. **The practices.** The plugin's SessionStart hook prints them (`ideal-agent start`); read them whole -
   they outrank any brief. Again at any time: `lab-agent guide` on the lab, or
   `${CLAUDE_PLUGIN_ROOT}/scripts/ideal-agent practices` anywhere.
2. **Sign your work.** In every shell that works on the ecosystem: `export OCOECO_BY=<your role>`
   (bring-up, orchestration, research-<feature>, ...) then `. oco-work`. Your name goes on every record and
   signs every commit (oco-work makes it the git author), so the face shows where each session stands. Never
   call `land` from a shell that has not sourced `oco-work` (without it `land` also merges pending release PRs).
3. **The inbox.** `lab-agent inbox` - print every unread message, answer each, then `lab-agent inbox --ack`.
   The hook prints them at the start, read but not acknowledged.
4. **Look before choosing.** From `~/Projects/marshall-oco/marshall-oco.github.io`: `ocoeco focus practices`,
   `ocoeco fulldome --surface as-outline`, `ocoeco focus still-showing`; the board (`lab-agent board show`);
   the project's own face and database (`tad q backlog`, `tad q decisions` in its tad place). Your brief - the
   domain part of your role - is in the project's `prompts/`; the method of the role is a skill here
   (orchestrate-agents, research-a-feature, bring-up-a-part).
5. **A domain you take up** is its rows: read its DOMAIN-how-it-stands research row and the rest of its
   research, decisions and backlog before anything else - that is how you become its expert (record-the-work,
   "Hand-over").
6. Choose by the weakest link the data shows, not by momentum; say on the board what you take up.
