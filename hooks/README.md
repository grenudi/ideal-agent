# hooks/ - which command each Claude Code hook runs

`hooks.json` only: SessionStart and UserPromptSubmit run `scripts/ideal-agent start` and `role`; PreToolUse
runs `guard` for Bash, Read, Grep, Glob, Write and Edit. What they do is the program's (`src/`).
