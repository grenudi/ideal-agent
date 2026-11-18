---
name: look-around-first
description: Before acting on a proposal or a change that is not trivial: where things stand, how others do it, reconsidered - kept as a research row with sources. Use before building, changing or adopting something.
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

## When a method is missing - research on your own initiative

Every role treats its brief and the author's words as a starting point; a coordinator (the top one, a lane's,
orchestration) also starts research itself when it sees a method missing - not only when a keeper asks
(decision 2026-11-17-research-on-own-initiative). The signs: work about to start that no record or skill
covers (bringing a new device up from nothing, with no general method recorded); the same question asked a
second time; a cluster in the patrol's "struggled with".

- **Bounded**: first the commons (the owning base, ideal-agent's base); then one round of a read-only
  researcher subagent on a lighter model; a research-lead launcher only for a question too long for that.
  One research per sign; recorded as research with sources; two lines to whom it reports.
- **What is general flows back**: a finding about how agents work, or how any device is brought up, goes to
  the place that keeps it - for agents, ideal-agent: an upstream item in your base naming grenudi/ideal-agent,
  and a message to its owner. The owner looks around in turn, decides, lands and reads back, and the domain
  keeps a pointer. No one changes a general practice, skill or card from a domain's place.

## After a fix - the class question

A fix answers one incident; the role that made it also asks what the incident was an instance of
(decision 2026-11-18-after-a-fix-the-class-question). Not after every fix - after one that met a trigger:

- the author's hands or witness were needed where none was planned;
- a resource was left not known-working, or a fallback did not catch it;
- a copy had drifted from its source (a file by hand, a pinned tool, a stored prompt);
- the same sign a second time, in any lane;
- a part fixed in one place that other places share (another device, another lane, upstream).

Then, in the same change that lands the fix, three answers - each a line in the fix's log entry:
1. **The class**: what kind of thing was this, in words that would name it in another place.
2. **Where else**: the other places it can bite - the commons' other devices and lanes, upstream - each
   looked up, not guessed: "also here", "not here", or "not known yet".
3. **The root**: what would make the class impossible or seen at once - a tool, a check, a source of truth,
   an upstream change - or "nothing cheaper than the fix". Where the root is an upstream's fault (a bug, a missing
   feature, a design gap), the fix goes into our fork of that upstream and is sent upstream as an item - not
   a workaround in our own tools. A workaround stands only until the fork's fix can be had, and only narrow
   (a quirk, not a broad hack); it names its item where it stands, `WORKAROUND(ITEM-ID): what it stands in
   for`, and goes when the item lands (`scripts/workarounds` finds the ones whose item has).

Who acts on them, by decision 2026-11-17-decisions-go-to-the-lane: a keeper sends the three lines to its lane
coordinator; the lane coordinator files "where else" and "the root" as items for their owners, and starts a
research (bounded, as above) only when the root is not known - one research per class: a class already open
gets the new instance added, never a second research. What spans lanes goes to orchestration; what holds for
any domain goes upstream to ideal-agent. The patrol reads the rounds' fixes across lanes and files the class
question where a fix met a trigger and no one asked it.
