<!-- Generated from data/ by tad. Do not edit by hand. -->

# Decisions

<a id="2026-11-17-one-home-for-agent-know-how"></a>
### 2026-11-17-one-home-for-agent-know-how

Date
: 2026-11-17

Brief
: grenudi/ideal-agent is the one home of general agent know-how, a tad database and one Claude Code plugin; each project keeps its domain.

Decision
: grenudi/ideal-agent is the one home of general agent know-how in the open commons: the practices (moved from the observatory), the shared skills (from creation-guidelines/lab-agent oco-agent/skills), the general hooks (from linux-phone-oco/lab-tools' devlab guard), the general role methods (from linux-phone-oco/docs prompts/) and the research and decisions on how agents are organised. It keeps them in its own tad database and ships them as one Claude Code plugin and marketplace, so one install gives any machine, project or cloud session the same agent. Each old place is retired with a pointer here once the new one behaves the same; what is one project's domain (a device's tool, a project's records, its how-it-stands rows) stays in that project's own data, so both improve in their own place.

Why
: The author, 2026-11-17: general agent know-how was scattered over four places - live practices in the observatory, skills symlinked from lab-agent only on the lab, general hook rules inside devlab's guard, role briefs written for linux-phone - and he asked for one repository under his own account, with its own tad database and the oco mark. Research (expert-agents-written-not-standing) found expertise is best written down and read by fresh sessions; Claude Code plugins are the standard way to package skills, hooks and agents for every machine.

Rejected
: A fifth place beside the four (one nature under five names: each would drift from the others - the move is done only when every old place points here). Keeping it under creation-guidelines (the author chose his own account, grenudi: the know-how is his way of working with agents, not one guideline among others). Skills and hooks kept only on the lab (symlinks into ~/.claude/skills and hooks in ~/.claude/settings.json reach no other machine, project or cloud session, and are versioned nowhere). Domain knowledge moved here too (it would rot away from the data it is about: linux-phone's records stay in its docs database).

About
: ideal-agent

Research
: expert-agents-written-not-standing


<a id="2026-11-17-agents-organised-by-the-data"></a>
### 2026-11-17-agents-organised-by-the-data

Date
: 2026-11-17

Brief
: Agents are named sessions with briefs; their shared state is the projects' data, not a tracker; research runs in parallel, a shared device is queued.

Decision
: Agents are sessions with a name, a brief and places they own; their shared state is the data the projects already keep - backlog, decisions, research, session logs, measured results - read through ocoeco's lenses, not a tracker of their own. An orchestration session tends the conveyor and the agents: it checks the ways by measuring hand-work, turns every rule it can into a mechanism (a tool's hold on a device, Claude Code command hooks, CI), keeps the face fluent, and starts a research session per hard feature. Research parallelises; changes to one shared device are queued through one session. (First decided for linux-phone-oco in its docs, decision of the same id; its linux-phone parts stay there.)

Why
: The author, 2026-11-17, asked for an orchestrating agent, per-feature agents, and the most efficient organisation found by research. Read: the Craft apps' fleet (ownership by subsystem, rules kept once, an oracle gating progress), Gas Town (work state in git, agents as identities), Anthropic (few well-bounded workers, parallel only when independent, about 15 times the tokens), Claude Code (hooks enforce, sessions and schedules run) - the landscape rows photocraft-fleet, gas-town, anthropic-multi-agent, claude-code-agents. The ecosystem already held most of it; what was missing was roles, briefs and enforcement.

Rejected
: A tracker beside ocoeco such as Beads (two records of one work); an always-on watching agent (tokens without end - patrols on a schedule and hooks do it cheaper); one agent per feature implementing in parallel on one device (changes race); an agents-orchestration repository beside lab-agent then (one nature under three names - superseded by ideal-agent, the one home: 2026-11-17-one-home-for-agent-know-how).

About
: organising-agents


<a id="2026-11-17-experts-written-not-standing"></a>
### 2026-11-17-experts-written-not-standing

Date
: 2026-11-17

Brief
: An expert is a domain written down in its records, not a session kept alive: sessions read the rows, hand over into them, and end.

Decision
: An expert is a domain written down in its project's data, not a session kept alive. Each hard domain is its rows - research, decisions, backlog, tests, sources - plus one research row per domain, DOMAIN-how-it-stands, that holds the tacit part (dead ends, where experiments live, what never to do again), added to item by item, never rewritten whole. A session takes up a domain by its brief and that domain's rows, and becomes its expert by reading them; it is ended when its part lands, or before its context is compacted, after writing its hand-over into those rows (not into notes outside the repository); the next starts fresh from them. Questions route by the record first: read the domain's rows; if they do not answer, message the domain's live session directly, not through orchestration; with no live session, start a short read-only consult from the rows on a lighter model, and write every answer back as a row. One writer per domain and one hand on a shared device. (First decided for linux-phone-oco in its docs, decision of the same id.)

Why
: Measured on the lab: sessions grow to 1000-1800 messages and re-read them every turn; multi-agent work costs about 15 times one agent; one device serialises all bring-up. Outside: compaction loses what only later proves to matter and summaries collapse; long contexts recall worse; relayed questions play telephone; every system that keeps experts across time (Gas Town, Anthropic's harness, Claude Code's agent memory, Letta) keeps the expertise outside the session. The author's aim - knowledge that does not depend on anyone remembering - is the first practice.

Rejected
: Standing expert sessions per domain (idle hours lose the cache, so each question re-reads a thousand-message context at full price; the expertise still vanishes at compaction; their context rots with length). Orchestration as the router of every question (a bottleneck and a game of telephone, and it fills orchestration's own context). Claude Code subagent memory or agent teams as the home (a second store beside the project's database; teams do not survive a resume). A Markdown expert-notes file per domain (a second place for what rows already hold). A vector or memory service such as Letta (outside the commons and opaque to the face). Forking a finished expert's whole session to consult it (kept as an exception for a session a few hours old whose tacit context is not yet written).

About
: organising-agents

Research
: expert-agents-written-not-standing


<a id="2026-11-17-skills-in-the-plugin"></a>
### 2026-11-17-skills-in-the-plugin

Date
: 2026-11-17

Brief
: The shared procedures and the methods of the roles are skills in the plugin; a project's brief keeps only its domain and names the skill.

Decision
: The procedures every agent follows are kept once, as Claude Code skills in this plugin (skills/): lab-session-start, land-a-change, record-the-work, change-a-device - and the general methods of the roles that were briefs for one project: orchestrate-agents, research-a-feature, bring-up-a-part. A project's brief (linux-phone-oco/docs prompts/) keeps only its domain part - what it owns, its device's tool, its places - and names the skill for the method. The skills replace creation-guidelines/lab-agent's oco-agent/skills, which were linked into the lab's ~/.claude/skills; that folder now points here. (lab-agent's decision 2026-11-17-skills-shared-by-every-agent, carried here: procedures shared as agent skills, discoverable rather than remembered.)

Why
: Read in ArtCraft's organisation (craft-skills, landscape photocraft-fleet). On 2026-11-17 four sessions worked on the lab at once and the same traps cost time more than once (a clone reset by fresh took unlanded files; rendered pages lagged a row entered after rendering; land merged a release PR from a shell without oco-work). Linked into one machine's ~/.claude/skills they reached no other machine and no cloud session; a plugin installs the same set anywhere, versioned with the know-how it carries.

Rejected
: Each brief repeating the procedures (copies drift, and a brief is read once at the start). Symlinks from a clone on the lab (one machine only). Role methods as subagents in agents/ (a role is a session of its own that lives for hours and owns places; a subagent works inside one session and returns - kept for the read-only researcher). Role briefs as template files to copy (a copy drifts; a skill is read where it is kept).

About
: skills


<a id="2026-11-17-hooks-in-the-plugin"></a>
### 2026-11-17-hooks-in-the-plugin

Date
: 2026-11-17

Brief
: The general hooks are the plugin's: no push to main, no Python, the practices at start - a Rust program built by the plugin; devlab keeps the phone's rule.

Decision
: The general ways Claude Code can hold are the plugin's hooks (hooks/hooks.json): PreToolUse on Bash refuses git push that reaches main and python/pip (exit 2, the reason given to the agent); SessionStart prints the practices and, where lab-agent is installed, the unread messages. They call ideal-agent, a small Rust program in this repository, declared with argenv; the plugin builds it once per version from the source it carries (scripts/ideal-agent, cargo) into the plugin's own data folder. The command reader and both rules are taken whole, with their tests, from devlab's guard (linux-phone-oco/lab-tools), which keeps only what is its device's: ssh, scp, rsync to a phone outside devlab.

Why
: linux-phone-oco/lab-tools #36 (2026-11-17) proved the rules live as hooks on the lab, but the push and Python rules are the commons' ways (practices 07 and 21), not a phone's: held in devlab they held only on the lab, and only where devlab was installed. Hooks are the deterministic way to hold a rule (landscape claude-code-agents); a plugin carries them to every machine.

Rejected
: The rules written in shell (a shell line read by shell is the fragile thing the Rust reader exists to avoid: quotes, sh -c, substitutions, sudo and env wrappers). Python (refused by the very rule). A prebuilt binary downloaded at install, first (needs a release pipeline; kept on the backlog as release-binaries for machines without cargo). Leaving devlab's copy of the rules in place beside the plugin's (two guards saying the same thing, drifting apart; devlab keeps only the phone's rule).

About
: hooks


<a id="2026-11-17-practices-kept-here"></a>
### 2026-11-17-practices-kept-here

Date
: 2026-11-17

Brief
: The practices are a table of ideal-agent's database; guide and the session start read them here; the observatory's copy is retired.

Decision
: The practices are a table of this database (data/practices.json), entered with tad r practice and exported in the mark, so the observatory gathers and shows them as ideal-agent's rows. lab-agent guide and ideal-agent start read them live from this repository's main; the observatory no longer keeps its own copy (its design/data/practices.json is retired with a pointer here).

Why
: The practices are the first thing an agent reads; kept in the observatory's design data they sat beside the observatory's own design, read by a URL hard-coded in lab-agent. In the one home of agent know-how they are kept, checked and rendered like every other row, and still seen in the observatory - which shows what places hold rather than holding it.

Rejected
: Leaving them in the observatory and pointing to them (the one home would not hold the one thing every session reads). Copying them (two lists drift - the copy was taken from the observatory's main on the day of the move, and the observatory's file is retired, not kept).

About
: practices


<a id="2026-11-17-one-database-no-glimpses"></a>
### 2026-11-17-one-database-no-glimpses

Date
: 2026-11-17

Brief
: ideal-agent's records live in its own tad database; the commit is the record of the work, no glimpses.

Decision
: ideal-agent keeps its records in its own tad database: data through tad (tad r NAME), landing through land; the commit and its pull request are the record of the work, signed by the name the session works under. No glimpses and no oco/ tables: the mark exports data/ directly.

Why
: The author asked for its own tad database. linux-phone-oco found the same way simpler on 2026-11-17 (its docs decision 2026-11-17-one-database): one tool writes the rows, git records the work.

Rejected
: ocoeco's oco/ tables beside a tad database (two tools writing rows in one place); glimpses (a second record of what the commit already says).

About
: records


<a id="2026-11-17-one-keeper-per-shared-resource"></a>
### 2026-11-17-one-keeper-per-shared-resource

Date
: 2026-11-17

Brief
: A resource only one may change has one keeper: a lasting role with disposable sessions, serving a queue others prepare; its tool holds the lock.

Decision
: The general pattern of a keeper of a shared resource is ideal-agent's: practice 44-one-keeper-per-shared-resource and the role skill keep-a-shared-resource. One owner per resource; a lasting role whose sessions are disposable (hand-over into the resource's how-it-stands record, then end); others prepare and queue items with their proofs, the keeper executes by the weakest link; the queue is a named query over the project's backlog, shown on its face; a scheduled run or the patrol starts the next keeper session when the queue has work and no keeper is live, and a lease keeps two keepers from running; the resource's own tool still holds the lock. A project keeps only its specifics - the resource, its tool, its queue's query, its lease, its schedule (linux-phone-oco: surya's phone keeper, in docs' ideal-agent-linux-phone-oco/, extending this).

Why
: The author, 2026-11-17: the general part goes upstream to ideal-agent, the linux-phone part to its orchestration session, which references it. Precedents: LAVA, CI-tron (postmarketOS's hardware CI), Gas Town's Refinery, the actor model (research keeper-of-a-shared-resource).

Rejected
: A standing keeper session kept alive (its context rots and is re-read at full price; the knowledge vanishes at compaction - decision 2026-11-17-experts-written-not-standing). Every session changing the resource when it needs to, guarded only by the tool's lock (safe but not flowing: changes race for the device and nobody chooses by the weakest link). A queue kept as its own list or tracker (a second record beside the backlog; a named query reads it off the backlog). The keeper as the lock (the tool already holds it; a keeper that crashes must not strand the resource).

About
: keeper

Research
: keeper-of-a-shared-resource


<a id="2026-11-17-follows-ideal-knowledge-base"></a>
### 2026-11-17-follows-ideal-knowledge-base

Date
: 2026-11-17

Brief
: ideal-agent follows ideal-knowledge-base by pin; every record names its subject in about, a row of subjects or a backlog item.

Decision
: ideal-agent follows grenudi/ideal-knowledge-base at 0.1.0, its second follower after linux-phone-oco/docs: idealkb's generic queries (about, next, find, row, recent), records (decision, research, backlog, session-log, source) and the generic parts of AGENTS.md and tables.toml, pinned in idealkb.toml. Every record of the standing tables names its subject in about - a row of the new subjects table (ideal-agent, practices, skills, hooks, organising-agents, keeper, knowledge-base, records) or a backlog item - checked by references rules. ideal-agent's own tables (practices, landscape, subjects, about) and records (practice, practice-shown-in, landscape, backlog-status) stay its own. queries/backlog.sql gives way to next. ocoeco's reasoning lens lets a decision be about a subject (ocoeco #458).

Why
: The author, 2026-11-17: yes to following it here, offered after idealkb asked (its backlog second-follower). ideal-agent is a single-place tad base, the closest fit to the generic shape; following makes an agent answer here the way it answers in any base (skill ask-a-knowledge-base).

Rejected
: Keeping its own records and naming them own in idealkb.toml (they differed only by lacking about: following is the point). Keeping the sources' by field (the generic sources table has none; the three values were folded into takes as Made by ...). Subjects as practice ids (44 numbered ids are a poor list of what the know-how is about; a short subjects table says it).

About
: knowledge-base


<a id="2026-11-17-nothing-of-ocoeco"></a>
### 2026-11-17-nothing-of-ocoeco

Date
: 2026-11-17

Brief
: ideal-agent depends on and teaches nothing of ocoeco; landit checks tad places with tad; what remains is passive or the author's words.

Decision
: ideal-agent depends on nothing of ocoeco and teaches nothing of it: its skills and its researcher look and record through the project's tad knowledge base and idealkb's queries and records only (about, next, find, row, recent, the named records); a place still kept with ocoeco is mentioned in one line as another project's legacy. landit checks a tad place with tad check and make verify, never ocoeco check, and makes no glimpse there (lab-agent #219). What remains is passive: the mark (oco.toml) and gather.yml, by which the author's observatory reads these tables; OCOECO_BY, the name oco-work signs commits with; and the practices' own text, in which 10 of 44 practices name ocoeco (14 counting its lenses and glimpses: 03, 05, 20, 22, 26, 29, 30, 31, 33, 37, 39, 40, 41, 42) - the author's words, reported to him, not rewritten here.

Why
: The author, 2026-11-17, sharply: our systems must not depend on ocoeco. Measured on main: orchestrate-agents and lab-session-start taught ocoeco focus, record-the-work taught ocoeco note, the researcher ocoeco focus; landit ran ocoeco check in every place, so ocoeco's reasoning lens decided what ideal-agent's about may name (ocoeco #458 was landed only to pass it).

Rejected
: Rewriting the practices that name ocoeco here, on the agent's own reading (they are the author's practices: what replaces each is his to say). Removing gather.yml and the mark (the author's observatory reads them; his to remove). Reverting ocoeco #458 (harmless to ocoeco, and nothing here needs it any more).

About
: ideal-agent


<a id="2026-11-17-look-around-first-skill"></a>
### 2026-11-17-look-around-first-skill

Date
: 2026-11-17

Brief
: Practice 22 is extended, not doubled: a proposal, the author's too, is met by a small look around, kept as a research row with dated material.

Decision
: Practice 22 is extended, not doubled: a proposal - the author's too - is met by looking at where things stand and how others do it and reconsidering it before acting; the finding is a research row with its sources and its material in a dated folder, and a subject's summary is renewed when due. The how is a skill, look-around-first: where things stand by the base's queries, how others do it (the researcher agent may take it), reconsider, keep the finding as a row plus a dated folder, renew the summary. ask-a-knowledge-base and record-the-work say the summary and the material. ideal-agent's pin moves to idealkb 0.3.0, so its own know-how follows the same layers.

Why
: The author, 2026-11-17: before acting on a proposal, his own included, look around, research a little, reconsider, and record the condensed finding with its research kept out of the way. Proposal processes elsewhere build the look in (research look-around-habit-world); idealkb 0.2.0 gave the place (its decisions 2026-11-17-summary-layer and 2026-11-17-research-aside).

Rejected
: A new practice beside 22: one nature under two names. A hook forcing a look before every edit: most edits need none, and a hook cannot tell a proposal from a typo. Folding the method into research-a-feature: that is a session's whole method for a hard feature; this is minutes before any change.

About
: practices

Research
: look-around-habit-world


<a id="2026-11-17-keep-the-start-light"></a>
### 2026-11-17-keep-the-start-light

Date
: 2026-11-17

Brief
: Housekeeping measures what a fresh agent reads per role, against a budget; the patrol measures, the owner simplifies, cutting words is only proposed.

Decision
: Housekeeping of the start is practice 45-keep-the-start-light and the skill keep-the-start-light, the upkeep of idealkb's summary and dated-research layers. Measured: the start weight per role (scripts/start-weight: the session start, skill descriptions, lab-session-start, AGENTS.md, the agent folder's README, the brief) and the base's fixed questions (idealkb's the-measure). Budget: 32 KB a role or a fifth more than last time; 3 KB a front file; 8 KB a brief; a summary due by idealkb's rule. Who: measuring is a duty of each project's patrol, every run (linux-phone: the linux-phone-patrol scheduled task); over budget it files one backlog item; the owner of the file simplifies. By itself: measure, record, file, write a due summary, mark research superseded; only proposed: removing words from a brief, a front file, a skill or the start, moving files, and any change to a practice. Applied first to ideal-agent's own part: the session start prints the practices without their reasons (22.5 KB to 13.9 KB), the reasons one command away.

Why
: The author, 2026-11-17, asked for it, built on what exists: the canon's cold read, the playbook's findability test, idealkb's kb-navigation-measured and its summary layer, the backlog item session-start-size. Measured on linux-phone-oco (research housekeeping-the-start): 35-46 KB before acting, half of it the practices' reasons.

Rejected
: A scheduled task of its own (a second waker looking at the same project beside the patrol: one look, two names - measuring is cheap and fits the patrol's round). Housekeeping writing its own digests of a subject (a second way of making summaries beside idealkb's summary record). Letting it cut text by itself (a cut hides what comes after it, practice 03; words are taken out only by their owner, the practices only by the author). Tokens as the measure (they depend on the model; bytes are what the files are, about four to a token). A budget per file for every file (only what every session reads is paid every time; a skill's body or a record is read when needed).

About
: housekeeping

Research
: housekeeping-the-start


<a id="2026-11-17-the-start-reads-first-sentences"></a>
### 2026-11-17-the-start-reads-first-sentences

Date
: 2026-11-17

Brief
: The session start prints each practice's first sentence, numbered by its id; the whole is one command away.

Decision
: The session start prints each practice's first sentence alone, numbered by its id and in the order of the ids; each practice whole, with why it holds and for whom, is one command away (lab-agent guide, scripts/ideal-agent practices). Practice 45's opening was split so that its first sentence says the practice.

Why
: The author chose it on 2026-11-17 (backlog practices-short-at-the-start, proposed by housekeeping): the practices' own words were 13.7 KB of the 14.4 KB session start; their first sentences are 7.9 KB. Numbering by position had shown practice 22, written last into the file, as the 44th.

Rejected
: The author shortening each long practice by hand (his choice was the first sentence; the whole stays as he wrote it). A summary line written for each practice (a second text of each practice to keep in step with the first).

About
: housekeeping

Research
: housekeeping-the-start


<a id="2026-11-17-briefs-first"></a>
### 2026-11-17-briefs-first

Date
: 2026-11-17

Brief
: ideal-agent follows idealkb 0.4.0: every decision and research has a brief, answers give briefs, the whole on demand.

Decision
: ideal-agent follows ideal-knowledge-base 0.4.0, whose concept brief-first gives every record a one-line brief and every answer briefs, the whole read only when the task needs it. Its 13 decisions and 4 research rows were given briefs by hand; ask-a-knowledge-base and record-the-work say to read briefs first and to record with --brief; a file opens with its brief, its first paragraph.

Why
: The author, 2026-11-17: records - tables and files - should all have a brief summary to begin with, read by default; deeper only when the task needs it - reflected in idealkb, referenced by ideal-agent as its implementation. Measured here: the about answer for housekeeping 1.4 KB by briefs against 5.3 KB whole.

Rejected
: Leaving the old rows to the first-sentence stand-in (they are few and ours; a written brief says what matters, a first sentence only what came first). A practice of its own (the skills every session uses already carry it; the start stays light).

About
: knowledge-base


<a id="2026-11-17-read-only-when"></a>
### 2026-11-17-read-only-when

Date
: 2026-11-17

Brief
: A session reads at its start only the practices' first sentences, its role's purpose and its task's summary; everything else only when a step needs it.

Decision
: lab-session-start says what to read at the start - the practices' first sentences, the first paragraph of the role's brief, the task's subject in summary (tad q about, or the queue's top) - and then 'read only when' for everything else: a record whole when its brief is not enough, another subject when the task reaches it, a skill's body when its step comes, a tool's help only when no command is named, the board and recent only when choosing. research-a-feature, bring-up-a-part and keep-a-shared-resource begin from the summary. keep-the-start-light measures what a started session read from its transcript (scripts/read-weight), not only the front files. Proposed to the author, not done: start roles that need no mail, web search or documents without those connectors.

Why
: The author, 2026-11-17, after the first surya keeper: an agent reads only summaries and goes into detail only when its current task needs it. Measured (what-a-started-agent-reads): 73-81k tokens before anything is read, nearly all harness and connectors' tools; the keepers' own reading before acting was 10-12k tokens, the largest part tool help and a subject read whole.

Rejected
: Cutting the briefs further first (they are 1-2k tokens of an 80k start). Turning the connectors off from here (a user setting that also becomes the default for new sessions: the author's). Measuring only the front files (the help and whole records a session reads for want of a named command are the part that grows).

About
: housekeeping

Research
: what-a-started-agent-reads


<a id="2026-11-17-connectors-off-by-default"></a>
### 2026-11-17-connectors-off-by-default

Date
: 2026-11-17

Brief
: Gmail and Claude Docs are off by default for new sessions; Firecrawl stays on for research; a session that needs one asks the author.

Decision
: The author turned Gmail and Claude Docs off by default for new sessions; Firecrawl stays on. The skills lab-session-start and keep-the-start-light say so: a session whose task needs Gmail or Claude Docs asks the author, who turns it on for that session from the composer's + menu, Connectors. The next keeper's start is measured with scripts/read-weight to confirm the saving.

Why
: The author, 2026-11-17. Measured (what-a-started-agent-reads): every session held 73-81k tokens before reading anything, nearly all Claude Code and the tools of every connector it carries - Gmail 31 tools, Claude Docs 8 - while the project's own start was 4-6k; a keeper or a research session uses neither.

Rejected
: Switching connectors per session from outside, by role (the app offers no such switch: it applies to one session and sets the default for new ones). Turning Firecrawl off too (research sessions need it to look outside the commons). Leaving all on (every session pays for tools it never calls).

About
: housekeeping

Research
: what-a-started-agent-reads


<a id="2026-11-17-hold-little-and-renew"></a>
### 2026-11-17-hold-little-and-renew

Date
: 2026-11-17

Brief
: Every role minds what it holds: small outputs, subagents for heavy reading; long-lived roles hand over and restart fresh at 250k, at 150k after a batch or an idle hour.

Decision
: keep-the-start-light extends to the whole of an agent's context. Hold little while working: name the exact command and limit its output, never a bare --help or a whole table; heavy reading goes to a subagent that returns a summary; skills and connectors only when needed; model and effort by role. Orchestration and the coordinator check how agents are brought up with read-weight on recent transcripts. A long-lived role renews itself: when its context passes 250k tokens, or a batch has landed with over 150k, or before resuming after an idle hour with over 150k, it writes ROLE-how-it-stands in its base (superseding the last), lands it, and says how the fresh session starts - the role brief's first paragraph and that row; the author starts it and archives the old. Applied first to the linux-phone coordinator (docs #195: coordinator.md and coordinator-how-it-stands).

Why
: The author, 2026-11-17: snappy bring-ups and every role aware of its own context; long-lived roles compact what they no longer need into summaries and restart fresh. Measured (long-lived-roles-renewed): the coordinator at 840k tokens after 1,222 turns, re-reading 574M; a fresh one starts at about 83k.

Rejected
: Relying on Claude Code's automatic compaction (it keeps the session and whatever its summary chose, written nowhere another session reads; none of today's sessions had compacted even at 926k). A new skill or practice beside keep-the-start-light (the same nature - what an agent holds - extended in its place, as the author asked). Renewing on a fixed timer (an idle session costs nothing until it resumes; the cold cache at resumption is what costs). A hand-over in a notes folder outside the base (not read by a fresh session; the role's record is).

About
: housekeeping

Research
: long-lived-roles-renewed


<a id="2026-11-17-roles-as-agent-cards"></a>
### 2026-11-17-roles-as-agent-cards

Date
: 2026-11-17

Brief
: A role is a card in ideal-agent's agents/; a brief names it (Role: NAME); the first prompt brings the card and only its practices

Decision
: Seven generic roles - coordinator, lane-coordinator, orchestrator, keeper, research-lead, builder, patrol - each a Claude Code agent card in ideal-agent's agents/: frontmatter model, effort, skills; a body of purpose, owns, never, loads, hands over, reports to; a Practices line. A project's brief names its role under its title (Role: keeper (ideal-agent), Also practices: 42 for a domain's own) and keeps only the domain; the prompt that starts a session begins with that line. A UserPromptSubmit hook (ideal-agent role) gives the card and only those practices at the first prompt, every practice when none is named, and remembers it for a resume or compaction; the SessionStart no longer prints the practices. claude --agent ideal-agent:ROLE starts a session as the role too. What replaces what: the card replaces a brief's generic part and the whole practice list; the role skills stay the methods the cards load; briefs stay the domain.

Why
: The author's task of 2026-11-17: a session gets a role and loads only what it needs. Claude Code's native card is the shape others already read (Gas Town primes a role at start; RACI and Team Topologies give owns and reports to); start_session cannot pass --agent, so the role is read from the prompt. Measured: start-weight on docs' main falls 1.4-3.6 KB per role with the Role line alone; the session start from 8.6 KB to 0.5 KB plus a 3.5-5.7 KB role.

Rejected
: A role table in tad with ideal-agent start --role (a parallel layer beside the agent cards Claude Code already loads, and the start cannot see the prompt); role-filtered practices by their for field (it says agents or people, not roles); new role skills such as coordinate-a-lane (would add to the skill descriptions every session reads, and repeat keep-a-shared-resource); agent teams (experimental, skills not applied, the lead not handed over); generated .claude/agents per project for connectors and hooks (a second copy to keep in sync; connectors stay off by default already); renaming the researcher subagent (in use; the session role is research-lead, Anthropic's word for the one holding a research).

About
: organising-agents

Research
: agent-roles


<a id="2026-11-17-a-neutral-start"></a>
### 2026-11-17-a-neutral-start

Date
: 2026-11-17

Brief
: ideal-agent owns a neutral start, scripts/agent-env: token, PATH, AGENT_NAME signing, branch and land - sourced before every Bash command; nothing of ocoeco

Decision
: scripts/agent-env (sh, sourced) gives GH_TOKEN from the lab-agent token, ~/.local/bin and ~/.cargo/bin on PATH, AGENT_NAME as the commits' author and committer (given; else the brief's Name: line, kept by the role hook; else the role; else OCOECO_BY; else agent), LAND_RELEASE=0, OCOECO_BY set to the same name for places still on ocoeco, and branch NAME (fetch, a new branch from origin/main, refused on uncommitted work, nothing cleaned). The SessionStart hook names it in CLAUDE_ENV_FILE, which Claude Code evaluates before every Bash command, so no brief or skill says . oco-work; a job outside Claude Code sources it from the clone. land stays lab-agent's command, reached on PATH.

Why
: The author (relayed by the coordinator, 2026-11-17): oco-work is used only for its environment, landit and fresh; its ocoeco helpers and the OCOECO_BY name are leftovers he does not want; a small neutral start owned by ideal-agent, keeping places on ocoeco working until they move. Claude Code's hooks docs: CLAUDE_ENV_FILE is given to SessionStart and evaluated before each Bash command; the session id reaches Bash as CLAUDE_CODE_SESSION_ID, so the start reads the session's role state.

Rejected
: Keeping . oco-work in every brief (it carries ocoeco and must be repeated in every shell, since an export does not outlive one Bash call); a copy of oco-work without ocoeco installed into ~/.local/bin by the hook (a second copy to keep, written outside the plugin); fresh's reset and clean (it loses files written before it - the hand-over warned of it); landit (its checks are the place's own CI, which land waits for; a commit and land --no-manual suffice); setting AGENT_NAME from the UserPromptSubmit hook's environment (that hook gets no CLAUDE_ENV_FILE); moving land into ideal-agent now (it is lab-agent's and used by everyone; left as a follow-up).

About
: organising-agents

Research
: agent-roles


<a id="2026-11-17-practices-without-oco-tools"></a>
### 2026-11-17-practices-without-oco-tools

Date
: 2026-11-17

Brief
: The practices keep OCO's ideas in tad and ikb terms; those that existed only for ocoeco's tools retire, their reasons kept in the row

Decision
: Sixteen practices are reworded without ocoeco's tools, their idea and reason kept: 02 (the owning project's backlog), 03 (the role's practices and how-it-stands at the start), 04 (lab-agent inbox and session messages), 05 (the board read off tad q next and about), 06 (work on the lab, in a session there or a lab job - not only through oco-agent), 16, 20 (the weakest link read from the data; hand-work counted on the backlog), 26 (refining recorded in the owning data), 29 (a named query, not a lens), 30 (--about names where work was made), 31 (look the id up with tad q), 33 (a query is seen, not tested), 34 (the author's turns in the place that keeps his words), 37 (ikb practice-shown-in), 38 (ideal-agent's start and skills are the shared words), 39 (what followed read off tad q recent and the landed pull requests). 40 (the window) and 41 (a lens changed in place) retire: a new field retired keeps when and why in the row, the start and the practices query skip it. The role cards take up 03, 05, 29, 31, 34, 37, 38 and 39 where they fit.

Why
: The author, 2026-11-17 (relayed by the coordinator): everything related to oco tools (not oco as a concept) became too bloated and here we are developing ways to detach from that and make better autonomous, independent tools. Keep the OCO concept - an open commons, knowledge kept as data, everything recorded, looked at before acting; retire what exists only for ocoeco's lenses, window, glimpses, focus, fits, fulldome and oco-agent jobs.

Rejected
: Deleting 40 and 41 (their reasons - why the inside was wanted - would be lost: Chesterton's fence); rewriting the why of each (the why is what happened; a note that it was reworded is added instead); retiring 05, 29, 30, 31, 39 with their tools (their ideas - read off, not typed; never an id from memory - hold in tad); retiring 06 (the lab as the one place where state lives still holds; only oco-agent as the sole way goes); renumbering (a practice is called by its number everywhere).

About
: practices

Research
: agent-roles


<a id="2026-11-17-start-health-folds-into-housekeeping"></a>
### 2026-11-17-start-health-folds-into-housekeeping

Date
: 2026-11-17

Brief
: No agents-healthkeeper role: read-weight measures what each role was given and never used, the patrol runs it per role, each owner trims - cards, briefs, the author's settings

Decision
: Start health folds into housekeeping. read-weight prints 'given, not used' (skills, agents, deferred tools and connectors listed and never used; the first prompt's role and practices). The patrol runs start-weight and read-weight on each role's latest session and files, naming the role, what is over budget or never used. Trimming stays with the owner: ideal-agent's owner for the role cards and what the plugin lists (its 11 skill descriptions cut from 4,080 to 2,287 B now); a project's owner for its briefs; the author for his settings (the skills and connectors every session lists - proposed, not changed). keep-the-start-light and the patrol card say so by reference.

Why
: The author, 2026-11-17: an agents-healthkeeper that looks at each role's bring-up and perfects the cards; research first, design only if research says so. Measured: ~18 KB of skills listed and unused by every role. Others: no analogue keeps a standing role - Gas Town's patrols watch liveness, SRE measures by rule and owners fix, friction logs are the newcomer's. Context engineering: smallest high-signal set.

Rejected
: A standing agents-healthkeeper role (it would add a card to every session's listing and a start of its own to do what a script and the patrol already do; it would trim what others own); a subagent the patrol spawns (the same read-weight run, with a start of ~80k tokens); per-role skill hiding through the role card (Claude Code has none for app-started sessions; the skills field only preloads); changing the author's skill settings here (his, like connectors: proposed); moving the role cards out of agents/ to save their 1.1 KB listing (kept for claude --agent until a measure shows no use).

About
: housekeeping

Research
: start-health


<a id="2026-11-17-role-health-is-the-patrols"></a>
### 2026-11-17-role-health-is-the-patrols

Date
: 2026-11-17

Brief
: One shape, no new role: role health - at start (given, not used) and at work (struggled with) - is read-weight per role, run by the patrol, sent to the top coordinator as a short list

Decision
: agents-healthkeeper and role-work-simplification are one method - reading a role's transcripts - and become one duty of the patrol, not one or two roles. read-weight prints both sections; the patrol runs it on each role's sessions since its last round and, where a cluster needs reading, gives those transcripts to a read-only researcher subagent; it sends the top coordinator a short list, each item with its evidence, its owner and a proposed fix; the author and the owners decide. What ideal-agent owns from today's list is fixed now: the guard names jq, awk, sed or tad for what Python was reached for; change-a-device says how to wait without sleep; the lane-coordinator card starts sessions on its own turn.

Why
: The author, 2026-11-17: a role that finds where agents waste time and tokens, weighed with agents-healthkeeper, one shape if they share a method. Measured: 301 of 5,375 calls failed and 56 writes by hand went past named records on one day; ikb-cli's one-off reading of 31 transcripts led to ikb. Others: detection is a script, clustering recurs now and then, owners fix (Anthropic, Microsoft, Langfuse, SRE, paper cuts); no source keeps a standing role.

Rejected
: Two roles, healthkeeper and work-simplification (one method under two names, two cards in every listing); one standing role-health session (holds ~80k tokens to run a script; no analogue keeps one); /insights per role (per user and machine, no split by role); PostToolUseFailure hooks as the measure (they miss schema and permission failures, which the transcript holds); the patrol fixing what it finds (fixes are the owners').

About
: housekeeping

Research
: work-struggles


<a id="2026-11-17-lab-agent-retires-by-parts"></a>
### 2026-11-17-lab-agent-retires-by-parts

Date
: 2026-11-17

Brief
: lab-agent retires by parts (token, timers, jobs, board/notify, inbox, root-exec, daemon); oco-agent is not its successor; notify and root-exec become shell

Decision
: lab-agent retires part by part, in this order, and oco-agent does not become its successor. Every part has an owner today that is not lab-agent, except two things nothing else gives: a phone push from an unattended unit (notify) and passwordless root for devlab (root-exec); those two move to a few lines of shell each, not to a program.
1 token: the token file moves to a neutral path (~/.config/ideal-agent/token, mode 600), the old path a symlink until land, agent-env, tad/idealkb-update-pins and oco-agent read the new one; cost ~1 h, lost nothing.
2 timers: oco-ask-to-observe (about 44 GitHub requests per 15 min), oco-say-what-is-new, oco-agent-shadow stop with ocoeco's legacy; cost minutes; lost: ocoeco's lenses of the lab (declared dead).
3 jobs: a long build that must outlive a session is `systemd-run --user --unit=NAME COMMAND` (journal is its log, a unit name is claimed once, `systemctl --user status/stop` is standing/stop); walleye-builder's six jobs today are the only use; lab-agent run, oco-agent run and the lab-agent-jobs polling every 5 s retire after one walleye build is proven this way; cost ~2 h; lost: submission from a sandbox that cannot reach the lab (no such driver since 2026-11-15), the replay measure of the 2026-11-11 stall (kept as a concept), and the two inhibit-idle units (the lab sleeps unless a session asks for keep-awake - the author decides).
4 board and notify: devlab watch pushes its alert with `curl` to the ntfy topic from a mode-600 file (notify, 3 lines) and files the standing alert as a backlog row; the board's 40 stale items go to their projects' backlogs or are closed; cost ~3 h; lost: the 5-minute board push on the phone.
5 inbox/guide: the plugin's session start stops running `lab-agent inbox`; practice 04 names the app's session messages; guide is `scripts/ideal-agent practices`. Needs the author's word first: does he still write to the lab from the ntfy app (older traffic NOT KNOWN; empty over the last 12 h)? cost ~1 h.
6 root-exec: devlab and the setup scripts call a root-owned wrapper in lab-machine (`lab-root`, exec argv) granted by one sudoers line - the same power as today, renamed - or, as the author may prefer, a narrower grant per command (fastboot, ufw, nix, pacman) which ends ad hoc root for sessions; the grant is the author's and root's to write, never mine; cost ~3 h plus the author's sudo.
7 self-update, then the daemon and its units, the sudoers line and the package go; cost minutes once 1-6 are done.
Check: `scripts/lab-agent-free` (shows 34 live uses now, must reach 0).
Resolving 2026-11-12-oco-agent ("oco-agent replaces lab-agent, tied to ocoeco by design"): it was decided when ocoeco was the system; 2026-11-17-nothing-of-ocoeco and the author's direction of the same day remove its premise, so it is superseded, not reinterpreted: oco-agent's ideas (the journal as the only truth, a job claimed once, leases, replay as a measure) stay as concepts in its own repository; its program is not extended, and its half-built switch steps 2, 3 and 5 are dropped. A row in that repository's own decisions says so when its owner lands part 3.

Why
: Measured 2026-11-17: no job through lab-agent since 2026-11-12 (1165 ever, 2 probes since); oco-agent's traffic was ocoeco speaking and walleye-builder's six builds today; live uses left are the token file, root-exec, board/notify for devlab watch, and the inbox call in the plugin's session start. Sessions on the lab, the plugin, ikb and land already do the rest. The author, 2026-11-17: tools of oco became too bloated, make better autonomous independent tools.

Rejected
: Keep lab-agent as is (a polling daemon, two inhibit units and 44 requests per 15 min for no job); finish oco-agent as the successor (its own premise, ocoeco, is retired; a second runner beside systemd); one big removal now (root-exec and the token are live: devlab flash and every land would break); rewrite lab-agent smaller in Rust (a new program for what systemd-run, curl and a sudoers line do); a Python or Claude-scheduled-task runner (no new Python; sessions end).

About
: organising-agents

Research
: lab-agent-what-it-is-2026-11-17


<a id="2026-11-17-a-harness-is-a-subject"></a>
### 2026-11-17-a-harness-is-a-subject

Date
: 2026-11-17

Brief
: The method stays agnostic; each harness we run is a subject whose how-to is dated, sourced research, superseded when it moves; nothing is kept for a harness we do not run

Decision
: Practices, role cards' bodies and skills speak of the method only, and skills keep to the open SKILL.md fields (name, description). Each harness the agents run is a subject of ideal-agent's base (claude-code today; records/subject.sql adds one); what the method uses of it is a research row about that subject - dated, with its sources and the version checked - and one summary on top; when the harness moves, a new row supersedes the old, which leaves the subject's answer and stays findable. A skill points to the harness's subject instead of naming its features. A role card's frontmatter (model, effort, skills) is the harness's reading of the card, its body the method. Only what a skill, role or script uses is kept; a harness we do not run, or a model's ranking, is not tracked. Moved now: the waiting, connector, transcript and session-start specifics out of change-a-device, keep-the-start-light and lab-session-start into claude-code-how-we-use-it.

Why
: The author, 2026-11-17: ideal-agent stays agent agnostic, so Claude or GPT or Qwen or whatever comes are just specifics on how to do it for the model; a small research on how ideal-agent's base is organised, so that latest records supersede old info or show they are old, point to where it was taken, and we do not housekeep records on the ever-changing LLM industry. Others: open formats for the method, a thin layer per harness, dated reviews.

Rejected
: A new table for harness notes (research with date, sources and supersedes already does it); a last-reviewed field and a staleness script now (the row's date and the subject's summary-due show age; added if a second harness comes); ruler or rulesync as a generator (Node, ~20 harnesses we do not run); our own generator to AGENTS.md and .agents/skills now (only one harness runs; when a second does, the skills are already portable); tracking models and their rankings (practice 35 says to check what the models are when choosing, not to keep a list).

About
: practices

Research
: harness-specifics-where


<a id="2026-11-17-decisions-go-to-the-lane"></a>
### 2026-11-17-decisions-go-to-the-lane

Date
: 2026-11-17

Brief
: A keeper's open question goes to its lane coordinator, who decides what can be undone (research first if needed), records it, tells the keeper, sends two lines up; only named classes reach the author

Decision
: The lane coordinator is the one decider for its lane. A keeper that cannot decide leaves the resource known-working, files the question in the lane's base, sends it to its lane coordinator and takes the next queue item. The lane coordinator decides a two-way door once the records show the way back; where they do not, one round of a read-only researcher subagent on a lighter model (a second only for what the first left open; a long question is a research-lead session on its own turn); it records research and decision before anyone acts, tells the keeper with the id, sends the top coordinator two lines with the id. Through the top coordinator to the author go only: his hands or witness; writes that cannot be undone or risk the resource's recovery; money; outward actions; permissions, security and agents' settings. The top coordinator does not re-decide a lane's decision unless the author does. Written into keep-a-shared-resource and the keeper, lane-coordinator and coordinator cards.

Why
: The author, 2026-11-17: decisions like walleye keeper-1's should not pop up and block work, but go to the lane coordinator, who, if he does not know, spawns research, records findings and decision as in our ways, sends a small summary to the top coordinator and tells the asking role; a change to roles after research. Others decide at the lowest level holding the information and escalate early (Bezos, RAPID, SRE, Gas Town); humans keep irreversible, outward, money and security (auto mode, LangGraph).

Rejected
: Keepers asking the author directly (the lane stalled three times in an afternoon); the top coordinator deciding lane questions (it lacks the lane's records and holds the author's conversation); a research-lead session by default (refused on a message's turn, and long for a flag); treating every device write as the author's (most are two-way doors once the way back is shown); unbounded research (endless searching, Anthropic); acting on a spoken answer before the record (the next session cannot see it).

About
: organising-agents

Research
: decision-routing


<a id="2026-11-17-a-lane-starts-its-own"></a>
### 2026-11-17-a-lane-starts-its-own

Date
: 2026-11-17

Brief
: A lane starts its own sessions through launchers - on-demand tasks the author sets up once - run on any turn, its work written into the base first; subagents stay for short questions

Decision
: Each lane has launchers: one on-demand stored task per role it starts (keeper, research lead, builder), created once by the author so it keeps his permission mode, with a fixed prompt - the role's head, the brief's path, where its work is read (the queue's top, the lane's open question) and the writes it may make. The lane coordinator writes the work into the base and runs the launcher, on any turn; a launcher runs once at a time, so one keeper per lane holds by itself. Short questions and read-only looks stay subagents within the turn. The role hook reads a role past a scheduled run's wrapper. Written into keep-a-shared-resource ('Starting the lane's sessions') and the lane-coordinator card; the harness's how-to into claude-code-how-we-use-it.

Why
: The author, 2026-11-17: research and do something about these pauses, so they do not occur in areas he is not competent in and can be done with proper research instead. Measured: seven cards, 4-40 minutes each, all start refusals on a message's turn. Probed: run_scheduled_task starts a session on such a turn.

Rejected
: A standing keeper per lane fed by messages (it grows without end, and renewing it with clear_session asks the author each time); the keeper starting its successor (its turns are messages' turns too, and a stranded keeper starts none); a cron waker as the starter (a scheduled run's own start_session is refused the same way; it stays a nudge); a headless run under systemd-run (not logged in on the lab; not seen, steered or archived); launchers created by a session (no permission mode saved, so runs stop to ask); a new launcher per question (prompts vary; one fixed prompt reading its work from the base keeps the board read off the work).

About
: organising-agents

Research
: lane-starts


<a id="2026-11-17-reading-is-not-acting"></a>
### 2026-11-17-reading-is-not-acting

Date
: 2026-11-17

Brief
: The author's bounds name acting on the world in his name, not reading it: a checked official download is the lane's; a mirror, a host binary, terms or money are not

Decision
: keep-a-shared-resource's bounds now say 'acting on the world in his name' - sending a message or mail, posting, publishing, a call, a pull request or patch to someone else's project, an account, terms accepted, anything bought or paid - instead of 'anything outward'; and a fourth point says what is the lane's asking no one: everything technical not on that list. A public file from a project's own release site, its checksum or signature checked against what the site publishes, kept in the work's own folder with its source and checksum recorded, used without being run on the lab host (a device may boot it from memory), needs no one's word. A file without a published checksum or signature, from a mirror, or a program to run on the host is the lane coordinator's decision, research first if needed. The coordinator card says the same.

Why
: The author, 2026-11-17: no need to ask his permission for downloads; roles should prevent this for the future; work should depend on him in such decisions as little as possible - it is a technical question, and roles' expertise handles it. Measured: two download asks in two minutes, from 'anything outward' in decision 2026-11-17-decisions-go-to-the-lane. Others gate the use of bytes, not reading them (auto mode, Copilot firewall, SLSA).

Rejected
: A list of allowed sites (it ages, and a project's own release site is already the test); every download through the lane coordinator (a checked official file needs no judgment); dropping the checksum check (a file from a hijacked link would be booted); counting licence acceptance as technical (it binds the author: his).

About
: organising-agents

Research
: reading-is-not-acting


<a id="2026-11-17-the-guard-refuses-tad-sql-writes"></a>
### 2026-11-17-the-guard-refuses-tad-sql-writes

Date
: 2026-11-17

Brief
: The guard refuses tad sql that writes and names ikb set and ikb remove; record-the-work says so; a one-time migration says TAD_SQL_WRITE_OK=1

Decision
: ideal-agent's PreToolUse guard refuses a Bash command whose tad sql holds an INSERT, UPDATE, DELETE, ALTER, DROP, CREATE, REPLACE or TRUNCATE statement, wherever it runs (sh -c included), and answers with ikb set and ikb remove (ikb 0.5.1, ideal-knowledge-base decision 2026-11-17-ikb-set-and-remove); reads pass; TAD_SQL_WRITE_OK=1 in the command lets a one-time migration through, recorded with it. record-the-work names ikb set and ikb remove for a row no record covers.

Why
: 56 tad sql writes by hand in 13 sessions on 2026-11-17 (research work-struggles), past the named records; ikb now covers them with a checked set and remove; the ikb session proposed the guard (its backlog tad-sql-guard).

Rejected
: A skill line alone (the writes happened with record-the-work already saying named records); refusing every tad sql (reads are how agents look); no override (a schema change needs a one-time fill, as docs #260 did).

About
: hooks

Research
: work-struggles


<a id="2026-11-17-research-on-own-initiative"></a>
### 2026-11-17-research-on-own-initiative

Date
: 2026-11-17

Brief
: Coordinators research on their own when a method is missing, bounded; what is general goes to ideal-agent as an upstream item its owner decides; every card carries practice 22

Decision
: look-around-first gains 'When a method is missing': a coordinator (top, lane, orchestration) starts research itself on a sign - work no record or skill covers, a question asked twice, a struggle cluster - bounded: the commons first, then one researcher-subagent round, a research-lead launcher only for a long question; one research per sign, recorded, two lines up. A finding about how agents work or how any device is brought up flows back to its owner: an upstream item naming grenudi/ideal-agent in the domain's base and a message to ideal-agent's owner, who looks around, decides, lands and reads back; no domain changes a general practice, skill or card. Every card carries practice 22; the coordinator, lane-coordinator and orchestrator cards point to the section; lab-session-start says the brief and the author's words are a starting point. The linux-phone briefs and ideal-agent as docs' upstream go through orchestration.

Why
: The author, 2026-11-17: a coordinator should decide to start research on its own and adjust ideal-agent from the results - not take his word for it, but research, look around and decide. The walleye lane lacked the general bring-up method nobody had asked for. Others: the lead decides on more research (Anthropic); findings become owned items in the owner's system (SRE postmortems); the rules themselves are revised (double loop).

Rejected
: Research only on a keeper's ask (the missing method was nobody's question); a domain session editing ideal-agent directly (two writers collided on 2026-11-17; the owner keeps one home and one judgment); unbounded initiative (a research per thought; one per sign, the commons first); a new role for it (the coordinators see the signs; a role adds a card to every listing).

About
: organising-agents

Research
: research-on-own-initiative


<a id="2026-11-17-a-run-end-wakes-the-lane"></a>
### 2026-11-17-a-run-end-wakes-the-lane

Date
: 2026-11-17

Brief
: A lane coordinator subscribes to its launchers' completion at its start and re-runs on the wake; a launched session's last act is two lines to it

Decision
: keep-a-shared-resource ('Starting the lane's sessions') and the lane-coordinator card: at the lane's start and after its renewal, the lane coordinator subscribes itself to each launcher's completion and re-runs on the wake; the keeper card: its last act, after the hand-over and the lease released, is two lines to its lane coordinator - a backstop where a subscription was lost (one subscriber per task, the newest replacing).

Why
: Surya's lane sat idle from 18:16 on 2026-11-17 until the author asked why: a launched run is not its coordinator's child, so its end sent no note. Both lane coordinators subscribed (surya 2, walleye 3 launchers) and the app accepted every one on a message's turn; the first wake is still to be seen.

Rejected
: Polling the runs (a turn spent per look); a cron nudge alone (up to an hour idle); the keeper's last message alone (a keeper that ends abruptly sends none); the subscription alone (a renewed coordinator or another subscriber replaces it).

About
: organising-agents

Research
: lane-starts


<a id="2026-11-17-a-device-from-nothing"></a>
### 2026-11-17-a-device-from-nothing

Date
: 2026-11-17

Brief
: bring-up-a-part gains 'A device from nothing': a way back, a feedback channel, a known-good boot, then mainline; and the one-part-at-a-time swap from an accepted stock image

Decision
: Taken from linux-phone's upstream item, changed to speak of any device: the order a way back, a feedback channel, a known-good boot (the vendor kernel, or the accepted stock image itself, with our initramfs, read-only), then mainline from the nearest sibling; and when stock is accepted and ours is not, swap one part of ours into the accepted image at a time. The walleye specifics stay in docs.

Why
: Its research (14 sources) and the walleye lane's afternoon: mainline images accepted and silent taught nothing; one swap into the stock image found the difference. Every sibling port had a channel or a known-good first.

Rejected
: The vendor kernel as the only known-good (no MSM8998 sibling had one; the stock image serves); phone words in the general skill (UART stays an example among channels); leaving it in docs (it holds for every device the commons brings up).

About
: skills

Research
: device-from-nothing


<a id="2026-11-17-a-domain-layer-names-its-base"></a>
### 2026-11-17-a-domain-layer-names-its-base

Date
: 2026-11-17

Brief
: A domain's layer stays where its knowledge is - docs' agent folder, lanes in their device repos - one brief per role it uses, deltas only, checked by overlay-check

Decision
: No separate repository: the domain layer is the agent folder of the domain's commons base (linux-phone-oco/docs, ideal-agent-linux-phone-oco/), mirroring ideal-agent's roles - one brief per role card it uses, named for it, beginning with the Role, Name and Also lines - and holding only what the domain adds: its devices, tools, places, queries, order of work. Anything general is named (the card, the skill and its section, the practice's number), never said again. A lane's brief (a device's coordinator, its keeper's device part) moves into that device repository's agent folder with its rows (move-device-rows), from a skeleton device-template ships. The folder's README names the ideal-agent version it was last checked against; scripts/overlay-check (6-word runs, 15%) is run by the patrol and by whoever changes a brief; a general finding goes back as an upstream item to ideal-agent (docs' upstream, as #56 did).

Why
: The author, 2026-11-17: the domain version keeps ideal-agent's structure, references it for general behaviour and adds only specifics, never copying. Measured: half the folder restates ideal-agent and two briefs now contradict it - a copy goes stale. Others pin a base and hold deltas (plugin dependencies, Helm, Kustomize, Debian); nobody checks restating, so the check is ours.

Rejected
: A separate ideal-agent-linux-phone repository with its own base (a third place to keep in step; the domain's knowledge is docs' and the devices'); a linux-phone Claude plugin depending on ideal-agent (later, if the domain grows skills of its own - today briefs are read by path, and a plugin adds a listing to every session); copying ideal-agent's tree into the domain (it is what went stale); waiting for the device move before trimming (the contradictions act now).

About
: organising-agents

Research
: domain-layer
