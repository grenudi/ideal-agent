---
name: record-the-work
description: Where and how thinking, decisions, research, backlog, hand-overs and the author's words are recorded - the project's tad base, through ikb. Use whenever you decide, learn, start or finish something, or hand over.
---

# Recording the work

Nothing may depend on anyone remembering: it goes into the owning project's own knowledge base, in the same
change as the work. General know-how - how agents work - goes to grenudi/ideal-agent; what is one project's
domain goes to that project (for every linux-phone place: linux-phone-oco/docs, each record naming its
place with `--project`).

A base is kept by tad and follows grenudi/ideal-knowledge-base (ask-a-knowledge-base). Record with
`ikb` (grenudi/ideal-knowledge-base; `sh bin/install-ikb.sh` in the base installs it): `ikb --help` lists the base's records
(one line each), `ikb help RECORD` gives the one you need - its options, written from its `records/*.sql` - then
`ikb RECORD --option value` (`ikb page` prints the whole on one page, for people). Long text goes in from a file or stdin (`--found @-`), the id is made from the
brief, canon and render run after; never SQL, never by hand. What to say in each:

- **Decisions** - always with what was rejected and why; a decision and a research carry a one-line `--brief`
  (at most 200 characters; answers read briefs first).
- **Research** - its finding is the row; its own material, if more than fits a row, in a dated folder:
  `--material auto` makes `research/SUBJECT/YYYY-MM-DD/`. Redone, a new row `--supersedes` the old.
- **Backlog** - one row per piece of work, its note saying where it stands and the next step; done only when
  what landed is read back from main.
- **Session log** and **sources** - what was done, considered, rejected; each source a row.
- **What it is about**: every record names its subject in `--about` (`tad q about` lists them), so that
  `tad q about --subject S` gives the subject's whole knowledge.
- **A summary**: when `tad q about` says one is due for a subject you recorded into, renew it in the same
  change - `ikb summary --about S --says ...`, how it stands now, naming the records that carry it.
- **A lesson about how a role must act** goes where that role reads at its start, in the same change as the
  log entry that records it: the role's brief when it is the domain's, or - when it holds for any domain - an
  upstream item to ideal-agent for its card or skill. A lesson only in a session log never reaches a
  launched run, which reads its card and brief, not logs (a surya keeper defused the watchdog on an
  unattended try, 2026-11-17, though docs #272 had logged the lesson).
- **A change to a row no record covers** (a status, a wait, a field of a decision, a source taken away):
  `ikb set TABLE --key K --field NAME=VALUE`, `ikb remove TABLE --key K` - checked; never `tad sql`, which the
  guard refuses for writes.
- **A missing record or query** is a new file in the base's `records/` or `queries/`, in the same change.
- **Work for others** - a patch, a finding, a correction or a question for a project, a list, a wiki, a standard, a
  person or another base (an improvement made in a domain base that is general goes to idealkb the same way) is an
  upstream item. Find and choose where first: `tad q upstreams` lists the places this base knows; for a new one
  look at how the place takes contributions (its contributing page, list rules, maintainers file), record what
  you found as research the dated way (`ikb research --material auto`, each source `ikb source`) and the place
  `ikb upstream` with its `--how`. Then `ikb upstream-item --upstream U --from-work ID --form F` - a patch or long
  message in its `--file auto` folder - and `ikb upstream-state` as it moves. A draft becomes `ready` only on the
  person's word; sending in their name is theirs to say. `tad q upstream` shows what waits.

Then land (land-a-change): the commit and its pull request are the record of
the work, signed by your AGENT_NAME. Hand-work done a third time asks for a tool: record it on the backlog.

A place still kept the older way, with ocoeco (oco/ tables, glimpses), is another project's legacy: follow
its own AGENTS.md there.

**The author's words**: every turn of his, as said, with its moment - `turn ID AFTER AT TEXT SAID` (a word of
oco-work, which that place still uses) into marshall-oco/conversation.

## Hand-over

An expert is a domain written down, not a session kept alive (ideal-agent decision
2026-11-17-experts-written-not-standing). When your part lands, or before your context is compacted:

1. Add to the domain's `DOMAIN-how-it-stands` research row, item by item, never rewriting it whole: where
   it stands, the dead ends, where experiments live (patches, switches, logs), what must never be done again.
2. Put the next step on its backlog item, and renew the domain's summary if `tad q about` says one is due:
   the short how-it-stands the next session reads first.
3. Then end the session; the next one starts fresh from the rows. Notes outside a repository (a side
   session's notes folder) are a pointer at most, never the only copy.

A question about a domain: read its rows first; if they do not answer, message the domain's live session
directly; with none live, a short read-only consult from the rows - and every answer is written back as a row.
