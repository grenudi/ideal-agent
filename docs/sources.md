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


<a id="claude-code-agent-teams"></a>
### Orchestrate teams of Claude Code sessions

Url
: https://code.claude.com/docs/en/agent-teams

Author
: Anthropic

Year
: 2026

Takes
: Teammates message each other directly, not only through the lead; 3-5 teammates; each starts from project context and a spawn prompt, never the lead's history; in-process teammates are not restored on resume; for routine or sequential work one session is cheaper.


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
