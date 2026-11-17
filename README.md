# Ideal agent

How an agent works in the open commons, written once: an open, transparent database of the practices every
session reads at its start, the decisions and research they rest on, and how others organise agents - and
the same know-how as one Claude Code plugin, so that any machine, project or cloud session gets the same
agent. What is one project's domain (a device, its records, how its hard parts stand) stays in that
project's own data; this place keeps only what is general.

## Install

```
/plugin marketplace add grenudi/ideal-agent
/plugin install ideal-agent@ideal-agent
```

or from a shell: `claude plugin marketplace add grenudi/ideal-agent && claude plugin install ideal-agent@ideal-agent`.
The first session start builds the hooks' program from this repository's source (Rust, with `cargo`; a few
seconds) into the plugin's data folder; until then nothing is refused.

## What it gives an agent

| | |
|---|---|
| **SessionStart hook** | names `scripts/agent-env` in the session's environment file, so every Bash command has the token, the PATH, `branch`, `land` and the work signed as AGENT_NAME - nothing of ocoeco; where `lab-agent` is installed, the unread messages; on a resume or a compaction, the session's role and practices again |
| **UserPromptSubmit hook** | at a session's first prompt, the role it names (`Role: keeper`) - its card and only its practices, live from this repository's main (`data/practices.json`) - or every practice when it names none |
| **PreToolUse hook** (Bash) | refuses a `git push` that reaches main, `tad sql` that writes (`ikb set` and `ikb remove` instead), and Python (`python`, `pip` and their kin) - in every simple command of a line, `sh -c` and `$(...)` included - and says why |
| **skills/** | `lab-session-start`, `land-a-change`, `record-the-work`, `change-a-device`; the methods of the roles: `orchestrate-agents`, `research-a-feature`, `bring-up-a-part`, `keep-a-shared-resource`; finding your way in a tad knowledge base: `ask-a-knowledge-base`; before acting on a proposal: `look-around-first`; keeping what a fresh agent reads light: `keep-the-start-light` (measured by `scripts/start-weight` and, from a transcript, `scripts/read-weight`) |
| **agents/** | the roles, one card each - `coordinator`, `lane-coordinator`, `orchestrator`, `keeper`, `research-lead`, `builder`, `patrol`: purpose, what it owns and never does, what it loads, its model and effort, how it hands over, whom it reports to, and its practices; and `researcher`, a read-only subagent for one question |

A project keeps only its domain - what a role owns there, its device's tool, its places - in its agent folder
(`ideal-agent-PROJECT/`, or the folder its AGENTS.md names). Each brief names its role on a line under its
title - `Role: keeper (ideal-agent)`, and `Also practices: 42` for a domain's own - and the prompt that
starts a session begins with that line; the card is the role, the brief only what the project adds to it.
A `Name: surya-keeper-6` line names who signs the work (AGENT_NAME; else the role). A session can also be
started as the role itself: `claude --agent ideal-agent:keeper`.

## The database

Kept with [tad](https://github.com/creation-guidelines/tad-engine): tables declared in `tables.toml`, one
JSON object per line in `data/`, every row checked, pages rendered into `docs/`.

| table | what |
|---|---|
| [practices](docs/practices.md) | how an agent works, why, and for whom - what every session reads first |
| [decisions](docs/decisions.md) | each decision with what was rejected |
| [research](docs/research.md), [sources](docs/sources.md) | what was learned, and from where |
| [landscape](docs/landscape.md) | how others organise agents, and what we take from it |
| [backlog](docs/backlog.md), [session_log](docs/session_log.md) | the work ahead, and how it went |
| [subjects](docs/subjects.md) | what each record is about: `tad q about --subject S` gives a subject's whole knowledge |

The base follows [ideal-knowledge-base](https://github.com/grenudi/ideal-knowledge-base) (`idealkb.toml`
pins its release): ask it with `tad q about`, `next`, `find`, `row`, `recent` - see AGENTS.md.

```
make setup     # tad from its pinned release
tad q practices; tad q about --subject keeper; tad q next
tad r practice --id NN-a-name --practice ... --why ... --for agents   # tad statements lists every record
make verify    # what CI runs
```

Nothing here depends on ocoeco. The mark (`oco.toml`) and `.github/workflows/gather.yml` only let the
author's observatory read these tables, a passive feed.

## Where things were before

The practices were the observatory's (`open-commons-observatory.github.io`, `design/data/practices.json`);
the skills were `creation-guidelines/lab-agent`'s `oco-agent/skills/`, linked into the lab's
`~/.claude/skills`; the push and Python rules were in `linux-phone-oco/lab-tools`' `devlab guard`, which
keeps only its phone's rule; the role methods were briefs in `linux-phone-oco/docs` `prompts/`, whose
domain part is now its `ideal-agent-linux-phone-oco/` folder. Why it moved: [decision 2026-11-17-one-home-for-agent-know-how](docs/decisions.md).

## Licence

MIT.
