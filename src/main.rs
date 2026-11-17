//! ideal-agent - what the plugin's hooks call. Its whole invocation surface is declared in contract.rs.
use anyhow::{Context, Result};
use argenv_resolve::generate::{self, Program, Shell};
use argenv_resolve::*;
use ideal_agent::{guard, practices, roles};
use std::io::Read;
use std::path::PathBuf;
use std::process::Command as Run;

mod contract;
use contract::{Command, Contract};

const ABOUT: &str = "The ideal agent's hooks: the ways held when they are tried, and the practices at the start of every session";

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
        see_also: &["claude(1)", "lab-agent(1)"],
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

fn main() {
    if let Err(e) = run() {
        eprintln!("ideal-agent: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(first) = args.first().cloned() else {
        print!("{}", usage());
        return Ok(());
    };
    match first.as_str() {
        "-h" | "--help" | "help" => {
            print!("{}", usage());
            return Ok(());
        }
        "-V" | "--version" => {
            println!("ideal-agent {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        _ => {}
    }
    let Some(command) = Command::parse(&first) else {
        eprint!("ideal-agent: '{first}' is not a command\n\n{}", usage());
        std::process::exit(2);
    };
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
    let from = get(&Contract::FROM).map(PathBuf::from);
    let root = get(&Contract::PLUGIN_ROOT).map(PathBuf::from);
    let env_file = get(&Contract::ENV_FILE).map(PathBuf::from);

    match command {
        Command::Guard => {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input)?;
            if let Some(why) = guard::hook(&input) {
                eprintln!("ideal-agent guard: {why}");
                std::process::exit(2);
            }
        }
        Command::Practices => {
            let (text, source) = practices::read(from.as_deref(), root.as_deref());
            print!("{}", practices::render(&text, &source));
        }
        Command::Start => {
            let event = read_event();
            if let (Some(f), Some(r)) = (env_file.as_deref(), root.as_deref()) {
                agent_env(f, r);
            }
            let session = event["session_id"].as_str().unwrap_or("");
            let source = event["source"].as_str().unwrap_or("");
            let agent = event["agent_type"]
                .as_str()
                .map(|a| a.trim_start_matches("ideal-agent:"));
            let card = |r: &str| {
                root.as_deref()
                    .is_some_and(|p| roles::card_path(p, r).is_file())
            };
            let (text, from_where) = practices::read(from.as_deref(), root.as_deref());
            if let Some(a) = agent.filter(|a| card(a)) {
                // started as a role (claude --agent ideal-agent:ROLE): its card is the system prompt already
                let named = roles::Named {
                    role: a.to_string(),
                    also: vec![],
                    name: None,
                };
                roles::remember(session, Some(&named));
                print!(
                    "{}",
                    roles::render(root.as_deref(), &named, &text, &from_where)
                );
            } else if let Some(given) = (source != "startup")
                .then(|| roles::recall(session))
                .flatten()
            {
                // a resume, a compaction or a clear reads again what this session was given
                match given {
                    Some(named) => print!(
                        "{}",
                        roles::render(root.as_deref(), &named, &text, &from_where)
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
                println!("They come with your first prompt: your role's card and only its practices when the brief names a role (`Role: NAME` - coordinator, lane-coordinator, orchestrator, keeper, research-lead, builder, patrol), else every practice.");
            }
            // where lab-agent is installed (the lab), what was said to this session is read too
            if let Ok(out) = Run::new("lab-agent").arg("inbox").output() {
                println!("\n## Unread messages (lab-agent inbox; answer each, then --ack)\n");
                print!("{}", String::from_utf8_lossy(&out.stdout));
            }
            println!(
                "\nThe ideal agent's hooks refuse git push to main and python/pip (ideal-agent guard); its skills say how to start, land, record and change shared hardware."
            );
        }
        Command::Role => {
            let event = read_event();
            let session = event["session_id"].as_str().unwrap_or("");
            // only the first prompt of a session that started with roles: never a later one, never a message
            if !roles::awaiting(session) {
                return Ok(());
            }
            let (text, from_where) = practices::read(from.as_deref(), root.as_deref());
            match roles::named_in(event["prompt"].as_str().unwrap_or("")) {
                Some(named) => {
                    roles::remember(session, Some(&named));
                    print!(
                        "{}",
                        roles::render(root.as_deref(), &named, &text, &from_where)
                    );
                }
                None => {
                    roles::remember(session, None);
                    print_all(&text, &from_where);
                }
            }
        }
        Command::Completions => {
            let shell = get(&Contract::SHELL)
                .and_then(|s| Shell::parse(&s))
                .context("bash, zsh, fish or nushell")?;
            let commands = commands();
            print!(
                "{}",
                generate::completions(shell, &program(&commands, ""), &Contract::records())?
            );
        }
        Command::Man => {
            let date = get(&Contract::DATE).unwrap_or_default();
            let commands = commands();
            print!(
                "{}",
                generate::man_page(&program(&commands, &date), &Contract::records())?
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_declaration_holds() {
        assert!(
            Contract::problems().is_empty(),
            "{:#?}",
            Contract::problems()
        );
    }
}
