//! A repository's layout, checked against its own map (decision 2026-11-23-structure-checks-refuse-wrong-homes,
//! linux-phone-oco/docs): every tracked top-level folder - dot-folders and LICENSES aside - has a README.md and a
//! line in the top README's `## Map`; a Map entry naming a folder that is not there is stale. The map is the
//! layout's data: the check reads it, so the structure explains itself. A folder whose every file is read as something
//! else says so in its Map line, `(no README: WHY)`, and needs none.
use std::collections::BTreeSet;

/// The folders a `## Map` section names: each `` `NAME/` `` written in it (a code span ending in a slash); with
/// `excused`, only those whose line says `(no README: WHY)` - a folder whose every file is read as something else
/// (a plugin's agents/, where a README would load as an agent), its meaning kept in the Map line instead.
pub fn mapped(readme: &str, excused: bool) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut inside = false;
    for line in readme.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            inside = h.trim().eq_ignore_ascii_case("map");
            continue;
        }
        if line.starts_with("# ") {
            inside = false;
        }
        if !inside || (excused && !line.contains("(no README:")) {
            continue;
        }
        for (k, span) in line.split('`').enumerate() {
            if k % 2 == 1 {
                if let Some(name) = span.strip_suffix('/') {
                    if !name.is_empty() && !name.contains('/') {
                        out.insert(name.to_string());
                    }
                }
            }
        }
    }
    out
}

/// What is wrong with a layout: the tracked top-level folders, the files tracked (to know which have a
/// README.md), and the top README; empty when it holds.
pub fn problems(folders: &[String], files: &BTreeSet<String>, readme: Option<&str>) -> Vec<String> {
    let mut out = Vec::new();
    let Some(readme) = readme else {
        return vec!["README.md is missing: the repository's map opens it".into()];
    };
    let map = mapped(readme, false);
    let excused = mapped(readme, true);
    if !readme
        .lines()
        .any(|l| l.trim().eq_ignore_ascii_case("## map"))
    {
        out.push(
            "README.md has no `## Map` section: one line per top-level folder, its name as `NAME/`"
                .into(),
        );
    }
    let counted: Vec<&String> = folders
        .iter()
        .filter(|f| !f.starts_with('.') && f.as_str() != "LICENSES")
        .collect();
    for f in &counted {
        if !files.contains(&format!("{f}/README.md")) && !excused.contains(f.as_str()) {
            out.push(format!("{f}/ has no README.md saying what it holds"));
        }
        if !map.contains(f.as_str()) {
            out.push(format!(
                "{f}/ has no line in README.md's ## Map (`{f}/` and what it holds)"
            ));
        }
    }
    for m in &map {
        if !counted.iter().any(|f| f.as_str() == m) {
            out.push(format!(
                "README.md's ## Map names `{m}/`, which is not a tracked folder here"
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(list: &[&str]) -> BTreeSet<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    const README: &str = "---\nwhat: x\n---\n\n# x\n\n## Map\n\n- `src/` - the program\n- `docs/` - pages\n\n## Other\n\n`stray/` is not in the map\n";

    #[test]
    fn the_map_is_read_from_its_section_only() {
        assert_eq!(mapped(README, false), files(&["src", "docs"]));
        let r = README.replace(
            "the program",
            "the program (no README: every file is a card)",
        );
        assert_eq!(mapped(&r, true), files(&["src"]));
        assert!(problems(
            &["src".into(), "docs".into()],
            &files(&["docs/README.md"]),
            Some(&r)
        )
        .is_empty());
    }

    #[test]
    fn a_folder_with_no_readme_or_map_line_is_refused() {
        let all = [
            "src/README.md",
            "docs/README.md",
            "foo/x.rs",
            ".github/x",
            "LICENSES/MIT.txt",
        ];
        let folders: Vec<String> = ["src", "docs", "foo", ".github", "LICENSES"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let p = problems(&folders, &files(&all), Some(README));
        assert_eq!(p.len(), 2, "{p:?}");
        assert!(p.iter().all(|w| w.starts_with("foo/")));
        // allowed: foo/README.md and a Map line
        let readme = README.replace("- `docs/`", "- `foo/` - more\n- `docs/`");
        let mut all = files(&all);
        all.insert("foo/README.md".into());
        assert!(problems(&folders, &all, Some(&readme)).is_empty());
    }

    #[test]
    fn a_stale_map_line_and_a_missing_map_are_named() {
        let folders = vec!["src".to_string()];
        let p = problems(&folders, &files(&["src/README.md"]), Some(README));
        assert!(
            p.iter().any(|w| w.contains("`docs/`, which is not")),
            "{p:?}"
        );
        assert!(problems(&folders, &files(&[]), Some("# x\n"))
            .iter()
            .any(|w| w.contains("no `## Map`")));
        assert!(problems(&folders, &files(&[]), None)[0].contains("missing"));
    }
}
