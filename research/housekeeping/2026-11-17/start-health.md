# Start health: a role for it, or housekeeping? (2026-11-17)

The author's question: an agents-healthkeeper, spawned and pointed at agents, that looks at each role's
bring-up (time to first act, bytes and tokens read, what the role did not need) and perfects the role cards.

## What is (read-weight, today's transcripts; "given, not used" is new in read-weight)
| session | first turn | skills listed / used | agents listed | deferred tools | first prompt |
|---|---|---|---|---|---|
| surya-keeper-7 (keeper) | 79,870 tok | 22,468 B / none | 6,522 B, none used | 9,686 B, none | 7,922 B (every practice: before 0.3.6) |
| surya coordinator (lane) | 79,899 tok | 22,455 B / none | 6,687 B, none | 9,914 B; session connectors used | 8,375 B |
| research-cleanroom (research lead) | 80,451 tok | 22,468 B / bring-up-a-part, land-a-change | 6,522 B, none | 9,686 B; ccd_turn | 7,922 B |
Unused in every one: Anthropic's 16 skills (9,976 B: documents, slides, spreadsheets, Google Workspace,
computer use, browsers, morning, deep research...) and Claude Code's 17 (8,079 B: dataviz, claude-api,
code-review, artifacts, schedule, loop...) - about 18 KB, some 4.5k tokens, every session. ideal-agent's
own descriptions: 4,080 B listed to every session. keeper-7 also searched for its own brief (2.4 KB, a
find) and set OCOECO_BY from its starter's prompt.

## What was there before roles
Decision 2026-11-17-keep-the-start-light: measure per role against a budget; the patrol measures, the owner
simplifies; housekeeping only proposes removals. The skill keep-the-start-light, scripts start-weight and
read-weight, the patrol card. Missing: a measure of what a role was given and never used.

## What others do
- Gas Town: Deacon, Witness, Boot, Dogs watch liveness and stuckness (thresholds in roles/*.toml); none
  measures or trims an agent's priming; a compactor is Go code, not an agent.
- Google SRE (toil): a quarterly survey against a 50% cap; owners turn toil into engineering.
- Friction logs (Resend, Stripe): each newcomer writes one; the owning team fixes.
- DX onboarding: a tool measures continuously; managers act.
- Anthropic, context engineering: the smallest set of high-signal tokens; bloated tool sets are a common
  failure; start minimal and add on failure.
- Claude Code: /context and /skill-doctor measure; skillOverrides (off, name-only, user-invocable-only),
  skillListingMaxDescChars, disableBundledSkills, Skill() deny rules, enabledPlugins per project scope trim;
  an agent's skills field only preloads; no setting hides skills per agent or per session started by the app.

## Lessons
Measuring is a rule (a script and a budget, run by the patrol); trimming belongs to whoever owns the thing
(the card's owner, the brief's owner, the author for his settings). No analogue keeps a standing role for it.
