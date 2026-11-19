//! ideal-agent - what the plugin's hooks call. Its whole invocation surface is declared in contract.rs; how each
//! command does its work is in cli.rs. Read top to bottom, this is the whole procedure.
use anyhow::Result;

mod cli;
mod contract;
use contract::Command;

fn main() {
    if let Err(e) = run() {
        eprintln!("ideal-agent: {e:#}");
        std::process::exit(1);
    }
}

/// One run: which command was asked, what it was given, then that command.
fn run() -> Result<()> {
    let Some(command) = cli::command_asked() else {
        return Ok(());
    };
    let given = cli::Given::resolve();
    match command {
        Command::Guard => cli::guard(),
        Command::Practices => cli::practices(&given),
        Command::Start => cli::start(&given),
        Command::Role => cli::role(&given),
        Command::Completions => cli::completions(&given),
        Command::Man => cli::man(&given),
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::Contract;

    #[test]
    fn the_declaration_holds() {
        assert!(
            Contract::problems().is_empty(),
            "{:#?}",
            Contract::problems()
        );
    }
}
