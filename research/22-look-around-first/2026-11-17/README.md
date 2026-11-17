# Looking around before acting on a proposal - how others do it (2026-11-17)

The material behind research `look-around-habit-world` in this base; the row holds the finding.

- **Rust RFCs** - the template (https://github.com/rust-lang/rfcs/blob/master/0000-template.md) asks every
  proposal for "Prior art", good and bad, and for "Rationale and alternatives": a proposal is set against what
  others did before it is taken.
- **PEPs** - PEP 12's template (https://peps.python.org/pep-0012/) keeps a "Rejected Ideas" section, and PEP 1
  (https://peps.python.org/pep-0001/) marks a later PEP Replaces / Superseded-By: the reasoning is kept with the
  proposal, and redoing it supersedes rather than deletes.
- **MADR** (https://adr.github.io/madr/): every decision lists its considered options with pros and cons.
- **Anthropic, context engineering** (https://www.anthropic.com/research/effective-context-engineering-for-ai-agents)
  and **Claude Code skills** (https://code.claude.com/docs/en/skills): keep identifiers in context and load the
  content just in time; a skill's body and its files load only when used - so a habit kept as a skill costs a
  line until it is needed, and research kept as a row plus a folder costs a row until it is questioned.
- **In the commons** - practice 22 (look around first) already says what is, why, what others learned, recorded
  as research; it said nothing of the author's own proposals, nor where research material goes. idealkb 0.2.0
  (grenudi/ideal-knowledge-base, research summary-layer-world and research-aside-world, with their own material)
  gave the place: a research row with material in research/SUBJECT/YYYY-MM-DD/ and supersedes.

Taken: the look is small and bounded (a prior-art section, not a study); alternatives and what was rejected are
written beside the choice; the finding is a row, the material a dated folder; a redo supersedes. Set aside: a
new practice beside 22 (one nature under two names - 22 is extended instead); a hook that forces a look before
every edit (most edits need none; a hook cannot tell a proposal from a typo).
