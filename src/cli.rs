//! How each command of ideal-agent does its work; main.rs reads which one runs.
use crate::contract::{Command, Contract};
use anyhow::{Context, Result};
use argenv_resolve::generate::{self, Program, Shell};
use argenv_resolve::*;
use ideal_agent::{guard, layout, practices, prune, roles};
use std::io::Read;
use std::path::{Path, PathBuf};

const ABOUT: &str = "The ideal agent's hooks: the ways held when they are tried, and the practices at the start of every session";

/// The command the arguments ask for; None when they only asked for help or the version (printed here). A word
/// that is no command ends the run with 2.
pub fn command_asked() -> Option<Command> {
    let first = std::env::args().nth(1);
    let Some(first) = first else {
        print!("{}", usage());
        return None;
    };
    match first.as_str() {
        "-h" | "--help" | "help" => {
            print!("{}", usage());
            return None;
        }
        "-V" | "--version" => {
            println!("ideal-agent {}", env!("CARGO_PKG_VERSION"));
            return None;
        }
        _ => {}
    }
    let Some(command) = Command::parse(&first) else {
        eprint!("ideal-agent: '{first}' is not a command\n\n{}", usage());
        std::process::exit(2);
    };
    Some(command)
}

/// What the invocation gave, resolved once against the contract.
pub struct Given {
    from: Option<PathBuf>,
    root: Option<PathBuf>,
    env_file: Option<PathBuf>,
    shell: Option<String>,
    date: Option<String>,
    mode: Option<String>,
}

impl Given {
    pub fn resolve() -> Given {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let resolved = Invocation {
            args: &args,
            env: &ProcessEnv,
        }
        .resolve(&Contract::records());
        let p = Contract::problems();
        assert!(p.is_empty(), "declaration errors: {p:#?}");
        handle_problems(
            &lint(&Contract::records(), &resolved),
            OnProblems::FailOnError,
        );
        let get = |input: &Input<String, Command>| {
            input
                .get_from_or_default(&resolved)
                .filter(|v| !v.is_empty())
        };
        Given {
            from: get(&Contract::FROM).map(PathBuf::from),
            root: get(&Contract::PLUGIN_ROOT).map(PathBuf::from),
            env_file: get(&Contract::ENV_FILE).map(PathBuf::from),
            shell: get(&Contract::SHELL),
            date: get(&Contract::DATE),
            mode: get(&Contract::MODE),
        }
    }

    fn practices(&self) -> (String, String) {
        practices::read(self.from.as_deref(), self.root.as_deref())
    }
}

/// The PreToolUse hook: a refused command exits 2, saying why.
pub fn guard() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    if let Some(why) = guard::hook(&input) {
        eprintln!("ideal-agent guard: {why}");
        std::process::exit(2);
    }
    Ok(())
}

/// Every practice, whole.
pub fn practices(given: &Given) -> Result<()> {
    let (text, source) = given.practices();
    print!("{}", practices::render(&text, &source));
    Ok(())
}

/// The SessionStart hook: the neutral start named, then the role's card and practices - or every practice.
pub fn start(given: &Given) -> Result<()> {
    let event = read_event();
    if let (Some(f), Some(r)) = (given.env_file.as_deref(), given.root.as_deref()) {
        agent_env(f, r);
        // only a real session start (Claude Code gave its environment file), never a test or a hand run
        prune_daily();
    }
    let root = given.root.as_deref();
    let session = event["session_id"].as_str().unwrap_or("");
    let source = event["source"].as_str().unwrap_or("");
    let agent = event["agent_type"]
        .as_str()
        .map(|a| a.trim_start_matches("ideal-agent:"));
    let card = |r: &str| root.is_some_and(|p| roles::card_path(p, r).is_file());
    let (text, from_where) = given.practices();
    if let Some(a) = agent.filter(|a| card(a)) {
        // started as a role (claude --agent ideal-agent:ROLE): its card is the system prompt already
        let named = roles::Named {
            role: a.to_string(),
            also: vec![],
            name: None,
            brief: None,
        };
        roles::remember(session, Some(&named));
        print!(
            "{}",
            roles::render(root, session, &named, &text, &from_where)
        );
    } else if let Some(kept) = (source != "startup")
        .then(|| roles::recall(session))
        .flatten()
    {
        // a resume, a compaction or a clear reads again what this session was given
        match kept {
            Some(named) => print!(
                "{}",
                roles::render(root, session, &named, &text, &from_where)
            ),
            None => print_all(&text, &from_where),
        }
    } else if session.is_empty() {
        print_all(&text, &from_where);
    } else if source != "startup" && !roles::awaiting(session) {
        // a resumed session with no state kept under the first-prompt rule - started before roles, or
        // re-roled by a message under 0.3.1: every practice, and no later prompt names its role
        roles::remember(session, None);
        print_all(&text, &from_where);
    } else {
        roles::await_first(session);
        println!("## The practices (ideal-agent)\n");
        println!("They come with your first prompt: your role's card and only its practices when the brief names a role (`Role: NAME` - coordinator, lane-coordinator, orchestrator, keeper, research-lead, builder, patrol, steward), else every practice.");
    }
    println!(
        "\nThe ideal agent's hooks refuse git push to main, python/pip, and reads in a shared clone behind its origin/main (ideal-agent guard); its skills say how to start, land, record and change shared hardware."
    );
    Ok(())
}

/// The UserPromptSubmit hook: on a session's first prompt only, the role it names - or every practice.
pub fn role(given: &Given) -> Result<()> {
    let event = read_event();
    let session = event["session_id"].as_str().unwrap_or("");
    // only the first prompt of a session that started with roles: never a later one, never a message
    if !roles::awaiting(session) {
        return Ok(());
    }
    let (text, from_where) = given.practices();
    match roles::named_in(event["prompt"].as_str().unwrap_or("")) {
        Some(named) => {
            roles::remember(session, Some(&named));
            print!(
                "{}",
                roles::render(given.root.as_deref(), session, &named, &text, &from_where)
            );
        }
        None => {
            roles::remember(session, None);
            print_all(&text, &from_where);
        }
    }
    Ok(())
}

/// What agents leave behind - build run folders and clean worktrees nothing wrote to for 3 days: removed or shown.
pub fn prune(given: &Given) -> Result<()> {
    let now = given.mode.as_deref() == Some("now");
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .context("HOME")?;
    let cache =
        std::env::var_os("XDG_CACHE_HOME").map_or_else(|| home.join(".cache"), PathBuf::from);
    let mut fates = prune::sweep_runs(&cache.join("agent-build"), 3, now);
    fates.extend(prune::sweep(&home.join("Projects"), 3, now));
    print!("{}", prune::report(&fates));
    Ok(())
}

/// Once a day, the session start leaves `prune now` running on its own - in a process group of its own, so the
/// hook returns at once - its report appended to ideal-agent's state (pruned.log).
fn prune_daily() {
    use std::os::unix::process::CommandExt;
    let Some(dir) = roles::state_dir().and_then(|d| d.parent().map(Path::to_path_buf)) else {
        return;
    };
    let stamp = dir.join("pruned");
    let fresh = std::fs::metadata(&stamp)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.elapsed().ok())
        .is_some_and(|age| age.as_secs() < 24 * 3600);
    if fresh || std::fs::create_dir_all(&dir).is_err() || std::fs::write(&stamp, "").is_err() {
        return;
    }
    let (Ok(exe), Ok(log)) = (
        std::env::current_exe(),
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("pruned.log")),
    ) else {
        return;
    };
    let _ = std::process::Command::new(exe)
        .args(["prune", "now"])
        .stdin(std::process::Stdio::null())
        .stdout(
            log.try_clone()
                .map_or_else(|_| std::process::Stdio::null(), Into::into),
        )
        .stderr(log)
        .process_group(0)
        .spawn();
}

/// The repository here, checked against its README's map: each problem named, exit 1 while any is left.
pub fn layout_check() -> Result<()> {
    let git = |args: &[&str]| -> Result<String> {
        let o = std::process::Command::new("git").args(args).output()?;
        anyhow::ensure!(
            o.status.success(),
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&o.stderr).trim()
        );
        Ok(String::from_utf8_lossy(&o.stdout).into_owned())
    };
    let top = PathBuf::from(git(&["rev-parse", "--show-toplevel"])?.trim());
    // what is tracked: the index, so the check holds before a commit as in CI
    let files: std::collections::BTreeSet<String> =
        git(&["-C", &top.to_string_lossy(), "ls-files"])?
            .lines()
            .map(String::from)
            .collect();
    let folders: Vec<String> = files
        .iter()
        .filter_map(|f| f.split_once('/').map(|(d, _)| d.to_string()))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let readme = std::fs::read_to_string(top.join("README.md")).ok();
    let problems = layout::problems(&folders, &files, readme.as_deref());
    for p in &problems {
        println!("{p}");
    }
    if !problems.is_empty() {
        eprintln!(
            "layout-check: {} problem(s) in {}",
            problems.len(),
            top.display()
        );
        std::process::exit(1);
    }
    println!(
        "layout-check: every top-level folder of {} has a README and a Map line",
        top.display()
    );
    Ok(())
}

/// Shell completions, from the contract.
pub fn completions(given: &Given) -> Result<()> {
    let shell = given
        .shell
        .as_deref()
        .and_then(Shell::parse)
        .context("bash, zsh, fish or nushell")?;
    let commands = commands();
    print!(
        "{}",
        generate::completions(shell, &program(&commands, ""), &Contract::records())?
    );
    Ok(())
}

/// The manual page, from the contract.
pub fn man(given: &Given) -> Result<()> {
    let date = given.date.clone().unwrap_or_default();
    let commands = commands();
    print!(
        "{}",
        generate::man_page(&program(&commands, &date), &Contract::records())?
    );
    Ok(())
}

fn usage() -> String {
    let mut s = format!(
        "ideal-agent {}: {ABOUT}\n\nusage: ideal-agent COMMAND [...]   (ideal-agent man for the whole)\n\n",
        env!("CARGO_PKG_VERSION")
    );
    for c in Command::ALL {
        s.push_str(&format!("  {:<12} {}\n", c.token(), c.summary()));
    }
    s
}

fn commands() -> Vec<generate::Command<'static>> {
    Command::ALL
        .iter()
        .map(|c| generate::Command {
            name: c.token(),
            summary: c.summary(),
        })
        .collect()
}

fn program<'a>(commands: &'a [generate::Command<'a>], date: &'a str) -> Program<'a> {
    Program {
        name: "ideal-agent",
        version: env!("CARGO_PKG_VERSION"),
        about: ABOUT,
        commands,
        exit_status: &[
            ("0", "done; for guard, the command may run"),
            ("1", "it could not be done"),
            ("2", "guard: the command is refused, and why is on standard error; otherwise the invocation is wrong"),
        ],
        see_also: &["claude(1)"],
        date,
        asked: &[],
    }
}

/// The hook's event, from standard input when it is not a terminal; an empty object otherwise.
fn read_event() -> serde_json::Value {
    use std::io::IsTerminal;
    let mut input = String::new();
    if !std::io::stdin().is_terminal() {
        let _ = std::io::stdin().read_to_string(&mut input);
    }
    serde_json::from_str(&input).unwrap_or(serde_json::Value::Null)
}

/// Name the neutral start (scripts/agent-env) in the session's environment file, once: every Bash command of the
/// session then sources it first (decision 2026-11-17-a-neutral-start).
fn agent_env(file: &std::path::Path, root: &std::path::Path) {
    let script = root.join("scripts/agent-env");
    if !script.is_file() {
        return;
    }
    let line = format!(
        ". '{}'\n",
        script.display().to_string().replace('\'', "'\\''")
    );
    let held = std::fs::read_to_string(file).unwrap_or_default();
    if !held.contains(line.trim_end()) {
        use std::io::Write;
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file)
            .and_then(|mut f| f.write_all(line.as_bytes()));
    }
}

fn print_all(text: &str, source: &str) {
    println!("## The practices (ideal-agent)\n");
    print!("{}", practices::render_short(text, source, None));
}
