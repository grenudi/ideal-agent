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
/// Only the head of a prompt is read - a brief names its role in its first lines - and never a message another
/// session sent, which arrives wrapped in a tag and may quote a role as an example.
pub fn named_in(prompt: &str) -> Option<Named> {
    if prompt.trim_start().starts_with('<') {
        return None;
    }
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
    format!(
        "## Your role: {} (ideal-agent)\n\n{}\n\n## The practices of this role (ideal-agent)\n\n{}",
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
        }
    }))
}

/// A session that started now waits for its first prompt to name its role: only that prompt is read for one, so
/// that neither a later message nor a session started before roles is given a role in mid-run.
pub fn await_first(session: &str) {
    if let Some(dir) = state_dir().filter(|_| safe(session)) {
        let _ = std::fs::create_dir_all(&dir)
            .and_then(|_| std::fs::write(dir.join(session), "role ?\n"));
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
            let mut t = format!("role {}\nalso {}\n", n.role, n.also.join(" "));
            if let Some(name) = &n.name {
                t.push_str(&format!("name {name}\n"));
            }
            t
        }
        None => "role -\n".to_string(),
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
        };
        assert!(
            render(Some(root), &none, practices::BUILT_IN, "t").contains("every practice instead")
        );
    }
}
