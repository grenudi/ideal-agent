---
name: keep-the-start-light
description: What a fresh agent reads and what a working one holds - measured (start-weight, read-weight: given, not used), budgeted, trimmed by its owner; renewing a long-lived role. Use at a start, before a large output, past 150k tokens.
---

# Keeping the start light

What every fresh session must read before it acts is paid by every session, every time; detail there is
read once and forgotten (practice 45-keep-the-start-light; why: research housekeeping-the-start). Housekeeping
is the upkeep of the layers a base already has - the summaries and the dated research of
ideal-knowledge-base - never a second way of making them.

## Measure

- **What a started session read**: the plugin's `scripts/read-weight TRANSCRIPT` on its transcript - the context it
  held at its first turn, at its first act and at its end, in tokens, the largest things it read before
  acting, what it was given and never used, and what it struggled with. The first turn is mostly the
  harness's own instructions, its skill listing and the tools of every connector the session carries (about
  75 thousand tokens on the lab, 2026-11-17): what a role does not use is turned off in the author's settings,
  proposed to him, not changed. Where the transcripts are and which settings trim what: ideal-agent's base,
  the harness's subject (`tad q about --subject claude-code`).
- **The start weight**, per role: the plugin's `scripts/start-weight PLACE` on a clone of the
  project's base at its main - the session start, the skills' descriptions, lab-session-start, AGENTS.md,
  the agent folder's README, and each brief. Record it, dated, in the project's base.
- **The answers**: the base's fixed questions (idealkb's measure of a base, `the-measure`) - the bytes
  `tad q about --subject S` and `tad q next` return for the subjects the roles work on, and whether
  `tad q about` says a summary is due.
- **A cold read**, now and then: give a fresh agent only the front files and the brief and ask what it
  must do first, and how it lands a change. Where it cannot say, a pointer is missing - not more text.

## The budget that calls for housekeeping

- a role's start weight over **32 KB** (about 8 thousand tokens), or grown by a fifth since it was last
  measured;
- AGENTS.md or the agent folder's README over **3 KB**; a brief over **8 KB**;
- a subject whose `tad q about` says a summary is due (idealkb's rule: over 8 KB of records with none, or
  over 4 KB recorded since it).

## Simplify

- **Move detail behind a summary.** What a brief or front file says at length about one subject goes into
  that subject's records, and the brief keeps one line and the query: `tad q about --subject S`. Write the
  summary with the base's own record (`tad r summary`) when one is due.
- **Keep what routes, drop what repeats.** The front files say what is where and what to ask; a method
  belongs to its skill, a fact to its row. What two files both say, one of them points to.
- **Supersede, do not delete.** Old research is marked superseded by the newer row (`supersedes`), so the
  answer leaves it while find and row still reach it.

## Who, how often, and what may be done without asking

- **Measuring is a duty of the project's patrol**, every run: cheap, read-only, and the number is
  recorded where the face can draw it. Over budget, the patrol files one backlog item, about the agent
  folder, with the numbers.
- **The simplifying is the owner's**: the session that owns the file takes the item (orchestration for a
  project's agent folder and front files; ideal-agent's owner for what the plugin prints and the role cards;
  the author for his settings - the skills and connectors every session lists). No standing role does it
  (decision 2026-11-17-start-health-folds-into-housekeeping).
- **By itself** housekeeping may: measure, record the numbers, file the item, write a summary that is due
  (it condenses; nothing is removed), and mark research superseded where a newer row says so.
- **It only proposes**: taking text out of a brief, AGENTS.md, a skill or the session start; moving a file;
  and any change to a practice's words - the practices are the author's.

## Hold little while you work

A session already holds about 75 thousand tokens before it reads anything (research
what-a-started-agent-reads), and every turn re-reads all it holds. So:

- **Name the exact command, and limit its output**: `--format csv`, `| head -40`, `tad q row --id` rather than
  a table; never a bare `--help` (surya's second keeper spent 12 KB on one) and never a whole table or log.
- **Heavy reading goes to a subagent** that returns only what you asked for, in so many characters: a long
  transcript, a vendor tree, a set of papers. Its reading never enters your context.
- **Skills and connectors only when needed**: a skill's body when you reach its step; a connector (mail,
  documents) only when the task asks for it - ask the author to turn it on.
- **Model and effort by role**: a keeper, a patrol or a waker on a lighter model at medium or low effort; a
  research turning point or a design decision on the most capable; check what the models are before saying so.

## How well agents are brought up

Orchestration and the coordinator check it now and then (each patrol run measures the start weight):
`scripts/read-weight` on the transcripts of the sessions started since the last look - the tokens at the
first turn, at the first act, and the largest reads before it. Then "given, not used": the skills, agents,
deferred tools and connectors listed and never used in the whole run, and the first prompt's role and
practices - by source, in bytes. What a role is given or reads and does not need is a sign: a skill or
connector no session of that role used, a brief that does not name a command or its own path, a subject
without a summary, a practice the role never meets. File it, naming the role, where its owner keeps work, as
for the start weight; over a role's budget, the patrol files it at once.

## Renew a long-lived role

A coordinator, an orchestration session or a keeper lives for hours; its context grows with every turn and
is re-read in full each time (on 2026-11-17 the coordinator held 840k tokens after 1,222 turns and had
re-read 574 million). Compaction keeps the session but summarises it blindly; a hand-over keeps what
matters, written where the next session reads it. Any session does this to itself:

**When** - the first of:
- its context passes **250 thousand tokens** (about three times a fresh start);
- a batch of its work has landed and its context is over **150 thousand**;
- it is about to resume after **an idle hour** with over 150 thousand (the cache has gone cold: the next turn
  pays for all of it again).

**How**:
1. **Write the hand-over** into the role's record in its base: a research row `ROLE-how-it-stands`, with a
   brief, superseding the previous one - its role in two lines, the sessions it relies on, what is in flight
   and its next step, what waits on the author, the author's standing words, what not to repeat. Land it and
   read it back. Records of decisions and work stay where they are; the hand-over names them by id.
2. **Say how the role starts fresh**: one line the author (or the session, starting it itself) gives a new
   session - read the first paragraph of the role's brief and `tad q row --id ROLE-how-it-stands`, nothing
   else until a step needs it.
3. **Hand the conversation over**: tell the author it is written and where; the old session ends when the new
   one has taken up the work (the author archives it).
