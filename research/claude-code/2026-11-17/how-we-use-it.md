# Claude Code: what our method uses of it (checked 2026-11-17, Claude Code 2.1.321 in the desktop app)

Only what a skill, role or script of ideal-agent uses. A change in Claude Code is a new research row about
claude-code that supersedes this one; a feature we stop using is dropped from the next.

| our method says | in Claude Code | source |
|---|---|---|
| a session gets its role and practices at its start | plugin hooks: SessionStart (stdout is context; `agent_type` when started with `claude --agent`), UserPromptSubmit (stdout is context; no env file) | code.claude.com/docs/en/hooks |
| the shell is ready for every command | SessionStart writes `. agent-env` into `$CLAUDE_ENV_FILE`, evaluated before each Bash command; `CLAUDE_CODE_SESSION_ID` is in every command's environment | code.claude.com/docs/en/hooks#persist-environment-variables |
| a started session names its role in its brief's head | the desktop app puts `<system-reminder>` (linked sessions) and the starter's `Background ... Task:` before the prompt; the hook reads past them | transcripts 6e79d078, 2026-11-17 |
| a role card | `agents/NAME.md`: name, description, model (opus, sonnet, haiku), effort, skills (preloaded whole, not a limit); a plugin's agents ignore hooks, mcpServers, permissionMode | code.claude.com/docs/en/sub-agents |
| start a session for another role | start_session (desktop): model and effort may be passed; refused on a turn a message or an event began - propose instead | the app's tool, refusals 2026-11-17 |
| wait for a condition | `sleep N` before a command is refused; run in the background or use Monitor | refusals 2026-11-17 (17) |
| what a session read | transcripts at `~/.claude/projects/PROJECT/SESSION.jsonl`; read-weight reads them; /context, /skill-doctor, /insights measure from inside | code.claude.com/docs/en/commands |
| list less at every start | the skill listing holds every skill's description each turn; `skillOverrides` (off, name-only, user-invocable-only) and `skillListingMaxDescChars` in settings trim it; no per-agent hiding | code.claude.com/docs/en/skills, settings-reference |
| connectors only when needed | turned on per session from the composer's + menu, Connectors; Gmail and Claude Docs off by default since 2026-11-17 (the author) | decision 2026-11-17-connectors-off-by-default |
| a released plugin reaches the lab | `claude plugin marketplace update ideal-agent`, `claude plugin update ideal-agent@ideal-agent`; running sessions keep their hooks until restarted | lab, 2026-11-17 |
| a lane asks a lighter model for research within its turn | the Agent tool with subagent_type ideal-agent:researcher and model sonnet (Sonnet 5.5 on 2026-11-17); it returns within the turn, which start_session cannot on a message's turn | the app's tools, 2026-11-17 |
| a lane starts its own sessions (launchers) | a scheduled task with no schedule (create_scheduled_task, no cron, no fireAt), run with run_scheduled_task on any turn - a message's too, where start_session is refused; refused while that task has a run in progress (one keeper per lane); keeps a permission mode only when created on a turn the author typed (else runs may stop to ask); the run's prompt is wrapped in `<scheduled-task ...>` with a line saying the user is absent and to make only the writes the task asks for | probe 2026-11-17 17:01 UTC; the tools' own terms |
