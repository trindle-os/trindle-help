//! The help's pages: site/content/*.md, put into one data file by build.rs
//! and read here into pages, sections and blocks. The website is built from
//! the same files (tools/site/docs.rs), so there is one copy of the writing.
//!
//! A section is the text under a `##` or `###` heading; the words before the
//! first heading are the page's top section (`TOP`). A line `Questions: a; b`
//! under a heading lists the questions that section answers, for the search;
//! it is never shown.

use super::search;

/// The data file made by build.rs: each page starts with a record
/// separator, then its file name, a line end, and its text.
const DATA: &str = include_str!(concat!(env!("OUT_DIR"), "/help.pages"));

/// The pictures of the pages, each by its file name in `site/content/img`, as build.rs lists them.
static IMAGES: &[(&str, &[u8])] = include!(concat!(env!("OUT_DIR"), "/help.images"));

/// The picture a page names, as the file holds it.
pub fn image_bytes(name: &str) -> Option<&'static [u8]> {
    IMAGES.iter().find(|(n, _)| *n == name).map(|(_, b)| *b)
}

/// The id of every page's first section, the words above its first heading.
pub const TOP: &str = "top";
/// The page the help opens on when no address is given.
pub const FIRST_PAGE: &str = "getting-started";

/// Where a link inside the writing leads.
#[derive(Clone, Debug, PartialEq)]
pub struct Link {
    pub words: String,
    pub page: String,
    pub section: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Paragraph,
    Bullets,
    Steps,
    Code,
    Note,
    /// A picture, shown as its caption for now.
    Picture,
}

/// A paragraph, list, code, note or picture, as plain lines, and the links
/// inside it.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub kind: Kind,
    pub lines: Vec<String>,
    pub links: Vec<Link>,
    /// For a picture: its file name in `img`, and what it shows, for a screen reader.
    pub picture: String,
    pub alt: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Part {
    pub id: String,
    pub heading: String,
    pub blocks: Vec<Block>,
    /// What is written for a more technical reader: shut to begin with and
    /// opened by a button (`:::expert` to `:::` in the writing).
    pub expert: Vec<Block>,
    pub questions: Vec<String>,
}

impl Part {
    /// The section's words as one plain text, for the search.
    pub fn text(&self) -> String {
        let mut out = Vec::new();
        for b in self.blocks.iter().chain(&self.expert) {
            out.push(b.lines.join(" "));
        }
        out.join(" ")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Doc {
    pub id: String,
    pub title: String,
    pub order: u32,
    /// The shelf the page stands on in the contents (Start, Using Trindle, Help).
    pub group: String,
    /// One sentence on what the page is for.
    pub summary: String,
    pub parts: Vec<Part>,
}

/// All the pages, in reading order.
pub fn all() -> Vec<Doc> {
    parse_all(DATA)
}

pub fn parse_all(data: &str) -> Vec<Doc> {
    let mut docs: Vec<Doc> = data.split('\u{1e}').filter(|c| !c.trim().is_empty()).filter_map(|chunk| {
        let (id, text) = chunk.split_once('\n')?;
        Some(parse_page(id.trim(), text))
    }).collect();
    docs.sort_by(|a, b| (a.order, &a.id).cmp(&(b.order, &b.id)));
    docs
}

/// The same pages, as the search takes them.
pub fn for_search(docs: &[Doc]) -> Vec<search::Page> {
    docs.iter().map(|d| search::Page {
        id: d.id.clone(),
        title: d.title.clone(),
        sections: d.parts.iter().map(|p| search::Section {
            id: p.id.clone(),
            heading: p.heading.clone(),
            text: p.text(),
            questions: p.questions.clone(),
        }).collect(),
    }).collect()
}

/// The same rule the website uses for a heading's address.
pub fn slug(s: &str) -> String {
    let mut out = String::new();
    let mut gap = false;
    for c in s.chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            if gap && !out.is_empty() {
                out.push('-');
            }
            gap = false;
            out.push(c);
        } else {
            gap = true;
        }
    }
    out
}

fn parse_page(id: &str, text: &str) -> Doc {
    let (title, order, group, summary, body) = front_matter(id, text);
    let mut reader = Reader {
        lines: body.lines().collect(),
        at: 0,
        parts: vec![Part { id: TOP.into(), heading: title.clone(), blocks: Vec::new(), expert: Vec::new(), questions: Vec::new() }],
        used: vec![TOP.to_string()],
        para: Vec::new(),
        expert_from: None,
    };
    reader.read();
    Doc { id: id.to_string(), title, order, group, summary, parts: reader.parts }
}

/// The page's title, order, shelf and summary from the lines between `---` at its head, and the text after them.
fn front_matter<'a>(id: &str, text: &'a str) -> (String, u32, String, String, &'a str) {
    let (mut title, mut order, mut body) = (id.to_string(), 100, text);
    let (mut group, mut summary) = (String::new(), String::new());
    if let Some(rest) = text.strip_prefix("---\n") {
        if let Some((front, after)) = rest.split_once("\n---\n") {
            for line in front.lines() {
                match line.split_once(':') {
                    Some(("title", v)) => title = v.trim().to_string(),
                    Some(("order", v)) => order = v.trim().parse().unwrap_or(100),
                    Some(("group", v)) => group = v.trim().to_string(),
                    Some(("summary", v)) => summary = v.trim().to_string(),
                    _ => {}
                }
            }
            body = after;
        }
    }
    (title, order, group, summary, body)
}

/// A list line's text, and whether the list is numbered.
fn list_item(l: &str) -> Option<(bool, String)> {
    if let Some(r) = l.strip_prefix("- ") {
        return Some((false, r.to_string()));
    }
    let digits = l.chars().take_while(char::is_ascii_digit).count();
    if digits > 0 {
        if let Some(r) = l[digits..].strip_prefix(". ") {
            return Some((true, r.to_string()));
        }
    }
    None
}

/// A page's lines being read into sections, one block at a time.
struct Reader<'a> {
    lines: Vec<&'a str>,
    at: usize,
    parts: Vec<Part>,
    used: Vec<String>,
    /// The lines of the paragraph being gathered.
    para: Vec<String>,
    /// Where the blocks of an expert area begin in the section's blocks.
    expert_from: Option<usize>,
}

impl Reader<'_> {
    fn read(&mut self) {
        while self.at < self.lines.len() {
            let line = self.lines[self.at];
            if line.trim().is_empty() {
                self.flush();
                self.at += 1;
            } else if let Some(q) = line.strip_prefix("Questions:") {
                self.flush();
                let last = self.parts.last_mut().unwrap();
                last.questions.extend(q.split(';').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()));
                self.at += 1;
            } else if line.trim_end() == ":::expert" {
                self.flush();
                self.expert_from = Some(self.parts.last().unwrap().blocks.len());
                self.at += 1;
            } else if line.trim_end() == ":::" {
                self.flush();
                self.end_expert();
                self.at += 1;
            } else if line.starts_with("<!--") {
                self.flush();
                self.at += 1;
            } else if line.starts_with("```") {
                self.flush();
                self.code();
            } else if let Some(h) = line.strip_prefix("## ").or_else(|| line.strip_prefix("### ")) {
                self.flush();
                self.heading(h);
            } else if let Some((ordered, _)) = list_item(line) {
                self.flush();
                self.list(ordered);
            } else if line.starts_with("> ") {
                self.flush();
                self.quote();
            } else {
                self.para.push(line.trim().to_string());
                self.at += 1;
            }
        }
        self.flush();
    }

    /// The paragraph gathered so far, as a block: a picture when it is one.
    fn flush(&mut self) {
        if self.para.is_empty() {
            return;
        }
        let text = self.para.join(" ");
        self.para.clear();
        let block = if let Some(rest) = text.strip_prefix("![") {
            // ![alt](path "caption")
            let alt = rest.split("](").next().unwrap_or("");
            let caption = rest.split('"').nth(1).filter(|c| !c.is_empty()).unwrap_or(alt);
            let file = rest.split("](").nth(1).and_then(|r| r.split(['"', ')']).next()).unwrap_or("").trim().rsplit('/').next().unwrap_or("").to_string();
            Block { kind: Kind::Picture, lines: vec![plain(caption).0], links: Vec::new(), picture: file, alt: plain(alt).0 }
        } else {
            let (t, links) = plain(&text);
            Block { kind: Kind::Paragraph, lines: vec![t], links, picture: String::new(), alt: String::new() }
        };
        self.parts.last_mut().unwrap().blocks.push(block);
    }

    /// The end of an expert area: the blocks since its start move to the section's expert blocks.
    fn end_expert(&mut self) {
        if let Some(from) = self.expert_from.take() {
            let last = self.parts.last_mut().unwrap();
            let held = last.blocks.split_off(from);
            last.expert.extend(held);
        }
    }

    /// A fenced block of code, to its closing fence.
    fn code(&mut self) {
        self.at += 1;
        let mut code = Vec::new();
        while self.at < self.lines.len() && !self.lines[self.at].starts_with("```") {
            code.push(self.lines[self.at].to_string());
            self.at += 1;
        }
        self.at += 1;
        self.parts.last_mut().unwrap().blocks.push(Block { kind: Kind::Code, lines: code, links: Vec::new(), picture: String::new(), alt: String::new() });
    }

    /// A new section, with an address no other section of the page has.
    fn heading(&mut self, h: &str) {
        let heading = plain(h.trim()).0;
        let base = slug(h.trim());
        let mut sid = base.clone();
        let mut n = 2;
        while self.used.contains(&sid) {
            sid = format!("{base}-{n}");
            n += 1;
        }
        self.used.push(sid.clone());
        self.parts.push(Part { id: sid, heading, blocks: Vec::new(), expert: Vec::new(), questions: Vec::new() });
        self.at += 1;
    }

    /// A bulleted or numbered list, its indented lines joined to the item above.
    fn list(&mut self, ordered: bool) {
        let mut items: Vec<String> = Vec::new();
        while self.at < self.lines.len() {
            if let Some((_, item)) = list_item(self.lines[self.at]) {
                items.push(item);
            } else if self.lines[self.at].starts_with("  ") && !items.is_empty() {
                let last = items.last_mut().unwrap();
                last.push(' ');
                last.push_str(self.lines[self.at].trim());
            } else {
                break;
            }
            self.at += 1;
        }
        let mut links = Vec::new();
        let lines = items.iter().map(|t| {
            let (t, l) = plain(t);
            links.extend(l);
            t
        }).collect();
        self.parts.last_mut().unwrap().blocks.push(Block { kind: if ordered { Kind::Steps } else { Kind::Bullets }, lines, links, picture: String::new(), alt: String::new() });
    }

    /// A quoted note.
    fn quote(&mut self) {
        let mut quote = Vec::new();
        while self.at < self.lines.len() && self.lines[self.at].starts_with("> ") {
            quote.push(self.lines[self.at][2..].to_string());
            self.at += 1;
        }
        let (t, links) = plain(&quote.join(" "));
        self.parts.last_mut().unwrap().blocks.push(Block { kind: Kind::Note, lines: vec![t], links, picture: String::new(), alt: String::new() });
    }
}

/// A line of the writing as plain words, and the links in it that lead to
/// another page or section. Marks for bold, italic and code are dropped; a
/// link to the web is kept as its words only (the help works offline).
pub fn plain(s: &str) -> (String, Vec<Link>) {
    let mut out = String::new();
    let mut links = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' {
            if let Some(close) = chars[i..].iter().position(|&c| c == ']').map(|p| p + i) {
                if chars.get(close + 1) == Some(&'(') {
                    if let Some(end) = chars[close..].iter().position(|&c| c == ')').map(|p| p + close) {
                        let words: String = plain(&chars[i + 1..close].iter().collect::<String>()).0;
                        let target: String = chars[close + 2..end].iter().collect();
                        if let Some(link) = link_to(&words, &target) {
                            links.push(link);
                        }
                        out.push_str(&words);
                        i = end + 1;
                        continue;
                    }
                }
            }
        }
        if chars[i] == '*' || chars[i] == '`' {
            i += 1;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    (out, links)
}

fn link_to(words: &str, target: &str) -> Option<Link> {
    let (file, section) = match target.split_once('#') {
        Some((f, s)) => (f, Some(s.to_string())),
        None => (target, None),
    };
    let page = file.strip_suffix(".html")?;
    if page.is_empty() || page.contains('/') || page.contains(':') {
        return None;
    }
    Some(Link { words: words.into(), page: page.into(), section: section.filter(|s| !s.is_empty()) })
}
