//! Lists the English word files into the program, and puts every page of the help (site/content/*.md) and its pictures into the program, so the website and the program share one copy of the writing.

use std::path::Path;

fn main() {
    trindle_common::build_support::write_english_list();
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let pages = Path::new(&root).join("site/content");
    println!("cargo:rerun-if-changed=site/content");
    let mut files: Vec<String> = std::fs::read_dir(&pages)
        .map(|r| r.flatten().filter_map(|e| e.file_name().into_string().ok()).filter(|n| n.ends_with(".md")).collect())
        .unwrap_or_default();
    files.sort();
    // Each page: a record separator, its file name (the page's id), a line end, its text. src/help/pages.rs reads it.
    let mut data = String::new();
    for f in &files {
        let text = std::fs::read_to_string(pages.join(f)).unwrap_or_default();
        data.push('\u{1e}');
        data.push_str(f.trim_end_matches(".md"));
        data.push('\n');
        data.push_str(&text);
    }
    let out = std::env::var("OUT_DIR").unwrap();
    std::fs::write(Path::new(&out).join("help.pages"), data).unwrap();
    // The pictures: a table of each picture's file name and its bytes, which src/help/pages.rs includes.
    let img = pages.join("img");
    let mut names: Vec<String> = std::fs::read_dir(&img)
        .map(|r| r.flatten().filter_map(|e| e.file_name().into_string().ok()).filter(|n| (n.ends_with(".png") || n.ends_with(".svg")) && !n.starts_with("errors-")).collect())
        .unwrap_or_default();
    names.sort();
    let mut table = String::from("&[\n");
    for n in &names {
        table.push_str(&format!("    ({n:?}, include_bytes!({:?})),\n", img.join(n).to_string_lossy()));
    }
    table.push(']');
    std::fs::write(Path::new(&out).join("help.images"), table).unwrap();
}
