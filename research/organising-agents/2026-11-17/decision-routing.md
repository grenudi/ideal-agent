# Who decides, and what reaches the author (2026-11-17)

## What is
- walleye lane, 2026-11-17 afternoon (lane coordinator transcript 6b97b7cb): three asks waited on the author -
  the go for the first `fastboot boot` (until a standing go was recorded), a builder session the app would
  not start on a message's turn (a card waiting for a click), and keeper-1's `fastboot set_active a` after
  the bootloader moved to slot b - a change to flags, reversible while slot a's image is known good.
- The cards said: the keeper "decides a turning point of design - that goes back up"; the lane coordinator
  "reports what it needs decided". No one was named to decide a lane's question.
- start_session is refused on a turn a message or event began (8 refusals today); an Agent-tool subagent is
  not, and returns within the turn.

## What others do
- Bezos (2015, 2016): two-way doors decided fast by those near the work, one-way doors slowly; decide with
  ~70% of the information; escalate at once, else exhaustion decides.
- Bain RAPID: one decider per decision; input shapes, it does not decide.
- Google SRE incident command: the commander holds what is not delegated; declare early; firm handoffs.
- Anthropic multi-agent research: the lead decides whether more research is needed; subagents need an
  objective, a format, tools - and bounds (endless searching seen).
- Anthropic, building effective agents: pause for humans at checkpoints or blockers; stopping conditions.
- Claude Code auto mode (2026): blocks destroying or exfiltrating, weakening security, crossing trust
  boundaries, affecting others; a human after 3 consecutive or 20 denials.
- Gas Town: "When blocked, you MUST escalate. Do NOT wait for human input ... Continue with other work";
  each tier resolves or forwards; every escalation tracked; stale after 4h, re-escalated at most twice.
- LangGraph interrupts and OpenAI Agents SDK: a human approves per critical action (money, external writes).

## Lessons
One decider at the lowest level holding the information; reversible decisions fast, once the way back is
shown; the worker escalates and goes on; research bounded; the decision recorded before it is acted on;
a small named set of classes reaches the human.
