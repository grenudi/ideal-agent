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
: Before (surya-keeper-2, transcript 3060e909, started by hand before the lean keeper brief, the waker change and connectors off): first turn 81,280 tokens, first act 92,176, 33.5 KB read before it (tad --help 12.1 KB, deep-sleep whole 9.4, oco-work 4.6, a mistyped about 4.0), peak 118,468 in 84 turns. After: the linux-phone patrol runs read-weight on every transcript of its last 6 hours (its item 6); its first round after the next keeper gives the number. Next: record it here and close.

About
: housekeeping

Opened
: 2026-11-17
