<!-- Generated from data/ by tad. Do not edit by hand. -->

# Sources

<a id="anthropic-context-engineering"></a>
### Effective context engineering for AI agents

Url
: https://www.anthropic.com/research/effective-context-engineering-for-ai-agents

Author
: Anthropic

Year
: 2025

Takes
: Context rot and an attention budget: more tokens, worse recall. Three tools for long work - compaction (risks losing what matters later), structured notes outside the window (NOTES.md), sub-agents that return 1-2 thousand token summaries; load knowledge just in time by reference.


<a id="anthropic-long-running-harness"></a>
### Effective harnesses for long-running agents

Url
: https://www.anthropic.com/research/effective-harnesses-for-long-running-agents

Author
: Anthropic

Year
: 2025

Takes
: Compaction alone was not enough: each fresh session gets up to speed from a progress file, the git log and a feature list, works one item, commits and writes its progress. Whether specialised agents beat one general agent is left open.


<a id="claude-code-subagent-memory"></a>
### Claude Code agent memory frontmatter

Url
: https://github.com/shanraisshan/claude-code-best-practice/blob/main/reports/claude-agent-memory.md

Author
: shanraisshan

Year
: 2026

Takes
: Since v2.1.33 a subagent can keep its own MEMORY.md (project scope in .claude/agent-memory/NAME/), its first 200 lines put in its prompt each time: the field writes expertise down per role rather than keeping it in a session; each role's memory is siloed from the others.


<a id="cognition-multi-agents-working"></a>
### Multi-Agents: What's Actually Working

Url
: https://cognition.com/blog/multi-agents-working

Author
: Walden Yan (Cognition)

Year
: 2026

Takes
: A narrower class works: writes stay single-threaded and extra agents contribute intelligence, not actions. Consulting a 'smart friend' works best by sharing a fork of the asker's full context; unstructured swarms of negotiating agents are a distraction.


<a id="langchain-multi-agent-benchmark"></a>
### Benchmarking Multi-Agent Architectures

Url
: https://www.langchain.com/blog/benchmarking-multi-agent-architectures

Author
: Will Fu-Hinthorn (LangChain)

Year
: 2025

Takes
: A supervisor that relays every message plays telephone: it scored lower and spent more tokens than agents handing off directly; removing routing chatter from the workers' context closed much of the gap.


<a id="ace-context-collapse"></a>
### Agentic Context Engineering: Evolving Contexts for Self-Improving Language Models

Url
: https://arxiv.org/abs/2510.04618

Author
: Zhang et al.

Year
: 2025

Takes
: Knowledge kept by rewriting one summary erodes (context collapse) and drifts toward the terse (brevity bias); itemised, incremental additions to a playbook keep the detail. ICLR 2026.


<a id="chroma-context-rot"></a>
### Context Rot: How Increasing Input Tokens Impacts LLM Performance

Url
: https://www.trychroma.com/research/context-rot

Author
: Hong, Troynikov, Huber (Chroma)

Year
: 2025

Takes
: Models do worse as input grows, even on simple tasks and well below the window's size; focused inputs beat full ones.


<a id="blackboard-llm-mas"></a>
### Exploring Advanced LLM Multi-Agent Systems Based on Blackboard Architecture

Url
: https://arxiv.org/abs/2507.01701

Author
: Han and Zhang

Year
: 2025

Takes
: Agents share one blackboard instead of private memories; a control unit picks who acts from what is on it. With 2510.01285 (agents volunteer for requests posted to the board): routing by a shared record, not by a central relay.


<a id="letta-sleep-time"></a>
### Sleep-time Compute

Url
: https://www.letta.com/blog/sleep-time-compute/

Author
: Letta

Year
: 2025

Takes
: Stateful agents keep editable memory blocks; a separate agent rewrites them between tasks into clean, concise learned context, so the working agent need not re-derive it.


<a id="lava-scheduler"></a>
### LAVA glossary: the scheduler

Url
: https://lava.readthedocs.io/en/latest/introduction/glossary.html

Author
: Linaro

By
: LAVA (Linaro Automated Validation Architecture)

Takes
: A single scheduler on the server assigns devices to submitted test jobs; dispatchers run them, one job per device at a time. Jobs queue; nobody touches a board outside the scheduler.


<a id="mesa-lava-ci"></a>
### LAVA CI - The Mesa 3D Graphics Library

Url
: https://docs.mesa3d.org/ci/LAVA.html

Author
: Mesa

Takes
: Mesa's merge requests reach real boards only through the LAVA instance, which schedules jobs onto the boards present.


<a id="ci-tron-introduction"></a>
### Introduction to CI-tron

Url
: https://docs.ci-tron.dev/introduction.html

By
: CI-tron (freedesktop)

Takes
: A CI gateway service that orchestrates and manages the devices under test: one gateway owns its devices and serves the jobs that come for them.


<a id="pmos-hw-ci-status"></a>
### Hardware testing automation: a status update - postmarketOS

Url
: https://nura.eco/blog/2025/05/13/hw-ci-status/

Year
: 2025

By
: postmarketOS

Takes
: postmarketOS's hardware CI runs phones behind a CI-tron gateway - the precedent nearest to a Linux-phone lab.


<a id="gastown-refinery"></a>
### Gas Town: project roles (Refinery: merge queue processor)

Url
: https://github.com/gastownhall/gastown

Author
: Steve Yegge

Takes
: The Refinery is the one agent that merges all changes to main, one at a time; a role, not a session - when a session for the role starts it takes up the next step, nudged by patrols.


<a id="actor-model"></a>
### A Universal Modular ACTOR Formalism for Artificial Intelligence

Url
: https://dl.acm.org/doi/10.5555/1624775.1624804

Author
: Carl Hewitt, Peter Bishop, Richard Steiger

Year
: 1973

Takes
: An actor owns its state alone and others reach it only by messages it takes one at a time: the single-writer principle that keeps shared state free of races.


<a id="rust-rfc-template"></a>
### Rust RFC template

Url
: https://github.com/rust-lang/rfcs/blob/master/0000-template.md

Author
: The Rust project

Takes
: Every proposal discusses prior art, good and bad, and the alternatives - set against what others did before it is taken.


<a id="pep-12"></a>
### PEP 12 - Sample reStructuredText PEP Template

Url
: https://peps.python.org/pep-0012/

Author
: Python

Takes
: A Rejected Ideas section kept with the proposal; PEP 1's Superseded-By and Replaces for a redo.


<a id="madr"></a>
### MADR: Markdown Architectural Decision Records 4.0

Url
: https://adr.github.io/madr/

Takes
: Considered options with pros and cons beside every decision; a status that says superseded by which.


<a id="claude-code-skills"></a>
### Extend Claude with skills

Url
: https://code.claude.com/docs/en/skills

Author
: Anthropic

Year
: 2026

Takes
: The description always in context, the body loaded when used; SKILL.md under 500 lines, reference in files read only when needed.


<a id="wikipedia-maintenance"></a>
### Wikipedia:Maintenance

Url
: https://en.wikipedia.org/wiki/Wikipedia:Maintenance

Author
: Wikipedia

Takes
: Upkeep is a standing backlog, not a campaign: a page that needs work is tagged (a cleanup template), the tags gather into backlog categories by kind, and anyone takes one and removes the tag when it is fixed - the tag proposes, the fix is the edit.


<a id="claude-code-memory"></a>
### How Claude remembers your project - Claude Code Docs

Url
: https://code.claude.com/docs/en/memory

Author
: Anthropic

Takes
: What is loaded every session is kept small: target under 200 lines per CLAUDE.md, since longer files consume context and reduce adherence; move what matters for only part of the work to rules that load when it is touched; imports organise a file but do not reduce its cost.


<a id="docs-freshness-for-agents"></a>
### Tools That Stop AI Agents Using Stale Documentation

Url
: https://getunblocked.com/blog/stop-agents-using-stale-documentation/

Author
: Unblocked

Takes
: Agents trust whatever they read: keep them off stale documentation by verification workflows, freshness metadata (when a page was last checked), documentation generated from the code, and weighting by authority.


<a id="claude-compaction"></a>
### Compaction overview - Claude Platform Docs

Url
: https://platform.claude.com/docs/en/build-with-claude/compaction

Author
: Anthropic

Takes
: Compaction replaces older turns with a summary Claude writes; the session goes on, but what is kept is whatever the summary chose.


<a id="context-engineering-cookbook"></a>
### Context engineering: memory, compaction, and tool clearing

Url
: https://platform.claude.com/cookbook/tool-use-context-engineering-context-engineering-tools

Author
: Anthropic

Takes
: Three primitives for long-running agents: clearing old tool results that can be fetched again, compaction of the conversation, and a memory of notes kept outside the window. Together they kept a run's peak context at half the baseline and its end at 14k tokens instead of 334k.


<a id="gastown-handoff"></a>
### Gas Town Emergency User Manual

Url
: https://steve-yegge.medium.com/gas-town-emergency-user-manual-cf0e4556d74b

Author
: Steve Yegge

Year
: 2026

Takes
: Use gt handoff liberally after every task, in every worker: sessions are cattle, the role's identity and its hook carry the work to the next session.


<a id="claude-code-subagents"></a>
### Claude Code: create custom subagents

Url
: https://code.claude.com/docs/en/sub-agents

Author
: Anthropic

Year
: 2026

By
: Anthropic

Takes
: agents/NAME.md is the role card: model, effort, skills in frontmatter; claude --agent NAME starts a main session as it; plugin agents ignore hooks, mcpServers, permissionMode


<a id="claude-code-agent-teams"></a>
### Claude Code: agent teams

Url
: https://code.claude.com/docs/en/agent-teams

Author
: Anthropic

Year
: 2026

By
: Anthropic

Takes
: teammates may be agent definitions, but their skills are not applied and the lead cannot hand over - not the route for lasting roles yet


<a id="anthropic-multi-agent-research"></a>
### How we built our multi-agent research system

Url
: https://www.anthropic.com/engineering/multi-agent-research-system

Author
: Anthropic

Year
: 2025

By
: Anthropic

Takes
: a lead writes each brief (objective, output, tools, boundaries); strong lead, lighter workers: model per role


<a id="gastown-roles"></a>
### Gas Town: role definitions and gt prime

Url
: https://github.com/steveyegge/gastown

Author
: Steve Yegge

Year
: 2026

By
: Steve Yegge

Takes
: each role a small definition plus a template, primed at SessionStart with its work and mail, lighter after compaction


<a id="crewai-agents"></a>
### CrewAI: agents (role, goal, backstory)

Url
: https://docs.crewai.com/concepts/agents

Author
: CrewAI

Year
: 2025

By
: CrewAI

Takes
: a role is a few fields; we keep purpose and leave the backstory out


<a id="team-topologies"></a>
### Team Topologies

Url
: https://teamtopologies.com

Author
: Matthew Skelton, Manuel Pais

Year
: 2019

By
: Skelton and Pais

Takes
: a team is its type, owned scope and interaction modes; with RACI's one accountable, the card's owns, never and reports to


<a id="claude-code-skills-listing"></a>
### Claude Code: skills (the skill listing and its budget)

Url
: https://code.claude.com/docs/en/skills

Author
: Anthropic

Year
: 2026

By
: Anthropic

Takes
: every listed skill costs context each turn; skillOverrides off/name-only, skillListingMaxDescChars and disableBundledSkills trim the listing; an agent's skills field only preloads


<a id="sre-eliminating-toil"></a>
### Site Reliability Engineering, ch. 5: Eliminating Toil

Url
: https://sre.google/sre-book/eliminating-toil/

Author
: Google

Year
: 2016

By
: Google SRE

Takes
: measure by a rule against a cap; the owners turn what is found into engineering - no standing toil-keeper


<a id="resend-friction-logs"></a>
### How we use friction logs to improve the product

Url
: https://resend.com/blog/how-we-use-friction-logs-to-improve-the-product

Author
: Resend

Year
: 2024

By
: Resend

Takes
: the newcomer's own log of friction is cheap signal; the owning team fixes


<a id="anthropic-writing-tools"></a>
### Writing effective tools for agents - with agents

Url
: https://www.anthropic.com/engineering/writing-tools-for-agents

Author
: Anthropic

Year
: 2025

By
: Anthropic

Takes
: read transcripts and evals for tool errors and redundant calls; fix the tool's description or shape


<a id="microsoft-agent-trajectories"></a>
### Investigating agent trajectories

Url
: https://microsoft.github.io/agent-experience/playbook/investigating-agent-trajectories/

Author
: Microsoft

Year
: 2026

By
: Microsoft

Takes
: note the first wrong call; bring the evidence to whoever owns the cause; a new skill is the last resort


<a id="langfuse-error-analysis"></a>
### Langfuse: error analysis

Url
: https://langfuse.com/academy/monitoring/error-analysis.md

Author
: Langfuse

Year
: 2025

By
: Langfuse

Takes
: read traces, cluster first failures, repeat after changes; the domain calls stay with people


<a id="mast-mas-failures"></a>
### Why Do Multi-Agent LLM Systems Fail? (MAST)

Url
: https://arxiv.org/abs/2503.13657

Author
: Cemri et al.

Year
: 2025

By
: Cemri et al.

Takes
: a taxonomy of 14 failure modes - system design, misalignment, verification


<a id="swarmia-paper-cuts"></a>
### Paper cuts in developer experience

Url
: https://swarmia.com/blog/paper-cuts-in-developer-experience

Author
: Rebecca Murphey

Year
: 2024

By
: Swarmia

Takes
: everyone reports small friction; owners fix; the pile becomes data for priorities


<a id="ikb-friction-in-transcripts"></a>
### What agents spend on recording into a tad knowledge base (ikb-friction-and-world)

Url
: https://github.com/grenudi/ideal-knowledge-base/blob/main/research/ikb/2026-11-17/friction-in-transcripts.md

Author
: ikb-cli session

Year
: 2026

By
: grenudi/ideal-knowledge-base

Takes
: 31 transcripts: 41 of 240 tad r calls failed (17%), each failure ~8.4 calls and ~11k tokens; after ikb 0-1 failures in 4 runs at equal tokens - a one-off subagent count, not rerunnable; the method read-weight's struggled-with now repeats by script


<a id="lab-agent-repo"></a>
### creation-guidelines/lab-agent (main, 0.1.53)

Url
: https://github.com/creation-guidelines/lab-agent

By
: research-lab-agent

Takes
: what ran, when, what depends on it


<a id="lab-agent-jobs-repo"></a>
### grenudi/lab-agent-jobs: job and board history

Url
: https://github.com/grenudi/lab-agent-jobs

By
: research-lab-agent

Takes
: what ran, when, what depends on it


<a id="agent-skills-spec"></a>
### Agent Skills specification

Url
: https://agentskills.io/specification

Author
: Anthropic, then open

Year
: 2026

By
: agentskills.io

Takes
: SKILL.md with name and description is read by Claude Code, Codex, Gemini CLI, Cursor and some 40 others; harness-only keys are not portable


<a id="agents-md"></a>
### AGENTS.md

Url
: https://agents.md

Author
: Agentic AI Foundation

Year
: 2026

By
: Linux Foundation

Takes
: one instructions file most harnesses read; Claude Code reads it where no CLAUDE.md is


<a id="rulesync"></a>
### rulesync

Url
: https://github.com/dyoshikawa/rulesync

Author
: dyoshikawa

Year
: 2026

By
: dyoshikawa

Takes
: one source generated into many harnesses; its end-of-life list of targets shows what tracking every harness costs


<a id="bezos-2016-letter"></a>
### 2016 Letter to Shareholders (one-way and two-way doors, disagree and commit)

Url
: https://www.aboutamazon.com/news/company-news/2016-letter-to-shareholders

Author
: Jeff Bezos

Year
: 2017

By
: Amazon

Takes
: two-way doors decided fast near the work; escalate at once, never by exhaustion; disagree and commit


<a id="bain-rapid"></a>
### RAPID decision making

Url
: https://www.bain.com/insights/rapid-decision-making/

Author
: Bain

Year
: 2023

By
: Bain

Takes
: one decider for each decision, who holds the accountability; input is not the decision


<a id="sre-managing-incidents"></a>
### Site Reliability Engineering: Managing Incidents

Url
: https://sre.google/sre-book/managing-incidents/

Author
: Google

Year
: 2016

By
: Google SRE

Takes
: the commander holds every role not delegated; declare early; handoffs are acknowledged


<a id="gastown-escalation"></a>
### Gas Town: escalation design

Url
: https://github.com/steveyegge/gastown

Author
: Steve Yegge

Year
: 2026

By
: Steve Yegge

Takes
: a blocked worker escalates and goes on; each tier resolves or forwards; every escalation tracked, re-escalated when stale


<a id="claude-code-auto-mode"></a>
### Claude Code auto mode

Url
: https://www.anthropic.com/engineering/claude-code-auto-mode

Author
: Anthropic

Year
: 2026

By
: Anthropic

Takes
: what still reaches a human: destroying or exfiltrating, weakening security, crossing trust boundaries, affecting others


<a id="copilot-agent-firewall"></a>
### Copilot coding agent: customize the agent firewall

Url
: https://docs.github.com/en/copilot/how-tos/use-copilot-agents/coding-agent/customize-the-agent-firewall

Author
: GitHub

Year
: 2026

By
: GitHub

Takes
: downloads from allowlisted official sources pass; other hosts are flagged, not asked for


<a id="slsa-verifying-artifacts"></a>
### SLSA v1.0: Verifying artifacts

Url
: https://slsa.dev/spec/v1.0/verifying-artifacts

Author
: OpenSSF

Year
: 2023

By
: SLSA

Takes
: check a file's hash or provenance against an expectation from a separate trusted channel before use


<a id="sre-postmortem-culture"></a>
### Site Reliability Engineering: Postmortem Culture

Url
: https://sre.google/sre-book/postmortem-culture/

Author
: Google

Year
: 2016

By
: Google SRE

Takes
: a finding becomes an owned, tracked action item in the owner's system - not a note in the incident


<a id="argyris-double-loop"></a>
### Double Loop Learning in Organizations

Url
: https://hbr.org/1977/09/double-loop-learning-in-organizations

Author
: Chris Argyris

Year
: 1977

By
: Harvard Business Review

Takes
: learning that changes the governing rules, not only the next action - the method itself is revised from what was found


<a id="pmos-porting-new-device"></a>
### postmarketOS: Porting to a new device

Url
: https://wiki.postmarketos.org/wiki/Porting_to_a_new_device

Author
: postmarketOS

Year
: 2026

By
: postmarketOS

Takes
: a feedback channel and a downstream (vendor) port as the base before mainlining


<a id="pmos-mainlining-guide"></a>
### postmarketOS: Mainlining guide

Url
: https://wiki.postmarketos.org/wiki/Mainlining

Author
: postmarketOS

Year
: 2026

By
: postmarketOS

Takes
: mainline from the nearest supported sibling, one change at a time, with UART, framebuffer or ramoops to see it


<a id="claude-code-plugin-dependencies"></a>
### Claude Code: plugin dependencies

Url
: https://code.claude.com/docs/en/plugins/dependencies.md

Author
: Anthropic

Year
: 2026

By
: Anthropic

Takes
: a plugin depends on another by a semver range resolved from release tags; names are namespaced; no agent extends another's body


<a id="helm-library-charts"></a>
### Helm: library charts

Url
: https://helm.sh/docs/topics/library_charts/

Author
: Helm

Year
: 2026

By
: CNCF

Takes
: the overlay pins its base and includes its templates by name, overriding only what differs


<a id="google-eng-practices-review"></a>
### Google engineering practices: the standard of code review

Url
: https://google.github.io/eng-practices/review/reviewer/standard.html

Author
: Google

Year
: 2019

By
: Google

Takes
: approve what improves the code's overall health; the style guide decides style, design is judged by principles


<a id="chatdev"></a>
### ChatDev: Communicative Agents for Software Development

Url
: https://arxiv.org/abs/2307.07924

Author
: Qian et al.

Year
: 2023

By
: Tsinghua

Takes
: programmer, reviewer and tester roles beat a bare model; separating review from writing helps


<a id="lean-jidoka"></a>
### Lean Enterprise Institute lexicon: jidoka

Url
: https://www.lean.org/lexicon-terms/jidoka/

Author
: Lean Enterprise Institute

Year
: 2024

By
: LEI

Takes
: stop at the first abnormality so its cause shows and is fixed at the root


<a id="postmortem-trigger-criteria"></a>
### Which incidents need a postmortem?

Url
: https://oneuptime.com/blog/post/2026-07-30-postmortem-trigger-criteria/view

Author
: OneUptime

Year
: 2026

By
: OneUptime

Takes
: choose reviews by impact and learning value; a repeat of a class or a cluster of minor events is a trigger


<a id="rootly-postmortem-template"></a>
### Incident postmortem template

Url
: https://rootly.com/incident-postmortems/template

Author
: Rootly

Year
: 2026

By
: Rootly

Takes
: a light note for minor incidents, a review for a recurring one; the exposed team does not waive its own review


<a id="kernel-coding-assistants"></a>
### Linux kernel: AI coding assistants

Url
: https://docs.kernel.org/process/coding-assistants.html

Author
: Linux kernel developers

Year
: 2025

By
: kernel.org

Takes
: an agent never adds Signed-off-by - only a person certifies; the tool is named in a trailer (Assisted-by)


<a id="github-co-authors"></a>
### GitHub: creating a commit with multiple authors

Url
: https://docs.github.com/en/pull-requests/committing-changes-to-your-project/creating-and-editing-commits/creating-a-commit-with-multiple-authors

Author
: GitHub

Year
: 2026

By
: GitHub

Takes
: Co-authored-by trailers; credit links only to an account's email - the run's name rides the trailer, the account stays the author


<a id="ansible-idempotency"></a>
### Ansible glossary: idempotency

Url
: https://docs.ansible.com/ansible/latest/reference_appendices/glossary.html

Author
: Red Hat

Year
: 2026

By
: Ansible

Takes
: a task reports changed or ok; run again on a system already in the wanted state, it changes nothing


<a id="unix-rule-of-silence"></a>
### The Art of Unix Programming: the Rule of Silence

Url
: https://web.archive.org/web/20250102222016/http://www.catb.org/esr/writings/taoup/html/ch01s06.html

Author
: Eric S. Raymond

Year
: 2003

By
: Eric S. Raymond

Takes
: when a program has nothing surprising to say, it says nothing - a person's attention is spent only on news


<a id="git-worktree"></a>
### git-worktree: manage multiple working trees

Url
: https://git-scm.com/docs/git-worktree

Author
: Git project

Year
: 2026

By
: git

Takes
: a linked worktree per branch shares the repository but not the checkout: one writer per working tree


<a id="actions-checkout"></a>
### actions/checkout

Url
: https://github.com/actions/checkout

Author
: GitHub

Year
: 2026

By
: GitHub

Takes
: CI never writes in a shared checkout: each job checks out its own, clean, at the ref it builds


<a id="debian-dep3"></a>
### DEP-3: Patch Tagging Guidelines

Url
: https://dep-team.pages.debian.net/deps/dep3/

Author
: Debian

Year
: 2009

By
: Debian

Takes
: every carried patch names its origin, its upstream bug, whether it was forwarded and when it was applied upstream


<a id="todocheck"></a>
### todocheck

Url
: https://github.com/preslavmihaylov/todocheck

Author
: Preslav Mihaylov

Year
: 2020

By
: todocheck

Takes
: a TODO names its issue, and a check fails when that issue is closed - a workaround dies with its reason


<a id="diataxis"></a>
### Diátaxis

Url
: https://diataxis.fr/

Author
: Daniele Procida

Year
: 2026

By
: diataxis.fr

Takes
: four forms of documentation for four needs: reference holds the facts; a how-to points to it rather than restating it


<a id="dita-conref"></a>
### DITA 1.3: the conref attribute

Url
: https://docs.oasis-open.org/dita/dita/v1.3/os/part3-all-inclusive/langRef/attributes/theconrefattribute.html

Author
: OASIS

Year
: 2016

By
: OASIS

Takes
: content is pulled from its one source by reference, never copied - the copy cannot drift


<a id="apache-lazy-consensus"></a>
### Lazy consensus

Url
: https://community.apache.org/committers/lazyConsensus.html

Author
: The Apache Software Foundation

By
: Apache projects

Takes
: Silence is assent: a proposal stands unless someone objects; no one replies only to agree


<a id="rfc1122-delayed-ack"></a>
### RFC 1122: Requirements for Internet Hosts, 4.2.3.2 When to Send an ACK Segment

Url
: https://www.rfc-editor.org/rfc/rfc1122

Author
: R. Braden (ed.)

Year
: 1989

By
: IETF

Takes
: An acknowledgement rides on the next data sent (delayed, piggybacked ACK) rather than costing a segment of its own


<a id="nohello"></a>
### no hello

Url
: https://nohello.net

By
: -

Takes
: A message that carries no content costs the reader an interruption; say the substance in the first message


<a id="platt-strong-inference"></a>
### Strong Inference

Url
: https://doi.org/10.1126/science.146.3642.347

Author
: John R. Platt

Year
: 1964

By
: -

Takes
: Devise alternative hypotheses, then a crucial experiment whose outcomes exclude some of them; an experiment that cannot exclude anything teaches nothing


<a id="nosek-preregistration"></a>
### The preregistration revolution

Url
: https://doi.org/10.1073/pnas.1708274114

Author
: Nosek, Ebersole, DeHaven, Mellor

Year
: 2018

By
: -

Takes
: Write the prediction and what each outcome would mean before the data, so the reading is not fitted after it


<a id="feynman-cargo-cult-science"></a>
### Cargo Cult Science

Url
: https://calteches.library.caltech.edu/51/2/CargoCult.htm

Author
: Richard Feynman

Year
: 1974

By
: -

Takes
: Calibrate the apparatus on a known case first (the rat-maze controls); the first principle is that you must not fool yourself


<a id="google-sre-postmortem"></a>
### Site Reliability Engineering, chapter 15: Postmortem Culture: Learning from Failure

Url
: https://sre.google/sre-book/postmortem-culture/

Author
: Google

Year
: 2016

By
: -

Takes
: A failure is written up blamelessly for its contributing causes in the system and process, not patched and forgotten


<a id="pmos-porting"></a>
### Porting to a new device

Url
: https://wiki.postmarketos.org/wiki/Porting_to_a_new_device

By
: postmarketOS

Takes
: Bring-up starts from the vendor (downstream) kernel source as the reference for what the device does


<a id="aosp-ab-updates"></a>
### A/B (seamless) system updates

Url
: https://source.android.com/docs/core/ota/ab

By
: Android Open Source Project

Takes
: The bootloader's slot flags (active, successful, unbootable, retry count) are documented: what a flash and a boot do to them is read here, not guessed


<a id="utsl"></a>
### UTSL: Use the Source, Luke

Url
: https://en.wikipedia.org/wiki/Use_the_source,_Luke

By
: hacker usage, the Jargon File

Takes
: When behaviour is in question and the source is open, read the source rather than guess
