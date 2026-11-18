//! Roles: a session's role is a card in the plugin's agents/ (Claude Code's own agent definition - purpose,
//! owns, never, loads, model and effort, hand-over, reports to - and a `Practices:` line). A brief names its
//! role on a line of its own (`Role: keeper`, or `Role: ideal-agent:keeper`), optionally with `Also
//! practices: 42` for a domain's own; a session started with `claude --agent ideal-agent:keeper` carries it
//! in the hook's event. The role's card and only its practices are what the session reads at its start,
//! instead of every practice (decision 2026-11-17-roles-as-agent-cards).

use crate::practices;
use std::path::{Path, PathBuf};

/// How many lines at a prompt's head may name the role, the name and the domain's practices.
const HEAD: usize = 8;

/// A role named in a prompt or an event: its name without the plugin's prefix, and the practices a domain
/// adds to it.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Named {
    pub role: String,
    pub also: Vec<String>,
    /// the name the work is signed with (`Name: surya-keeper-5`), read by scripts/agent-env as AGENT_NAME
    pub name: Option<String>,
    /// the domain's brief for this role (`Brief: ~/path/to/brief.md`), printed after the card so a start holds only
    /// the head and the path
    pub brief: Option<String>,
}

fn bare(name: &str) -> String {
    name.trim()
        .trim_start_matches("ideal-agent:")
        .trim_matches(|c: char| c == '*' || c == '`')
        .to_string()
}

fn numbers(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_ascii_digit())
        .filter(|n| !n.is_empty())
        .map(|n| format!("{:02}", n.parse::<u32>().unwrap_or(0)))
        .collect()
}

/// The role a brief names: the first line that begins `Role:` (markdown emphasis allowed), and an `Also
/// practices:` line anywhere.
/// Where the brief begins in a first prompt: past the reminders the app puts in front of it (`<system-reminder>`
/// blocks, such as the linked sessions), past a scheduled run's `<scheduled-task ...>` opening, and past the background a starting session gives (`Background (from the
/// session that started this one):` ... `Task:`). A prompt that then begins with another tag - a message from
/// another session - names no role.
pub fn brief_of(prompt: &str) -> Option<&str> {
    let mut s = prompt.trim_start();
    while let Some(rest) = s.strip_prefix("<system-reminder>") {
        s = rest.split_once("</system-reminder>")?.1.trim_start();
    }
    // a scheduled run: its prompt is wrapped in <scheduled-task ...>, with one line of the harness's own first
    if s.starts_with("<scheduled-task") {
        s = s.split_once('\n')?.1.trim_start();
    }
    if s.starts_with('<') {
        return None;
    }
    if s.starts_with("Background (from the session that started this one):") {
        s = s.split_once("\nTask:\n")?.1.trim_start();
    }
    Some(s)
}

/// Only the head of a prompt is read - a brief names its role in its first lines - and never a message another
/// session sent, which arrives wrapped in a tag and may quote a role as an example.
pub fn named_in(prompt: &str) -> Option<Named> {
    let prompt = brief_of(prompt)?;
    let head: String = prompt.lines().take(HEAD).collect::<Vec<_>>().join("\n");
    let prompt = head.as_str();
    let clean = |l: &str| {
        l.trim()
            .trim_start_matches(['*', '#', '>', '-', ' '])
            .to_string()
    };
    let mut role = None;
    let mut also = Vec::new();
    for line in prompt.lines().map(clean) {
        if role.is_none() {
            if let Some(r) = line
                .strip_prefix("Role:")
                .or_else(|| line.strip_prefix("Role**:"))
            {
                let r = bare(
                    r.trim_start_matches(['*', ' '])
                        .split_whitespace()
                        .next()
                        .unwrap_or(""),
                );
                if !r.is_empty() && r.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
                    role = Some(r);
                }
            }
        }
        if let Some(a) = line.strip_prefix("Also practices:") {
            also.extend(numbers(a));
        }
    }
    role.map(|role| Named {
        role,
        also,
        name: name_in(prompt),
        brief: brief_in(prompt),
    })
}

/// The name a prompt gives the session to sign its work with: a line `Name: NAME` (lowercase, digits, dashes).
pub fn name_in(prompt: &str) -> Option<String> {
    prompt.lines().take(HEAD).find_map(|l| {
        let n = l
            .trim()
            .trim_start_matches(['*', '-', ' '])
            .strip_prefix("Name:")?;
        let n = n
            .trim()
            .trim_matches(|c: char| c == '*' || c == '`' || c == ' ');
        (!n.is_empty()
            && n.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'))
        .then(|| n.to_string())
    })
}

/// The brief a prompt's head names: a line `Brief: PATH` (a path, `~` for home).
pub fn brief_in(prompt: &str) -> Option<String> {
    prompt.lines().take(HEAD).find_map(|l| {
        let p = l
            .trim()
            .trim_start_matches(['*', '-', ' '])
            .strip_prefix("Brief:")?;
        let p = p
            .trim()
            .trim_matches(|c: char| c == '`' || c == '*' || c == ' ');
        (!p.is_empty() && !p.contains(char::is_whitespace)).then(|| p.to_string())
    })
}

/// A brief as its repository's main has it - fetched, then read from origin/main - so that a shared clone left on
/// a branch or an old commit never gives a stale brief; else the file on disk, said so (no clone, no network,
/// or a brief not on main yet).
fn read_brief(path: &Path) -> Option<(String, &'static str)> {
    use std::process::Command;
    let dir = path.parent()?;
    let git = |args: &[&str]| {
        Command::new("timeout")
            .arg("8")
            .arg("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
    };
    if let Some(top) = git(&["rev-parse", "--show-toplevel"]) {
        let top = PathBuf::from(top.trim());
        if let Ok(rel) = path
            .canonicalize()
            .unwrap_or_else(|_| path.to_path_buf())
            .strip_prefix(&top)
        {
            let _ = git(&["fetch", "-q", "origin", "main"]);
            if let Some(t) = git(&["show", &format!("origin/main:{}", rel.display())]) {
                return Some((t, "from its repository's main"));
            }
        }
    }
    std::fs::read_to_string(path)
        .ok()
        .map(|t| (t, "from the file on disk - not on its repository's main"))
}

/// The retired routes a text still names - each with what does its work now - as a section to print after it,
/// so a start sees the replacement where it would have followed the old route (decision
/// 2026-11-18-instructions-name-intents). A line that says the route is retired or forbids it is not counted.
pub fn retired_named(root: &Path, text: &str) -> String {
    let Ok(routes) = std::fs::read_to_string(root.join("data/routes.json")) else {
        return String::new();
    };
    let quiet = ["retired", "never", "nothing of", "no longer"];
    let forbids = |low: &str| {
        quiet.iter().any(|q| low.contains(q))
            || low
                .split(|c: char| !c.is_alphanumeric())
                .any(|w| w == "no" || w == "not")
    };
    let mut out = String::new();
    for r in routes
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
    {
        let (Some(id), Some(instead)) = (r["id"].as_str(), r["instead"].as_str()) else {
            continue;
        };
        let named = text.lines().any(|l| {
            let low = l.to_lowercase();
            l.split(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_'))
                .any(|w| w == id)
                && !forbids(&low)
        });
        if named {
            out.push_str(&format!("- `{id}` is retired - instead: {instead}\n"));
        }
    }
    if out.is_empty() {
        out
    } else {
        format!("\n## Retired routes your brief still names (follow the instead; tell its owner)\n\n{out}")
    }
}

fn expand(path: &str) -> PathBuf {
    match path.strip_prefix("~/") {
        Some(rest) => std::env::var_os("HOME")
            .map_or_else(|| PathBuf::from(path), |h| PathBuf::from(h).join(rest)),
        None => PathBuf::from(path),
    }
}

/// A role card: its body (the frontmatter left to Claude Code) and the practice numbers it carries.
pub struct Card {
    pub body: String,
    pub practices: Vec<String>,
}

pub fn card_path(root: &Path, role: &str) -> PathBuf {
    root.join("agents").join(format!("{role}.md"))
}

pub fn parse_card(text: &str) -> Card {
    let body = match text
        .strip_prefix("---\n")
        .and_then(|t| t.find("\n---\n").map(|i| &t[i + 5..]))
    {
        Some(b) => b,
        None => text,
    };
    let mut kept = Vec::new();
    let mut practices = Vec::new();
    for line in body.lines() {
        match line.strip_prefix("Practices:") {
            Some(p) => practices.extend(numbers(p)),
            None => kept.push(line),
        }
    }
    Card {
        body: kept.join("\n").trim().to_string(),
        practices,
    }
}

/// What a session reads when its role is known: the card, then only its practices (the card's and the
/// domain's), each its first sentence; or, without a card of that name, every practice and a word why.
pub fn render(root: Option<&Path>, named: &Named, text: &str, source: &str) -> String {
    let card = root
        .map(|r| card_path(r, &named.role))
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|t| parse_card(&t));
    let Some(card) = card else {
        return format!(
            "No role card '{}' in ideal-agent's agents/ - every practice instead.\n\n## The practices (ideal-agent)\n\n{}",
            named.role,
            practices::render_short(text, source, None)
        );
    };
    let mut keep = card.practices.clone();
    keep.extend(named.also.iter().cloned());
    let brief = named.brief.as_deref().map_or_else(String::new, |p| match read_brief(&expand(p)) {
        Some((t, from)) => format!(
            "\n## Your brief ({p}, {from})\n\n{}\n{}",
            t.trim(),
            root.map(|r| retired_named(r, &t)).unwrap_or_default()
        ),
        None => format!("\n## Your brief ({p})\n\nIt could not be read: tell whoever started you, and work from the card alone.\n"),
    });
    format!(
        "## Your role: {} (ideal-agent)\n\n{}\n\n## The practices of this role (ideal-agent)\n\n{}{brief}",
        named.role,
        card.body,
        practices::render_short(text, source, Some(&keep))
    )
}

/// Where the role a session read is kept, so that a resume or a compaction reads the same again.
pub fn state_dir() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/state")))?;
    Some(base.join("ideal-agent/sessions"))
}

fn safe(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// What a session was given: `Some(Some(named))` a role, `Some(None)` every practice, `None` nothing yet.
/// The state is one line per key - `role R` (or `role -`), `also N...`, `name N` - so that scripts/agent-env
/// reads the name with sed.
pub fn recall(session: &str) -> Option<Option<Named>> {
    if !safe(session) {
        return None;
    }
    let t = std::fs::read_to_string(state_dir()?.join(session)).ok()?;
    // only a state written under the first-prompt rule is trusted: one from before it may hold a role read off a
    // later message (0.3.0 and 0.3.1 read any prompt)
    if !t.lines().any(|l| l == KEPT) {
        return None;
    }
    let value = |key: &str| {
        t.lines()
            .find_map(|l| l.strip_prefix(key).map(|v| v.trim().to_string()))
    };
    let role = value("role ")?;
    if role == "?" {
        return None;
    }
    let role = Some(role).filter(|r| r != "-" && !r.is_empty());
    Some(role.map(|role| {
        Named {
            role,
            also: value("also ")
                .map(|a| a.split_whitespace().map(String::from).collect())
                .unwrap_or_default(),
            name: value("name ").filter(|n| !n.is_empty()),
            brief: value("brief ").filter(|b| !b.is_empty()),
        }
    }))
}

/// A session that started now waits for its first prompt to name its role: only that prompt is read for one, so
/// that neither a later message nor a session started before roles is given a role in mid-run.
/// The line that marks a state as written under the first-prompt rule (0.3.2 on).
pub const KEPT: &str = "kept first-prompt";

pub fn await_first(session: &str) {
    if let Some(dir) = state_dir().filter(|_| safe(session)) {
        let _ = std::fs::create_dir_all(&dir)
            .and_then(|_| std::fs::write(dir.join(session), format!("role ?\n{KEPT}\n")));
    }
}

pub fn awaiting(session: &str) -> bool {
    state_dir()
        .filter(|_| safe(session))
        .and_then(|d| std::fs::read_to_string(d.join(session)).ok())
        .is_some_and(|t| t.lines().any(|l| l == "role ?"))
}

pub fn remember(session: &str, named: Option<&Named>) {
    let Some(dir) = state_dir().filter(|_| safe(session)) else {
        return;
    };
    let text = match named {
        Some(n) => {
            let mut t = format!("role {}\nalso {}\n{KEPT}\n", n.role, n.also.join(" "));
            if let Some(name) = &n.name {
                t.push_str(&format!("name {name}\n"));
            }
            if let Some(brief) = &n.brief {
                t.push_str(&format!("brief {brief}\n"));
            }
            t
        }
        None => format!("role -\n{KEPT}\n"),
    };
    let _ = std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(dir.join(session), text));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_brief_names_its_role_on_a_line_of_its_own() {
        let n = named_in(
            "# Keeper of surya\n\nRole: ideal-agent:keeper\nAlso practices: 42, 10\nThe rest.",
        )
        .expect("named");
        assert_eq!(n.role, "keeper");
        assert_eq!(n.also, vec!["42", "10"]);
        assert_eq!(
            named_in("**Role:** `coordinator`")
                .map(|n| n.role)
                .as_deref(),
            Some("coordinator")
        );
        assert_eq!(
            named_in("- Role: patrol").map(|n| n.role).as_deref(),
            Some("patrol")
        );
        assert!(named_in("The role: of this session is to keep").is_none());
        assert!(
            named_in("Role: Keeper of all things").is_none(),
            "a name, not prose"
        );
    }

    #[test]
    fn a_prompt_names_who_signs() {
        let p = "Role: keeper (ideal-agent)\nName: surya-keeper-6\n";
        assert_eq!(
            named_in(p).and_then(|n| n.name).as_deref(),
            Some("surya-keeper-6")
        );
        assert_eq!(
            name_in("**Name:** `walleye-lane`").as_deref(),
            Some("walleye-lane")
        );
        assert!(name_in("Name: Some Prose here").is_none());
        assert!(name_in("no name").is_none());
    }

    #[test]
    fn a_role_is_read_only_at_the_head_and_never_from_a_message() {
        let msg = "<cross-session-message from=\"x\">\nRole: keeper (ideal-agent)\n</cross-session-message>";
        assert!(named_in(msg).is_none(), "a message may quote a role");
        let deep = format!("{}Role: keeper", "line\n".repeat(HEAD));
        assert!(named_in(&deep).is_none(), "only the head names it");
        assert!(name_in(&format!("{}Name: x", "line\n".repeat(HEAD))).is_none());
    }

    #[test]
    fn a_started_session_names_its_role_past_the_apps_reminders_and_the_background() {
        let p = "<system-reminder>\nLinked sessions now: x = local_1 (idle).\n</system-reminder>\n\nBackground (from the session that started this one):\nYou are surya's keeper.\n\nRole: patrol in the background is not the brief.\n\nTask:\nRole: keeper (ideal-agent)\nName: surya-keeper-8\nAlso practices: 42\n\nThe work.";
        let n = named_in(p).expect("named past the reminder and the background");
        assert_eq!(
            (n.role.as_str(), n.name.as_deref(), n.also.clone()),
            ("keeper", Some("surya-keeper-8"), vec!["42".to_string()])
        );
        let msg = "<system-reminder>\nx\n</system-reminder>\n<cross-session-message from=\"y\">\nRole: keeper\n</cross-session-message>";
        assert!(
            named_in(msg).is_none(),
            "a message behind a reminder names none"
        );
        assert!(named_in("<system-reminder>\nunclosed\nRole: keeper").is_none());
    }

    #[test]
    fn a_scheduled_run_names_its_role_past_the_task_wrapper() {
        let p = "<scheduled-task name=\"surya-keeper\" file=\"/x/SKILL.md\">\nThis is an automated run of a scheduled task. The user is not present.\n\nRole: keeper (ideal-agent)\nName: surya-keeper\nAlso practices: 42\n\nThe work.\n</scheduled-task>";
        let n = named_in(p).expect("named past the wrapper");
        assert_eq!(
            (n.role.as_str(), n.name.as_deref()),
            ("keeper", Some("surya-keeper"))
        );
    }

    #[test]
    fn a_state_from_before_the_first_prompt_rule_is_not_trusted() {
        let dir = std::env::temp_dir().join(format!("ia-state-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("ideal-agent/sessions")).unwrap();
        std::fs::write(
            dir.join("ideal-agent/sessions/old"),
            "role keeper\nalso 42\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("ideal-agent/sessions/new"),
            format!("role keeper\nalso 42\n{KEPT}\n"),
        )
        .unwrap();
        let read = |s: &str| {
            let t = std::fs::read_to_string(dir.join("ideal-agent/sessions").join(s)).unwrap();
            t.lines().any(|l| l == KEPT)
        };
        assert!(!read("old"), "0.3.1's state carries no mark");
        assert!(read("new"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_brief_named_in_the_head_is_read_after_the_card() {
        let dir = std::env::temp_dir().join(format!("ia-brief-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("patrol.md");
        std::fs::write(&f, "# The patrol of linux-phone\nMeasure the lanes.\n").unwrap();
        let p = format!(
            "Role: patrol (ideal-agent)\nName: linux-phone-patrol\nBrief: {}\n",
            f.display()
        );
        let n = named_in(&p).expect("named");
        assert_eq!(n.brief.as_deref(), Some(f.to_str().unwrap()));
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let r = render(Some(root), &n, practices::BUILT_IN, "t");
        assert!(r.contains("## Your brief") && r.contains("Measure the lanes."));
        let missing = Named {
            brief: Some("/no/such/brief.md".into()),
            ..n
        };
        assert!(
            render(Some(root), &missing, practices::BUILT_IN, "t").contains("could not be read")
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn the_roles_that_meet_incidents_ask_the_class_question() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let skill =
            std::fs::read_to_string(root.join("skills/look-around-first/SKILL.md")).unwrap();
        assert!(
            skill.contains("## After a fix - the class question"),
            "the method has its section"
        );
        for c in ["keeper", "lane-coordinator", "orchestrator", "patrol"] {
            let card = std::fs::read_to_string(card_path(root, c)).unwrap();
            assert!(
                card.contains("\"After a fix\""),
                "{c} names the class question"
            );
        }
    }

    #[test]
    fn a_brief_naming_a_retired_route_is_warned_at_the_start() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let w = retired_named(
            root,
            "Run the tty reader through oco-work.\nNothing of ocoeco here.\n",
        );
        assert!(w.contains("`oco-work` is retired - instead:"), "{w}");
        assert!(
            !w.contains("`ocoeco`"),
            "a line that forbids it is not counted"
        );
        assert!(retired_named(root, "Use land.\n").is_empty());
    }

    #[test]
    fn no_card_or_skill_names_a_retired_route() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let out = std::process::Command::new(root.join("scripts/route-check"))
            .arg(root.join("agents"))
            .arg(root.join("skills"))
            .output()
            .expect("route-check runs");
        assert!(
            out.status.success(),
            "a card or skill names a retired route:\n{}",
            String::from_utf8_lossy(&out.stdout)
        );
    }

    #[test]
    fn a_run_signs_as_its_role_and_run_with_the_owners_mailbox() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let home = std::env::temp_dir().join(format!("ia-env-{}", std::process::id()));
        let sessions = home.join(".local/state/ideal-agent/sessions");
        std::fs::create_dir_all(&sessions).unwrap();
        std::fs::write(
            home.join(".gitconfig"),
            "[user]\n\temail = owner@example.org\n",
        )
        .unwrap();
        let id = "3f9a2c11-0000-4000-8000-000000000000";
        std::fs::write(
            sessions.join(id),
            format!("role keeper\nname surya-keeper\n{KEPT}\n"),
        )
        .unwrap();
        let out = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!(
                ". {}/scripts/agent-env; echo \"$AGENT_NAME $GIT_AUTHOR_EMAIL\"",
                root.display()
            ))
            .env_clear()
            .env("HOME", &home)
            .env("PATH", "/usr/bin:/bin")
            .env("CLAUDE_CODE_SESSION_ID", id)
            .output()
            .unwrap();
        let said = String::from_utf8_lossy(&out.stdout);
        assert_eq!(
            said.trim(),
            "surya-keeper.3f9a2c owner+surya-keeper.3f9a2c@example.org"
        );
        let _ = std::fs::remove_dir_all(home);
    }

    #[test]
    fn who_lands_reads_landed_by_then_co_authors() {
        use std::process::Command;
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let repo = std::env::temp_dir().join(format!("ia-who-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).unwrap();
        let git = |args: &[&str]| {
            let ok = Command::new("git")
                .arg("-C")
                .arg(&repo)
                .args(args)
                .env("GIT_AUTHOR_NAME", "owner")
                .env("GIT_AUTHOR_EMAIL", "o@x")
                .env("GIT_COMMITTER_NAME", "owner")
                .env("GIT_COMMITTER_EMAIL", "o@x")
                .status()
                .unwrap()
                .success();
            assert!(ok, "git {args:?}");
        };
        git(&["init", "-q"]);
        std::fs::write(
            repo.join("AGENTS.md"),
            "# x\n\n**Owner**: `ideal-agent-builder` - the role.\n",
        )
        .unwrap();
        git(&["add", "."]);
        git(&[
            "commit",
            "-q",
            "-m",
            "one\n\nCo-authored-by: ideal-agent-builder.3f9a2c <o+a@x>",
        ]);
        git(&[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "two\n\nCo-authored-by: someone.ab12cd <o+b@x>\nLanded-by: another-role.c0ffee",
        ]);
        git(&["update-ref", "refs/remotes/origin/main", "HEAD"]);
        let out = Command::new(root.join("scripts/who-lands"))
            .arg(&repo)
            .arg("1 day ago")
            .output()
            .unwrap();
        let said = String::from_utf8_lossy(&out.stdout);
        assert!(said.contains("1\tideal-agent-builder\towner"), "{said}");
        assert!(
            said.contains("1\tanother-role\tother"),
            "Landed-by wins over the co-author: {said}"
        );
        assert!(!said.contains("someone"), "{said}");
        assert_eq!(out.status.code(), Some(1), "another role landed");
        let _ = std::fs::remove_dir_all(repo);
    }

    #[test]
    fn a_card_gives_its_body_and_its_practices() {
        let c = parse_card(
            "---\nname: keeper\nmodel: sonnet\n---\n\nYou keep it.\n\nPractices: 01 4 44\n",
        );
        assert_eq!(c.body, "You keep it.");
        assert_eq!(c.practices, vec!["01", "04", "44"]);
    }

    #[test]
    fn every_card_in_the_plugin_names_only_practices_that_exist() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let ids: Vec<String> = practices::BUILT_IN
            .lines()
            .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
            .filter_map(|v| v["id"].as_str().map(|s| s[..2].to_string()))
            .collect();
        let mut cards = 0;
        for e in std::fs::read_dir(root.join("agents")).unwrap() {
            let p = e.unwrap().path();
            let c = parse_card(&std::fs::read_to_string(&p).unwrap());
            for n in &c.practices {
                assert!(
                    ids.contains(n),
                    "{} names practice {n}, which is not",
                    p.display()
                );
            }
            cards += usize::from(!c.practices.is_empty());
        }
        assert!(cards >= 7, "the roles carry their practices");
    }

    #[test]
    fn a_role_reads_its_card_and_only_its_practices() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let n = Named {
            role: "keeper".into(),
            also: vec!["42".into()],
            name: None,
            brief: None,
        };
        let r = render(Some(root), &n, practices::BUILT_IN, "test");
        assert!(r.starts_with("## Your role: keeper"));
        assert!(r.contains("44. "), "the card's own");
        assert!(r.contains("42. "), "the domain's");
        assert!(!r.contains("\n41. "), "not another role's");
        assert!(
            !r.contains("model: sonnet"),
            "the frontmatter is Claude Code's"
        );
        let none = Named {
            role: "nobody".into(),
            also: vec![],
            name: None,
            brief: None,
        };
        assert!(
            render(Some(root), &none, practices::BUILT_IN, "t").contains("every practice instead")
        );
    }
}
