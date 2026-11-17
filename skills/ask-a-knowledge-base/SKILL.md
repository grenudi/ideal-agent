---
name: ask-a-knowledge-base
description: How to find your way in a knowledge base kept with tad (a repository with tables.toml and data/, such as linux-phone-oco/docs or grenudi/ideal-agent) - read its AGENTS.md, answer by asking a named query, never by reading data/ or docs/, and record through its named records with what each is about. Use before answering any question from a project's data, and whenever you record into one.
---

# Asking a knowledge base

A base that follows grenudi/ideal-knowledge-base is organised so that an agent answers by one query. What a
base is, and why, is kept there; this is how to use one.

1. **Read its AGENTS.md first** (under 3 KB): its orientation table says what it holds and which query
   answers what. A base may add its own rows to that table.
2. **Ask; never read the files.** `data/` is the database's own files and `docs/` the pages for people -
   reading them to answer costs many times the tokens and misses what the queries join:

   | To know | Ask |
   |---|---|
   | how a subject stands, then all that is known of it | `tad q about --subject S` - its summary first, the condensed records by id (`--whole true` for all); `tad q about` lists the subjects and where a summary is due |
   | the work ahead | `tad q next` (`--about S` for one subject) |
   | where something is said | `tad q find --text WORDS` |
   | one record whole | `tad q row --id ID` |
   | what was recorded lately | `tad q recent` (`--days N`) |
   | anything else | `tad statements`: every query and record, with its parameters |

   **Briefs first, the whole on demand** (idealkb's concept brief-first): every answer gives each record
   by its brief, its one line. Go deeper - `tad q row --id ID`, `--whole true`, a file past its first
   paragraph - only when your task needs more than the briefs say. Long answers: `--format csv` (the default
   box is about six times larger).
3. **Record through the named records**, never by hand: `tad r decision` (with what was rejected),
   `tad r research`, `tad r source`, `tad r backlog`, `tad r session-log` - each with `--about`, what it is
   about, so that `tad q about` finds it; a decision and a research with `--brief`, the record in one line
   of at most 200 characters (the title is the brief of the rest) - and renew the subject's summary when `tad q about` says one is due
   (record-the-work). A missing record or query is a new file in `records/` or
   `queries/`, in the same change. Then `tad canon && tad render`, and land (land-a-change).
4. **Where a row ends and a file begins**: a thing there are many of with one shape is a row; prose read
   once in order (a README, a brief, a long essay cited by a row) is a file, and opens with its own brief:
   its first paragraph says what it holds.

A base that does not follow yet (no `queries/about.sql`): read its AGENTS.md, then `tad statements` for
what it can be asked.
