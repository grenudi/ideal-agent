---
name: keep-the-start-light
description: Housekeeping of what a fresh agent must read before its first useful action - the session start, the skills' descriptions, the project's AGENTS.md, its agent folder and the role's brief - measured, held to a budget, and simplified by moving detail behind the latest summaries. Use when a patrol or orchestration run checks a project, when the start weight is over budget, or before adding text to anything every session reads.
---

# Keeping the start light

What every fresh session must read before it acts is paid by every session, every time; detail there is
read once and forgotten (practice 45-keep-the-start-light; why: research housekeeping-the-start). Housekeeping
is the upkeep of the layers a base already has - the summaries and the dated research of
ideal-knowledge-base - never a second way of making them.

## Measure

- **The start weight**, per role: `${CLAUDE_PLUGIN_ROOT}/scripts/start-weight PLACE` on a clone of the
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
  project's agent folder and front files; ideal-agent's own work for what the plugin prints).
- **By itself** housekeeping may: measure, record the numbers, file the item, write a summary that is due
  (it condenses; nothing is removed), and mark research superseded where a newer row says so.
- **It only proposes**: taking text out of a brief, AGENTS.md, a skill or the session start; moving a file;
  and any change to a practice's words - the practices are the author's.
