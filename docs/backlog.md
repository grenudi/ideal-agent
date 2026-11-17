<!-- Generated from data/ by tad. Do not edit by hand. -->

# Backlog

<a id="release-binaries"></a>
### Prebuilt ideal-agent binaries, so the hooks hold on a machine without cargo

Status
: open

Note
: The plugin builds ideal-agent from its source with cargo at the first session start (scripts/ideal-agent); a machine or cloud session without Rust gets no guard and is told so. Next: a release workflow (release-plz and musl builds, as tad-engine has) and scripts/ideal-agent fetching the pinned release with its checksum before falling back to cargo.

About
: hooks

Opened
: 2026-11-17


<a id="guard-stop-unlanded"></a>
### A Stop hook: a turn does not end with unlanded changes in a place the session worked in

Status
: open

Note
: Was lab-tools' backlog guard-stop-unlanded (devlab guard); general to every project, so it belongs to the plugin's hooks. Next: decide what counts as a place the session worked in (the cwd's repository and the worktrees it made) and prove it against its own event.

About
: hooks

Opened
: 2026-11-17


<a id="session-start-size"></a>
### The session start says less: its output is about 20 KB

Status
: done

Note
: Done 2026-11-17: the session start prints each practice's words without its reason (22.5 KB to 13.9 KB, measured on main with scripts/start-weight); the reasons are one command away (lab-agent guide, scripts/ideal-agent practices). Kept by practice 45-keep-the-start-light.

About
: housekeeping

Opened
: 2026-11-17


<a id="devlab-reads-commands-here"></a>
### devlab's guard reads shell commands with ideal-agent's reader instead of its own copy

Status
: done

Note
: Done 2026-11-17 by orchestration: devlab's guard uses ideal_agent::shell::{simple_commands, unwrapped}, pinned in devlab/Cargo.toml at rev 5967ea8 (read back from lab-tools' main); its own copy is gone. A quoted here-document naming the phone's address passes, a real ssh there is refused. When the reader changes here, devlab takes it by bumping the rev.

About
: hooks

Opened
: 2026-11-17


<a id="move-in"></a>
### Move the general agent know-how here and retire the old places with a pointer

Status
: done

Note
: Done 2026-11-17, each read back from main: ideal-agent #1 and #2; lab-tools #65 (devlab guard keeps only the phone's rule; installed; the lab's settings keep only its PreToolUse hook); lab-agent #218 (guide reads the practices here, built-in copy refreshed, oco-agent/skills retired with a pointer; installed); docs #159 (briefs keep their domain and name the skills; decisions for lab-tools and docs); marshall-oco #32 and ecosystem #159 gather ideal-agent; ocoeco #456 and ecosystem #160 repoint the links to the practices; the observatory's design/data/practices.json retired with a pointer. The plugin is installed on the lab (user scope) and proven live in a fresh session: practices and inbox at start, push to main and pip refused by ideal-agent guard, a branch push and grep python allowed, ssh to the phone refused by devlab guard, a quoted here-document allowed, all eight skills and the researcher listed. The old ~/.claude/skills links are in ~/.claude/backups/skills-links-before-ideal-agent.

About
: ideal-agent

Opened
: 2026-11-17


<a id="follow-ideal-knowledge-base"></a>
### ideal-agent follows grenudi/ideal-knowledge-base, as its second follower

Status
: done

Note
: Done 2026-11-17: follows idealkb 0.1.0 (idealkb.toml), about on every record, subjects table with references rules (decision 2026-11-17-follows-ideal-knowledge-base).

About
: knowledge-base

Opened
: 2026-11-17


<a id="sources-by-back"></a>
### Move the sources' 'Made by ...' back into by, once idealkb v0.3.0 is pinned

Status
: done

Note
: Done 2026-11-17: idealkb 0.3.0 is pinned and declares by; the three 'Made by ...' prefixes are back in by (lava-scheduler, ci-tron-introduction, pmos-hw-ci-status).

Waits
: the idealkb pin moving to 0.3.0

About
: knowledge-base

Opened
: 2026-11-17


<a id="practices-short-at-the-start"></a>
### Proposed to the author: the session start shows each practice's first sentence only

Status
: done

Note
: Done 2026-11-17 at the author's word: the session start prints first sentences (decision 2026-11-17-the-start-reads-first-sentences).

Waits
: the author

About
: housekeeping

Opened
: 2026-11-17


<a id="measure-start-without-connectors"></a>
### Measure the next keeper's first turn without Gmail and Claude Docs

Status
: open

Note
: Before: surya-keeper-2 (started by hand) first turn 81,280 tokens, 33.5 KB read before acting. A first 'after' (waker run 45833ff5, 12:33 UTC): first turn 75,649, 39.1 KB read before acting (devlab start 10.8 KB, the phone queue asked 5 times ~13 KB, a tad sql 5.1 KB), first act 26 min in (a permission prompt). Not yet comparable: that run began before Gmail and Claude Docs were turned off, and the scheduled runs measured earlier started at 73.5-75.6k too - a scheduled run's system part is smaller than a hand-started session's. Next: a keeper started after the connector change, by the same route as its before, measured with read-weight.

About
: housekeeping

Opened
: 2026-11-17


<a id="measure-a-role-start"></a>
### Measure a role-named session's first turn with read-weight on plugin 0.3.0

Status
: in-progress

Note
: keeper-6 had no Role line (every practice at the first prompt): 80,864 tokens, as before. The clean after is surya-keeper-7, started with Role: keeper; orchestration measures it.

Waits
: plugin 0.3.0 installed on the lab and a session started with a Role line

About
: housekeeping

Opened
: 2026-11-17


<a id="briefs-name-their-role"></a>
### linux-phone's briefs name their role and drop what the card says

Status
: done

Note
: docs #230: every brief names its role, Name and Also practices; trimming is orchestration's.

Waits
: orchestration's word

About
: organising-agents

Opened
: 2026-11-17


<a id="briefs-drop-oco-work"></a>
### linux-phone's briefs drop . oco-work and OCOECO_BY for Role and Name lines

Status
: done

Note
: docs #230: no brief says . oco-work, OCOECO_BY or landit.

Waits
: orchestration's word

About
: organising-agents

Opened
: 2026-11-17


<a id="skills-listed-per-role"></a>
### Which skills and plugins a role's session lists at its start

Status
: open

Note
: Measured (read-weight given, not used; keeper-7, surya lane, research lead): Anthropic's 16 skills (9,976 B) and Claude Code's 17 (8,079 B) are used by none. ideal-agent's descriptions cut to 2,287 B (0.3.7). Next: the author's word on skillOverrides name-only for the unused ones, or skillListingMaxDescChars ~250 (about 3-4k tokens off every session).

Waits
: the author's word on his settings

About
: housekeeping

Opened
: 2026-11-17
