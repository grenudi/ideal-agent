<!-- Generated from data/ by tad. Do not edit by hand. -->

# Backlog

<a id="release-binaries"></a>
### Prebuilt ideal-agent binaries, so the hooks hold on a machine without cargo

Status
: open

Note
: The plugin builds ideal-agent from its source with cargo at the first session start (scripts/ideal-agent); a machine or cloud session without Rust gets no guard and is told so. Next: a release workflow (release-plz and musl builds, as tad-engine has) and scripts/ideal-agent fetching the pinned release with its checksum before falling back to cargo.

Opened
: 2026-11-17


<a id="guard-stop-unlanded"></a>
### A Stop hook: a turn does not end with unlanded changes in a place the session worked in

Status
: open

Note
: Was lab-tools' backlog guard-stop-unlanded (devlab guard); general to every project, so it belongs to the plugin's hooks. Next: decide what counts as a place the session worked in (the cwd's repository and the worktrees it made) and prove it against its own event.

Opened
: 2026-11-17


<a id="session-start-size"></a>
### The session start says less: its output is about 20 KB

Status
: open

Note
: Seen by orchestration on 2026-11-17: whether Claude Code keeps all of it in context is unverified. Next: look at a new session's context, and if it is cut, print the practices' sentences without their reasons (the reasons stay one command away).

Opened
: 2026-11-17


<a id="devlab-reads-commands-here"></a>
### devlab's guard reads shell commands with ideal-agent's reader instead of its own copy

Status
: open

Note
: The command reader (src/shell.rs) was taken whole from devlab's guard, which keeps a copy for its phone rule. ideal-agent's has since learned here-documents (their body is data): devlab's copy still reads a quoted here-document's text as commands, so a file whose text names a phone address would be refused. Next: devlab depends on the ideal-agent crate (git dependency at a tag) and drops its copy - orchestration's, as devlab's owner.

Opened
: 2026-11-17


<a id="move-in"></a>
### Move the general agent know-how here and retire the old places with a pointer

Status
: in-progress

Note
: Landing the repository first (mark, database, plugin). Then, each beside the old way until both behave the same: install the plugin on the lab and prove each hook and skill; lab-agent guide reads the practices here and the observatory's copy is retired; devlab guard keeps only the phone's rule and the lab's settings drop its start hook; lab-agent's oco-agent/skills and the ~/.claude/skills links point here; docs' prompts keep their domain part and name the skills.

Opened
: 2026-11-17
