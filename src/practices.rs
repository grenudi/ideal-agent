//! The practices - how an agent works in the open commons - read from this repository's database
//! (data/practices.json): live from its main, so that an improvement made once reaches every agent; else
//! the copy the plugin was installed with; else the copy built into this program.

use std::path::Path;
use std::process::Command;

pub const LIVE: &str =
    "https://raw.githubusercontent.com/grenudi/ideal-agent/main/data/practices.json";
const BUILT_IN: &str = include_str!("../data/practices.json");
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

/// The practices, numbered, each with why and for whom; lines that are not practices are skipped.
pub fn render(text: &str, source: &str) -> String {
    let mut out = format!("How to work in the open-commons ecosystem ({source})\n\n");
    let mut n = 0;
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        let Some(practice) = v["practice"].as_str() else {
            continue;
        };
        n += 1;
        out.push_str(&format!(
            "{n}. {practice}\n   why: {}\n   for: {}\n\n",
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
    fn the_built_in_copy_holds_practices() {
        assert!(render(BUILT_IN, "built-in").contains("1. "));
    }
}
