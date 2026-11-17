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
