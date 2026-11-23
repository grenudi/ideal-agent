//! The ways, held when they are tried rather than remembered: Claude Code's PreToolUse hook gives each
//! Bash command to `ideal-agent guard`; a refused one exits 2 and says why on standard error (Claude Code
//! gives that to the agent and does not run the command).
//!
//! What is refused, in each simple command of a line (`sh -c` and `$(...)` included):
//!   - `git push` that reaches main (or master): every change gets its own branch and lands through checks
//!   - `python`, `pip` and their kin: no Python of our own - shell to orchestrate, Rust for logic; installing a
//!     third-party tool a step requires, by name, in its own environment under ~/.local, is let through
//!   - `git` that changes the checkout or its history (commit, add, reset, checkout, switch, clean, rebase,
//!     merge, stash, cherry-pick, revert, am, restore) in a shared clone's main checkout under ~/Projects:
//!     sessions write in a worktree of their own (`branch NAME`); `SHARED_CLONE_OK=1` lets a deliberate one through
//!   - reading in a shared clone's main checkout whose HEAD is behind its local origin/main - a command run there
//!     (or naming a path there) other than git, `branch` and a few that only look around, and Read, Grep and Glob
//!     of its paths: what a role acts on is origin/main, read in a worktree of its own or with `git show`
//!     (decision 2026-11-19-roles-act-on-origin-main); `SHARED_CLONE_OK=1` lets a deliberate one through
//!
//! What only one place's device needs (devlab's phone, say) stays with that place's own guard.

use crate::shell::{simple_commands, unwrapped};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Reads the hook's event on standard input; refuses with exit 2, or lets the command run.
pub fn hook(input: &str) -> Option<String> {
    let event: serde_json::Value = serde_json::from_str(input).unwrap_or_default();
    let cwd = event["cwd"]
        .as_str()
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_default();
    let deliberate = std::env::var("SHARED_CLONE_OK").is_ok_and(|v| v == "1");
    let behind = |d: &Path| (!deliberate).then(|| behind_main(d)).flatten();
    match event["tool_name"].as_str() {
        Some("Bash") => {
            let command = event["tool_input"]["command"].as_str().unwrap_or_default();
            refused(
                command,
                &cwd,
                &current_branch,
                &shared_main,
                &behind,
                &loose_in_projects,
            )
        }
        Some("Write" | "Edit" | "MultiEdit" | "NotebookEdit") => {
            let input = &event["tool_input"];
            let path = ["file_path", "notebook_path"]
                .iter()
                .find_map(|k| input[k].as_str())
                .map(|p| cwd.join(p))?;
            loose_in_projects(&path).then(|| loose_write(&path))
        }
        Some("Read" | "Grep" | "Glob") => {
            let input = &event["tool_input"];
            let path = ["file_path", "path"]
                .iter()
                .find_map(|k| input[k].as_str())
                .map_or_else(|| cwd.clone(), |p| cwd.join(p));
            behind(&path).map(|(top, n)| stale_read("this read", &top, n))
        }
        _ => None,
    }
}

/// The clone a path is in and how many commits its HEAD is behind its local origin/main, when the path is in a
/// shared clone's main checkout under ~/Projects (`shared_main`) that is behind; no fetch - every worktree add
/// fetches through the clone, so its origin/main is as new as the last session's.
pub fn behind_main(path: &Path) -> Option<(PathBuf, u32)> {
    let projects = std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Projects"))?;
    if !path.starts_with(&projects) {
        return None;
    }
    let dir = path.ancestors().find(|a| a.is_dir())?;
    if !shared_main(dir) {
        return None;
    }
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
    let n: u32 = git(&["rev-list", "--count", "HEAD..refs/remotes/origin/main"])?
        .parse()
        .ok()?;
    let top = git(&["rev-parse", "--show-toplevel"])?;
    (n > 0).then(|| (PathBuf::from(top), n))
}

fn stale_read(what: &str, top: &Path, n: u32) -> String {
    let t = top.display();
    format!(
        "{what} is refused: {t} is a shared clone {n} commit(s) behind its origin/main, and what a role acts on is \
         main. Read main in a worktree of your own - `branch --read {t}` makes one at origin/main in \
         the worktree home and moves you there - or one file with `git -C {t} show origin/main:PATH`; \
         the brief your start named is whole in the file it gave. `git pull --ff-only` there brings the clone to \
         main (SHARED_CLONE_OK=1 for a deliberate read)."
    )
}

/// Programs that run in a stale shared clone without reading its state: git (how main is reached), `branch`
/// (which leaves for a worktree), and those that only look where they are.
fn reads_no_state(program: &str) -> bool {
    matches!(
        program,
        "git" | "branch" | "ls" | "pwd" | "echo" | "printf" | "true" | "false" | "test" | "["
    )
}

/// Where every worktree lives (decision 2026-11-23-structure-lab-machine-homes, linux-phone-oco/docs):
/// $AGENT_WORKTREES, else ~/.local/share/agent-worktrees - what agent-env's `branch` makes.
pub fn worktree_home() -> PathBuf {
    std::env::var_os("AGENT_WORKTREES")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share/agent-worktrees"))
        })
        .unwrap_or_default()
}

/// A word as the shell would give it for a path: `~`, `~/`, `$HOME` and `${HOME}` expanded; a relative path
/// joined to where the command runs; any other variable left as it is.
fn expand_path(word: &str, here: &Path) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    let w = if word == "~" || word == "$HOME" || word == "${HOME}" {
        home.clone()
    } else if let Some(rest) = word
        .strip_prefix("~/")
        .or_else(|| word.strip_prefix("$HOME/"))
        .or_else(|| word.strip_prefix("${HOME}/"))
    {
        format!("{home}/{rest}")
    } else {
        word.to_string()
    };
    if w.starts_with('/') || w.starts_with('$') {
        PathBuf::from(w)
    } else {
        here.join(w)
    }
}

/// `git worktree add PATH` with PATH outside the worktree home. A path that begins with a variable other than
/// HOME cannot be told and passes; one with a variable later is judged by what comes before it.
fn worktree_outside_home(args: &[String], here: &Path) -> Option<String> {
    // git's own options before the subcommand: -C DIR moves where a relative path is read from
    let mut here = here.to_path_buf();
    let mut i = 0;
    while i < args.len() && args[i].starts_with('-') {
        if args[i] == "-C" {
            if let Some(d) = args.get(i + 1) {
                here = expand_path(d, &here);
            }
            i += 1;
        } else if args[i] == "-c" {
            i += 1;
        }
        i += 1;
    }
    if args.get(i).map(String::as_str) != Some("worktree")
        || args.get(i + 1).map(String::as_str) != Some("add")
    {
        return None;
    }
    let mut rest = args[i + 2..].iter();
    let path = loop {
        let a = rest.next()?;
        match a.as_str() {
            "-b" | "-B" | "--reason" => {
                rest.next();
            }
            "--" => break rest.next()?,
            o if o.starts_with('-') => {}
            _ => break a,
        }
    };
    let p = expand_path(path, &here).to_string_lossy().into_owned();
    let known = p.split('$').next().unwrap_or_default();
    if known.is_empty() {
        return None;
    }
    let home = worktree_home().to_string_lossy().into_owned();
    let inside = Path::new(known).starts_with(&home)
        || (p.contains('$') && format!("{home}/").starts_with(known));
    (!inside).then(|| {
        format!(
            "`git worktree add {path}` is refused: every worktree lives in {home}/REPO/NAME (decision \
             2026-11-23-structure-lab-machine-homes) - `branch ROLE/WHAT` makes one to write in from origin/main, \
             `branch --read CLONE` a detached one to read main in; a tool makes its own there, under its name \
             (REPO/TOOL/NAME)."
        )
    })
}

/// Where a shell line writes a file by redirection (`> F`, `>> F`, `2>F`), best effort: what the parse sees.
fn redirect_targets(words: &[String], here: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut it = words.iter().peekable();
    while let Some(w) = it.next() {
        let Some(at) = w.rfind('>') else { continue };
        let after = &w[at + 1..];
        if after.starts_with('&') || w[..at].ends_with('<') {
            continue;
        }
        let target = if after.is_empty() {
            match it.peek() {
                Some(n) => n.as_str(),
                None => continue,
            }
        } else {
            after
        };
        if target != "/dev/null" && !target.starts_with('&') {
            out.push(expand_path(target, here));
        }
    }
    out
}

/// Whether a path under ~/Projects is outside every git working tree there - a loose file where only clones
/// belong (decision 2026-11-23-structure-checks-refuse-wrong-homes). A place outside git that says what it is -
/// a README.md at its top, as a build area kept under ~/Projects has - is a home of its own; and a README.md
/// itself is how a place says so, never loose.
pub fn loose_in_projects(path: &Path) -> bool {
    let Some(projects) = std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Projects")) else {
        return false;
    };
    if !path.starts_with(&projects) || path.file_name().is_some_and(|n| n == "README.md") {
        return false;
    }
    let explained = path
        .ancestors()
        .skip(1)
        .take_while(|a| *a != projects)
        .any(|a| a.join("README.md").is_file());
    if explained {
        return false;
    }
    let Some(dir) = path
        .parent()
        .and_then(|p| p.ancestors().find(|a| a.is_dir()))
    else {
        return false;
    };
    !Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .is_ok_and(|o| o.status.success() && o.stdout.starts_with(b"true"))
}

fn loose_write(path: &Path) -> String {
    format!(
        "writing {} is refused: under ~/Projects only clones live - and places outside git whose README says what \
         they are - and this is in neither (decision 2026-11-23-structure-checks-refuse-wrong-homes). A file belongs in a repository - in a \
         worktree of your own (`branch ROLE/WHAT`) - a run's output in $AGENT_BUILD, a note in your session's \
         scratchpad.",
        path.display()
    )
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
    behind: &dyn Fn(&Path) -> Option<(PathBuf, u32)>,
    loose: &dyn Fn(&Path) -> bool,
) -> Option<String> {
    // where each simple command runs: a `cd` moves the ones after it
    let mut here = cwd.to_path_buf();
    for words in simple_commands(command) {
        let words = unwrapped(&words);
        let Some(first) = words.first() else { continue };
        let program = first.rsplit('/').next().unwrap_or(first);
        let args = &words[1..];
        if program == "cd" {
            here = expand_path(args.first().map_or("~", String::as_str), &here);
            continue;
        }
        for t in redirect_targets(&words, &here) {
            if loose(&t) {
                return Some(loose_write(&t));
            }
        }
        let cwd = here.as_path();
        if matches!(program, "sh" | "bash" | "zsh" | "dash" | "fish") {
            if let Some(i) = args
                .iter()
                .position(|a| a == "-c" || a == "-lc" || a == "-ec")
            {
                if let Some(inner) = args.get(i + 1) {
                    if let Some(why) = refused(inner, cwd, branch_of, shared, behind, loose) {
                        return Some(why);
                    }
                }
            }
            continue;
        }
        if !reads_no_state(program) && !command.contains("SHARED_CLONE_OK=1") {
            let home = std::env::var_os("HOME").map(PathBuf::from);
            let named = args.iter().filter_map(|a| match a.strip_prefix("~/") {
                Some(rest) => home.as_ref().map(|h| h.join(rest)),
                None => a.starts_with('/').then(|| PathBuf::from(a)),
            });
            if let Some((top, n)) = std::iter::once(here.clone())
                .chain(named)
                .find_map(|p| behind(&p))
            {
                return Some(stale_read(&format!("`{program}` there"), &top, n));
            }
        }
        if is_python(program) && !installs_a_tool(first, program, args) {
            return Some(format!(
                "`{program}` is refused: no Python of our own, and our work never depends on it - shell to \
                 orchestrate, Rust for logic (rust-script for a single file). For JSON reach for jq, for text awk or \
                 sed, for a table tad or duckdb. A third-party tool a step requires (dtschema for the kernel's \
                 dt_binding_check) is installed in its own environment - `pipx install PKG`, or `python3 -m venv \
                 ~/.local/share/venvs/NAME` and its `bin/pip install PKG` - and run by name (practice 21)."
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
            if let Some(why) = worktree_outside_home(args, cwd) {
                return Some(why);
            }
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

/// Installing a third-party tool a step requires, in an environment of its own under ~/.local: `pipx`
/// managing packages by name, `python -m venv` making one under ~/.local, or such a venv's own `bin/pip`
/// installing packages by name (decision 2026-11-19-python-tools-run-not-written). A path, a requirements file
/// or an editable install is our own code, and stays refused; so does `pipx run`, which runs a file.
fn installs_a_tool(first: &str, program: &str, args: &[String]) -> bool {
    let bare = program.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    let by_name = |rest: &[String]| {
        rest.iter().all(|a| {
            !matches!(
                a.as_str(),
                "-r" | "--requirement" | "-e" | "--editable" | "-c"
            ) && (a.starts_with('-')
                || !(a.contains('/')
                    || a.starts_with('.')
                    || a.starts_with('~')
                    || a.ends_with(".py")))
        })
    };
    let under_local = |p: &str| {
        p.starts_with("~/.local/")
            || p.starts_with("$HOME/.local/")
            || std::env::var("HOME").is_ok_and(|h| p.starts_with(&format!("{h}/.local/")))
    };
    let Some(verb) = args.first() else {
        return false;
    };
    match bare {
        "pipx" => {
            matches!(
                verb.as_str(),
                "install"
                    | "upgrade"
                    | "upgrade-all"
                    | "uninstall"
                    | "reinstall"
                    | "inject"
                    | "list"
                    | "ensurepath"
            ) && by_name(&args[1..])
        }
        "python" => {
            args.len() >= 3
                && verb == "-m"
                && args[1] == "venv"
                && args[2..].iter().filter(|a| !a.starts_with('-')).count() == 1
                && args[2..]
                    .iter()
                    .filter(|a| !a.starts_with('-'))
                    .all(|a| under_local(a))
        }
        "pip" => {
            under_local(first)
                && first.contains("/bin/pip")
                && matches!(verb.as_str(), "install" | "uninstall" | "list" | "show")
                && by_name(&args[1..])
        }
        _ => false,
    }
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
            &|_| None,
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
            &|_| None,
            &|_| false,
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
            "git worktree add ~/.local/share/agent-worktrees/docs/x -b x origin/main",
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
            "pipx run x.py",
            "pipx install ./ours",
            "pip install dtschema",
            "python3 -m venv /tmp/v",
            "python3 -m venv ~/.local/share/venvs/x && python3 ours.py",
            "~/.local/share/venvs/x/bin/pip install -e .",
            "~/.local/share/venvs/x/bin/pip install -r req.txt",
            "~/.local/share/venvs/x/bin/python ours.py",
        ] {
            assert!(says(c, "feat").is_some(), "refused: {c}");
        }
        for c in [
            "grep python file",
            "echo 'python3 x'",
            "rg -n pip src",
            "cargo build --features python",
            "ls ~/.python",
            "pipx install dtschema",
            "pipx inject dtschema yamllint",
            "pipx list",
            "python3 -m venv ~/.local/share/venvs/dtschema",
            "~/.local/share/venvs/dtschema/bin/pip install dtschema yamllint",
            "make dt_binding_check DT_SCHEMA_FILES=x.yaml",
            "dt-doc-validate x.yaml",
        ] {
            assert!(says(c, "feat").is_none(), "allowed: {c}");
        }
    }

    /// As says, with /home/u/Projects/docs a shared clone 3 behind its origin/main and every other place current.
    fn says_stale(command: &str, cwd: &str) -> Option<String> {
        refused(
            command,
            Path::new(cwd),
            &|_| Some("main".into()),
            &|d: &Path| d.starts_with("/home/u/Projects/docs"),
            &|d: &Path| {
                d.starts_with("/home/u/Projects/docs")
                    .then(|| (PathBuf::from("/home/u/Projects/docs"), 3))
            },
            &|_| false,
        )
    }

    #[test]
    fn reading_in_a_stale_shared_clone_is_refused_and_main_is_not() {
        for c in [
            "tad q ready --for keeper",
            "cat briefs/builder.md",
            "cd /home/u/Projects/docs && tad q next",
            "grep -r x /home/u/Projects/docs/data",
            "sh -c 'cd /home/u/Projects/docs; ikb backlog --id x'",
        ] {
            let cwd = if c.starts_with("cat") || c.starts_with("tad") {
                "/home/u/Projects/docs"
            } else {
                "/tmp"
            };
            assert!(
                says_stale(c, cwd).is_some_and(
                    |w| w.contains("3 commit(s) behind") && w.contains("branch --read")
                ),
                "refused: {c}"
            );
        }
        for c in [
            "git pull --ff-only",
            "git fetch -q",
            "git show origin/main:briefs/builder.md",
            "git worktree add --detach $AGENT_WORKTREES/docs/r origin/main",
            "branch fix-x",
            "ls",
            "SHARED_CLONE_OK=1 tad q next",
        ] {
            assert!(
                says_stale(c, "/home/u/Projects/docs").is_none(),
                "allowed: {c}"
            );
        }
        assert!(
            says_stale("tad q next", "/home/u/wt/docs-r").is_none(),
            "a worktree of its own"
        );
        assert!(
            says_stale("tad q next", "/home/u/Projects/lab").is_none(),
            "a current clone"
        );
    }

    /// As says, with /home/u/Projects/loose and /home/u/Projects/ORG a place outside every clone.
    fn says_loose(command: &str, cwd: &str) -> Option<String> {
        refused(
            command,
            Path::new(cwd),
            &|_| Some("feat".into()),
            &|_| false,
            &|_| None,
            &|p: &Path| p.parent() == Some(Path::new("/home/u/Projects/ORG")),
        )
    }

    #[test]
    fn a_worktree_lives_in_the_worktree_home() {
        let home = worktree_home();
        let h = home.display();
        for c in [
            "git worktree add -q --detach /home/u/Projects/linux-phone-oco/wt-keeper-1 origin/main",
            "git -C /home/u/Projects/docs worktree add ../docs-read-$(date +%s) origin/main",
            "git worktree add /tmp/x",
            "git worktree add -b x /home/u/Projects/wt-$R origin/main",
        ] {
            assert!(
                says(c, "feat").is_some_and(|w| w.contains("branch --read")),
                "refused: {c}"
            );
        }
        for c in [
            format!("git worktree add -q -b builder/x {h}/docs/builder/x origin/main"),
            format!("git -C /x worktree add --detach {h}/docs/rhizoid/int-$R v7.3"),
            "git worktree add $AGENT_WORKTREES/docs/x origin/main".to_string(),
            "git worktree list".to_string(),
            "git worktree remove /home/u/Projects/wt-old".to_string(),
        ] {
            assert!(says(&c, "feat").is_none(), "allowed: {c}");
        }
    }

    #[test]
    fn a_loose_file_under_projects_is_refused_where_the_parse_sees_it() {
        for c in [
            "echo x > /home/u/Projects/ORG/notes.md",
            "cat a >>/home/u/Projects/ORG/log",
            "cd /home/u/Projects/ORG && ls 2> err.log",
        ] {
            assert!(
                says_loose(c, "/tmp").is_some_and(|w| w.contains("only clones live")),
                "refused: {c}"
            );
        }
        for c in [
            "echo x > /home/u/Projects/ORG/clone/notes.md",
            "make 2>&1 | tee build.log",
            "cmd > /dev/null 2>&1",
            "git clone https://example.org/x /home/u/Projects/ORG/x",
        ] {
            assert!(says_loose(c, "/tmp").is_none(), "allowed: {c}");
        }
    }

    #[test]
    fn cd_home_goes_home() {
        let home = std::env::var("HOME").unwrap();
        for w in ["~", "$HOME", "${HOME}"] {
            assert_eq!(expand_path(w, Path::new("/x")), PathBuf::from(&home));
        }
        assert_eq!(
            expand_path("~/a", Path::new("/x")),
            PathBuf::from(&home).join("a")
        );
        assert_eq!(expand_path("a", Path::new("/x")), PathBuf::from("/x/a"));
        // a stale clone left by `cd ~` is left: the next command does not run there
        let r = refused(
            "cd ~ && cat f",
            Path::new("/home/u/Projects/docs"),
            &|_| Some("main".into()),
            &|_| true,
            &|d: &Path| {
                d.starts_with("/home/u/Projects/docs")
                    .then(|| (PathBuf::from("/d"), 1))
            },
            &|_| false,
        );
        assert!(r.is_none(), "{r:?}");
    }

    #[test]
    fn bash_and_reads_are_read_and_nothing_else() {
        assert!(
            hook(r#"{"tool_name":"Write","tool_input":{"command":"pip install x"}}"#).is_none()
        );
        assert!(hook(r#"{"tool_name":"Bash","tool_input":{"command":"pip install x"}}"#).is_some());
        assert!(hook("not json").is_none());
    }
}
