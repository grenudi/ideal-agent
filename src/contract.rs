//! ideal-agent's command line, declared once with argenv: every command and every input it reads. The
//! parsing, the help, the shell completions and the man page all come from these declarations.
use argenv_resolve::*;

/// Every command.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    Guard,
    Start,
    Role,
    Practices,
    Completions,
    Man,
}

impl Command {
    pub const ALL: &'static [Command] = &[
        Command::Guard,
        Command::Start,
        Command::Role,
        Command::Practices,
        Command::Completions,
        Command::Man,
    ];

    /// How it is typed.
    pub const fn token(self) -> &'static str {
        match self {
            Command::Guard => "guard",
            Command::Start => "start",
            Command::Role => "role",
            Command::Practices => "practices",
            Command::Completions => "completions",
            Command::Man => "man",
        }
    }

    /// One line for the help and the man page.
    pub const fn summary(self) -> &'static str {
        match self {
            Command::Guard => "PreToolUse hook: read the event on standard input; refuse (exit 2, the reason on standard error) a Bash command that pushes to main or runs Python",
            Command::Start => "SessionStart hook (the event on standard input): a resumed or compacted session's role and practices again, or a role given by --agent; at a new start only where they come from - then the unread messages where lab-agent is installed (read, not acknowledged)",
            Command::Role => "UserPromptSubmit hook (the event on standard input): at a session's first prompt, the role its brief names (Role: NAME) - its card and only its practices - or every practice when none is named",
            Command::Practices => "The practices, numbered, each with why and for whom - live from ideal-agent's main, else the installed copy",
            Command::Completions => "Print shell completions: bash, zsh, fish or nushell",
            Command::Man => "Print the man page",
        }
    }

    pub fn parse(token: &str) -> Option<Command> {
        Command::ALL.iter().copied().find(|c| c.token() == token)
    }
}

impl serde::Serialize for Command {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.token())
    }
}

contract! {
    /// A file to read the practices from instead.
    FROM: String, Command = Input {
        key:         "from",
        ty:          Type::Path,
        subcommands: &[Command::Start, Command::Role, Command::Practices],
        arg:         Some(Arg { value_name: "FILE", ..Arg::long("from") }),
        summary:     "Read the practices from this file (one JSON object per line) instead of ideal-agent's main",
        ..Input::EMPTY
    };
    /// Where the plugin is installed.
    PLUGIN_ROOT: String, Command = Input {
        key:         "plugin_root",
        ty:          Type::Path,
        subcommands: &[Command::Start, Command::Role, Command::Practices],
        env:         Some(Env { name: "CLAUDE_PLUGIN_ROOT", ..Env::EMPTY }),
        summary:     "Where Claude Code installed the plugin: its data/practices.json is read when main cannot be reached",
        ..Input::EMPTY
    };
    /// Where Claude Code keeps what every Bash command of the session evaluates first.
    ENV_FILE: String, Command = Input {
        key:         "env_file",
        ty:          Type::Path,
        subcommands: &[Command::Start],
        env:         Some(Env { name: "CLAUDE_ENV_FILE", ..Env::EMPTY }),
        summary:     "The session's environment file (Claude Code gives it to the SessionStart hook): the start names scripts/agent-env in it, so that every Bash command has the token, the PATH, the name the work is signed with, branch and land",
        ..Input::EMPTY
    };
    /// Which shell.
    SHELL: String, Command = Input {
        key:         "shell",
        ty:          Type::Enum,
        required:    true,
        allowed:     &["bash", "zsh", "fish", "nushell"],
        subcommands: &[Command::Completions],
        arg:         Some(Arg { value_name: "SHELL", ..Arg::positional(0) }),
        summary:     "The shell to print completions for",
        ..Input::EMPTY
    };
    /// The man page's date.
    DATE: String, Command = Input {
        key:         "date",
        ty:          Type::String,
        subcommands: &[Command::Man],
        arg:         Some(Arg { value_name: "YYYY-MM-DD", ..Arg::long("date") }),
        summary:     "The man page's date, so that it is reproducible",
        ..Input::EMPTY
    };
}
