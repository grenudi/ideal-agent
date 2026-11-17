//! The practices - how an agent works in the open commons - read from this repository's database
//! (data/practices.json): live from its main, so that an improvement made once reaches every agent; else
//! the copy the plugin was installed with; else the copy built into this program.

use std::path::Path;
use std::process::Command;

pub const LIVE: &str =
    "https://raw.githubusercontent.com/grenudi/ideal-agent/main/data/practices.json";
pub const BUILT_IN: &str = include_str!("../data/practices.json");
const HOME: &str = "https://github.com/grenudi/ideal-agent (data/practices.json)";

/// The practices' text and where it came from: the file given; else live; else the plugin's copy; else
/// the built-in one.
pub fn read(from: Option<&Path>, plugin_root: Option<&Path>) -> (String, String) {
    if let Some(f) = from {
        if let Ok(t) = std::fs::read_to_string(f) {
            return (t, format!("from {}", f.display()));
        }
    }
    if let Some(t) = live() {
        return (t, "live from ideal-agent's main".into());
    }
    if let Some(t) =
        plugin_root.and_then(|r| std::fs::read_to_string(r.join("data/practices.json")).ok())
    {
        return (
            t,
            "the copy the plugin was installed with - its main could not be reached".into(),
        );
    }
    (
        BUILT_IN.to_string(),
        "the built-in copy - nothing newer could be reached".into(),
    )
}

/// curl, where it is, with a short timeout: a session must not wait on the network to start.
fn live() -> Option<String> {
    let out = Command::new("curl")
        .args(["-fsSL", "--max-time", "8", LIVE])
        .output()
        .ok()?;
    let text = String::from_utf8(out.stdout).ok()?;
    (out.status.success() && text.contains("\"practice\"")).then_some(text)
}

/// The practices in the order of their ids, each with the number its id begins with (01-record-in-tad is
/// 1), so that a practice is called by the same number wherever it stands in the file.
fn in_order(text: &str) -> Vec<(String, serde_json::Value)> {
    let mut all: Vec<(String, serde_json::Value)> = text
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|v| v["practice"].is_string())
        .map(|v| (v["id"].as_str().unwrap_or("").to_string(), v))
        .collect();
    all.sort_by(|a, b| a.0.cmp(&b.0));
    all.into_iter()
        .enumerate()
        .map(|(i, (id, v))| {
            let n = id
                .split('-')
                .next()
                .and_then(|d| d.parse::<u32>().ok())
                .map_or((i + 1).to_string(), |n| n.to_string());
            (n, v)
        })
        .collect()
}

/// A practice's first sentence: its words up to the first full stop that ends a sentence.
pub fn first_sentence(practice: &str) -> &str {
    match practice.find(". ") {
        Some(i) => &practice[..=i],
        None => practice,
    }
}

/// The practices as a session start reads them: numbered, each its first sentence alone - the rest of its
/// words, its reason and whom it is for are one command away (`ideal-agent practices`), so the start stays
/// light (practice 45-keep-the-start-light; the author chose first sentences on 2026-11-17).
pub fn render_short(text: &str, source: &str, only: Option<&[String]>) -> String {
    let mut out = format!("How to work in the open-commons ecosystem ({source})\n\n");
    for (n, v) in in_order(text) {
        let id = v["id"].as_str().unwrap_or("");
        if only.is_some_and(|keep| !keep.iter().any(|k| id.starts_with(k.as_str()))) {
            continue;
        }
        let practice = v["practice"].as_str().unwrap_or("");
        out.push_str(&format!("{n}. {}\n", first_sentence(practice)));
    }
    out.push_str("\nEach practice whole, with why it holds and for whom: `lab-agent guide` on the lab, or the plugin's `scripts/ideal-agent practices`.\n");
    out
}

/// The practices, numbered, each whole with why and for whom; lines that are not practices are skipped.
pub fn render(text: &str, source: &str) -> String {
    let mut out = format!("How to work in the open-commons ecosystem ({source})\n\n");
    for (n, v) in in_order(text) {
        out.push_str(&format!(
            "{n}. {}\n   why: {}\n   for: {}\n\n",
            v["practice"].as_str().unwrap_or(""),
            v["why"].as_str().unwrap_or(""),
            v["for"].as_str().unwrap_or("")
        ));
    }
    out.push_str(&format!(
        "Keep your memory and preferences in line with these. When you find a better way, carry it back:\n{HOME}\n"
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practices_are_numbered_with_their_reasons() {
        let t = "{\"id\":\"a\",\"practice\":\"Record it.\",\"why\":\"Memory fails.\",\"for\":\"agents\"}\nnot json\n{\"id\":\"b\",\"practice\":\"Land it.\",\"why\":\"Checks.\",\"for\":\"people\"}\n";
        let r = render(t, "test");
        assert!(r.starts_with("How to work in the open-commons ecosystem (test)"));
        assert!(r.contains("1. Record it.\n   why: Memory fails.\n   for: agents"));
        assert!(
            r.contains("2. Land it."),
            "the bad line is skipped, the numbering goes on"
        );
    }

    #[test]
    fn the_start_reads_the_practices_without_their_reasons() {
        let t = "{\"id\":\"a\",\"practice\":\"Record it.\",\"why\":\"Memory fails.\",\"for\":\"agents\"}\n";
        let r = render_short(t, "test", None);
        assert!(r.contains("1. Record it.\n"));
        assert!(
            !r.contains("Memory fails."),
            "the reason is one command away"
        );
        assert!(r.contains("ideal-agent practices"));
    }

    #[test]
    fn the_start_reads_first_sentences_in_the_order_of_the_ids() {
        let t = "{\"id\":\"22-look\",\"practice\":\"Look first. Then say what is.\",\"why\":\"w\",\"for\":\"agents\"}\n{\"id\":\"03-memory\",\"practice\":\"Adjust your memory.\",\"why\":\"w\",\"for\":\"agents\"}\n";
        let r = render_short(t, "test", None);
        let three = r
            .find("3. Adjust your memory.\n")
            .expect("numbered by its id");
        let twenty_two = r
            .find("22. Look first.\n")
            .expect("its first sentence alone");
        assert!(
            three < twenty_two,
            "in the order of the ids, not of the file"
        );
        assert!(!r.contains("Then say what is."));
        assert!(
            render(t, "test").contains("22. Look first. Then say what is.\n"),
            "the whole is one command away"
        );
    }

    #[test]
    fn the_built_in_copy_holds_practices() {
        assert!(render(BUILT_IN, "built-in").contains("1. "));
    }
}
