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


<a id="agent-roles"></a>
### agent-roles

Date
: 2026-11-17

Brief
: Roles already exist in practice (seven); Claude Code's agent card fits them, but desktop sessions start from a prompt, so the role is read from the first prompt

Question
: What roles have emerged, how do others define a role and load only it, and how can a started session receive only its role's part of the general instructions?

Found
: Seven roles worked on 2026-11-17: coordinator, lane coordinator, orchestration, keeper, research lead, builder, patrol; their briefs repeat generic text and every session read all 45 practices (8.6 KB). Claude Code's agents/NAME.md carries model, effort, skills and a body, and claude --agent starts a session as it; start_session cannot, so a UserPromptSubmit hook reads Role: NAME from the first prompt. Gas Town primes each role at start; CrewAI, AG2, OpenAI SDK keep a role to a few fields; RACI and Team Topologies give owns, never and reports to.

Sources
: claude-code-subagents,claude-code-agent-teams,anthropic-multi-agent-research,gastown-roles,crewai-agents,team-topologies,anthropic-context-engineering

About
: organising-agents

Material
: research/organising-agents/2026-11-17/


<a id="ideal-agent-how-it-stands"></a>
### ideal-agent-how-it-stands

Date
: 2026-11-17

Brief
: ideal-agent hand-over, 2026-11-17 evening: plugin 0.3.4 - roles, the neutral start agent-env, practices without ocoeco's tools; open: measure-a-role-start, release-binaries.

Question
: Where does the ideal-agent session's work stand, for a fresh session to take it up?

Found
: ROLE: keeps grenudi/ideal-agent, the one home of general agent know-how - its tad database (practices, decisions, research, landscape, subjects; follows ideal-knowledge-base, pinned 0.5.0, the weekly pin job moves it to 0.7.1) and the Claude Code plugin (skills, hooks, the role cards and the researcher agent in agents/; scripts/ideal-agent, start-weight, read-weight), installed on the lab at 0.3.4 (after each landing: claude plugin marketplace update ideal-agent, claude plugin update ideal-agent@ideal-agent; the next session start builds the program). Skills record with ikb and read a base's IKB.md (ideal-agent #25, by the ikb session local_2017aeff). The domain is elsewhere: linux-phone in linux-phone-oco/docs, owned by orchestration (fresh session local_7d95b264; hand-over orchestration-how-it-stands); the knowledge-base shape in grenudi/ideal-knowledge-base (build session local_b8ca2fb5; it checks every follower before a release). SESSIONS: ideal-agent owner ideal-agent-roles (this hand-over's writer); coordinator local_12b18c70 (the author's), orchestration local_7d95b264, idealkb local_b8ca2fb5, ikb local_2017aeff. ROLES (decision 2026-11-17-roles-as-agent-cards): seven cards in agents/ - coordinator, lane-coordinator, orchestrator, keeper, research-lead, builder, patrol; a brief names one (Role: NAME, Also practices: N) and the UserPromptSubmit hook (ideal-agent role) gives its card and only its practices at the first prompt; a new role is a card, never a new layer. START (decision 2026-11-17-a-neutral-start): scripts/agent-env, named in CLAUDE_ENV_FILE by the session start, gives every Bash command the token, PATH, AGENT_NAME (the brief's Name: line, else the role) as git author, branch and land - no ocoeco, no . oco-work; OCOECO_BY kept equal for places still on ocoeco. A role is read only from a new session's first prompt, its first 8 lines, never a tagged message (0.3.2). PRACTICES (decision 2026-11-17-practices-without-oco-tools): sixteen reworded in tad and ikb terms, 40 and 41 retired - a retired field keeps when and why; ikb practice-retired; the start skips them. The briefs of linux-phone name their roles (docs #230). IN FLIGHT (tad q next): the briefs' trimming of what the cards say (orchestration); measure-a-role-start (read-weight on the first role-named session on 0.3.0); measure-start-without-connectors - before 81,280 tokens first turn (keeper 2, by hand); the first after (a scheduled run, 75,649) is not comparable; orchestration sends a scheduled-route keeper started after Gmail and Claude Docs were turned off; orchestration takes tad q phone-queue --next into the keeper brief. release-binaries (hooks on machines without cargo); guard-stop-unlanded (a Stop hook). STANDING WORDS: nothing depends on ocoeco; tad, idealkb and ikb only; look around first with sources; briefs first, deeper only when a step needs it; keep the start light and hold little (keep-the-start-light: budgets, read-weight, renew a long-lived role at 250k, or 150k after a batch or an idle hour); the practices are the author's words - propose, never rewrite; Rust or shell, never Python, executables in scripts/; bump plugin.json and Cargo.toml together. DON'T: write a file before fresh - it cleans untracked files (read-weight was lost so once); run ocoeco check on a tad place; read a whole transcript yourself - give it to a subagent. Fresh start: the first paragraph of README.md and this row, nothing else until a step needs it.

About
: ideal-agent


<a id="start-health"></a>
### start-health

Date
: 2026-11-17

Brief
: Every role's start lists ~18 KB of skills no session of any role used; measuring it is a rule (read-weight: given, not used), trimming is each owner's - no analogue keeps a standing role

Question
: Should a role (agents-healthkeeper) look at each role's bring-up and perfect the role cards, or does that fold into existing housekeeping?

Found
: Today's keeper, lane coordinator and research lead each held ~80k tokens at the first turn: 22.5 KB of skills listed (Anthropic's 16 and Claude Code's 17, ~18 KB, used by none), 6.5 KB of agents (none used), 9.7 KB of deferred tools (session connectors only). The concept existed before roles: decision keep-the-start-light (patrol measures, owner simplifies). Gas Town's patrols watch liveness, not priming; SRE measures toil by survey and cap, owners fix; friction logs are written by newcomers; Claude Code gives /context, /skill-doctor, skillOverrides and skillListingMaxDescChars but no per-agent hiding.

Sources
: claude-code-skills-listing,claude-code-subagents,anthropic-context-engineering,gastown-roles,sre-eliminating-toil,resend-friction-logs

About
: housekeeping

Material
: research/housekeeping/2026-11-17/


<a id="work-struggles"></a>
### work-struggles

Date
: 2026-11-17

Brief
: 5.6% of 5,375 tool calls failed on 2026-11-17; Python refusals, land's --manual, SQL past named records, sleep and session starts lead; others detect by script, cluster now and then, owners fix

Question
: Is a role needed that finds what a role's agents struggle with while working, and is it one with the start-health measure?

Found
: read-weight's struggled-with over 41 transcripts: 301 of 5,375 calls failed - Python refused 27, land's --manual and --help 21, sleep refused 17, checks failed 11, start_session refused 8, ssh to the phone refused 7, oco-work turn failed 5; 56 tad sql writes past named records in 13 sessions (backlog fields 22). Anthropic, Microsoft, Langfuse: read transcripts for tool errors and first wrong calls, cluster, owners fix; SRE and paper cuts: owners decide. Detection shares one pass with start health. Before it, by hand: ikb-cli's count (ideal-knowledge-base research ikb-friction-and-world) - 17% of tad r calls failed, ~11k tokens each; ikb then brought failures to 0-1 in 4 runs at equal tokens.

Sources
: anthropic-writing-tools,microsoft-agent-trajectories,langfuse-error-analysis,mast-mas-failures,sre-eliminating-toil,swarmia-paper-cuts,ikb-friction-in-transcripts

About
: housekeeping

Material
: research/housekeeping/2026-11-17/


<a id="lab-agent-what-it-is-2026-11-17"></a>
### lab-agent-what-it-is-2026-11-17

Date
: 2026-11-17

Brief
: lab-agent's job runner idle since 2026-11-12; live today: root-exec, token file, board/inbox/notify; oco-agent runs walleye builds; ocoeco jobs dead

Question
: What is lab-agent used for today, what depends on it, what supersedes each part?

Found
: # lab-agent: what it is used for today, what depends on it, what supersedes each part (2026-11-17)

Measured on the lab (grig) 2026-11-17 ~17:30-18:20 CET, nothing changed. A first look by orchestration
(side-session-notes lab-agent-what-it-is-2026-11-17.md) was the lead; every claim below was re-checked.

## What it is
creation-guidelines/lab-agent (Rust, 0.1.53; /usr/bin is 0.1.52, ~/.local/bin 0.1.53 is what runs - a drop-in
`use-the-fixed-build.conf` points the unit there). Begun 2026-11-04 so a claude.ai sandbox that could not reach
the lab could push numbered shell scripts to grenudi/lab-agent-jobs; the lab polls (every 5 s, GitHub API), runs
each, pushes results to grenudi/lab-agent-results. Around that grew: root-exec, notify/notify-setup, board,
inbox, ask, guide, self-update, connect, monitor, submit/wait/logs/jobs. The same repo holds oco-agent
(workspace dir oco-agent/), the redesign after the 2026-11-11 stall (decision 2026-11-12-oco-agent).

## 1. Use today (measured)
Jobs, lab-agent: 1165 job files ever added to the jobs repo (git history). By the day in the file name's epoch:
11-09 184, 11-10 526, 11-11 293, 11-12 81, 11-15 2 (two probes, "probe-the-way-in", "a-driver-speaks-with-git-alone"),
11-13/14/16/17 none. Last real job 2026-11-12 04:09 (the-switch-1-proven). The daemon is idle since: its journal
since 11-16 is only DNS failures reading board and inbox, two self-switches (10:58, 13:03 today) and a
"1137 jobs already have results" line. Real work (phones, builds, landing) runs in Claude sessions on the lab.
Jobs, oco-agent (same jobs repo, oco-agent/jobs): 956 files 11-12..11-16 (133, 135, 188, 203, 305 per day), 228 of
them "window" (`. oco-work; said ...` - ocoeco speaking), 62 of the 400 newest carry `by=claude`; last 11-16 23:50.
That traffic is ocoeco's own. TODAY, one non-ocoeco use: walleye-builder (session 04b24088) submitted
walleye-kernel-build-2..4, walleye-busybox-1, walleye-images-1, walleye-sleep-test through `oco-agent submit/run/wait/log`;
`oco-agent standing`: kernel-build-1 failed, -2 and -3 "parked: the agent restarted while doing it", -4, busybox, images,
sleep-test finished. That is the only job runner use measured today, and it is a long build that must outlive a session.
root-exec: used today by three sessions (220b014f fastboot devices; 7007ce97 dmesg, /proc/PID/stack, `pacman -S`
perl-io-socket-ssl; 8b91886a only my own greps) and by lab-machine setup (nix.sh, locale.sh) and devlab. In use.
board: board.json has 42 tasks; 40 last touched 2026-11-10/11 (surya-audio, oco, ocoeco, "rust-toolset": stale). Live: `sim`
(touched 11-16 23:52 by a session) and health-xiaomi-surya (set/done by `devlab watch` 11-17 09:35/14:50). Each change
is pushed to the author's phone on the ntfy info topic: 10 pushes today (cache window ~12 h), titled "lab-agent:
what I'm doing", body "You received a file: attachment.txt".
inbox: runs at every session start (the plugin hook; 33 times in today's transcripts, always empty at the start of this one).
The ntfy inbox topic holds nothing in its cache window (~12 h); older traffic is NOT KNOWN. 4 `inbox --ack` today.
guide: 33 calls today = the session-start hook's neighbours, plus the plugin's own start prints practices; the plugin's
`scripts/ideal-agent practices` is the same table. notify: one `notify --urgent` today (220b014f). ask: none today. self-update:
lab-agent-update.timer daily (last 11-17 00:58). Shadow timer (10 min), oco-ask-to-observe (15 min, ~44 GitHub requests each)
and oco-say-what-is-new (5 min): ocoeco-era timers, all running.
Units active: lab-agent.service (inhibits idle), oco-agent.service (`run --listen grenudi/lab-agent-jobs`, inhibits sleep),
devlab-watch@surya-lab, 4 oco timers + lab-agent-update.timer.

## 2. Dependents (grep of ~/Projects, ~/.local/bin, the plugin, units)
- Token file ~/.config/lab-agent/token: lab-machine/bin/land:31; ideal-agent scripts/agent-env:19 (GH_TOKEN for every session);
  lab-machine/systemd tad-update-pins.service:10, idealkb-update-pins.service:12; oco-agent command-line/src/main.rs:31-40
  (falls back to it). ~/.config/lab-agent/env holds a second token (LAB_AGENT_GITHUB_TOKEN) for the daemon. Only the FILE.
- root-exec: sudoers /etc/sudoers.d/lab-agent (written by `lab-agent connect`, connect.rs:127-): `mll ALL=(root) NOPASSWD:
  /usr/bin/lab-agent` (`sudo -n -l` confirms; /usr/bin/lab-agent is root-owned; ~/.local/bin copy is NOT granted). root_exec
  (main.rs:119) runs ANY argv as root: root for anything, by design. Callers: devlab flash.rs:12, provision.rs:57,95
  (DEVLAB_ROOT overrides), guard.rs:179 (a rule's text); lab-machine setup/nix.sh:15, locale.sh:6; sessions ad hoc.
- board: devlab watch.rs:22-24 (LAB_AGENT overrides; unit devlab-watch@.service sets LAB_AGENT=/usr/bin/lab-agent).
- inbox/guide: plugin src/main.rs:198 runs `lab-agent inbox` at session start; skill lab-session-start names it; practice 04
  says "lab-agent inbox"; practices.rs:92 and tables.toml:18 name `lab-agent guide`. (Practice 06 now says "through a lab job", not oco-agent.)
- self-update: lab-machine/systemd/lab-agent-update.{service,timer}.
- units: lab-agent.service, stay-awake.conf, use-the-fixed-build.conf, oco-agent.service (+ shadow, observe, say timers).
- oco-agent: walleye-builder's jobs (today); marshall-oco/tad-engine oco/ tables and ocoeco (`. oco-work`) - legacy.
- Not found: any cloud/sandbox driver submitting after 11-15; any caller of `ask`, `submit`, `wait` outside the jobs themselves.

## 3. Parts, what supersedes, what would be lost, cost
See the decision 2026-11-17-lab-agent-retires-by-parts for the table and the order.

## Sources
repo creation-guidelines/lab-agent (src/main.rs, connect.rs, notify.rs, oco-agent/README.md, oco/decisions.json),
grenudi/lab-agent-jobs (git history, board.json, oco-agent/jobs), lab journal (journalctl --user -u lab-agent),
`sudo -n -l`, `systemctl --user list-units/list-timers`, ntfy topic caches, ~/.claude/projects transcripts of 2026-11-17.

Sources
: lab-agent-repo,lab-agent-jobs-repo

About
: organising-agents

Material
: research/organising-agents/2026-11-17/


<a id="harness-specifics-where"></a>
### harness-specifics-where

Date
: 2026-11-17

Brief
: Others keep the method in open formats (AGENTS.md, SKILL.md, md roles) and a thin dated layer per harness; our base already has dated, sourced, superseded research per subject

Question
: Where do harness and model specifics live, so the practices, roles and skills stay agent-agnostic and each specific is dated, sourced and superseded, without tracking the model industry?

Found
: AGENTS.md (most harnesses), SKILL.md under the Agent Skills spec (Claude Code, Codex, Gemini CLI, Cursor, ~40 more) and markdown roles converge; each harness adds its own keys, paths and manifest. ruler and rulesync generate one source into ~20 harnesses and keep an end-of-life list. Dated last-reviewed records and staleness checks (Optimism, Microsoft hve-core). ideal-agent's skills use only name and description; its base has research with date, sources and supersedes, and a summary per subject.

Sources
: agent-skills-spec,agents-md,rulesync,claude-code-skills-listing

About
: practices

Material
: research/claude-code/2026-11-17/


<a id="claude-code-skill-overrides"></a>
### claude-code-skill-overrides

Date
: 2026-11-17

Brief
: 2026-11-17: the 33 skills no role used are name-only in the author's settings; before, every start listed 44 skills in 22,455 B; the after is measured on the next fresh session

Question
: How much of every session's start does skillOverrides name-only take away, for the skills no role uses?

Found
: The author chose it (his words, through the coordinator, and his yes in ideal-agent-roles' session): ~/.claude/settings.json skillOverrides name-only for Anthropic's 16 skills and Claude Code's 17 built-ins - documents, slides, spreadsheets, PDF, Google Workspace, browsers, computer use, morning, deep research, dataviz, artifacts, code review, the Claude API, schedule, loop and the rest - all still callable by name; ideal-agent's 11 untouched. Before: 44 skills listed in 22,455 B at every start (read-weight, 2026-11-17). After: not known yet - the next fresh session's read-weight.

Sources
: claude-code-skills-listing

About
: claude-code

Material
: research/claude-code/2026-11-17/


<a id="decision-routing"></a>
### decision-routing

Date
: 2026-11-17

Brief
: Others decide at the lowest level holding the information: reversible decisions fast, the worker escalates and goes on, research bounded, recorded first; a small set of classes reaches the human

Question
: Who should decide a question a keeper cannot, so that a reversible choice does not wait on the author, and what must still reach him?

Found
: The walleye lane waited on the author three times on 2026-11-17 (a boot go, a session start, keeper-1's set_active a, a reversible flag change); no card named a decider for a lane's question. Bezos: two-way doors fast, escalate at once; RAPID: one decider; SRE: the commander holds what is not delegated; Anthropic: the lead decides on more research, bound it; Gas Town: escalate, do not wait, keep working, track every escalation; auto mode and LangGraph: humans for irreversible, outward, money, security. Reversible holds only where the way back is shown.

Sources
: bezos-2016-letter,bain-rapid,sre-managing-incidents,anthropic-multi-agent-research,gastown-escalation,claude-code-auto-mode

About
: organising-agents

Material
: research/organising-agents/2026-11-17/


<a id="lane-starts"></a>
### lane-starts

Date
: 2026-11-17

Brief
: Seven lane pauses on 2026-11-17 waited 4-40 min for the author's click; run_scheduled_task starts a session on a message's turn where start_session is refused

Question
: How can a lane coordinator start its keeper, research and builder sessions on its own, so the author is stopped only by his bounds?

Found
: Seven start refusals on the surya and walleye lanes (13:08-16:58) became cards that waited about 4 to 40 minutes for the author's click; a lane coordinator wakes on its keepers' reports, so nearly every turn of its is a message's, where start_session is refused. A probe showed run_scheduled_task starts a new session from such a turn; it runs once at a time per task, keeps a permission mode only when the author created the task, and wraps the prompt in <scheduled-task>. SendMessage to a standing session works but its renewal (clear_session) asks the author; subagents work within the turn; a headless run is not logged in and not seen.

Sources
: gastown-escalation,claude-code-skills-listing

About
: organising-agents

Material
: research/organising-agents/2026-11-17/


<a id="reading-is-not-acting"></a>
### reading-is-not-acting

Date
: 2026-11-17

Brief
: Two official, checksummed image downloads reached the author on 2026-11-17 as 'outward'; others gate what is done with bytes, not reading them

Question
: Which technical asks reach the author though they need not, and how should the bounds be worded so reading from the world is the lane's?

Found
: Today's asks to the author were mostly his by right (design, hands and witness, acting in his name, his settings, a key); the technical ones were device writes and session starts, now settled, and two downloads of LineageOS's official walleye images the lane read as 'anything outward'. Claude Code auto mode passes reads and does not block downloads, only running external code and sending data out; Copilot's firewall allows official sources; SLSA verifies against a separately published hash.

Sources
: claude-code-auto-mode,copilot-agent-firewall,slsa-verifying-artifacts

About
: organising-agents

Material
: research/organising-agents/2026-11-17/


<a id="research-on-own-initiative"></a>
### research-on-own-initiative

Date
: 2026-11-17

Brief
: Research starts only when a keeper asks; practice 22 is on 4 of 7 cards and in no domain brief; docs has upstream records but not ideal-agent as an upstream

Question
: Should coordinators start research on their own when a method is missing, how should a general finding flow back into ideal-agent, and should every role treat the author's words as a starting point?

Found
: Inside: decision decisions-go-to-the-lane starts research only when a keeper cannot decide; the walleye lane researched walleye but not the general method of bringing a phone up from nothing; practice 22 and look-around-first ('a proposal is a starting point, the author's too') are on the coordinator, orchestrator, research-lead and builder cards, not keeper, lane coordinator or patrol, and no linux-phone brief points to them; docs keeps upstreams and upstream_items, without grenudi/ideal-agent. Outside: Anthropic's lead decides itself whether more research is needed; SRE postmortems turn findings into owned action items in the owner's system; Argyris's double loop revises the rules from what was found; Gas Town tracks every escalation.

Sources
: anthropic-multi-agent-research,sre-postmortem-culture,argyris-double-loop,gastown-escalation

About
: organising-agents


<a id="device-from-nothing"></a>
### device-from-nothing

Date
: 2026-11-17

Brief
: linux-phone's walleye lane: a device is brought up feedback channel first, then a known-good boot, then mainline; and stock-vs-ours is bisected one part at a time

Question
: Is there a general order for bringing a device up from nothing, before its first part, that bring-up-a-part should carry?

Found
: Upstream item from linux-phone-oco/docs (walleye-coordinator, research walleye-bootstrap-method-how-it-stands, decision 2026-11-17-walleye-vendor-kernel-first, docs #307; 14 sources: postmarketOS porting and mainlining guides, Qualcomm mainline porting, Halium early-init, MSM8998 ports, ramoops, the debug cable): every sibling port had a channel or a known-good first; walleye went straight to mainline images the bootloader accepted and that never started, and learned nothing until the accepted stock image was repacked with only the ramdisk swapped.

Sources
: pmos-porting-new-device,pmos-mainlining-guide

About
: skills


<a id="domain-layer"></a>
### domain-layer

Date
: 2026-11-17

Brief
: linux-phone's agent folder: 49% restates ideal-agent (mostly paraphrase), two briefs contradict it; others pin a base and hold deltas, and none checks restating

Question
: Where should a domain's layer over ideal-agent live, how does it mirror ideal-agent without copying it, and how does it pin, follow and send back?

Found
: docs' ideal-agent-linux-phone-oco: 26 KB, 49% general, 36% domain, 15% one lane; phone-keeper and surya-coordinator contradict the current cards; overlay-check flags feature-research 27%, orchestration 22%, README 17% (6-word runs). No device repository has a base or an agent folder yet. Others: plugin dependencies by semver range, Helm library charts, Kustomize overlays, Debian Standards-Version - a pinned base, an overlay of deltas; nobody checks restating.

Sources
: claude-code-plugin-dependencies,helm-library-charts,agents-md,claude-code-subagents

About
: organising-agents

Material
: research/organising-agents/2026-11-17/


<a id="claude-code-how-we-use-it"></a>
### claude-code-how-we-use-it

Date
: 2026-11-17

Brief
: Claude Code 2.1.321 (2026-11-17): the hooks, env file, role cards, session starts, waits, transcripts and skill-listing settings our method uses - nothing else

Question
: What of Claude Code does ideal-agent's method use, and how, as of now?

Found
: SessionStart and UserPromptSubmit hooks carry the role and practices (stdout is context; agent_type with --agent; no env file for UserPromptSubmit); CLAUDE_ENV_FILE is evaluated before each Bash command (agent-env); started sessions' first prompt begins with the app's reminder and the starter's Background...Task:; agents/NAME.md cards (model, effort, skills preload only); start_session is refused on a message's or event's turn; sleep before a command is refused (background, Monitor); transcripts in ~/.claude/projects; skillOverrides and skillListingMaxDescChars trim the listing, no per-agent hiding; connectors per session from the + menu; plugin updates need a restart. A lane's research within its turn: the Agent tool, subagent_type ideal-agent:researcher, model sonnet (Sonnet 5.5). A launcher run's end wakes its lane coordinator through update_scheduled_task notifyOnCompletion true, one subscriber per task. notifyOnCompletion's wake was not seen by either lane on the night of 2026-11-17; a run still closing refuses a new run; a background wait (run_in_background) re-invokes the session.

Sources
: claude-code-subagents,claude-code-skills-listing

About
: claude-code

Material
: research/claude-code/2026-11-17/


<a id="programmer-role"></a>
### programmer-role

Date
: 2026-11-17

Brief
: The author's canon is three sources that overlap and that ideal-agent never names; today's builders slipped on argenv, C and CI; others split tools, a writer's core, a fresh reviewer

Question
: Should ideal-agent have a dedicated programmer that applies the author's software canon, so other roles need not carry it - and in what shape?

Found
: software-engineering-canon (tad base, 44 ids), ideal-project-playbook (37 concepts, an agent kit), design-canon (one README, likely its earlier version) overlap in about ten rules; ideal-agent cites none. Today: devlab and tone changed and still on clap; a C helper added then replaced by Rust but left in the tree; walleye's lab CI runs no cargo. Others: tools enforce the checkable, the writer carries a short core, a separate reviewer in a fresh context judges the rest (Claude Code subagents, Google's review standard, ChatDev); no study isolates a reviewer against a guide.

Sources
: claude-code-subagents,google-eng-practices-review,chatdev

About
: organising-agents

Material
: research/organising-agents/2026-11-17/


<a id="stored-starts"></a>
### stored-starts

Date
: 2026-11-17

Brief
: Seven stored starts hold 17 KB of prompts that restate domain rules; the patrol's ran an old copy on 2026-11-17; the patrol card lacked lanes and host

Question
: How should scheduled roles (the patrol) and on-demand ones (launchers) start, so a change to a card or a brief reaches every start with no hand edit?

Found
: Stored tasks on 2026-11-17/18: linux-phone-patrol 5,201 B, lab-tools-builder 2,330, walleye-keeper 2,333, walleye-builder 1,887, walleye-research 1,813, surya-research 1,747, surya-keeper 1,658 - each a Role head then restated work queries, worktree routine and allowed writes; the patrol's ran an old copy using ocoeco and filing everything in docs, and was stopped. docs has no patrol brief. A scheduled run's wrapper says to make only the writes its task asks for. The patrol card had role health but not idle lanes, stale waits or the host.

Sources
: claude-code-subagents

About
: organising-agents


<a id="after-a-fix"></a>
### after-a-fix

Date
: 2026-11-18

Brief
: Three of the author's asks on 2026-11-17/18 were the same missing step: after a local fix, nobody asked what class it was, where else it bites, and what at the root would prevent it

Question
: How can roles notice that a fixed incident points at a wider gap, and propose or start the research themselves, without a flood?

Found
: Cases: surya's suspend loop ran a hand-copied devlab-usb-awake for a week after main changed (surya-lab #195-207) - the author asked whether the fix reaches other phones and upstream; device declarations as typed Rust rather than drifting config; coordinators researching on their own and a patrol. ideal-agent had research on a missing method, not after an incident. Others: SRE postmortems on triggers (on-call intervention, a monitoring failure, data loss, a repeat), shared and reviewed; jidoka stops at the first abnormality to fix the root; vendors: a light note for minor incidents, one review per recurring class, the exposed team not waiving its own.

Sources
: sre-postmortem-culture,lean-jidoka,postmortem-trigger-criteria,rootly-postmortem-template

About
: organising-agents


<a id="2026-11-18-lab-agent-still-involved"></a>
### 2026-11-18-lab-agent-still-involved

Date
: 2026-11-18

Brief
: linux-phone-oco names lab-agent in 5 code lines (the root prefix); the job route is replaceable by systemd-run; every other part waits on the author

Question
: How is lab-agent still involved, what is each use for, where should it live, why did the retirement stall, should the job route exist, what replaces root-exec, and is the 401 fix worth it?

Found
: Counts corrected for 83 worktree clones: root-exec 90 lines/78 files but 5 code lines; guide 54 (briefs); runs 18 (prose); inbox 0. Plan at step 1 of 6 because the five others wait on the author's words. systemd-run --user works from a session on the lab (probe run, Linger=yes), so the job route is not needed; walleye's builds are its only use. root-exec: wrapper first, per-command grants later (author's decision). Others (GitHub runners, buildbot, LAVA) all exist because the controller is remote; take their short-lived credentials and visible bad state. The 401 fix was worth its cost; extend nothing.

About
: organising-agents

Material
: research/organising-agents/2026-11-18/


<a id="2026-11-18-lpoco-root-from-one-roof"></a>
### 2026-11-18-lpoco-root-from-one-roof

Date
: 2026-11-18

Brief
: From lpoco's roof root is three fixed kinds of need (etc files, image bake, phone network); best shape: no standing passwordless root, a RootNeed enum declared once

Question
: What root do lpoco-host, lpoco-lab and lpoco-bringup actually need, how should lpoco declare it once, and what grant shape follows?

Found
: Read from lab-tools source: lpoco-host asks root for udev/modules-load files, udevadm, modprobe, link config (rare, author-present); devlab flash for a loop-mount image bake (losetup, mount, cp, install, ln, umount); devlab provision for host networking (sysctl, nft table, ufw route); bringup, health, tone none. Declare as one RootNeed enum in the manifest crate with typed params, read by plan, the sudoers fragment and the check. Shapes: 1 no standing grant (sudo prepare by the author, a udev-started root unit for the phone network, rootfs built without mount); 2 one root-owned helper restricted to the enum; 3 per-command sudoers rejected (install/mount cannot be narrowed); 4 general wrapper rejected by the author. Two probes still to run; sudoers untouched.

About
: organising-agents

Material
: research/organising-agents/2026-11-18/
