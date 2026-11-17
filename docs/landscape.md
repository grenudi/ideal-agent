<!-- Generated from data/ by tad. Do not edit by hand. -->

# Landscape

<a id="photocraft-fleet"></a>
### ArtCraft: seventeen apps, one organisation

Project
: ArtCraft (storytold; getartcraft.com)

What
: Seven, then thirteen clean-room Rust apps built with about 8 parallel Claude agents, each owning one subsystem directory. House rules kept once (craft-contrib/standards) and stamped into every repository's AGENTS.md with a checksum; CI shared from one actions repository; agents check an issue and claim it before working; a generated scorecard, ranked gaps list and parity checklists say what is left; a test oracle (Photoshop's own renders, a CPU compositor for the GPU one) decides what counts as progress; remaining work estimated in agent-hours.

For us
: The same shape fits us, almost one to one: craft-contrib (rules and RFCs, stamped into every AGENTS.md) is creation-guidelines and the practices; craft-common-actions (CI called, not copied) is ocoeco's gather and the package conveyor's workflows; craft-launcher (installs and updates every app) is the package conveyor's Alpine repository to come; craft-repo-status-app and craft-wiki are the face and docs; every app agent-ready (CLI, JSON control, MCP) is devlab mcp and agent access. What they have and we lack: craft-skills - procedures shared as agent skills, discoverable by every agent rather than remembered (our oco-work words, as skills); and *-corpus repositories - reference data an oracle compares against (for a phone: what Android does on the same hardware - sleep current, audio levels, sensor readings - measured once and kept). Worth taking too: claim before work, estimates in agent-hours, a ranked gaps list and scorecard drawn from data, and the lesson that breadth is fast and only an oracle keeps the numbers honest.

Link
: https://github.com/storytold/photocraft/blob/main/AGENTS.md

Checked
: 2026-11-17


<a id="gas-town"></a>
### Gas Town and Beads

Project
: Steve Yegge

What
: An orchestrator for 20-30 coding agents: work state lives in git as JSON-lines issues (Beads), not in agents' memory; an agent is a persistent identity, not a session; workflows are chains of small issues an agent walks one at a time, so a crash or a full context loses nothing; supervisor roles patrol with backoff; its author says not to use it unless juggling five or more agents daily.

For us
: Our backlog, decisions and session logs in tad/ocoeco data are already the git-kept work state; ocoeco's 'where the drivers stand' is the identity view. Not to adopt: a second tracker beside ocoeco would be one nature under two names. To take: an agent is a name (OCOECO_BY) with a history in the data, and work is small steps that survive a cut-off session.

Link
: https://steve-yegge.medium.com/welcome-to-gas-town-4f25ee16dd04

Checked
: 2026-11-17


<a id="anthropic-multi-agent"></a>
### Anthropic's orchestrator-worker research

Project
: Anthropic

What
: A lead agent plans and starts 3-5 workers in parallel, each with its own context, an explicit objective, output format and boundary ('not X - that is another worker's job'); about 15 times the tokens of one agent; worth it only for work that is truly independent and valuable; later guidance argues most systems should have fewer, better-scoped agents.

For us
: Parallelise research (it is independent and reads only); serialise what touches the one phone; write each brief with its boundary; spend agents where the value is, and say the cost.

Link
: https://www.anthropic.com/engineering/multi-agent-research-system

Checked
: 2026-11-17


<a id="claude-code-agents"></a>
### Claude Code: sessions, teams, hooks, schedules

Project
: Anthropic

What
: Background sessions with cross-session messages (the desktop app), subagents inside one session, experimental agent teams with a shared task list claimed by file lock, worktrees to keep writers apart, hooks that run commands at fixed points (PreToolUse, Stop, SessionStart) and can block - the deterministic way to enforce a rule - and scheduled runs: desktop tasks on the lab, cloud routines, GitHub Actions.

For us
: Enforce the ways with command hooks on the lab rather than by an agent watching; start sessions from versioned briefs (prompts/); run patrols on a schedule rather than an agent always on.

Link
: https://code.claude.com/docs/en/agents

Checked
: 2026-11-17
