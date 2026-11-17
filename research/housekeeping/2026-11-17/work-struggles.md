# What agents struggle with while working (2026-11-17)

Measured with read-weight's "struggled with" over the 41 transcripts of 2026-11-17 over 50 KB
(~/.claude/projects/-home-mll), this session's own left out: 5,375 tool results, 301 failed (5.6%).

| what | count | owner | proposed fix |
|---|---|---|---|
| Python reached for, refused by a guard | 27 (ideal-agent guard 18, devlab 9) | ideal-agent guard, devlab | the refusal names the replacement (jq, awk, sed, tad/duckdb) - done in ideal-agent 0.3.9 |
| land's ocoeco-era --manual/--no-manual, no --help | 21 (say --manual 9, manual work not recorded 6, unknown option --help 6) | lab-agent's land | default to no manual record; answer --help |
| SQL written by hand with tad sql past the named records | 56 commands in 13 sessions: backlog status/waits/note 22, practices 5, decisions 8, sources 9, test_results 4, research 2, subjects 2 | ikb / idealkb, each base | a generic "set a field of a row" record, or the missing records (backlog waits, a source's removal) |
| sleep before a command, refused by Claude Code | 17 | ideal-agent skills | wait with the tool's own wait, the background, or a Monitor - done in change-a-device |
| start_session refused on a turn a message or event began | 8 | ideal-agent lane-coordinator card | start on its own turn, else propose - done in the card |
| a check failed or none appeared at land | 11 | each repository | not clustered yet |
| ssh to the phone refused by devlab | 7 | keeper brief | the brief names devlab phone sh; the guard teaches |
| oco-work's turn word failed (the turn was not kept) | 5 | the author's conversation, oco-work | moves with the detachment from oco tools |
| the same command run again unchanged | 6 | - | low |
Earlier, by hand (ikb-cli, 31 transcripts): 41 of 240 tad r calls failed (17%), five causes, which led to ikb.

## What others do
Anthropic (writing tools for agents): read transcripts and evals - tool errors for invalid parameters mean
clearer descriptions; redundant calls mean rightsizing. Microsoft (agent trajectories): note the first wrong
call, bring the evidence to the owner; a new skill is the last resort. Langfuse: read traces, cluster, recur
after changes; "you make the domain calls". MAST: 14 failure modes in 3 groups. SRE: toil tracked, ranked by
cost against time saved, owners choose. Stripe paper cuts: reported by all, fixed by owners. Claude Code:
/insights (per user, friction points), OpenTelemetry tool_result success and error_type, PostToolUseFailure
(not for schema or permission failures - those are only in the transcript).

## Lessons
Detecting is mechanical and shares its pass with start health (the same transcripts, per role); clustering
needs a model only now and then; fixes are the owners'; a report names owner, evidence and cost.
