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
| **SessionStart hook** | the practices, live from this repository's main (`data/practices.json`), and - where `lab-agent` is installed - the unread messages |
| **PreToolUse hook** (Bash) | refuses a `git push` that reaches main, and Python (`python`, `pip` and their kin) - in every simple command of a line, `sh -c` and `$(...)` included - and says why |
| **skills/** | `lab-session-start`, `land-a-change`, `record-the-work`, `change-a-device`; the methods of the roles: `orchestrate-agents`, `research-a-feature`, `bring-up-a-part`, `keep-a-shared-resource`; finding your way in a tad knowledge base: `ask-a-knowledge-base`; before acting on a proposal: `look-around-first` |
| **agents/** | `researcher`: read-only research on one question, returned with its sources |

A project keeps only its domain - what a role owns, its device's tool, its places - in its agent folder
(`ideal-agent-PROJECT/`, or the folder its AGENTS.md names), whose briefs name the skill for each method.

## The database

Kept with [tad](https://github.com/creation-guidelines/tad-engine): tables declared in `tables.toml`, one
JSON object per line in `data/`, every row checked, pages rendered into `docs/`. It follows
[grenudi/ideal-knowledge-base](https://github.com/grenudi/ideal-knowledge-base) at the release `idealkb.toml`
names: ask it with `tad q about --subject S` (S a practice, as `22-look-around-first`), `next`, `find`, `row`,
`recent`; research material lives in `research/SUBJECT/YYYY-MM-DD/`.

| table | what |
|---|---|
| [practices](docs/practices.md) | how an agent works, why, and for whom - what every session reads first |
| [decisions](docs/decisions.md) | each decision with what was rejected |
| [research](docs/research.md), [sources](docs/sources.md) | what was learned, and from where |
| [landscape](docs/landscape.md) | how others organise agents, and what we take from it |
| [backlog](docs/backlog.md), [session_log](docs/session_log.md) | the work ahead, and how it went |

```
make setup     # tad from its pinned release
tad q practices
tad r practice --id NN-a-name --practice ... --why ... --for agents   # tad statements lists every record
make verify    # what CI runs
```

The observatory gathers these tables by this repository's mark (`oco.toml`) and shows them as its rows.

## Where things were before

The practices were the observatory's (`open-commons-observatory.github.io`, `design/data/practices.json`);
the skills were `creation-guidelines/lab-agent`'s `oco-agent/skills/`, linked into the lab's
`~/.claude/skills`; the push and Python rules were in `linux-phone-oco/lab-tools`' `devlab guard`, which
keeps only its phone's rule; the role methods were briefs in `linux-phone-oco/docs` `prompts/`, whose
domain part is now its `ideal-agent-linux-phone-oco/` folder. Why it moved: [decision 2026-11-17-one-home-for-agent-know-how](docs/decisions.md).

## Licence

MIT.
