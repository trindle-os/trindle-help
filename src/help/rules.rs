//! Tests that hold the rules Trindle's own source checks held for Help before it moved here.

use std::path::{Path, PathBuf};

/// Names that must never appear in what Trindle ships, split here so this file does not hold them. The community home's name is allowed only in the files listed, where bug reports point to it.
#[test]
fn names_that_must_not_be_in_trindle_are_not_in_it() {
    let banned = [["cos", "mic"].concat(), ["ne", "mo"].concat(), ["cinn", "amon"].concat(), ["muf", "fin"].concat(), ["light", "morphic"].concat()];
    let community = ["build", "morphic"].concat();
    const COMMUNITY_FILES: [&str; 6] = ["README.md", "CONTRIBUTING.md", "SECURITY.md", "site/content/report-a-problem.md", "site/content/img/report-flow.svg", "tools/site/diagrams.py"];
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut stack: Vec<PathBuf> = ["src", "site", "share", "tools", "packaging", "README.md", "CONTRIBUTING.md", "SECURITY.md", "CHANGELOG.md", "Cargo.toml", "build.rs"].iter().map(|d| root.join(d)).filter(|d| d.exists()).collect();
    let mut hits = Vec::new();
    while let Some(p) = stack.pop() {
        if p.ends_with("site/dist") || p.ends_with("src/help/rules.rs") {
            continue;
        }
        if p.is_dir() {
            stack.extend(std::fs::read_dir(&p).into_iter().flatten().flatten().map(|e| e.path()));
        } else if let Ok(text) = std::fs::read_to_string(&p) {
            let low = text.to_lowercase();
            for b in &banned {
                if low.replace("mnemonic", "").contains(b.as_str()) {
                    hits.push(format!("{} has {b}", p.display()));
                }
            }
            let rel = p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
            if low.contains(&community) && !COMMUNITY_FILES.contains(&rel.as_str()) {
                hits.push(format!("{rel} has {community} outside its allowed places"));
            }
        }
    }
    assert!(hits.is_empty(), "{hits:#?}");
}

/// Help has one window: a second copy raises the first through the bus instead of opening another.
#[test]
fn a_second_copy_of_help_raises_the_first() {
    let code = include_str!("mod.rs").split("#[cfg(test)]\nmod rules;").next().unwrap();
    assert!(code.contains("once::claim(") && code.contains("once::serve_window("));
}

/// The test that paints the error page for the table of pairs is run again in every theme that comes with Trindle (`share/themes` in the Trindle repository, found beside this one or named by TRINDLE_THEMES), with that theme's sheet as the theme in use. It fails, never skips, without them.
#[test]
fn the_help_window_is_painted_in_every_theme_that_comes_with_trindle() {
    use crate::toolkit::theme::drawn::NAMED;
    const INNER: &str = "TRINDLE_HELP_PAIRS_INNER";
    if std::env::var_os(INNER).is_some() {
        return;
    }
    let themes = std::env::var_os("TRINDLE_THEMES").map(PathBuf::from).unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../trindle/share/themes"));
    let mut names: Vec<PathBuf> = std::fs::read_dir(&themes).unwrap_or_else(|e| panic!("the themes are in the Trindle repository: put it beside this one or set TRINDLE_THEMES ({}: {e})", themes.display()))
        .flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "css")).collect();
    names.sort();
    assert!(names.len() >= 4, "{names:?}");
    for theme in names {
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([NAMED, "--test-threads=1"]).env("TRINDLE_TEST_THEME", &theme).env(INNER, "1").output().unwrap();
        let said = String::from_utf8_lossy(&out.stdout);
        assert!(out.status.success() && said.lines().any(|l| l.starts_with("test ") && l.ends_with("::the_help_error_page_draws_only_pairs_the_table_holds ... ok")),
                "in the theme {}:\n{}", theme.display(), said.lines().filter(|l| !l.ends_with("... ok")).take(60).collect::<Vec<_>>().join("\n"));
    }
}
