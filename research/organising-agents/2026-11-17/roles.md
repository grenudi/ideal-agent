# Agent roles: what is, and what others do (2026-11-17)

## What is (the commons)
- ideal-agent: eleven skills, four of them a role's method (orchestrate-agents, keep-a-shared-resource,
  research-a-feature, bring-up-a-part); agents/ held only `researcher` (a read-only subagent). The
  SessionStart hook printed all 45 practices' first sentences (8.6 KB) to every session, whatever its job.
- linux-phone-oco/docs, ideal-agent-linux-phone-oco/: six briefs - coordinator, surya-coordinator (lane),
  orchestration, phone-keeper (bring-up folded in), feature-research, bring-up (a stub). Generic text repeated
  across them: the start line, "read only when", the renew thresholds (also in keep-the-start-light), how to
  record (three times), "the phone only through its keeper" (four times). No walleye brief yet.
- Roles seen in practice on 2026-11-17: the top coordinator (the author's one session), a lane coordinator per
  shared resource, orchestration, keepers, researchers (feature, landscape, reverse engineering), tool
  builders, the patrol.
- Start weight (start-weight, docs' main cbec011): 22.0-27.9 KB per role, 21.4 KB of it common. A keeper's
  first turn (read-weight, two transcripts of 2026-11-17): 72,474 and 79,801 tokens, the session start 8,615 B.

## What others do
- Claude Code agent definitions (code.claude.com/docs/en/sub-agents): agents/NAME.md, frontmatter name,
  description, tools, disallowedTools, model, effort, skills (preloaded whole), mcpServers, hooks, memory,
  omitClaudeMd...; `claude --agent NAME` starts a main session as the role, its body replacing the system
  prompt. Plugin agents ignore hooks, mcpServers, permissionMode. Unknown frontmatter keys are ignored.
- Agent teams (code.claude.com/docs/en/agent-teams, experimental): a lead and teammates; a teammate may be an
  agent definition, but its `skills` are not applied; the lead cannot be handed over.
- Anthropic, multi-agent research system (2025): a lead (Opus) writes each worker's brief - objective,
  output, tools and sources, boundaries; workers (Sonnet) start clean. Context engineering (2025): the right
  altitude, minimal tools, just in time, not preloaded.
- Gas Town (Yegge): roles mayor, deacon, witness, refinery, polecats, crew, each a small TOML (role, scope,
  session, env GT_ROLE, health) plus a prompt template; `gt prime` at SessionStart gives role, hooked work and
  mail, lighter after compaction; /handoff to a fresh session.
- CrewAI role/goal/backstory; AG2 name/system_message/description; OpenAI Agents SDK Agent(name, instructions,
  tools, handoffs, model); LangGraph supervisor routing to named workers.
- Human teams: RACI (one Accountable per decision), Team Topologies (team type, owned scope, interaction
  mode), role charters (purpose, accountabilities, decision rights, interfaces).

## Lessons taken
1. Use the native card (agents/ROLE.md): model, effort, skills in frontmatter; purpose, owns, never, loads,
   hand-over, reports to in a short body.
2. Prime at the start from the role, Gas Town's way - but the role is known only from the prompt in the
   desktop app (start_session takes no agent), so the first prompt's hook primes.
3. Preload little: the card names its method skill; the rest stays one Skill call away.
4. Renewal and reports-to belong to the card (Gas Town's handoff; RACI's one accountable).
