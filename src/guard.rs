//! The ways, held when they are tried rather than remembered: Claude Code's PreToolUse hook gives each
//! Bash command to `ideal-agent guard`; a refused one exits 2 and says why on standard error (Claude Code
//! gives that to the agent and does not run the command).
//!
//! What is refused, in each simple command of a line (`sh -c` and `$(...)` included):
//!   - `git push` that reaches main (or master): every change gets its own branch and lands through checks
//!   - `python`, `pip` and their kin: no new Python - shell to orchestrate, Rust for logic
//!
//! What only one place's device needs (devlab's phone, say) stays with that place's own guard.

use crate::shell::{simple_commands, unwrapped};
use std::path::Path;
use std::process::Command;

/// Reads the hook's event on standard input; refuses with exit 2, or lets the command run.
pub fn hook(input: &str) -> Option<String> {
    let event: serde_json::Value = serde_json::from_str(input).unwrap_or_default();
    if event["tool_name"].as_str() != Some("Bash") {
        return None;
    }
    let command = event["tool_input"]["command"].as_str().unwrap_or_default();
    let cwd = event["cwd"]
        .as_str()
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_default();
    refused(command, &cwd, &current_branch)
}

fn current_branch(dir: &Path) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["symbolic-ref", "--short", "-q", "HEAD"])
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string()).filter(|b| !b.is_empty())
}

/// Why this command is refused, if it is.
pub fn refused(
    command: &str,
    cwd: &Path,
    branch_of: &dyn Fn(&Path) -> Option<String>,
) -> Option<String> {
    for words in simple_commands(command) {
        let words = unwrapped(&words);
        let Some(first) = words.first() else { continue };
        let program = first.rsplit('/').next().unwrap_or(first);
        let args = &words[1..];
        if matches!(program, "sh" | "bash" | "zsh" | "dash" | "fish") {
            if let Some(i) = args
                .iter()
                .position(|a| a == "-c" || a == "-lc" || a == "-ec")
            {
                if let Some(inner) = args.get(i + 1) {
                    if let Some(why) = refused(inner, cwd, branch_of) {
                        return Some(why);
                    }
                }
            }
            continue;
        }
        if is_python(program) {
            return Some(format!(
                "`{program}` is refused: no new Python, not even for one-offs - shell to orchestrate, \
                 Rust for logic (rust-script for a single file). For JSON reach for jq, for text awk or sed, for a
                 table tad or duckdb."
            ));
        }
        if program == "git" {
            if let Some(why) = git_push_to_main(args, cwd, branch_of) {
                return Some(why);
            }
        }
    }
    None
}

fn is_python(program: &str) -> bool {
    let bare = program.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    matches!(bare, "python" | "pip" | "pipx" | "ipython" | "pypy")
}

fn git_push_to_main(
    args: &[String],
    cwd: &Path,
    branch_of: &dyn Fn(&Path) -> Option<String>,
) -> Option<String> {
    let mut dir = cwd.to_path_buf();
    let mut i = 0;
    while i < args.len() && args[i].starts_with('-') {
        match args[i].as_str() {
            "-C" => {
                if let Some(d) = args.get(i + 1) {
                    dir = cwd.join(d);
                }
                i += 2;
            }
            "-c" | "--git-dir" | "--work-tree" | "--namespace" => i += 2,
            _ => i += 1,
        }
    }
    if args.get(i).map(String::as_str) != Some("push") {
        return None;
    }
    let rest = &args[i + 1..];
    let refuse = |what: &str| {
        Some(format!(
            "`git push` {what} is refused: every change gets its own branch and lands through its checks \
             (`land`, from a shell that sourced oco-work, on the lab)."
        ))
    };
    if rest.iter().any(|a| a == "--all" || a == "--mirror") {
        return refuse("of every branch");
    }
    let deleting = rest.iter().any(|a| a == "--delete" || a == "-d");
    let mut positional = Vec::new();
    let mut j = 0;
    while j < rest.len() {
        let a = &rest[j];
        if matches!(
            a.as_str(),
            "-o" | "--push-option" | "--repo" | "--receive-pack" | "--exec"
        ) {
            j += 2;
            continue;
        }
        if !a.starts_with('-') {
            positional.push(a.as_str());
        }
        j += 1;
    }
    let here = || branch_of(&dir);
    let is_main = |b: &str| matches!(b, "main" | "master");
    let refspecs = positional.get(1..).unwrap_or(&[]);
    if refspecs.is_empty() {
        return match here() {
            Some(b) if is_main(&b) => refuse("from main"),
            _ => None,
        };
    }
    if deleting {
        return refspecs
            .iter()
            .any(|r| is_main(r.trim_start_matches("refs/heads/")))
            .then(|| "`git push --delete` of main is refused.".to_string());
    }
    for spec in refspecs {
        let spec = spec.trim_start_matches('+');
        let dest = spec.rsplit(':').next().unwrap_or(spec);
        let dest = dest.trim_start_matches("refs/heads/");
        let dest = if dest == "HEAD" || dest == "@" {
            here().unwrap_or_default()
        } else {
            dest.to_string()
        };
        if is_main(&dest) {
            return refuse("to main");
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn says(command: &str, branch: &'static str) -> Option<String> {
        refused(command, Path::new("/tmp"), &move |_| {
            Some(branch.to_string())
        })
    }

    #[test]
    fn a_push_that_reaches_main_is_refused() {
        for c in [
            "git push origin main",
            "git push -f origin main",
            "git push origin HEAD:main",
            "git push origin +feat:refs/heads/main",
            "cd x && git push",
            "git -C ../docs push origin master",
            "git push --all",
            "GIT_TRACE=1 git push -u origin HEAD",
            "bash -c 'git push origin main'",
        ] {
            assert!(says(c, "main").is_some(), "refused: {c}");
        }
    }

    #[test]
    fn a_push_of_a_branch_is_not() {
        for c in [
            "git push origin feat",
            "git push -u origin HEAD",
            "git push",
            "git push origin --delete old-branch",
            "echo git push origin main",
            "git log main && git status",
            "land",
        ] {
            assert!(says(c, "feat").is_none(), "allowed: {c}");
        }
    }

    #[test]
    fn python_is_refused_wherever_it_runs() {
        for c in [
            "python3 -c 'print(1)'",
            "/usr/bin/python script.py",
            "pip install x",
            "pip3 install --user y",
            "sudo python3 x.py",
            "env FOO=1 python x",
            "timeout 10 python3 x",
            "ls | xargs -n1 python3",
            "echo $(python3 -c 1)",
            "X=\"$(python -V)\"",
            "sh -c \"python3 x\"",
        ] {
            assert!(says(c, "feat").is_some(), "refused: {c}");
        }
        for c in [
            "grep python file",
            "echo 'python3 x'",
            "rg -n pip src",
            "cargo build --features python",
            "ls ~/.python",
        ] {
            assert!(says(c, "feat").is_none(), "allowed: {c}");
        }
    }

    #[test]
    fn only_bash_is_read() {
        assert!(
            hook(r#"{"tool_name":"Write","tool_input":{"command":"pip install x"}}"#).is_none()
        );
        assert!(hook(r#"{"tool_name":"Bash","tool_input":{"command":"pip install x"}}"#).is_some());
        assert!(hook("not json").is_none());
    }
}
