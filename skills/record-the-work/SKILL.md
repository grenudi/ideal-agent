---
name: record-the-work
description: Where thinking, decisions, research, backlog, hand-overs and the author's words are recorded in the open-commons ecosystem, and how - through the project's tad knowledge base and its named records. Use whenever you decide, learn, start or finish something, hand a domain over, or the author says something.
---

# Recording the work

Nothing may depend on anyone remembering: it goes into the owning project's own knowledge base, in the same
change as the work. General know-how - how agents work - goes to grenudi/ideal-agent; what is one project's
domain goes to that project (for every linux-phone place: linux-phone-oco/docs, each record naming its
place with `--project`).

A base is kept by tad and follows grenudi/ideal-knowledge-base (ask-a-knowledge-base). Record with
`ikb` (grenudi/ideal-knowledge-base; `sh bin/install-ikb.sh` in the base installs it): read the base's own
`IKB.md` once - every record with its options, written from its `records/*.sql` - then
`ikb RECORD --option value`. Long text goes in from a file or stdin (`--found @-`), the id is made from the
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
- **A missing record or query** is a new file in the base's `records/` or `queries/`, in the same change.

Then land (land-a-change): the commit and its pull request are the record of
the work, signed by your OCOECO_BY. Hand-work done a third time asks for a tool: record it on the backlog.

A place still kept the older way, with ocoeco (oco/ tables, glimpses), is another project's legacy: follow
its own AGENTS.md there.

**The author's words**: every turn of his, as said, with its moment - `turn ID AFTER AT TEXT SAID` (a word of
oco-work) into marshall-oco/conversation.

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
