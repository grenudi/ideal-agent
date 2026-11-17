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
: open

Note
: Seen by orchestration on 2026-11-17: whether Claude Code keeps all of it in context is unverified. Next: look at a new session's context, and if it is cut, print the practices' sentences without their reasons (the reasons stay one command away).

About
: hooks

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
