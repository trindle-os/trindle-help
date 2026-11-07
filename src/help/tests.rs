//! Tests for the Help program.

use super::*;

fn docs() -> Vec<Doc> {
    pages::all()
}

#[test]
fn the_data_file_has_every_page_of_the_website() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("site/content");
    let mut want: Vec<String> = std::fs::read_dir(dir).unwrap().flatten()
        .filter_map(|e| e.file_name().into_string().ok()).filter_map(|n| n.strip_suffix(".md").map(String::from)).collect();
    want.sort();
    let mut have: Vec<String> = docs().iter().map(|d| d.id.clone()).collect();
    have.sort();
    assert_eq!(want, have);
    for d in docs() {
        assert!(d.parts.len() > 1 || d.id == "glossary", "{} has no sections", d.id);
        assert!(d.parts.iter().skip(1).all(|p| !p.heading.is_empty() && !p.blocks.is_empty() || p.id.starts_with("top")), "{}", d.id);
    }
}

#[test]
fn a_questions_line_is_kept_for_the_search_and_never_shown() {
    let docs = docs();
    let folders = docs.iter().find(|d| d.id == "folders").unwrap();
    let part = folders.parts.iter().find(|p| p.id == "other-computers").unwrap();
    assert!(part.questions.len() >= 3);
    for d in &docs {
        for p in &d.parts {
            for b in &p.blocks {
                assert!(!b.lines.iter().any(|l| l.starts_with("Questions:")), "{} shows its questions", d.id);
            }
        }
    }
}

#[test]
fn an_address_opens_at_its_section() {
    let docs = docs();
    let at = resolve(&docs, "folders#other-computers");
    assert!(at.found);
    assert_eq!(docs[at.page].id, "folders");
    assert_eq!(docs[at.page].parts[at.part].id, "other-computers");
    let app = Helper::new(self::docs(), "folders#other-computers", "");
    assert_eq!(app.here, (at.page, at.part));
    assert!(app.unknown.is_none());
}

#[test]
fn an_unknown_address_falls_back_to_the_top_page_and_says_so() {
    let docs = docs();
    for address in ["nonesuch", "folders#nonesuch", "nonesuch#other-computers"] {
        let at = resolve(&docs, address);
        assert!(!at.found, "{address}");
        assert_eq!((docs[at.page].id.as_str(), at.part), (pages::FIRST_PAGE, 0), "{address}");
        let app = Helper::new(self::docs(), address, "");
        assert_eq!(app.unknown.as_deref(), Some(address));
        let said = app.view().said().join(" ");
        assert!(said.contains(&t!("help-address-unknown", address = address)), "{address}: {said}");
    }
    assert!(resolve(&docs, "").found);
    assert!(Helper::new(self::docs(), "", "").unknown.is_none());
}

#[test]
fn every_link_in_the_writing_leads_to_a_section() {
    let app = Helper::new(docs(), "", "");
    for d in &app.docs {
        for p in &d.parts {
            for b in &p.blocks {
                // features.html is made from the features list for the website only; Help shows its words without a button.
                for l in b.links.iter().filter(|l| l.page != "features") {
                    assert!(app.find(&l.page, l.section.as_deref()).is_some(), "{}: a link to {}#{:?}", d.id, l.page, l.section);
                }
            }
        }
    }
}

#[test]
fn going_and_coming_back() {
    let mut app = Helper::new(docs(), "", "");
    let start = app.here;
    app.update(Msg::Go("folders".into(), Some("other-computers".into())));
    assert_ne!(app.here, start);
    app.update(Msg::Back);
    assert_eq!(app.here, start);
    assert!(app.before.is_empty());
}

#[test]
fn every_section_can_be_read() {
    let mut app = Helper::new(docs(), "", "");
    for p in 0..app.docs.len() {
        for s in 0..app.docs[p].parts.len() {
            app.here = (p, s);
            let _ = app.view();
        }
    }
}

#[test]
fn the_help_window_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    drawn::none(drawn::in_every_look("the help window", 8, || {
        for (state, address, typed) in [("first page", "", ""), ("a section", "folders#other-computers", ""),
                                        ("answers", "", "why won't my NAS open"), ("nothing found", "", "zzzzqqqq"),
                                        ("unknown address", "nonesuch", "")] {
            let mut app = Helper::new(docs(), address, "");
            if !typed.is_empty() {
                app.update(Msg::Typed(typed.into()));
            }
            drawn::window(state, app.view(), Theme::chosen(), SIZE, false);
        }
        let mut app = Helper::new(docs(), "", "");
        app.update(Msg::Go("getting-started".into(), Some("what-you-need".into())));
        drawn::window("a page being read", app.view(), Theme::chosen(), SIZE, false);
    }));
}

fn sample() -> Vec<Doc> {
    pages::parse_all("\u{1e}sample\n---\ntitle: Sample\n---\n\n## Part\nQuestions: a; b; c\n\nOpen words.\n\n:::expert\nTechnical words.\n\n- one\n- two\n:::\n\nAfter words.\n")
}

#[test]
fn an_expert_area_is_kept_apart_from_the_words_and_found_by_the_search() {
    let docs = sample();
    let part = docs[0].parts.iter().find(|p| p.id == "part").unwrap();
    let said = |bs: &[Block]| bs.iter().flat_map(|b| b.lines.clone()).collect::<Vec<_>>().join(" ");
    assert_eq!(said(&part.blocks), "Open words. After words.");
    assert_eq!(said(&part.expert), "Technical words. one two");
    assert!(part.text().contains("Technical words"));
}

#[test]
fn an_expert_area_is_shut_to_begin_with_and_opened_and_shut_by_its_button() {
    let mut app = Helper::new(sample(), "sample#part", "");
    let words = |app: &Helper| app.view().said().join(" ");
    let shut = words(&app);
    assert!(shut.contains("Open words.") && !shut.contains("Technical words."), "{shut}");
    assert!(shut.contains(t!("help-expert")) && shut.contains(&format!("button: {}", t!("help-expert-open"))), "{shut}");
    app.update(Msg::Expert);
    let open = words(&app);
    assert!(open.contains("Technical words.") && open.contains(&format!("button: {}", t!("help-expert-shut"))), "{open}");
    app.update(Msg::Expert);
    assert!(!words(&app).contains("Technical words."));
    // Going to another section, or back, shuts it again.
    app.update(Msg::Expert);
    app.update(Msg::Go("sample".into(), None));
    assert!(!app.expert);
    app.update(Msg::Expert);
    app.update(Msg::Back);
    assert!(!app.expert);
}

#[test]
fn a_section_with_no_expert_area_has_no_button_for_it() {
    let app = Helper::new(docs(), "getting-started", "");
    assert!(!app.view().said().join(" ").contains(t!("help-expert")));
}

#[test]
fn every_expert_area_in_the_writing_is_closed_and_holds_no_heading() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("site/content");
    for e in std::fs::read_dir(dir).unwrap().flatten().filter(|e| e.path().extension().is_some_and(|x| x == "md")) {
        let text = std::fs::read_to_string(e.path()).unwrap();
        let mut inside = false;
        for line in text.lines() {
            match line.trim_end() {
                ":::expert" => {
                    assert!(!inside, "{:?}: an expert area inside another", e.path());
                    inside = true;
                }
                ":::" => {
                    assert!(inside, "{:?}: an area closed that was not opened", e.path());
                    inside = false;
                }
                l if inside => assert!(!l.starts_with('#'), "{:?}: a heading inside an expert area", e.path()),
                _ => {}
            }
        }
        assert!(!inside, "{:?}: an expert area never closed", e.path());
    }
}

/// One real error text for every kind in `errors::CASES`, with its section.
const REAL_ERRORS: &[(&str, &str)] = &[
    ("errors#the-package-database-is-locked", "E: Could not get lock /var/lib/dpkg/lock-frontend. It is held by process 4021 (apt)"),
    ("errors#the-package-database-is-locked", "E: Unable to acquire the dpkg frontend lock (/var/lib/dpkg/lock-frontend), is another process using it?"),
    ("errors#there-is-no-space-left", "Not updated. Flatpak said: error: Not enough disk space to complete this operation"),
    ("errors#there-is-no-space-left", "dpkg: error: failed to write: No space left on device"),
    ("errors#a-driver-would-not-build-for-the-new-kernel", "Error! Bad return status for module build on kernel: 6.12.9 (x86_64)\ndpkg: error processing package nvidia-kernel-dkms (--configure)"),
    ("errors#a-driver-would-not-build-for-the-new-kernel", "dkms autoinstall on 6.12.9 failed for rtl8821ce(10)"),
    ("errors#updates-were-kept-back", "Not all updates went in. Kept back: linux-image-amd64, firmware-misc-nonfree."),
    ("errors#packages-have-unmet-dependencies", "The following packages have unmet dependencies:\n foo : Depends: libbar (>= 2) but it is not going to be installed"),
    ("errors#packages-have-unmet-dependencies", "E: Unmet dependencies. Try 'apt --fix-broken install' with no packages"),
    ("errors#a-package-would-not-configure", "dpkg: error processing package cups (--configure):\n installed cups package post-installation script subprocess returned error exit status 1"),
    ("errors#a-package-would-not-configure", "E: dpkg was interrupted, you must manually run 'dpkg --configure -a' to correct the problem."),
    ("errors#a-download-failed", "E: Failed to fetch http://deb.debian.org/debian/pool/main/f/foo.deb  404  Not Found [IP: 151.101.0.204 80]"),
    ("errors#a-download-failed", "Err:3 http://deb.debian.org/debian trixie InRelease\n  Temporary failure resolving 'deb.debian.org'"),
    ("errors#a-download-failed", "E: Failed to fetch file.deb  Hash Sum mismatch"),
];

#[test]
fn every_error_case_has_a_section_with_meaning_causes_and_steps() {
    let docs = docs();
    let raw = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("site/content/errors.md")).unwrap();
    let mut all: Vec<&str> = errors::CASES.iter().map(|(a, _)| *a).collect();
    all.push(errors::GENERAL);
    for address in all {
        let at = resolve(&docs, address);
        assert!(at.found && at.part != 0, "{address} has no section");
        let part = &docs[at.page].parts[at.part];
        assert!(part.questions.len() >= 3, "{address}: {} questions", part.questions.len());
        let first = |kind: BlockKind| part.blocks.iter().position(|b| b.kind == kind);
        let says = |start: &str| part.blocks.iter().position(|b| b.kind == BlockKind::Paragraph && b.lines.join(" ").starts_with(start));
        let (meaning, causes, steps) = (says("What it means"), says("Likely causes"), says("Steps to fix it"));
        assert!(meaning.is_some() && causes.is_some() && steps.is_some(), "{address} lacks its meaning, causes or steps");
        assert!(meaning < causes && causes < steps, "{address}: the order is meaning, causes, steps");
        assert!(first(BlockKind::Bullets).is_some_and(|b| b > causes.unwrap()) && first(BlockKind::Steps).is_some_and(|b| b > steps.unwrap()), "{address}: causes are bullets, steps are numbered");
        let id = address.split_once('#').unwrap().1;
        let section = raw.split("\n## ").find(|s| pages::slug(s.lines().next().unwrap_or("")) == id).unwrap();
        assert!(section.contains("<!-- screenshot: ") || section.contains("!["), "{id} has no picture and names none to take");
        let words = part.text();
        assert!(!words.contains('\u{2014}') && !words.contains('\u{2013}'), "{address} has a dash");
    }
    for (address, needles) in errors::CASES {
        assert!(needles.iter().all(|n| *n == n.to_lowercase()), "{address}: needles are lower case");
    }
}

#[test]
fn a_sample_of_real_errors_is_told_apart() {
    for (want, text) in REAL_ERRORS {
        assert_eq!(errors::classify(text), Some(*want), "{text}");
        assert_eq!(errors::classify(&text.to_uppercase()), Some(*want), "{text} in capitals");
    }
    for (address, _) in errors::CASES {
        assert!(REAL_ERRORS.iter().any(|(a, _)| a == address), "{address} has no sample");
    }
    assert_eq!(errors::classify("Something nobody has seen before"), None);
    assert_eq!(errors::classify(""), None);
}

#[test]
fn help_opened_from_more_shows_only_the_error_section() {
    let docs = docs();
    for (want, text) in REAL_ERRORS {
        let app = Helper::new(self::docs(), "apps#an-update-did-not-finish", text);
        let at = resolve(&docs, want);
        assert_eq!(app.here, (at.page, at.part), "{text}");
        assert!(app.focus && app.unknown.is_none());
        let said = app.view().said().join(" ");
        assert!(said.contains(text.lines().next().unwrap()), "the real text is shown: {said}");
        assert!(said.contains(t!("help-error")) && said.contains(&format!("button: {}", t!("help-error-copy"))), "{said}");
        for starter in [t!("help-search").to_string(), t!("help-lead").to_string()] {
            assert!(!said.contains(&starter), "{want}: the starter block {starter:?} shows: {said}");
        }
        assert!(said.contains(&format!("button: {}", t!("help-all"))), "{said}");
    }
}

#[test]
fn an_error_with_no_kind_keeps_the_named_section_or_gets_the_general_one() {
    let docs = docs();
    let named = Helper::new(self::docs(), "apps#an-update-did-not-finish", "Something nobody has seen before");
    let at = resolve(&docs, "apps#an-update-did-not-finish");
    assert_eq!(named.here, (at.page, at.part));
    assert!(named.view().said().join(" ").contains("Something nobody has seen before"));
    for address in ["", "nonesuch#nothing"] {
        let app = Helper::new(self::docs(), address, "Something nobody has seen before");
        let general = resolve(&docs, errors::GENERAL);
        assert_eq!(app.here, (general.page, general.part), "{address:?}");
        assert!(app.view().said().join(" ").contains("Something nobody has seen before"));
    }
    // Without any words nothing is added: an address alone opens its section.
    let plain = Helper::new(self::docs(), "apps#an-update-did-not-finish", "");
    assert!(plain.error.is_none() && plain.focus);
    assert!(!plain.view().said().join(" ").contains(t!("help-error")));
}

#[test]
fn all_of_help_brings_back_the_search_and_the_error_is_cleaned_and_copied() {
    let mut app = Helper::new(self::docs(), "apps#an-update-did-not-finish", "--version\u{7}\u{1b}[31m No space left on device");
    assert_eq!(app.error.as_deref(), Some("--version  [31m No space left on device"));
    assert!(app.to_clipboard().is_none());
    app.update(Msg::CopyError);
    assert_eq!(app.to_clipboard().as_deref(), Some("--version  [31m No space left on device"));
    assert!(app.to_clipboard().is_none());
    app.update(Msg::All);
    let said = app.view().said().join(" ");
    assert!(said.contains(t!("help-title")) && said.contains(t!("help-lead")), "{said}");
    let long = "x".repeat(10_000);
    assert!(Helper::new(self::docs(), "", &long).error.unwrap().len() <= clean::ERROR_LIMIT);
}

#[test]
fn a_kept_back_list_naming_a_driver_package_goes_to_the_kept_back_page() {
    let kept = "Not all updates went in. Kept back: virtualbox-dkms, linux-image-amd64.";
    assert_eq!(errors::classify(kept), Some("errors#updates-were-kept-back"));
    assert_eq!(errors::classify("dkms autoinstall on 6.12.9 failed for rtl8821ce(10)"), Some("errors#a-driver-would-not-build-for-the-new-kernel"));
    assert_eq!(errors::classify("The following packages have been kept back: nvidia-kernel-dkms"), Some("errors#updates-were-kept-back"));
}

#[test]
fn a_call_that_does_not_decode_leaves_the_place_and_search_alone() {
    let mut app = Helper::new(self::docs(), "apps#an-update-did-not-finish", "");
    app.update(Msg::Typed("update".into()));
    let (here, typed, hits) = (app.here, app.typed.clone(), app.hits.len());
    app.asked_to_aim(None);
    assert_eq!((app.here, app.typed.clone(), app.hits.len()), (here, typed, hits));
    app.asked_to_aim(Some(("apps#an-update-did-not-finish".into(), "No space left on device".into())));
    assert!(app.typed.is_empty() && app.error.is_some(), "a good call still aims");
}

#[test]
fn the_help_error_page_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::{drawn, Theme};
    drawn::none(drawn::in_every_look("the help window at an error", 8, || {
        let app = Helper::new(docs(), "apps#an-update-did-not-finish", "dpkg: error processing package cups (--configure)");
        drawn::window("an error", app.view(), Theme::chosen(), SIZE, false);
    }));
}

#[test]
fn every_picture_the_pages_name_can_be_drawn_and_a_drawing_shows_its_words() {
    let mut named = 0;
    for doc in docs() {
        for part in &doc.parts {
            for b in part.blocks.iter().chain(&part.expert).filter(|b| b.kind == BlockKind::Picture && !b.picture.starts_with(OWN_PICTURES)) {
                named += 1;
                let bytes = pages::image_bytes(&b.picture).unwrap_or_else(|| panic!("{}: no picture {}", doc.id, b.picture));
                let img = if b.picture.ends_with(".svg") { svg::draw(bytes, PICTURE_MOST) } else { image::load_from_memory(bytes).ok() };
                assert!(img.is_some(), "{} cannot be drawn", b.picture);
            }
        }
    }
    assert!(named > 100, "{named}");
    let steps = svg::draw(pages::image_bytes("install-steps.svg").unwrap(), PICTURE_MOST).unwrap().to_rgba8();
    let words = steps.pixels().filter(|p| p.0[..3] == [0xe8, 0xed, 0xf5]).count();
    assert!(words > 500, "the words of the drawing are not drawn: {words} pixels");
}
