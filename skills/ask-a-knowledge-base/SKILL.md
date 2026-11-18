---
name: ask-a-knowledge-base
description: Finding your way in a tad knowledge base (tables.toml, data/): ask a named query, never read data/; record through its named records with ikb. Use before answering from a project's data, and when recording.
---

# Asking a knowledge base

A base that follows grenudi/ideal-knowledge-base is organised so that an agent answers by one query. What a
base is, and why, is kept there; this is how to use one.

1. **Read its AGENTS.md first** (under 3 KB): its orientation table says what it holds and which query
   answers what. A base may add its own rows to that table.
2. **Ask; never read the files.** `data/` is the database's own files and `docs/` the pages for people -
   reading them to answer costs many times the tokens and misses what the queries join. Which query answers
   what is the base's own: its AGENTS.md, "Finding what is known" (idealkb lays that table in every base, so
   it is current there and not repeated here); `tad statements` lists every query and record with its
   parameters. Briefs first, the whole on demand: each answer gives a record by its brief; go deeper only
   when your task needs more. Long answers: `--format csv`.
3. **Record with `ikb`** (record-the-work): `ikb --help`, then `ikb help RECORD` for the record you need, then `ikb RECORD --option value` - never SQL,
   never by hand; each record with `--about`, a decision and a research with `--brief` (one line, at most 200
   characters). A missing record or query is a new file in `records/` or `queries/`, in the same change. Then
   land (land-a-change).
4. **Where a row ends and a file begins**: a thing there are many of with one shape is a row; prose read
   once in order (a README, a brief, a long essay cited by a row) is a file, and opens with its own brief:
   its first paragraph says what it holds.

A base that does not follow yet (no `queries/about.sql`): read its AGENTS.md, then `tad statements` for
what it can be asked.
