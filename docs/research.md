<!-- Generated from data/ by tad. Do not edit by hand. -->

# Research

<a id="expert-agents-written-not-standing"></a>
### expert-agents-written-not-standing

Date
: 2026-11-17

Brief
: Everyone who keeps experts across time keeps the expertise outside the session; standing expert sessions cost most and recall worst.

Question
: Should linux-phone-oco keep standing expert sessions, one per hard domain (audio, modem, sleep...), with the orchestrator routing questions to them - or keep expertise somewhere else?

Found
: The world has tried it under several names - specialised workers under a supervisor, hierarchical teams, blackboard systems, stateful agents - and converges on one split: the expert is a role and a body of written knowledge; a session is a disposable worker that takes the role up. Gas Town says it outright (an agent is a persistent identity, sessions are cattle; any session can hand off and restart from its hook). Anthropic's long-running harness found compaction alone not enough and had every fresh session get up to speed from a progress file, git and a feature list; its context-engineering guide names notes outside the window as the tool for iterative work with milestones. Claude Code itself moved this way: subagents may keep a per-role MEMORY.md, and agent teams start each teammate from project context and a spawn prompt, never the lead's history, and do not restore teammates on resume. Against expertise held in a long context: models recall worse as input grows (Chroma, context rot; Anthropic's attention budget), repeated summarising erodes detail (ACE: context collapse, brevity bias), and a session of 1000-1800 messages re-reads all of it each turn - after an idle hour without the cache, at full price - so a standing expert answering an occasional question is the costliest and not the most reliable way to answer it. Against a central router: a supervisor that relays every message plays telephone - lower scores and more tokens than agents handing off directly (LangChain); misalignment between agents (lost context, withheld information) is one of the three failure groups in 1600 traces (MAST). What works (Cognition, 2026): one writer per thing, other agents adding intelligence not actions; a consult works best given a fork of the asker's full context. Blackboard systems route by a shared record agents read, not by a relay. For us: most of the audio expertise is already rows in docs (5 research, 2 decisions, 27 backlog items name the microphone); what is not written is the tacit part - dead ends, where the experiment patches are, what hung the phone - and it now lives in hand-over notes outside any repository.

Sources
: anthropic-context-engineering,anthropic-long-running-harness,claude-code-agent-teams,claude-code-subagent-memory,cognition-multi-agents-working,langchain-multi-agent-benchmark,ace-context-collapse,mast-mas-failures,chroma-context-rot,blackboard-llm-mas,letta-sleep-time

About
: organising-agents


<a id="keeper-of-a-shared-resource"></a>
### keeper-of-a-shared-resource

Date
: 2026-11-17

Brief
: Test farms, merge queues and the actor model all give a shared resource one owner that serves a queue.

Question
: How should many agent sessions share one resource that only one may change at a time - a test device, a lab machine, a release or merge queue - so that work flows without races and without a session kept alive forever?

Found
: Everyone who shares such a resource gives it one owner that serves a queue. Hardware test farms: LAVA has a single scheduler that assigns devices to submitted jobs (Mesa reaches its boards only through it); CI-tron puts one gateway in charge of its devices under test, and postmarketOS runs its phones behind one. Gas Town gives the merge queue to one role, the Refinery, which merges every change to main one at a time; the role outlives any session - a new session for it takes up the next step, nudged by patrols. The actor model and the single-writer principle say the same for state: one owner, reached only by messages it takes one at a time. Anthropic's research system parallelises only what is independent. Put with this commons' own finding that experts are written down, not standing (decision 2026-11-17-experts-written-not-standing): the keeper is a lasting role whose sessions are disposable - each works until its piece lands or its context grows large, writes its hand-over into the resource's how-it-stands record and ends; others prepare in parallel and queue items with their proofs; a named query over the backlog is the queue; a schedule or patrol wakes the role when the queue has work and no keeper is live; and the resource's own tool still holds the lock for every change, so the keeper adds flow, not safety. Raised by the author on 2026-11-17 for the lab's phone.

Sources
: lava-scheduler,mesa-lava-ci,ci-tron-introduction,pmos-hw-ci-status,gastown-refinery,actor-model

About
: keeper


<a id="look-around-habit-world"></a>
### look-around-habit-world

Date
: 2026-11-17

Brief
: Proposal processes build a small look around in - prior art, alternatives, rejected ideas - written beside the choice; agents load it just in time.

Question
: How do others make looking around - where things stand, how others do it - part of acting on a proposal, and keep what was found without loading every later reader?

Found
: Proposal processes build it in as a section: Rust RFCs ask for prior art and alternatives, PEPs keep rejected ideas, MADR lists considered options - a small bounded look, written beside the choice, not a study. A redo supersedes, never deletes (PEP Replaces). Agents load such know-how just in time: a skill costs a line until used, research a row until questioned (Anthropic's context engineering, Claude Code skills). In the commons, practice 22 already holds the habit; it lacked the author's own proposals and a place for the material, which idealkb 0.2.0 gives (research row with material in research/SUBJECT/YYYY-MM-DD/, supersedes, a summary per subject).

Sources
: rust-rfc-template,pep-12,madr,anthropic-context-engineering,claude-code-skills

About
: practices

Material
: research/22-look-around-first/2026-11-17/


<a id="housekeeping-the-start"></a>
### housekeeping-the-start

Date
: 2026-11-17

Brief
: A fresh linux-phone session read 35-46 KB before acting, half of it the practices' reasons; measure per role, budget it, patrol measures.

Question
: How should ideal-agent and each project's agent folder and knowledge base be kept so that what a fresh agent must read before acting stays small - what is measured, what calls for housekeeping, who does it and how often, and what it may change by itself?

Found
: Measured on linux-phone-oco, main, 2026-11-17 (scripts/start-weight): a fresh session read 35-46 KB before its first useful action (bring-up 45.8, orchestration 38.7, phone-keeper 37.2, feature-research 35.0): the session start 22.5 (the 44 practices with their reasons - half of everything), skill descriptions 3.3-3.6, lab-session-start 2.0, docs' AGENTS.md 2.1, the agent folder's README 2.3, and the brief (bring-up 13.2, orchestration 6.1, phone-keeper 4.6, feature-research 2.4). Printing the practices without their reasons at the start cut the session start to 13.9 KB (bring-up 37.2); the practices' own words are 13.7 KB, their first sentences 7.9. In the commons: the canon's cold read (give only the front files and ask what the program does) and the playbook's findability test (real questions, each answered near the top) test a start by asking it; idealkb measured the bytes an answer costs (kb-navigation-measured: 206 KB of pages against 9.4 KB of rows) and made a summary layer that says itself when a summary is due (over 8 KB with none, or 4 KB since). Outside: Wikipedia keeps upkeep as a standing backlog - a tag proposes, anyone fixes and removes it; Claude Code keeps what is loaded every session under 200 lines and moves the rest where it loads only when touched; context engineering keeps identifiers (paths, stored queries) in context and loads content just in time; freshness for agents is metadata and verification, since an agent trusts what it reads; Gas Town's patrols are scheduled rounds that look and file work rather than do it. So: measure the start (bytes per role) and the answers (the base's fixed questions); a budget calls for housekeeping; the patrol measures, the owner simplifies; by itself only what adds or condenses, never what removes words.

Sources
: wikipedia-maintenance,claude-code-memory,docs-freshness-for-agents,anthropic-context-engineering,gastown-refinery

About
: housekeeping


<a id="what-a-started-agent-reads"></a>
### what-a-started-agent-reads

Date
: 2026-11-17

Brief
: A started session holds 73-81k tokens before reading anything, nearly all Claude Code and its connectors' tools; our start is about 4-6k; keepers read 10-12k more before acting.

Question
: What does a started agent actually read before and while it works - per role, from its transcript - and where is the weight?

Found
: Measured 2026-11-17 with scripts/read-weight on the transcripts. First turn, before anything is read: 80.0k tokens (surya keeper 1), 81.3k (keeper 2), 79.7k (ideal-agent), 73.5k and 75.6k (two scheduled keeper-waker runs) - Claude Code's own instructions and the tools of every connector the session carries (Gmail 31 tools, Firecrawl 18, Claude Docs 8, the browser, the artifact tool); of it ours is about 4-6k: the session start 2-9 KB (the practices; 8.6 KB since first sentences), the skills' descriptions 4 KB, the brief 2.6-5.9 KB. Keeper 1 (session 8b8368b4, brief 4.4 KB): read its brief, the queue and devlab's help and touched the phone 22 seconds after it began, at 90k tokens; over its whole run it read 24 KB of briefs, records and help in 13 results against 91 KB of phone output in 59, wrote 98k output tokens and ended at 189k - its context went to the work and its own reasoning, and most of what it held at the start was the harness. Keeper 2 (session 3060e909, lean brief 2.6 KB): 33.5 KB read before its first act - 12.1 KB of it tad's --help, explored because no command was named, 9.4 KB the deep-sleep subject whole, 4 KB a mistyped about - then 92k tokens at the act. So: what can be cut at the start is small beside the harness, and the reading that grows is help and whole records read for want of a named command or a brief; the larger lever is starting a role without connectors it does not use (the author's setting).

Sources
: anthropic-context-engineering,claude-code-memory

About
: housekeeping


<a id="long-lived-roles-renewed"></a>
### long-lived-roles-renewed

Date
: 2026-11-17

Brief
: Long-lived sessions held 560-926k tokens after 730-1,385 turns, re-reading 255-747M in all, never compacting; a written hand-over and a fresh start cut a turn to about 83k.

Question
: How should a long-lived role - coordinator, orchestration, a keeper - keep its context small, when should it renew itself, and how?

Found
: Measured 2026-11-17 from the transcripts: the coordinator held 840k tokens after 1,222 turns in 13 hours and had re-read 574M tokens in all; the other long sessions 783k (548M, 11 h), 926k (747M, 9 h) and ideal-agent itself 560k (255M, 3 h); none had compacted. A fresh coordinator from its brief's first paragraph (0.5 KB), its hand-over row (2.7 KB) and the queue's top (1.9 KB) starts at about 83k tokens with the harness - a tenth of a turn now. Claude Code compacts automatically near the window's end, keeping whatever its summary chose; Anthropic's guidance clears tool results that can be fetched again and keeps notes outside the window (a run's end context 14k instead of 334k); Gas Town hands off after every task, the role's identity carrying the work to a fresh session. The hand-over written into the role's record keeps what matters and is read by any next session; a subagent can read a whole transcript and return the summary (the coordinator's 12.9 MB in 2.5 KB, its 77k tokens never entering the asking session).

Sources
: claude-compaction,context-engineering-cookbook,gastown-handoff,anthropic-context-engineering

About
: housekeeping


<a id="ideal-agent-how-it-stands"></a>
### ideal-agent-how-it-stands

Date
: 2026-11-17

Brief
: ideal-agent hand-over, 2026-11-17 13:30 UTC: the plugin 0.2.6 is on the lab; open work is release-binaries, guard-stop-unlanded and measuring a keeper without connectors.

Question
: Where does the ideal-agent session's work stand, for a fresh session to take it up?

Found
: ROLE: keeps grenudi/ideal-agent, the one home of general agent know-how - its tad database (practices, decisions, research, landscape, subjects; follows ideal-knowledge-base 0.5.0) and the Claude Code plugin (skills, hooks, the researcher agent; scripts/ideal-agent, start-weight, read-weight), installed on the lab (claude plugin update ideal-agent@ideal-agent after each landing; the next session start builds the program). Everything else is the domain's: linux-phone in linux-phone-oco/docs (orchestration local_325ef65d owns it), the knowledge-base shape in grenudi/ideal-knowledge-base (build session local_b8ca2fb5 owns it; it checks followers before each release). SESSIONS: coordinator local_12b18c70 (the author's; renews from docs' coordinator-how-it-stands), orchestration local_325ef65d, idealkb local_b8ca2fb5, summaries local_52d57a2a. IN FLIGHT: tad q next - release-binaries (hooks without cargo), guard-stop-unlanded (a Stop hook), measure-start-without-connectors (orchestration sends the next keeper's read-weight first-turn tokens; before 73-81k). STANDING WORDS: nothing depends on ocoeco; tad and idealkb only; look around first with sources; briefs first, read deeper only when a step needs it; keep the start light (budgets in keep-the-start-light); practices are the author's words - propose, never rewrite; Rust or shell, never Python, executables in scripts/. DON'T: run ocoeco check on a tad place (landit no longer does); copy whole files from an old clone; read a whole transcript yourself - give it to a subagent. Fresh start: read the first paragraph of README.md and this row (tad q row --id ideal-agent-how-it-stands), nothing else until a step needs it.

About
: ideal-agent
