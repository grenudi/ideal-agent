---
name: lab-session-start
description: The start of every session: your role's card and practices, your brief's first paragraph, your task's subject in summary, the inbox - then act; read the rest only when a step needs it.
---

# Starting a session

Your context is for the work, not for briefings: a session already holds tens of thousands of tokens before
it reads anything. Read at the start only what your current task needs; everything else stays one query or
one file away, named but not read (practice 45-keep-the-start-light; ideal-knowledge-base's brief-first).

## Read at the start - only this

1. **Your role and its practices** - the first prompt brings them: the role your brief names (`Role: NAME`, a
   card in the plugin's agents/ - purpose, owns, never, loads, hand-over, reports to) and only that role's
   practices, each its first sentence; with no role named, every practice. Each whole, with its reason, only
   when one bears on a choice you are making: `lab-agent guide` (or `scripts/ideal-agent practices`).
2. **Your brief's first paragraph** - what the project adds to the role (its agent folder,
   `ideal-agent-PROJECT/`). The rest of the brief only when a step of yours needs it.
3. **Your task's subject, in summary** - `tad q about --subject S` (it leads with the summary and gives each
   record by its brief), or the top of your queue (`tad q next --about S`, a role's queue query). That is
   enough to act.
4. **Your shell is ready** - the session's start sources scripts/agent-env before every Bash command: the
   token, the PATH, `branch` and `land`, and your work signed as AGENT_NAME (the brief's `Name:` line, else
   your role). A job outside Claude Code sources it itself (`. ~/Projects/grenudi/ideal-agent/scripts/agent-env`). **The inbox** - `lab-agent inbox`:
   answer each unread message, then `--ack`.

Then act.

## Read only when

- **a record's whole** - only when its brief is not enough for the step in hand: `tad q row --id ID`, or
  `tad q about --subject S --whole true`;
- **another subject** - only when your task reaches it: `tad q about --subject OTHER`;
- **the rest of your brief, a skill's body** - only when you come to the step it is for (land-a-change when
  you land, change-a-device before you change a device, record-the-work when you record);
- **a tool's help** - only when the brief or skill does not name the command you need;
- **what was recorded lately, the board, the face** - only when choosing what to take up next, not to start
  a task already given;
- **a research's material, a file past its first paragraph** - only when the finding's brief leaves your
  question open.

New sessions start without Gmail and Claude Docs (the author, 2026-11-17: their tools are carried at every
start). If your task needs either, ask the author to turn it on for your session - the composer's + menu,
Connectors. Firecrawl, for research, stays on.

Choose by the weakest link the data shows, not by momentum; say on the board what you take up.
