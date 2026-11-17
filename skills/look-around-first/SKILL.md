---
name: look-around-first
description: The small look-around before acting on a proposal - the author's too - or before any change that is not trivial - where things stand, how others do it, reconsidered, and the finding kept as a research row with its sources and its material in a dated folder. Use before you build, change or adopt something someone proposed, and when a brief says "look around first".
---

# Looking around first

A proposal is a starting point, not a specification - the author's too. Before acting on it, spend a small,
bounded look (practice 22): minutes and a few sources, not a feature research (research-a-feature is that).

1. **Where things stand.** Ask the owning base, never its files (ask-a-knowledge-base): `tad q about
   --subject S` - its summary first, then what was recorded since - and `tad q find --text WORDS` for what was
   already decided or tried. Read the code and data it touches. Measure what you can. Before replacing
   anything, find why it exists (Chesterton's fence).
2. **How others do it.** Inside the commons first (our crates and tools), then outside: the two to five
   practices nearest to the question, each read at its source, each with what to take. The `researcher`
   agent can do this part, read-only, in parallel with step 1.
3. **Reconsider.** Set the proposal against both. Say what it gets right, where the world or the data
   disagrees, and what you will do instead or as well - in your own words, not the proposal's. What
   already exists is reused before anything is added.
4. **Keep the finding, out of the way.** In the base that owns the subject:
   - a research row: `tad r research --id ... --about S --question ... --found ... --sources a,b` with
     each source recorded (`tad r source`). `found` is the condensed finding - what a later reader needs;
   - its own material, if there is more than fits a row (the sources read, excerpts, measurements, a
     researcher's report): a folder `research/SUBJECT/YYYY-MM-DD/`, named with `--material`. Nobody reads
     it unless the finding is questioned;
   - research redone is a new row with `--supersedes OLD` and a new dated folder; the old stays;
   - the decision that rests on it cites it (`--research ID`) and says what was rejected.
5. **Renew the summary when due.** If `tad q about` says a summary is due for the subject, write it in the
   same change: `tad r summary --about S --says "..."` - how it stands now, in at most 1500 characters,
   naming the records that carry it. The records stay; the summary only condenses.

Report in a few plain lines: what is, what others do, what you took and what you set aside, and why.
