//! ideal-agent - what the plugin's hooks call. Its whole invocation surface is declared in contract.rs.
use anyhow::{Context, Result};
use argenv_resolve::generate::{self, Program, Shell};
use argenv_resolve::*;
use ideal_agent::{guard, practices};
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
            let (text, source) = practices::read(from.as_deref(), root.as_deref());
            println!("## The practices (ideal-agent)\n");
            print!("{}", practices::render(&text, &source));
            // where lab-agent is installed (the lab), what was said to this session is read too
            if let Ok(out) = Run::new("lab-agent").arg("inbox").output() {
                println!("\n## Unread messages (lab-agent inbox; answer each, then --ack)\n");
                print!("{}", String::from_utf8_lossy(&out.stdout));
            }
            println!(
                "\nThe ideal agent's hooks refuse git push to main and python/pip (ideal-agent guard); its skills say how to start, land, record and change shared hardware."
            );
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
