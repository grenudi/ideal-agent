//! What agents leave behind, pruned once a day by the session start (`ideal-agent prune now`). Agent worktrees: a linked worktree of a clone under ~/Projects that nothing wrote to for three days
//! is removed - its branch stays, so no commit is lost - and one with uncommitted changes is kept and named, for
//! its owner (backlog stale-worktrees-pruned). A session that cds into an old worktree reads very old state, and the
//! guard's stale-clone check sees only a clone's main checkout.
use std::path::{Path, PathBuf};
use std::process::Command;

/// What became of one worktree.
#[derive(Debug, PartialEq, Eq)]
pub enum Fate {
    Removed,
    /// would be removed (show)
    Old,
    /// uncommitted changes: kept, for its owner
    Dirty,
    /// `git worktree remove` refused it: kept, with git's word
    Refused(String),
}

/// A linked worktree, as `git worktree list --porcelain` gives it: its path, and whether it is locked or prunable.
#[derive(Debug, PartialEq, Eq)]
pub struct Linked {
    pub path: PathBuf,
    pub locked: bool,
    pub missing: bool,
}

/// The linked worktrees in `git worktree list --porcelain` - the first block is the main checkout.
pub fn linked(porcelain: &str) -> Vec<Linked> {
    porcelain
        .split("\n\n")
        .skip(1)
        .filter_map(|block| {
            let path = block.lines().find_map(|l| l.strip_prefix("worktree "))?;
            Some(Linked {
                path: PathBuf::from(path),
                locked: block
                    .lines()
                    .any(|l| l == "locked" || l.starts_with("locked ")),
                missing: block
                    .lines()
                    .any(|l| l == "prunable" || l.starts_with("prunable ")),
            })
        })
        .collect()
}

fn git(dir: &Path, args: &[&str]) -> Option<String> {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
}

/// Whether anything under `dir` was written in the last `days` days - its first such file found ends the walk.
fn written_within(dir: &Path, days: u32) -> bool {
    Command::new("find")
        .arg(dir)
        .args(["-mtime", &format!("-{days}"), "-print", "-quit"])
        .output()
        .map_or(true, |o| !o.stdout.is_empty() || !o.status.success())
}

/// The clones under ~/Projects: each `.git` directory four levels down at most.
fn clones(projects: &Path) -> Vec<PathBuf> {
    let out = Command::new("find")
        .arg(projects)
        .args(["-maxdepth", "4", "-name", ".git", "-type", "d", "-prune"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    out.lines()
        .filter_map(|l| Path::new(l).parent().map(Path::to_path_buf))
        .collect()
}

/// Each worktree of the clones under `projects` that nothing wrote to for `days` days, and what became of it;
/// `now` removes, else only shows. A locked one is never touched; one whose folder is gone is pruned by git.
pub fn sweep(projects: &Path, days: u32, now: bool) -> Vec<(PathBuf, Fate)> {
    let mut out = Vec::new();
    for clone in clones(projects) {
        let Some(list) = git(&clone, &["worktree", "list", "--porcelain"]) else {
            continue;
        };
        for w in linked(&list) {
            if w.locked || w.missing || written_within(&w.path, days) {
                continue;
            }
            let dirty = git(
                &w.path,
                &["status", "--porcelain", "--untracked-files=normal"],
            )
            .is_none_or(|s| !s.trim().is_empty());
            let fate = if dirty {
                Fate::Dirty
            } else if !now {
                Fate::Old
            } else {
                let o = Command::new("git")
                    .arg("-C")
                    .arg(&clone)
                    .args(["worktree", "remove"])
                    .arg(&w.path)
                    .output();
                match o {
                    Ok(o) if o.status.success() => Fate::Removed,
                    Ok(o) => Fate::Refused(String::from_utf8_lossy(&o.stderr).trim().to_string()),
                    Err(e) => Fate::Refused(e.to_string()),
                }
            };
            out.push((w.path, fate));
        }
        if now {
            let _ = git(&clone, &["worktree", "prune"]);
        }
    }
    out
}

/// Build run folders (AGENT_BUILD's parent, one folder per run) nothing wrote to for `days` days: removed with
/// `now`, else shown. A run's session ended; its large outputs would fill the disk in place of /tmp.
pub fn sweep_runs(builds: &Path, days: u32, now: bool) -> Vec<(PathBuf, Fate)> {
    let Ok(entries) = std::fs::read_dir(builds) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for e in entries.flatten() {
        let path = e.path();
        if !path.is_dir() || written_within(&path, days) {
            continue;
        }
        let fate = if !now {
            Fate::Old
        } else {
            match std::fs::remove_dir_all(&path) {
                Ok(()) => Fate::Removed,
                Err(e) => Fate::Refused(e.to_string()),
            }
        };
        out.push((path, fate));
    }
    out
}

/// One line per worktree or run: what became of it.
pub fn report(fates: &[(PathBuf, Fate)]) -> String {
    fates
        .iter()
        .map(|(p, f)| {
            let p = p.display();
            match f {
                Fate::Removed => format!("removed {p}\n"),
                Fate::Old => format!("old {p}: nothing written for the days given - `now` removes it\n"),
                Fate::Dirty => format!("kept {p}: uncommitted changes, nothing written for the days given - its owner commits or drops them\n"),
                Fate::Refused(why) => format!("kept {p}: git refused - {why}\n"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_linked_worktrees_are_read_off_the_porcelain() {
        let p = "worktree /p/docs\nHEAD a\nbranch refs/heads/main\n\nworktree /p/wt-a\nHEAD b\ndetached\n\nworktree /p/wt-b\nHEAD c\nbranch refs/heads/x\nlocked busy\n\nworktree /p/wt-c\nHEAD d\ndetached\nprunable gitdir file points to non-existent location\n";
        let l = linked(p);
        assert_eq!(l.len(), 3);
        assert_eq!(
            l[0],
            Linked {
                path: "/p/wt-a".into(),
                locked: false,
                missing: false
            }
        );
        assert!(l[1].locked && !l[1].missing);
        assert!(l[2].missing);
    }

    /// Live: a clone with an old clean worktree, an old dirty one and a fresh one - `show` names them, `now` removes
    /// only the clean old one and keeps its branch.
    #[test]
    fn an_old_clean_worktree_is_removed_and_a_dirty_one_kept() {
        let base = std::env::temp_dir().join(format!("ia-prune-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let projects = base.join("Projects");
        let clone = projects.join("lab");
        std::fs::create_dir_all(&clone).unwrap();
        let g = |dir: &Path, args: &[&str]| {
            let ok = Command::new("git")
                .arg("-C")
                .arg(dir)
                .args(["-c", "user.email=t@t", "-c", "user.name=t"])
                .args(args)
                .status()
                .unwrap()
                .success();
            assert!(ok, "git {args:?}");
        };
        g(&clone, &["init", "-q", "-b", "main"]);
        g(&clone, &["commit", "-q", "--allow-empty", "-m", "one"]);
        for (name, branch) in [("wt-old", "old"), ("wt-dirty", "dirty"), ("wt-new", "new")] {
            let path = projects.join(name);
            g(
                &clone,
                &[
                    "worktree",
                    "add",
                    "-q",
                    "-b",
                    branch,
                    path.to_str().unwrap(),
                ],
            );
        }
        std::fs::write(projects.join("wt-dirty/x"), "x").unwrap();
        // the two old ones: every file four days old
        for name in ["wt-old", "wt-dirty"] {
            let ok = Command::new("find")
                .arg(projects.join(name))
                .args(["-exec", "touch", "-h", "-d", "4 days ago", "{}", "+"])
                .status()
                .unwrap()
                .success();
            assert!(ok);
        }
        let shown = sweep(&projects, 3, false);
        assert_eq!(shown.len(), 2, "{shown:?}");
        assert!(shown.contains(&(projects.join("wt-old"), Fate::Old)));
        assert!(shown.contains(&(projects.join("wt-dirty"), Fate::Dirty)));
        assert!(projects.join("wt-old").is_dir(), "show removes nothing");
        let done = sweep(&projects, 3, true);
        assert!(
            done.contains(&(projects.join("wt-old"), Fate::Removed)),
            "{done:?}"
        );
        assert!(!projects.join("wt-old").exists());
        assert!(projects.join("wt-dirty/x").is_file() && projects.join("wt-new").is_dir());
        g(&clone, &["rev-parse", "-q", "--verify", "refs/heads/old"]);
        assert!(report(&done).contains("kept") && report(&done).contains("removed"));
        let _ = std::fs::remove_dir_all(&base);
    }
}
