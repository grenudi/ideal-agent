---
name: record-the-work
description: Where thinking, decisions, research, backlog, milestones, hand-overs and the author's words are recorded in the open-commons ecosystem, and how. Use whenever you decide, learn, start or finish something, hand a domain over, or the author says something.
---

# Recording the work

Nothing may depend on anyone remembering: it goes into the owning project's own data, in the same change
as the work, and propagates to the observatory by itself. General know-how - how agents work - goes to
grenudi/ideal-agent; what is one project's domain goes to that project.

**A tad place** (one database, `tables.toml` + `data/`: linux-phone-oco/docs for every linux-phone place,
grenudi/ideal-agent for itself): data through tad, landing through land, nothing of ocoeco by hand. Write
with its named records - `tad statements` lists them with their parameters (`tad r decision ...`,
`tad r research`, `tad r source`, `tad r backlog`, `tad r backlog-status`, `tad r session-log`, ...); read
with `tad q NAME`. In linux-phone-oco each record names its place (`--project surya-lab`). A missing record
is a new file in `records/` in the same change. `tad canon && tad render`, then `land`.

**An ocoeco place** (oco/ tables, glimpses), everywhere else:

- **Decisions**: `ocoeco note decisions --id DATE-slug --set date=... --set decision=... --set why=...
  --set rejected=...` - always with what was rejected and why; `about=` what it is about.
- **Research**: `ocoeco note research --id ... --set question=... --set found=...`.
- **Backlog**: one row per piece of work; its note says where it stands and the next step; `status` done
  only when what landed is read back from main. Say that you take an item up before working on it.
- **Session log**: `ocoeco note session_log --id DATE-slug --set entry_date=... --set title=... --set done=...`.
- **Links**: ask `ocoeco fits` for an id; a link that points nowhere is refused.
- **Hand-work**: `ocoeco manual "ACTION"` in the place that keeps it; a third time by hand asks for a tool.
- **A gap or overlap in the ecosystem itself** is a sign: recorded where the tool is kept, answered by
  refining the whole.

**The author's words**: every turn of his, as said, with its moment - `turn ID AFTER AT TEXT SAID` (a word of
oco-work) into marshall-oco/conversation.

## Hand-over

An expert is a domain written down, not a session kept alive (ideal-agent decision
2026-11-17-experts-written-not-standing). When your part lands, or before your context is compacted:

1. Add to the domain's `DOMAIN-how-it-stands` research row, item by item, never rewriting it whole: where
   it stands, the dead ends, where experiments live (patches, switches, logs), what must never be done again.
2. Put the next step on its backlog item.
3. Then end the session; the next one starts fresh from the rows. Notes outside a repository (a side
   session's notes folder) are a pointer at most, never the only copy.

A question about a domain: read its rows first; if they do not answer, message the domain's live session
directly; with none live, a short read-only consult from the rows - and every answer is written back as a row.
