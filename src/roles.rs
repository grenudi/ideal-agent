//! Roles: a session's role is a card in the plugin's agents/ (Claude Code's own agent definition - purpose,
//! owns, never, loads, model and effort, hand-over, reports to - and a `Practices:` line). A brief names its
//! role on a line of its own (`Role: keeper`, or `Role: ideal-agent:keeper`), optionally with `Also
//! practices: 42` for a domain's own; a session started with `claude --agent ideal-agent:keeper` carries it
//! in the hook's event. The role's card and only its practices are what the session reads at its start,
//! instead of every practice (decision 2026-11-17-roles-as-agent-cards).

use crate::practices;
use std::path::{Path, PathBuf};

/// A role named in a prompt or an event: its name without the plugin's prefix, and the practices a domain
/// adds to it.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Named {
    pub role: String,
    pub also: Vec<String>,
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
pub fn named_in(prompt: &str) -> Option<Named> {
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
    role.map(|role| Named { role, also })
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
pub fn recall(session: &str) -> Option<Option<Named>> {
    if !safe(session) {
        return None;
    }
    let t = std::fs::read_to_string(state_dir()?.join(session)).ok()?;
    let mut words = t.split_whitespace();
    Some(match words.next() {
        Some("-") | None => None,
        Some(r) => Some(Named {
            role: r.to_string(),
            also: words.map(String::from).collect(),
        }),
    })
}

pub fn remember(session: &str, named: Option<&Named>) {
    let Some(dir) = state_dir().filter(|_| safe(session)) else {
        return;
    };
    let line = match named {
        Some(n) => format!("{} {}\n", n.role, n.also.join(" ")),
        None => "-\n".to_string(),
    };
    let _ = std::fs::create_dir_all(&dir).and_then(|_| std::fs::write(dir.join(session), line));
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
        };
        assert!(
            render(Some(root), &none, practices::BUILT_IN, "t").contains("every practice instead")
        );
    }
}
