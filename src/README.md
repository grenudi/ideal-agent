# src/ - the program

`ideal-agent`, in Rust, built per version into the plugin's data folder by `scripts/ideal-agent`. Its
commands and inputs are declared once in `contract.rs` (argenv); `main.rs` reads as the procedure and `cli.rs`
holds each command: guard (`guard.rs`, shell words by `shell.rs`), start and role (`roles.rs`), practices
(`practices.rs`), prune (`prune.rs`), layout-check (`layout.rs`).
