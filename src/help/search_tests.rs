//! Tests for Help's search: ranking, typing mistakes and speed.

use super::search::*;
use std::time::Duration;

fn sec(id: &str, heading: &str, text: &str, questions: &[&str]) -> Section {
    Section {
        id: id.into(),
        heading: heading.into(),
        text: text.into(),
        questions: questions.iter().map(|q| q.to_string()).collect(),
    }
}

fn page(id: &str, title: &str, sections: Vec<Section>) -> Page {
    Page { id: id.into(), title: title.into(), sections }
}

fn fixture() -> Vec<Page> {
    vec![
        page("folders", "Folders", vec![
            sec("nas-open", "A network drive will not open",
                "Trindle opens NFS on version 4 only. A server without it gets a plain message. Ask whoever runs the server to turn version 4 on.",
                &["Why won't my NAS open?", "Why does my shared folder say it cannot connect?"]),
            sec("two-folders", "See two folders at once",
                "Press the split button in the top bar. Each side keeps its own place. Drag a file from one side to the other to copy it.",
                &["How do I see two folders at once?", "Can I copy between two windows?"]),
            sec("copy-slow", "A copy is slow",
                "Copies to a network drive depend on the network. Folders shows the speed beside the bar while it copies.",
                &["Why is copying slow?"]),
            sec("trash", "Deleting files",
                "A deleted file goes to the bin first. Empty the bin to free the space.",
                &["How do I delete a file?", "How do I get a deleted file back?"]),
        ]),
        page("settings", "Settings", vec![
            sec("wallpaper", "Wallpaper",
                "Open Settings, then Look, then Wallpaper. Pick a picture from the list or add your own from a folder.",
                &["How do I change the wallpaper?", "How do I set a picture on my desktop?"]),
            sec("text-size", "Text size",
                "Open Settings, then Look. Move the text size slider to make everything bigger or smaller. The change shows at once.",
                &["How do I make the text bigger?", "The writing is too small to read."]),
            sec("wifi", "Wi-Fi",
                "Open Settings, then Wi-Fi, and pick your network. Type the password once; Trindle remembers it. If the connection keeps dropping, move closer to the router or turn off power saving for Wi-Fi.",
                &["How do I join the Wi-Fi?", "My wifi keeps dropping.", "Why does the wireless disconnect?"]),
            sec("bluetooth", "Bluetooth",
                "Open Settings, then Bluetooth. Switch it on, put the headphones in pairing mode and pick them from the list.",
                &["How do I connect Bluetooth headphones?", "How do I pair a speaker?"]),
            sec("sound", "Sound",
                "Open Settings, then Sound. Pick where sound plays and move the volume slider. The ring has a volume button too.",
                &["There is no sound.", "How do I change the volume?"]),
            sec("mic", "Microphone",
                "The microphone switch in the ring cuts the microphone for every program until you press it again.",
                &["How do I mute the microphone?", "Is an app listening to me?"]),
            sec("printer", "Printers",
                "Open Settings, then Printers, and press Add. Trindle finds printers on your network by itself.",
                &["How do I add a printer?", "My printer is not found."]),
            sec("language", "Language",
                "Open Settings, then Language. Pick the language and the keyboard layout, then sign out and in again.",
                &["How do I change the language?", "How do I change the keyboard layout?"]),
            sec("dark", "Dark and light",
                "Open Settings, then Look, and pick a theme. Themes change every Trindle program at once.",
                &["How do I switch to dark mode?", "How do I change the colours?"]),
            sec("power", "Power",
                "Open Settings, then Power. Choose when the screen turns off and when the computer sleeps. Keep awake in the ring stops both for a while.",
                &["How do I stop the screen turning off?", "How do I keep the computer awake?"]),
            sec("display", "Screen size",
                "Open Settings, then Displays. Pick a size for each screen. Scale makes everything larger on a sharp screen.",
                &["How do I change the screen size?", "The screen looks wrong after I plugged in a monitor."]),
        ]),
        page("ring", "The ring", vec![
            sec("open-ring", "Open the ring",
                "Move the pointer to the very edge of the screen and click. The Super key opens it too.",
                &["How do I open the ring?", "The ring will not open when I click the edge."]),
            sec("lock", "Lock the screen",
                "Press the lock button in the ring, or press Super and L. Your password brings the screen back.",
                &["How do I lock the screen?", "How do I stop others using my computer?"]),
            sec("windows", "Switch windows",
                "Press Alt and Tab to move between open windows. Hold Alt to see them all.",
                &["How do I switch between windows?"]),
            sec("screenshot", "Screenshots",
                "Press the camera button in the ring. Drag over the part of the screen to keep. The picture is saved in Pictures.",
                &["How do I take a screenshot?", "Where do my screenshots go?"]),
        ]),
        page("apps", "Apps", vec![
            sec("install", "Install an app",
                "Open Apps, find the program and press Install. It lives in its own box and asks before it reaches your files.",
                &["How do I install an app?", "Where do I get a web browser?"]),
            sec("remove", "Remove an app",
                "Open Apps, then Installed, and press the remove button beside the program. Everything it kept goes with it.",
                &["How do I remove an app?", "How do I uninstall a program?"]),
            sec("update", "Updates",
                "Security updates install themselves every day. Other updates wait in Apps until you press Update all.",
                &["My app will not install its update.", "How do I update my apps?"]),
            sec("perm", "Permissions",
                "Open Settings, then Permissions. Each app has round buttons for the files, camera and microphone it may reach.",
                &["Why did an app ask to use my camera?"]),
        ]),
        page("drives", "Drives", vec![
            sec("usb", "USB sticks",
                "Plug the stick in and it shows in Folders on the left. Press the eject button before you pull it out.",
                &["How do I use a USB stick?", "How do I safely remove a flash drive?"]),
            sec("report", "Report a problem",
                "Open Settings, then Crash reports, and press Send a report. Nothing is sent until you press it.",
                &["How do I report a problem?", "Does Trindle send anything from my computer?"]),
        ]),
    ]
}

/// Real questions, and the section each must find in the first three.
const QUESTIONS: &[(&str, &str)] = &[
    ("why won't my NAS open", "nas-open"),
    ("my network drive will not connect", "nas-open"),
    ("cannot open shared folder on the file server", "nas-open"),
    ("how do I change the wallpaper", "wallpaper"),
    ("how can I change my desktop background", "wallpaper"),
    ("my wifi keeps dropping", "wifi"),
    ("wireless keeps disconnecting", "wifi"),
    ("how do I join the wi-fi", "wifi"),
    ("how do I make the text bigger", "text-size"),
    ("the writing is too small", "text-size"),
    ("can I make the font larger", "text-size"),
    ("how do I connect bluetooth headphones", "bluetooth"),
    ("there is no sound", "sound"),
    ("how do I turn the volume up", "sound"),
    ("how do I mute the mic", "mic"),
    ("how do I add a printer", "printer"),
    ("how do I change the language", "language"),
    ("how do I switch to dark mode", "dark"),
    ("how do I stop the screen turning off", "power"),
    ("keep my computer awake", "power"),
    ("how do I change the screen size", "display"),
    ("how do I open the ring", "open-ring"),
    ("how do I lock the screen", "lock"),
    ("how do I switch between windows", "windows"),
    ("how do I take a screenshot", "screenshot"),
    ("how do I install an app", "install"),
    ("how do I uninstall a program", "remove"),
    ("an update will not install", "update"),
    ("how do I safely remove a usb stick", "usb"),
    ("how do I report a problem", "report"),
    ("how do I see two folders at once", "two-folders"),
    ("how do I get a deleted file back", "trash"),
    ("why is copying to my nas so slow", "copy-slow"),
    ("what is the camera permission for", "perm"),
];

fn rank(index: &Index, q: &str, want: &str) -> Option<usize> {
    index.query(q, 3).iter().position(|h| h.section == want)
}

#[test]
fn every_question_finds_its_section_in_the_first_three() {
    let index = Index::build(&fixture());
    let missed: Vec<_> = QUESTIONS.iter().filter(|(q, s)| rank(&index, q, s).is_none()).collect();
    assert!(QUESTIONS.len() >= 25);
    assert!(missed.is_empty(), "missed: {missed:?}");
}

#[test]
fn the_best_questions_come_first() {
    let index = Index::build(&fixture());
    for (q, s) in [("why won't my NAS open", "nas-open"), ("how do I change the wallpaper", "wallpaper"), ("my wifi keeps dropping", "wifi")] {
        assert_eq!(rank(&index, q, s), Some(0), "{q}");
    }
}

#[test]
fn typing_mistakes_are_forgiven() {
    let index = Index::build(&fixture());
    for (q, s) in [
        ("wallpapr", "wallpaper"),
        ("how do i chnage the wallpaper", "wallpaper"),
        ("blutooth headphones", "bluetooth"),
        ("screnshot", "screenshot"),
        ("pritner", "printer"),
        ("micophone", "mic"),
    ] {
        assert!(rank(&index, q, s).is_some(), "{q}");
    }
}

#[test]
fn word_endings_are_forgiven() {
    let index = Index::build(&fixture());
    assert!(rank(&index, "changing wallpapers", "wallpaper").is_some());
    assert!(rank(&index, "printers", "printer").is_some());
    assert!(rank(&index, "locked screens", "lock").is_some());
}

#[test]
fn empty_and_question_only_queries_return_nothing() {
    let index = Index::build(&fixture());
    assert!(index.query("", 5).is_empty());
    assert!(index.query("   ", 5).is_empty());
    assert!(index.query("how do I", 5).is_empty());
    assert!(index.query("???", 5).is_empty());
    assert!(index.query("zzzzqqq", 5).is_empty());
    assert!(index.query("wallpaper", 0).is_empty());
    assert!(Index::build(&[]).query("wallpaper", 5).is_empty());
}

#[test]
fn the_limit_is_kept() {
    let index = Index::build(&fixture());
    assert!(index.query("open settings", 2).len() <= 2);
}

#[test]
fn the_snippet_is_the_answering_sentence_cut_from_the_text() {
    let pages = fixture();
    let index = Index::build(&pages);
    let hit = &index.query("wifi keeps dropping", 1)[0];
    assert!(hit.snippet.contains("keeps dropping"), "{}", hit.snippet);
    assert!(hit.snippet.chars().count() <= 161);
    let hit = &index.query("why won't my NAS open", 1)[0];
    assert!(hit.snippet.contains("NFS"), "{}", hit.snippet);
    for (q, _) in QUESTIONS {
        for h in index.query(q, 3) {
            let text = pages
                .iter()
                .flat_map(|p| &p.sections)
                .find(|s| s.id == h.section)
                .map(|s| s.text.clone())
                .unwrap();
            let body = h.snippet.trim_end_matches('\u{2026}');
            assert!(text.contains(body), "invented: {}", h.snippet);
        }
    }
}

#[test]
fn a_long_sentence_is_cut_at_about_160_characters() {
    let long = format!("Open the thing {} and then it ends.", "and carry on ".repeat(40));
    let pages = vec![page("p", "P", vec![sec("s", "Thing", &long, &[])])];
    let hit = &Index::build(&pages).query("thing", 1)[0];
    assert!(hit.snippet.chars().count() <= 160, "{}", hit.snippet.chars().count());
    assert!(hit.snippet.ends_with('\u{2026}'));
}

#[test]
fn hits_name_their_page_and_section() {
    let index = Index::build(&fixture());
    let hit = &index.query("wallpaper", 1)[0];
    assert_eq!((hit.page.as_str(), hit.section.as_str()), ("settings", "wallpaper"));
}

#[test]
fn equal_scores_keep_a_fixed_order() {
    let same = |id: &str| sec(id, "Same words", "The same sentence about gadgets.", &[]);
    let pages = vec![
        page("a", "A", vec![same("one"), same("two")]),
        page("b", "B", vec![same("three"), same("four")]),
    ];
    let index = Index::build(&pages);
    let order = |i: &Index| i.query("gadgets", 10).into_iter().map(|h| h.section).collect::<Vec<_>>();
    assert_eq!(order(&index), ["one", "two", "three", "four"]);
    assert_eq!(order(&Index::build(&pages)), order(&index));
}

#[test]
fn plain_word_alternatives_are_in_one_table_and_two_way() {
    let index = Index::build(&fixture());
    for q in ["wireless", "wifi", "wi-fi"] {
        assert_eq!(index.query(q, 1)[0].section, "wifi", "{q}");
    }
    for q in ["nas", "network drive", "shared folder", "file server"] {
        assert_eq!(index.query(q, 1)[0].section, "nas-open", "{q}");
    }
}

#[test]
fn building_and_asking_over_2000_sections_is_quick() {
    let mut seed = 12345u64;
    let mut next = move || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (seed >> 33) as usize
    };
    let sections: Vec<Section> = (0..2000)
        .map(|i| {
            let text: Vec<String> = (0..60).map(|_| format!("word{}", next() % 4000)).collect();
            sec(&format!("s{i}"), &format!("Heading word{} word{}", next() % 4000, next() % 4000), &text.join(" "), &["How do I word7?"])
        })
        .collect();
    let pages = vec![page("big", "Big", sections)];
    // The time this thread itself spent, so other work on a busy computer
    // does not count; an unoptimised build is given three times as long.
    let spent = || {
        let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        // SAFETY: one timespec, owned here, for the kernel to fill.
        unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut t) };
        Duration::new(t.tv_sec as u64, t.tv_nsec as u32)
    };
    let slack = if cfg!(debug_assertions) { 3 } else { 1 };
    let t = spent();
    let index = Index::build(&pages);
    let build = spent() - t;
    let t = spent();
    let hits = index.query("how do I fix word123 and word2000 please", 10);
    let ask = spent() - t;
    eprintln!("build {build:?}, query {ask:?}");
    assert!(!hits.is_empty());
    assert!(build < Duration::from_millis(1000) * slack, "build {build:?}");
    assert!(ask < Duration::from_millis(250) * slack, "query {ask:?}");
}
