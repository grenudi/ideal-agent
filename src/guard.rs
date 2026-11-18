//! The ways, held when they are tried rather than remembered: Claude Code's PreToolUse hook gives each
//! Bash command to `ideal-agent guard`; a refused one exits 2 and says why on standard error (Claude Code
//! gives that to the agent and does not run the command).
//!
//! What is refused, in each simple command of a line (`sh -c` and `$(...)` included):
//!   - `git push` that reaches main (or master): every change gets its own branch and lands through checks
//!   - `python`, `pip` and their kin: no new Python - shell to orchestrate, Rust for logic
//!   - `git` that changes the checkout or its history (commit, add, reset, checkout, switch, clean, rebase,
//!     merge, stash, cherry-pick, revert, am, restore) in a shared clone's main checkout under ~/Projects:
//!     sessions write in a worktree of their own (`branch NAME`); `SHARED_CLONE_OK=1` lets a deliberate one through
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
    refused(command, &cwd, &current_branch, &shared_main)
}

/// Whether a directory is in the main checkout of a clone under ~/Projects - the one other sessions read and land
/// from - rather than in a linked worktree of its own (a worktree's git dir differs from the common one).
pub fn shared_main(dir: &Path) -> bool {
    let git = |args: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    };
    let (Some(top), Some(own), Some(common)) = (
        git(&["rev-parse", "--show-toplevel"]),
        git(&["rev-parse", "--absolute-git-dir"]),
        git(&["rev-parse", "--path-format=absolute", "--git-common-dir"]),
    ) else {
        return false;
    };
    let projects = std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join("Projects"));
    own == common && projects.is_some_and(|p| Path::new(&top).starts_with(p))
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
    shared: &dyn Fn(&Path) -> bool,
) -> Option<String> {
    // where each simple command runs: a `cd` moves the ones after it
    let mut here = cwd.to_path_buf();
    for words in simple_commands(command) {
        let words = unwrapped(&words);
        let Some(first) = words.first() else { continue };
        let program = first.rsplit('/').next().unwrap_or(first);
        let args = &words[1..];
        if program == "cd" {
            if let Some(d) = args.first() {
                here = match d.strip_prefix("~/") {
                    Some(rest) => std::env::var_os("HOME")
                        .map_or_else(|| here.join(d), |h| std::path::PathBuf::from(h).join(rest)),
                    None => here.join(d),
                };
            }
            continue;
        }
        let cwd = here.as_path();
        if matches!(program, "sh" | "bash" | "zsh" | "dash" | "fish") {
            if let Some(i) = args
                .iter()
                .position(|a| a == "-c" || a == "-lc" || a == "-ec")
            {
                if let Some(inner) = args.get(i + 1) {
                    if let Some(why) = refused(inner, cwd, branch_of, shared) {
                        return Some(why);
                    }
                }
            }
            continue;
        }
        if is_python(program) {
            return Some(format!(
                "`{program}` is refused: no new Python, not even for one-offs - shell to orchestrate, \
                 Rust for logic (rust-script for a single file). For JSON reach for jq, for text awk or sed, for a \
                 table tad or duckdb."
            ));
        }
        if program == "tad"
            && args.first().is_some_and(|a| a == "sql")
            && args[1..].iter().any(|a| writes(a))
            && !command.contains("TAD_SQL_WRITE_OK=1")
        {
            return Some(
                "`tad sql` that writes is refused: a change to a row goes through its named record, or, where none \
                 covers it, `ikb set TABLE --key K --field NAME=VALUE` or `ikb remove TABLE --key K` - checked, \
                 listing the tables and fields (a one-time migration says TAD_SQL_WRITE_OK=1 and is recorded)."
                    .to_string(),
            );
        }
        if program == "git" {
            if let Some(why) = git_push_to_main(args, cwd, branch_of) {
                return Some(why);
            }
            if !command.contains("SHARED_CLONE_OK=1") {
                if let Some(why) = git_writes_in_shared(args, cwd, shared) {
                    return Some(why);
                }
            }
        }
    }
    None
}

/// SQL that changes rows or tables, in any of the statements it holds.
fn writes(sql: &str) -> bool {
    sql.split(';').any(|s| {
        let s = s.trim_start().to_ascii_uppercase();
        [
            "INSERT", "UPDATE", "DELETE", "ALTER", "DROP", "CREATE", "REPLACE", "TRUNCATE",
        ]
        .iter()
        .any(|w| s.starts_with(w))
    })
}

fn is_python(program: &str) -> bool {
    let bare = program.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    matches!(bare, "python" | "pip" | "pipx" | "ipython" | "pypy")
}

/// A git command that changes the checkout or its history, run in a shared clone's main checkout.
fn git_writes_in_shared(
    args: &[String],
    cwd: &Path,
    shared: &dyn Fn(&Path) -> bool,
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
    let sub = args.get(i)?.as_str();
    let ff_only = args[i..].iter().any(|a| a == "--ff-only");
    let writes = matches!(
        sub,
        "commit"
            | "add"
            | "reset"
            | "checkout"
            | "switch"
            | "clean"
            | "rebase"
            | "stash"
            | "cherry-pick"
            | "revert"
            | "am"
            | "restore"
    ) || (matches!(sub, "merge" | "pull") && !ff_only);
    (writes && shared(&dir)).then(|| {
        format!(
            "`git {sub}` in a shared clone's main checkout is refused: other sessions read and land from it. \
             Work in a worktree of your own - `branch NAME` makes one from origin/main and moves you there; \
             `git pull --ff-only` keeps this checkout at main (SHARED_CLONE_OK=1 for a deliberate one)."
        )
    })
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
             (`land`, from a branch of a worktree of your own)."
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
        refused(
            command,
            Path::new("/tmp"),
            &move |_| Some(branch.to_string()),
            &|_| false,
        )
    }

    /// As says, with /home/u/Projects/docs a shared clone's main checkout and every other place a worktree.
    fn says_shared(command: &str, cwd: &str) -> Option<String> {
        refused(
            command,
            Path::new(cwd),
            &|_| Some("main".into()),
            &|d: &Path| d.starts_with("/home/u/Projects/docs"),
        )
    }

    #[test]
    fn writing_in_a_shared_clone_is_refused_and_a_worktree_is_not() {
        for c in [
            "git commit -m x",
            "git reset --hard origin/main",
            "git checkout old",
            "git clean -fd",
            "git add -A",
            "git stash",
            "git pull",
            "git merge feature",
        ] {
            assert!(
                says_shared(c, "/home/u/Projects/docs").is_some_and(|w| w.contains("branch NAME")),
                "refused: {c}"
            );
        }
        assert!(
            says_shared("cd /home/u/Projects/docs && git commit -m x", "/tmp").is_some(),
            "a cd moves the check"
        );
        assert!(
            says_shared("git -C /home/u/Projects/docs reset --hard", "/tmp").is_some(),
            "-C too"
        );
        for c in [
            "git status",
            "git fetch -q",
            "git log",
            "git pull --ff-only",
            "git worktree add ../wt -b x origin/main",
            "SHARED_CLONE_OK=1 git reset --hard origin/main",
            "git diff --stat",
        ] {
            assert!(
                says_shared(c, "/home/u/Projects/docs").is_none(),
                "allowed: {c}"
            );
        }
        assert!(
            says_shared("git commit -m x", "/home/u/wt/docs-branch").is_none(),
            "a worktree of its own"
        );
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
    fn tad_sql_that_writes_is_refused() {
        for c in [
            "tad sql \"UPDATE backlog SET status = 'done' WHERE id = 'x'\"",
            "cd docs && tad sql 'insert into sources values (1)'",
            "tad sql \"SELECT 1; DELETE FROM research WHERE id = 'y'\"",
            "sh -c \"tad sql 'DROP TABLE t'\"",
        ] {
            let why = says(c, "feat").unwrap_or_default();
            assert!(why.contains("ikb set"), "refused with the way: {c}");
        }
        for c in [
            "tad sql \"SELECT * FROM backlog\"",
            "tad sql 'select id from research where note like \"%update%\"'",
            "TAD_SQL_WRITE_OK=1 tad sql \"UPDATE backlog SET device = 'x'\"",
            "tad r backlog --id x --title y",
            "ikb set backlog --key x --field status=done",
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
