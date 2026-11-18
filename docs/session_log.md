<!-- Generated from data/ by tad. Do not edit by hand. -->

# Session Log

<a id="2026-11-17-ideal-agent-made"></a>
### ideal-agent made, and the general know-how moved in

Entry date
: 2026-11-17

Done
: Created grenudi/ideal-agent with its mark (kind canon, topics oco-project and oco-kind-canon), its tad database (practices, decisions, research, sources, landscape, backlog, session_log; CI checks it with tad's pinned release) and a Claude Code plugin and marketplace: skills, hooks (ideal-agent guard and start, a Rust program declared with argenv, built by the plugin with cargo) and the researcher agent. Moved in the practices, the shared skills, the general hook rules, the role methods and the research and decisions on organising agents; added the keeper of a shared resource at the author's word. Each old place now points here (see backlog move-in).

Considered
: Proving the guard live before narrowing devlab (orchestration asked for that order); it was proven by giving both guards the same 19 events first and live in a fresh session after.

Rejected
: A shell guard (fragile reading of shell lines); prebuilt binaries first (backlog release-binaries).

About
: ideal-agent

---


<a id="2026-11-17-agent-folder"></a>
### Briefs live in a project's agent folder, not prompts/

Entry date
: 2026-11-17

Done
: linux-phone-oco/docs moved its briefs into ideal-agent-linux-phone-oco/ (docs #171, decision 2026-11-17-ideal-knowledge-base). The README, lab-session-start and orchestrate-agents now say a project keeps its domain in its agent folder (ideal-agent-PROJECT/, or the folder its AGENTS.md names); practice 42 keeps its rule and points to that folder's README for the domain detail. Plugin 0.1.2.

About
: skills

---


<a id="2026-11-17-ask-a-knowledge-base"></a>
### The skill ask-a-knowledge-base

Entry date
: 2026-11-17

Done
: How an agent finds its way in a tad knowledge base - read AGENTS.md, ask the five queries, never read data/ or docs/, record with --about - is ideal-agent's skill ask-a-knowledge-base, pointing to grenudi/ideal-knowledge-base for what a base is (idealkb's backlog navigation-skill). Plugin 0.1.3.

About
: knowledge-base

---


<a id="2026-11-17-follows-idealkb"></a>
### ideal-agent follows ideal-knowledge-base

Entry date
: 2026-11-17

Done
: Added the subjects table and about on every record, folded the sources' by into takes, ran idealkb's bin/follow.sh at 0.1.0 (it replaced the five records, which lacked about, and added the generic parts of AGENTS.md and tables.toml); tad q about now gives each subject's whole knowledge.

About
: knowledge-base

---


<a id="2026-11-17-nothing-of-ocoeco"></a>
### Nothing of ocoeco

Entry date
: 2026-11-17

Done
: Skills and the researcher teach tad and idealkb's queries and records only; landit checks tad places with tad (lab-agent #219); plugin 0.1.4.

About
: ideal-agent

---


<a id="2026-11-17-look-around-first"></a>
### Look around first, as a skill; ideal-agent at idealkb 0.3.0

Entry date
: 2026-11-17

Done
: Practice 22 extended to the author's own proposals and to where research is kept; skill look-around-first; ask-a-knowledge-base and record-the-work say the summary and the material. The pin moves to idealkb 0.3.0 (summaries, research material and supersedes). Research look-around-habit-world with its material in research/22-look-around-first/2026-11-17/.

About
: practices

---


<a id="2026-11-17-briefs-first"></a>
### Briefs first

Entry date
: 2026-11-17

Done
: idealkb 0.4.0 (idealkb #11, released): brief on decisions and research, answers by briefs. ideal-agent follows it, its records have briefs, its skills say so. Plugin 0.2.3.

About
: knowledge-base

---


<a id="2026-11-17-read-only-when"></a>
### Read only when

Entry date
: 2026-11-17

Done
: Measured both surya keepers from their transcripts (scripts/read-weight); lab-session-start says what to read at the start and 'read only when' for the rest; role skills begin from the summary; plugin 0.2.4.

About
: housekeeping

---


<a id="2026-11-17-hold-little-and-renew"></a>
### Hold little, and renew

Entry date
: 2026-11-17

Done
: keep-the-start-light gains holding little, how agents are brought up, and renewing a long-lived role; practice 45 extended; orchestrate-agents step 7; the coordinator's hand-over and brief in docs (#195); plugin 0.2.6.

About
: housekeeping

---


<a id="2026-11-17-read-weight-lands"></a>
### read-weight lands

Entry date
: 2026-11-17

Done
: scripts/read-weight was written and measured with before fresh cleaned it away untracked; it lands now (plugin 0.2.7). A sign for land-a-change's first rule: branch first, then write.

About
: housekeeping

---


<a id="2026-11-17-record-the-work-and-ask-a-knowledge-base-point"></a>
### record-the-work and ask-a-knowledge-base point to ikb

Entry date
: 2026-11-17

Done
: Both skills now say: read the base's IKB.md once, record with `ikb RECORD --option value` (idealkb's ikb 0.2.0, installed by the base's bin/install-ikb.sh); the per-record tad r lines are gone, briefs-first, about, summary and material rules stay. Descriptions unchanged. Measured on one recording: 1 failed tad r call and hand-escaped text before, 0 failed and no quoting after (idealkb session-log 2026-11-17-ikb-first-release). Plugin 0.2.8.

Rejected
: Copying IKB.md's options into the skills; longer skill descriptions (read by every session)

About
: knowledge-base

---


<a id="2026-11-17-roles"></a>
### Agent roles: cards, a first-prompt hook, plugin 0.3.0

Entry date
: 2026-11-17

Done
: Looked around (research agent-roles, six sources); seven role cards in agents/; ideal-agent role (UserPromptSubmit) and a lighter start; start-weight reads a brief's Role line; lab-session-start and orchestrate-agents point at roles; README; measured before and after on docs' main.

Considered
: A tad role table; filtering practices by their for field; role skills; agent teams; per-project generated agents; a probe session for read-weight (refused: this session cannot start sessions; headless claude is not logged in).

Rejected
: See decision 2026-11-17-roles-as-agent-cards.

About
: organising-agents

---


<a id="2026-11-17-neutral-start"></a>
### A neutral start: agent-env instead of . oco-work (plugin 0.3.1)

Entry date
: 2026-11-17

Done
: scripts/agent-env; the start names it in CLAUDE_ENV_FILE; the role hook keeps a Name: line; land-a-change, lab-session-start, record-the-work, orchestrate-agents, AGENTS.md and README drop oco-work, fresh, landit and OCOECO_BY; tried end to end (name, token, PATH, branch refusing a dirty clone, OCOECO_BY honoured).

Considered
: Where the name comes from: an export in the brief (does not outlive a Bash call), the UserPromptSubmit hook (no env file), the session's role state read by agent-env (chosen).

Rejected
: See decision 2026-11-17-a-neutral-start.

About
: organising-agents

---


<a id="2026-11-17-role-first-prompt-only"></a>
### A role is read only from a new session's first prompt (plugin 0.3.2)

Entry date
: 2026-11-17

Done
: Sign from orchestration: on 0.3.x, a running session started before roles was given the keeper card when a message to it quoted Role: keeper - the hook took any prompt of a session without state as its first, and read the whole prompt. Now the start marks a new session as waiting (role ?), the hook reads only that session's first prompt, only its first 8 lines, and never a prompt that begins with a tag (a cross-session message). Tried: old session 0 B, wrapped message every practice, brief its role, later message 0 B.

Considered
: Counting earlier user turns in the transcript (depends on when Claude Code writes it); stripping quoted text (a message can quote without tags).

Rejected
: Reading Role anywhere in the prompt: an example in a message re-roles a session.

About
: organising-agents

---


<a id="2026-11-17-record-the-work-says-how-to-find-and-choose-an"></a>
### record-the-work says how to find and choose an upstream

Entry date
: 2026-11-17

Done
: The skill now has a Work for others paragraph: tad q upstreams first, a new place looked at and recorded as research the dated way, ikb upstream / upstream-item / upstream-state, a draft ready only on the person's word. Description unchanged.

Rejected
: A new skill (another description every session reads)

About
: knowledge-base

---


<a id="2026-11-17-practices-without-oco-tools"></a>
### Practices without ocoeco's tools (plugin 0.3.3)

Entry date
: 2026-11-17

Done
: Sixteen reworded, 40 and 41 retired with a new retired field and record (practice-retired); the start, ideal-agent practices and tad q practices skip retired ones; role cards take up the reworded ones that fit.

Considered
: Which to retire: only those whose content is only an ocoeco command (40, 41).

Rejected
: See decision 2026-11-17-practices-without-oco-tools.

About
: practices

---


<a id="2026-11-17-the-skills-send-an-agent-to-ikb-help-and-ikb"></a>
### the skills send an agent to ikb --help and ikb help RECORD

Entry date
: 2026-11-17

Done
: record-the-work and ask-a-knowledge-base say: ikb --help, then ikb help RECORD, then ikb RECORD; IKB.md is the page for people. Plugin 0.3.5.

About
: knowledge-base

---


<a id="2026-11-17-role-past-the-reminders"></a>
### A started session's role is read past the app's reminders and the background (0.3.6)

Entry date
: 2026-11-17

Done
: Sign from orchestration: surya-keeper-7 named Role: keeper at the top of its brief but got every practice - a session started from another session begins with the app's <system-reminder> (linked sessions) and the starter's Background ... Task:, so 0.3.2's never-from-a-tag rule skipped it. brief_of now skips leading system-reminder blocks and, when present, the background up to Task:, then reads the first 8 lines; a cross-session message is still refused. Replayed on keeper-7's real first prompt: keeper card, name surya-keeper-7.

Considered
: Searching the whole prompt for Role (re-opens the mid-run re-role sign of 0.3.1).

Rejected
: Asking starters to avoid the background: the app adds it.

About
: organising-agents

---


<a id="2026-11-17-land-reads-its-diff"></a>
### Landing reads its diff against main

Entry date
: 2026-11-17

Done
: Sign from orchestration: docs #226, cut before #230, carried the eight briefs as they were and put #230 back when it merged; nothing caught it. land-a-change now says: rebase on origin/main and read git diff --stat origin/main...HEAD before land. A check in land itself (refuse a diff that reverts lines changed on main since its base) is lab-agent's - proposed to the coordinator.

Considered
: A check in land (its owner's).

Rejected
: Nothing: a skill line alone does not stop a careless landing, so the land check is proposed too.

About
: housekeeping

---


<a id="2026-11-17-lab-agent-research"></a>
### Researched lab-agent: use, dependents, successors

Entry date
: 2026-11-17

Done
: Measured jobs history, journal, units, ntfy caches, today's transcripts; grepped dependents; recorded research, decision, six backlog items and scripts/lab-agent-free. Changed nothing on the lab.

Considered
: oco-agent as successor; systemd-run; curl for ntfy; a wrapper or narrow grants for root

Rejected
: Stopping any unit to see what breaks: not mine to do on the lab.

About
: organising-agents

---


<a id="2026-11-17-lab-agent-step-1-token-to-a-neutral-path"></a>
### lab-agent step 1: token to a neutral path

Entry date
: 2026-11-17

Done
: Moved the token with mv to ~/.config/ideal-agent/token (mode 600), symlinked the old path; gh auth status works from the new path. Readers moved: lab-machine bin/land, 2 pins units, README (PR 29, installed to ~/.local/bin and ~/.config/systemd/user), scripts/agent-env here. lab-agent-free 34 -> 28.

Considered
: Leaving oco-agent, oco-work and watcher tools on the symlink: they go in step 2.

Rejected
: Editing oco-agent readers now: retired next, no point.

About
: organising-agents

---


<a id="2026-11-17-lane-next"></a>
### A lane's next item: lane-next in linux-phone's docs

Entry date
: 2026-11-17

Done
: With orchestration's yes: docs #260 - backlog.device, needs phone|research|build, lane-next --device --needs, phone-next and phone-queue require --device, the records refuse needs without a device, the briefs name it. keep-a-shared-resource names the lane's next-item query.

Considered
: An optional --device, the project as the lane.

Rejected
: See docs decision 2026-11-17-a-backlog-item-names-its-device.

About
: organising-agents

---


<a id="2026-11-17-land-plain"></a>
### land asks no manual record and refuses a revert

Entry date
: 2026-11-17

Done
: lab-machine #30 (the lab-agent-retires session): --manual optional, --help answered, a change that puts back lines main changed is refused. land-a-change says plain land and names the refusal.

Considered
: Keeping --no-manual in the skill (accepted, but noise).

Rejected
: Nothing else.

About
: housekeeping

---


<a id="2026-11-17-overlay-check-extra"></a>
### overlay-check takes a domain's own text too

Entry date
: 2026-11-17

Done
: From ikb-cli's proposal: overlay-check FOLDER [N] [LIMIT] [EXTRA] - EXTRA is a file of the domain's own text (its principles, from tad q principles) a brief must not restate either. Measured on docs f7b0d1c with linux-phone's five principles: every brief still passes (bring-up 11% the highest).

Considered
: An environment variable OVERLAY_EXTRA (as proposed).

Rejected
: The environment variable: a fourth argument keeps the whole surface in the usage line (practice 43).

About
: organising-agents

---


<a id="2026-11-17-resume-trusts-only-kept-state"></a>
### A resume trusts only state kept under the first-prompt rule (0.3.26)

Entry date
: 2026-11-17

Done
: Sign from orchestration: its session, started 13:03 with no Role line, was given the keeper card on resume - 0.3.1 had read Role: keeper off a later message of mine and kept it. The state now carries 'kept first-prompt'; a resume trusts only state with it; a resume with none gets every practice and is marked as having no role, so no later prompt names one. Tried: a 0.3.1 state on resume (every practice, then a Role prompt read: 0 B), no state on resume (the same), a new session's first prompt and its resume (the card both times).

Considered
: Distrusting state older than 0.3.2's landing by file time (hidden, clock-bound).

Rejected
: Awaiting the next prompt on a resume without state: that is how a later message re-roles a session.

About
: organising-agents

---


<a id="2026-11-18-brief-from-main"></a>
### A brief is read from its repository's main (0.3.28)

Entry date
: 2026-11-17

Done
: Sign from the coordinator: Brief: paths point into the shared docs clone, which sat detached at an old commit twice on 2026-11-17 - a start would print a stale brief. The role hook now fetches and reads origin/main:PATH when the brief is in a git clone, says 'from its repository's main', and falls back to the file on disk (said so) with no clone, no network or a brief not on main. Tried: the patrol brief equals docs' origin/main; a brief outside any clone reads from disk.

Considered
: Keeping the shared clones at main from the host (another owner's, and it fights the sessions that work in them).

Rejected
: Reading the working tree as it stands (what drifted).

About
: organising-agents

---


<a id="2026-11-18-launched-runs-message-their-report"></a>
### A launched builder or research lead sends its report as a message

Entry date
: 2026-11-18

Done
: Sign from the coordinator: two lab-tools-builder runs (lab-tools #103, #104) landed and never reported; orchestration found them through the lanes-waker. The builder and research-lead cards now say their last act, after the final landing, is two lines sent as a message to whom they report - as the keeper card has said since #55 - since a launched run's closing words reach no one.

Considered
: A line in docs' builder.md alone (the rule holds for every domain: the card).

Rejected
: Leaving 'report' unsaid as a message (it was read as closing words).

About
: organising-agents

---


<a id="2026-11-18-decisions-superseded"></a>
### record-the-work names superseding a decision and tad q decided

Entry date
: 2026-11-18

Done
: From idealkb v0.14.0 (decision 2026-11-18-decisions-are-superseded-not-edited): record-the-work says a decision is superseded or amended by a new one, never edited, and where decisions stand is tad q decided. ideal-agent's own base is pinned to idealkb 0.5.0 and has neither yet.

Considered
: Restating the options' meanings (ikb help decision has them).

Rejected
: Editing decisions in place (the ikb session's research: the record of why is lost).

About
: practices

---


<a id="2026-11-18-ideal-agent-follows-idealkb-0-14"></a>
### ideal-agent follows idealkb 0.14.0 (it was 0.5.0)

Entry date
: 2026-11-18

Done
: bin/follow.sh from idealkb v0.14.0 into ideal-agent: generic queries and records (decided, carried, upstreams, backlog-blocked), AGENTS.md's and tables.toml's generic parts, bin/install-ikb.sh; no generic file of ours was changed (follow ended 0). tad check and make verify pass; tad q decided answers. THE CLASS: a follower's pin drifted behind its source while the source released fast - a drifted copy. WHERE ELSE (looked up on each main): linux-phone-oco/docs 0.14.0, current; device-template, surya-lab and google-walleye-lab 0.9.1, five releases behind, and not in idealkb's consumers list, so the job never moves them. THE ROOT: the pin job is weekly (Mondays 07:00; last ran 2026-11-17 12:12, the next 2026-11-23) while idealkb released nine times in two days, and its consumers list is kept by hand. Proposed to its owner: run update-pins on each idealkb release, and derive the followers from each base's idealkb.toml (or an upstreams row naming idealkb) rather than a list.

Considered
: update-pins.sh on the shared clone (it resets the clone hard; a worktree with follow.sh is safe).

Rejected
: Leaving the pin for Monday's job (the skill already points to tad q decided).

About
: knowledge-base

---


<a id="2026-11-18-lab-agent-how-still-involved-research"></a>
### lab-agent: how still involved (research)

Entry date
: 2026-11-18

Done
: Counted the mentions in main checkouts, ran the systemd-run probe, read how GitHub runners, buildbot, LAVA, systemd and sudo-rs/polkit do it, wrote research 2026-11-18-lab-agent-still-involved, decision 2026-11-18-job-route-retires-next, backlog lab-agent-0 and -7, step 3 note, and the failing test scripts/lab-agent-route-free.

Considered
: Starting a research-lead session: refused by the app on an automated turn; done here with a researcher subagent for the outside look.

About
: organising-agents
