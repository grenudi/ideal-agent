<!-- Generated from data/ by tad. Do not edit by hand. -->

# Decisions

<a id="2026-11-17-one-home-for-agent-know-how"></a>
### 2026-11-17-one-home-for-agent-know-how

Date
: 2026-11-17

Decision
: grenudi/ideal-agent is the one home of general agent know-how in the open commons: the practices (moved from the observatory), the shared skills (from creation-guidelines/lab-agent oco-agent/skills), the general hooks (from linux-phone-oco/lab-tools' devlab guard), the general role methods (from linux-phone-oco/docs prompts/) and the research and decisions on how agents are organised. It keeps them in its own tad database and ships them as one Claude Code plugin and marketplace, so one install gives any machine, project or cloud session the same agent. Each old place is retired with a pointer here once the new one behaves the same; what is one project's domain (a device's tool, a project's records, its how-it-stands rows) stays in that project's own data, so both improve in their own place.

Why
: The author, 2026-11-17: general agent know-how was scattered over four places - live practices in the observatory, skills symlinked from lab-agent only on the lab, general hook rules inside devlab's guard, role briefs written for linux-phone - and he asked for one repository under his own account, with its own tad database and the oco mark. Research (expert-agents-written-not-standing) found expertise is best written down and read by fresh sessions; Claude Code plugins are the standard way to package skills, hooks and agents for every machine.

Rejected
: A fifth place beside the four (one nature under five names: each would drift from the others - the move is done only when every old place points here). Keeping it under creation-guidelines (the author chose his own account, grenudi: the know-how is his way of working with agents, not one guideline among others). Skills and hooks kept only on the lab (symlinks into ~/.claude/skills and hooks in ~/.claude/settings.json reach no other machine, project or cloud session, and are versioned nowhere). Domain knowledge moved here too (it would rot away from the data it is about: linux-phone's records stay in its docs database).

Research
: expert-agents-written-not-standing


<a id="2026-11-17-agents-organised-by-the-data"></a>
### 2026-11-17-agents-organised-by-the-data

Date
: 2026-11-17

Decision
: Agents are sessions with a name, a brief and places they own; their shared state is the data the projects already keep - backlog, decisions, research, session logs, measured results - read through ocoeco's lenses, not a tracker of their own. An orchestration session tends the conveyor and the agents: it checks the ways by measuring hand-work, turns every rule it can into a mechanism (a tool's hold on a device, Claude Code command hooks, CI), keeps the face fluent, and starts a research session per hard feature. Research parallelises; changes to one shared device are queued through one session. (First decided for linux-phone-oco in its docs, decision of the same id; its linux-phone parts stay there.)

Why
: The author, 2026-11-17, asked for an orchestrating agent, per-feature agents, and the most efficient organisation found by research. Read: the Craft apps' fleet (ownership by subsystem, rules kept once, an oracle gating progress), Gas Town (work state in git, agents as identities), Anthropic (few well-bounded workers, parallel only when independent, about 15 times the tokens), Claude Code (hooks enforce, sessions and schedules run) - the landscape rows photocraft-fleet, gas-town, anthropic-multi-agent, claude-code-agents. The ecosystem already held most of it; what was missing was roles, briefs and enforcement.

Rejected
: A tracker beside ocoeco such as Beads (two records of one work); an always-on watching agent (tokens without end - patrols on a schedule and hooks do it cheaper); one agent per feature implementing in parallel on one device (changes race); an agents-orchestration repository beside lab-agent then (one nature under three names - superseded by ideal-agent, the one home: 2026-11-17-one-home-for-agent-know-how).


<a id="2026-11-17-experts-written-not-standing"></a>
### 2026-11-17-experts-written-not-standing

Date
: 2026-11-17

Decision
: An expert is a domain written down in its project's data, not a session kept alive. Each hard domain is its rows - research, decisions, backlog, tests, sources - plus one research row per domain, DOMAIN-how-it-stands, that holds the tacit part (dead ends, where experiments live, what never to do again), added to item by item, never rewritten whole. A session takes up a domain by its brief and that domain's rows, and becomes its expert by reading them; it is ended when its part lands, or before its context is compacted, after writing its hand-over into those rows (not into notes outside the repository); the next starts fresh from them. Questions route by the record first: read the domain's rows; if they do not answer, message the domain's live session directly, not through orchestration; with no live session, start a short read-only consult from the rows on a lighter model, and write every answer back as a row. One writer per domain and one hand on a shared device. (First decided for linux-phone-oco in its docs, decision of the same id.)

Why
: Measured on the lab: sessions grow to 1000-1800 messages and re-read them every turn; multi-agent work costs about 15 times one agent; one device serialises all bring-up. Outside: compaction loses what only later proves to matter and summaries collapse; long contexts recall worse; relayed questions play telephone; every system that keeps experts across time (Gas Town, Anthropic's harness, Claude Code's agent memory, Letta) keeps the expertise outside the session. The author's aim - knowledge that does not depend on anyone remembering - is the first practice.

Rejected
: Standing expert sessions per domain (idle hours lose the cache, so each question re-reads a thousand-message context at full price; the expertise still vanishes at compaction; their context rots with length). Orchestration as the router of every question (a bottleneck and a game of telephone, and it fills orchestration's own context). Claude Code subagent memory or agent teams as the home (a second store beside the project's database; teams do not survive a resume). A Markdown expert-notes file per domain (a second place for what rows already hold). A vector or memory service such as Letta (outside the commons and opaque to the face). Forking a finished expert's whole session to consult it (kept as an exception for a session a few hours old whose tacit context is not yet written).

Research
: expert-agents-written-not-standing


<a id="2026-11-17-skills-in-the-plugin"></a>
### 2026-11-17-skills-in-the-plugin

Date
: 2026-11-17

Decision
: The procedures every agent follows are kept once, as Claude Code skills in this plugin (skills/): lab-session-start, land-a-change, record-the-work, change-a-device - and the general methods of the roles that were briefs for one project: orchestrate-agents, research-a-feature, bring-up-a-part. A project's brief (linux-phone-oco/docs prompts/) keeps only its domain part - what it owns, its device's tool, its places - and names the skill for the method. The skills replace creation-guidelines/lab-agent's oco-agent/skills, which were linked into the lab's ~/.claude/skills; that folder now points here. (lab-agent's decision 2026-11-17-skills-shared-by-every-agent, carried here: procedures shared as agent skills, discoverable rather than remembered.)

Why
: Read in ArtCraft's organisation (craft-skills, landscape photocraft-fleet). On 2026-11-17 four sessions worked on the lab at once and the same traps cost time more than once (a clone reset by fresh took unlanded files; rendered pages lagged a row entered after rendering; land merged a release PR from a shell without oco-work). Linked into one machine's ~/.claude/skills they reached no other machine and no cloud session; a plugin installs the same set anywhere, versioned with the know-how it carries.

Rejected
: Each brief repeating the procedures (copies drift, and a brief is read once at the start). Symlinks from a clone on the lab (one machine only). Role methods as subagents in agents/ (a role is a session of its own that lives for hours and owns places; a subagent works inside one session and returns - kept for the read-only researcher). Role briefs as template files to copy (a copy drifts; a skill is read where it is kept).


<a id="2026-11-17-hooks-in-the-plugin"></a>
### 2026-11-17-hooks-in-the-plugin

Date
: 2026-11-17

Decision
: The general ways Claude Code can hold are the plugin's hooks (hooks/hooks.json): PreToolUse on Bash refuses git push that reaches main and python/pip (exit 2, the reason given to the agent); SessionStart prints the practices and, where lab-agent is installed, the unread messages. They call ideal-agent, a small Rust program in this repository, declared with argenv; the plugin builds it once per version from the source it carries (scripts/ideal-agent, cargo) into the plugin's own data folder. The command reader and both rules are taken whole, with their tests, from devlab's guard (linux-phone-oco/lab-tools), which keeps only what is its device's: ssh, scp, rsync to a phone outside devlab.

Why
: linux-phone-oco/lab-tools #36 (2026-11-17) proved the rules live as hooks on the lab, but the push and Python rules are the commons' ways (practices 07 and 21), not a phone's: held in devlab they held only on the lab, and only where devlab was installed. Hooks are the deterministic way to hold a rule (landscape claude-code-agents); a plugin carries them to every machine.

Rejected
: The rules written in shell (a shell line read by shell is the fragile thing the Rust reader exists to avoid: quotes, sh -c, substitutions, sudo and env wrappers). Python (refused by the very rule). A prebuilt binary downloaded at install, first (needs a release pipeline; kept on the backlog as release-binaries for machines without cargo). Leaving devlab's copy of the rules in place beside the plugin's (two guards saying the same thing, drifting apart; devlab keeps only the phone's rule).


<a id="2026-11-17-practices-kept-here"></a>
### 2026-11-17-practices-kept-here

Date
: 2026-11-17

Decision
: The practices are a table of this database (data/practices.json), entered with tad r practice and exported in the mark, so the observatory gathers and shows them as ideal-agent's rows. lab-agent guide and ideal-agent start read them live from this repository's main; the observatory no longer keeps its own copy (its design/data/practices.json is retired with a pointer here).

Why
: The practices are the first thing an agent reads; kept in the observatory's design data they sat beside the observatory's own design, read by a URL hard-coded in lab-agent. In the one home of agent know-how they are kept, checked and rendered like every other row, and still seen in the observatory - which shows what places hold rather than holding it.

Rejected
: Leaving them in the observatory and pointing to them (the one home would not hold the one thing every session reads). Copying them (two lists drift - the copy was taken from the observatory's main on the day of the move, and the observatory's file is retired, not kept).


<a id="2026-11-17-one-database-no-glimpses"></a>
### 2026-11-17-one-database-no-glimpses

Date
: 2026-11-17

Decision
: ideal-agent keeps its records in its own tad database: data through tad (tad r NAME), landing through land; the commit and its pull request are the record of the work, signed by the name the session works under. No glimpses and no oco/ tables: the mark exports data/ directly.

Why
: The author asked for its own tad database. linux-phone-oco found the same way simpler on 2026-11-17 (its docs decision 2026-11-17-one-database): one tool writes the rows, git records the work.

Rejected
: ocoeco's oco/ tables beside a tad database (two tools writing rows in one place); glimpses (a second record of what the commit already says).


<a id="2026-11-17-one-keeper-per-shared-resource"></a>
### 2026-11-17-one-keeper-per-shared-resource

Date
: 2026-11-17

Decision
: The general pattern of a keeper of a shared resource is ideal-agent's: practice 44-one-keeper-per-shared-resource and the role skill keep-a-shared-resource. One owner per resource; a lasting role whose sessions are disposable (hand-over into the resource's how-it-stands record, then end); others prepare and queue items with their proofs, the keeper executes by the weakest link; the queue is a named query over the project's backlog, shown on its face; a scheduled run or the patrol starts the next keeper session when the queue has work and no keeper is live, and a lease keeps two keepers from running; the resource's own tool still holds the lock. A project keeps only its specifics - the resource, its tool, its queue's query, its lease, its schedule (linux-phone-oco: surya's phone keeper, in docs' ideal-agent-linux-phone-oco/, extending this).

Why
: The author, 2026-11-17: the general part goes upstream to ideal-agent, the linux-phone part to its orchestration session, which references it. Precedents: LAVA, CI-tron (postmarketOS's hardware CI), Gas Town's Refinery, the actor model (research keeper-of-a-shared-resource).

Rejected
: A standing keeper session kept alive (its context rots and is re-read at full price; the knowledge vanishes at compaction - decision 2026-11-17-experts-written-not-standing). Every session changing the resource when it needs to, guarded only by the tool's lock (safe but not flowing: changes race for the device and nobody chooses by the weakest link). A queue kept as its own list or tracker (a second record beside the backlog; a named query reads it off the backlog). The keeper as the lock (the tool already holds it; a keeper that crashes must not strand the resource).

Research
: keeper-of-a-shared-resource
