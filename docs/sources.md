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


<a id="mast-mas-failures"></a>
### Why Do Multi-Agent LLM Systems Fail?

Url
: https://arxiv.org/abs/2503.13657

Author
: Cemri et al.

Year
: 2025

Takes
: 1600+ traces, 14 failure modes in three groups: system design, misalignment between agents (lost context, ignored input, withheld information), task verification. Gains from multi-agent systems are often minimal. NeurIPS 2025.


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
