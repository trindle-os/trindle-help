// Builds Trindle's documentation website: site/content/*.md and
// Trindle's features list (--features FILE, or TRINDLE_FEATURES; default spec/features.md) in, plain static files out in site/dist/.
//
// Build and run, from the repository root:
//     rustc --edition 2021 -O tools/site/docs.rs -o target/site-docs
//     target/site-docs            build site/dist/
//     target/site-docs --check    build, then run the checks (below)
//     target/site-docs --features ../trindle/spec/features.md   say where the features list is
//
// Standard library only: no crates, no downloads, no tracking, no cookies,
// no outside fonts or scripts. Every page is a plain file the host can serve
// as it is, and the pages work from a folder on disk too (all links are
// relative).
//
// The checks fail the build for: a banned word, an em dash, a picture with no
// alt text, a link to a page or picture that is not there, a heading not in
// sentence case, a pair of colours below WCAG 2.2 AA contrast.
//
// The small pattern matcher at the top is here because the standard library
// has none; it understands only what the patterns below use.
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

// ------------------------------------------------------------ pattern matcher

#[derive(Clone)]
enum Item {
    Range(char, char),
    Space(bool),
    Digit(bool),
    Word(bool),
}

enum Node {
    Char(char),
    Any,
    Class(Vec<Item>, bool),
    Start,
    End,
    WordB,
    Group(Box<Node>, Option<usize>),
    Alt(Vec<Node>),
    Cat(Vec<Node>),
    Rep(Box<Node>, usize, usize, bool),
    Look(Box<Node>, bool, bool),
    Back(usize),
}

type Caps = Vec<Option<(usize, usize)>>;
type Cont<'a> = &'a mut dyn FnMut(usize, &mut Caps) -> bool;

struct Parser {
    c: Vec<char>,
    i: usize,
    groups: usize,
}

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.c.get(self.i).copied()
    }
    fn starts(&self, s: &str) -> bool {
        s.chars().enumerate().all(|(k, ch)| self.c.get(self.i + k) == Some(&ch))
    }
    fn alt(&mut self) -> Node {
        let mut alts = vec![self.cat()];
        while self.peek() == Some('|') {
            self.i += 1;
            alts.push(self.cat());
        }
        if alts.len() == 1 { alts.pop().unwrap() } else { Node::Alt(alts) }
    }
    fn cat(&mut self) -> Node {
        let mut v = Vec::new();
        while let Some(ch) = self.peek() {
            if ch == '|' || ch == ')' {
                break;
            }
            let a = self.atom();
            v.push(self.quant(a));
        }
        Node::Cat(v)
    }
    fn number(&mut self) -> Option<usize> {
        let s = self.i;
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.i += 1;
        }
        if s == self.i { None } else { self.c[s..self.i].iter().collect::<String>().parse().ok() }
    }
    fn quant(&mut self, a: Node) -> Node {
        let (min, max) = match self.peek() {
            Some('*') => { self.i += 1; (0, usize::MAX) }
            Some('+') => { self.i += 1; (1, usize::MAX) }
            Some('?') => { self.i += 1; (0, 1) }
            Some('{') => {
                self.i += 1;
                let lo = self.number().expect("a number after {");
                let hi = if self.peek() == Some(',') {
                    self.i += 1;
                    self.number().unwrap_or(usize::MAX)
                } else {
                    lo
                };
                assert_eq!(self.peek(), Some('}'));
                self.i += 1;
                (lo, hi)
            }
            _ => return a,
        };
        let greedy = if self.peek() == Some('?') { self.i += 1; false } else { true };
        Node::Rep(Box::new(a), min, max, greedy)
    }
    fn hex(&mut self, n: usize) -> char {
        let s: String = self.c[self.i..self.i + n].iter().collect();
        self.i += n;
        char::from_u32(u32::from_str_radix(&s, 16).unwrap()).unwrap()
    }
    // One escaped thing after a backslash: either a class or a character.
    fn escape(&mut self) -> Result<char, Item> {
        let c = self.c[self.i];
        self.i += 1;
        match c {
            's' => Err(Item::Space(false)),
            'S' => Err(Item::Space(true)),
            'd' => Err(Item::Digit(false)),
            'D' => Err(Item::Digit(true)),
            'w' => Err(Item::Word(false)),
            'W' => Err(Item::Word(true)),
            'n' => Ok('\n'),
            't' => Ok('\t'),
            'x' => Ok(self.hex(2)),
            'U' => Ok(self.hex(8)),
            other => Ok(other),
        }
    }
    fn class(&mut self) -> Node {
        let neg = if self.peek() == Some('^') { self.i += 1; true } else { false };
        let mut items = Vec::new();
        while self.peek() != Some(']') {
            let lo = if self.peek() == Some('\\') {
                self.i += 1;
                match self.escape() {
                    Ok(c) => c,
                    Err(item) => { items.push(item); continue; }
                }
            } else {
                let c = self.c[self.i];
                self.i += 1;
                c
            };
            if self.peek() == Some('-') && self.c.get(self.i + 1) != Some(&']') {
                self.i += 1;
                let hi = if self.peek() == Some('\\') {
                    self.i += 1;
                    self.escape().ok().expect("a character ends a range")
                } else {
                    let c = self.c[self.i];
                    self.i += 1;
                    c
                };
                items.push(Item::Range(lo, hi));
            } else {
                items.push(Item::Range(lo, lo));
            }
        }
        self.i += 1;
        Node::Class(items, neg)
    }
    fn atom(&mut self) -> Node {
        let c = self.c[self.i];
        self.i += 1;
        match c {
            '(' => {
                let (cap, look) = if self.starts("?:") {
                    self.i += 2;
                    (false, None)
                } else if self.starts("?!") {
                    self.i += 2;
                    (false, Some((true, true)))
                } else if self.starts("?=") {
                    self.i += 2;
                    (false, Some((true, false)))
                } else if self.starts("?<!") {
                    self.i += 3;
                    (false, Some((false, true)))
                } else if self.starts("?<=") {
                    self.i += 3;
                    (false, Some((false, false)))
                } else {
                    (true, None)
                };
                let idx = if cap { self.groups += 1; Some(self.groups) } else { None };
                let inner = self.alt();
                assert_eq!(self.peek(), Some(')'));
                self.i += 1;
                match look {
                    Some((ahead, neg)) => Node::Look(Box::new(inner), ahead, neg),
                    None => Node::Group(Box::new(inner), idx),
                }
            }
            '[' => self.class(),
            '.' => Node::Any,
            '^' => Node::Start,
            '$' => Node::End,
            '\\' => {
                if self.peek().map_or(false, |d| d.is_ascii_digit() && d != '0') {
                    let n = self.number().unwrap();
                    return Node::Back(n);
                }
                if self.peek() == Some('b') {
                    self.i += 1;
                    return Node::WordB;
                }
                match self.escape() {
                    Ok(ch) => Node::Char(ch),
                    Err(item) => Node::Class(vec![item], false),
                }
            }
            other => Node::Char(other),
        }
    }
}

struct Re {
    node: Node,
    groups: usize,
    dotall: bool,
    icase: bool,
}

struct M {
    start: usize,
    end: usize,
    g: Vec<Option<String>>,
}

impl M {
    fn grp(&self, i: usize) -> &str {
        self.g[i].as_deref().unwrap_or("")
    }
    fn has(&self, i: usize) -> bool {
        self.g[i].is_some()
    }
}

impl Re {
    fn new(pat: &str, dotall: bool, icase: bool) -> Re {
        let mut p = Parser { c: pat.chars().collect(), i: 0, groups: 0 };
        let node = p.alt();
        assert!(p.i == p.c.len(), "bad pattern {}", pat);
        Re { node, groups: p.groups, dotall, icase }
    }

    fn item_hit(&self, it: &Item, ch: char) -> bool {
        match it {
            Item::Range(lo, hi) => {
                if *lo <= ch && ch <= *hi {
                    return true;
                }
                if self.icase {
                    let l = ch.to_lowercase().next().unwrap_or(ch);
                    let u = ch.to_uppercase().next().unwrap_or(ch);
                    return (*lo <= l && l <= *hi) || (*lo <= u && u <= *hi);
                }
                false
            }
            Item::Space(n) => ch.is_whitespace() != *n,
            Item::Digit(n) => ch.is_ascii_digit() != *n,
            Item::Word(n) => is_word(ch) != *n,
        }
    }

    fn single(&self, n: &Node, ch: char) -> bool {
        match n {
            Node::Char(c) => {
                *c == ch || (self.icase && c.to_lowercase().eq(ch.to_lowercase()))
            }
            Node::Any => self.dotall || ch != '\n',
            Node::Class(items, neg) => items.iter().any(|it| self.item_hit(it, ch)) != *neg,
            _ => false,
        }
    }

    fn m(&self, n: &Node, t: &[char], pos: usize, caps: &mut Caps, k: Cont) -> bool {
        match n {
            Node::Char(_) | Node::Any | Node::Class(..) => {
                pos < t.len() && self.single(n, t[pos]) && k(pos + 1, caps)
            }
            Node::Start => pos == 0 && k(pos, caps),
            Node::End => (pos == t.len() || (pos + 1 == t.len() && t[pos] == '\n')) && k(pos, caps),
            Node::WordB => {
                let before = pos > 0 && is_word(t[pos - 1]);
                let after = pos < t.len() && is_word(t[pos]);
                before != after && k(pos, caps)
            }
            Node::Cat(v) => self.seq(v, t, pos, caps, k),
            Node::Alt(v) => {
                for a in v {
                    if self.m(a, t, pos, caps, k) {
                        return true;
                    }
                }
                false
            }
            Node::Group(inner, idx) => match idx {
                None => self.m(inner, t, pos, caps, k),
                Some(i) => {
                    let i = *i;
                    self.m(inner, t, pos, caps, &mut |p, c| {
                        let old = c[i];
                        c[i] = Some((pos, p));
                        if k(p, c) {
                            true
                        } else {
                            c[i] = old;
                            false
                        }
                    })
                }
            },
            Node::Rep(inner, min, max, greedy) => {
                if matches!(**inner, Node::Char(_) | Node::Any | Node::Class(..)) {
                    let mut cnt = 0;
                    while cnt < *max && pos + cnt < t.len() && self.single(inner, t[pos + cnt]) {
                        cnt += 1;
                    }
                    if cnt < *min {
                        return false;
                    }
                    if *greedy {
                        let mut c = cnt;
                        loop {
                            if k(pos + c, caps) {
                                return true;
                            }
                            if c == *min {
                                return false;
                            }
                            c -= 1;
                        }
                    }
                    for c in *min..=cnt {
                        if k(pos + c, caps) {
                            return true;
                        }
                    }
                    return false;
                }
                self.rep(inner, *min, *max, *greedy, 0, t, pos, caps, k)
            }
            Node::Look(inner, ahead, neg) => {
                let found = if *ahead {
                    self.m(inner, t, pos, caps, &mut |_, _| true)
                } else {
                    (0..=pos).rev().any(|s| self.m(inner, t, s, caps, &mut |p, _| p == pos))
                };
                found != *neg && k(pos, caps)
            }
            Node::Back(i) => match caps[*i] {
                Some((s, e)) => {
                    let len = e - s;
                    pos + len <= t.len() && t[s..e] == t[pos..pos + len] && k(pos + len, caps)
                }
                None => false,
            },
        }
    }

    fn seq(&self, v: &[Node], t: &[char], pos: usize, caps: &mut Caps, k: Cont) -> bool {
        if v.is_empty() {
            return k(pos, caps);
        }
        self.m(&v[0], t, pos, caps, &mut |p, c| self.seq(&v[1..], t, p, c, k))
    }

    #[allow(clippy::too_many_arguments)]
    fn rep(&self, inner: &Node, min: usize, max: usize, greedy: bool, count: usize, t: &[char], pos: usize, caps: &mut Caps, k: Cont) -> bool {
        if greedy {
            if count < max {
                let hit = self.m(inner, t, pos, caps, &mut |p, c| {
                    p != pos && self.rep(inner, min, max, greedy, count + 1, t, p, c, k)
                });
                if hit {
                    return true;
                }
            }
            count >= min && k(pos, caps)
        } else {
            if count >= min && k(pos, caps) {
                return true;
            }
            count < max
                && self.m(inner, t, pos, caps, &mut |p, c| {
                    p != pos && self.rep(inner, min, max, greedy, count + 1, t, p, c, k)
                })
        }
    }

    fn exec(&self, t: &[char], start: usize) -> Option<Caps> {
        let mut caps: Caps = vec![None; self.groups + 1];
        let mut end = None;
        if self.m(&self.node, t, start, &mut caps, &mut |p, _| {
            end = Some(p);
            true
        }) {
            caps[0] = Some((start, end.unwrap()));
            Some(caps)
        } else {
            None
        }
    }

    fn make(&self, offs: &[usize], s: &str, caps: &Caps) -> M {
        let (a, b) = caps[0].unwrap();
        M {
            start: offs[a],
            end: offs[b],
            g: caps.iter().map(|c| c.map(|(x, y)| s[offs[x]..offs[y]].to_string())).collect(),
        }
    }

    // Where each character starts, plus the end, so character places can be
    // turned into places in the string.
    fn prep(s: &str) -> (Vec<char>, Vec<usize>) {
        let mut offs: Vec<usize> = s.char_indices().map(|(i, _)| i).collect();
        offs.push(s.len());
        (s.chars().collect(), offs)
    }

    fn search(&self, s: &str) -> Option<M> {
        let (t, offs) = Re::prep(s);
        (0..=t.len()).find_map(|st| self.exec(&t, st)).map(|c| self.make(&offs, s, &c))
    }

    // Like Python's re.match: only at the very start.
    fn at_start(&self, s: &str) -> Option<M> {
        let (t, offs) = Re::prep(s);
        self.exec(&t, 0).map(|c| self.make(&offs, s, &c))
    }

    fn is_match(&self, s: &str) -> bool {
        self.search(s).is_some()
    }

    fn find_all(&self, s: &str) -> Vec<M> {
        let (t, offs) = Re::prep(s);
        let mut out = Vec::new();
        let mut st = 0;
        while st <= t.len() {
            match (st..=t.len()).find_map(|x| self.exec(&t, x)) {
                Some(c) => {
                    let (a, b) = c[0].unwrap();
                    out.push(self.make(&offs, s, &c));
                    st = if b == a { b + 1 } else { b };
                }
                None => break,
            }
        }
        out
    }

    fn sub(&self, s: &str, f: &mut dyn FnMut(&M) -> String) -> String {
        let mut out = String::new();
        let mut last = 0;
        for m in self.find_all(s) {
            out.push_str(&s[last..m.start]);
            out.push_str(&f(&m));
            last = m.end;
        }
        out.push_str(&s[last..]);
        out
    }

    fn sub_str(&self, s: &str, with: &str) -> String {
        self.sub(s, &mut |_| with.to_string())
    }

    fn split(&self, s: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut last = 0;
        for m in self.find_all(s) {
            out.push(s[last..m.start].to_string());
            last = m.end;
        }
        out.push(s[last..].to_string());
        out
    }
}

thread_local! {
    static CACHE: RefCell<HashMap<(String, bool, bool), Rc<Re>>> = RefCell::new(HashMap::new());
}

fn cached(p: &str, dotall: bool, icase: bool) -> Rc<Re> {
    CACHE.with(|c| {
        c.borrow_mut()
            .entry((p.to_string(), dotall, icase))
            .or_insert_with(|| Rc::new(Re::new(p, dotall, icase)))
            .clone()
    })
}

fn re(p: &str) -> Rc<Re> {
    cached(p, false, false)
}

// With "." matching a new line as well.
fn res(p: &str) -> Rc<Re> {
    cached(p, true, false)
}

// Ignoring capitals.
fn rei(p: &str) -> Rc<Re> {
    cached(p, false, true)
}

// ------------------------------------------------------------------ settings

const SITE: &str = "Trindle help";

// Words that keep a capital inside a heading: Trindle's own part names and
// the proper names the pages use.
const CAPITALS: &[&str] = &[
    "Trindle", "Apps", "Folders", "Settings", "Text", "Flow", "Drives", "Flathub",
    "Flatpak", "AppImage", "AppImages", "Super", "Wi-Fi", "Bluetooth", "Pocketkey",
    "Welcome", "Tailscale", "Btrfs", "Ctrl", "Alt", "Shift", "Caps", "Lock", "Esc",
    "TPM", "PIN", "USB", "HTML", "PDF", "Markdown", "Windows", "Linux", "Secure",
    "Boot", "Plugins", "Installed", "Updates", "Discover", "Single", "Dual",
    "Thunderbolt", "Samba", "Save", "Copy", "Cancel", "Install", "Remove", "Allow", "Keep", "GitHub",
];

// The colours of the site, one place. Pairs below are checked against each
// other for contrast, so a colour change that makes words hard to read stops
// the build.
const TOKENS: [(&str, &str); 13] = [
    ("bg", "#090d14"),
    ("surface", "#101825"),
    ("surface-2", "#16223a"),
    ("edge", "#2c3b58"),
    ("field-edge", "#7f8fb0"),
    ("text", "#e8edf5"),
    ("muted", "#b0bace"),
    ("accent", "#ffc61a"),
    ("accent-ink", "#14100a"),
    ("focus", "#ffe07a"),
    ("mark-bg", "#4a3b0a"),
    ("mark-text", "#fff3c4"),
    ("code-bg", "#0c1220"),
];

const PAIRS: [(&str, &str, f64); 21] = [
    ("text", "bg", 4.5), ("text", "surface", 4.5), ("text", "surface-2", 4.5),
    ("muted", "bg", 4.5), ("muted", "surface", 4.5), ("muted", "surface-2", 4.5),
    ("accent", "bg", 4.5), ("accent", "surface", 4.5), ("accent", "surface-2", 4.5),
    ("accent-ink", "accent", 4.5), ("accent-ink", "focus", 4.5), ("mark-text", "mark-bg", 4.5),
    ("text", "code-bg", 4.5), ("focus", "bg", 3.0), ("focus", "surface", 3.0),
    ("muted", "code-bg", 4.5), ("accent", "code-bg", 4.5),
    ("focus", "surface-2", 3.0), ("focus", "code-bg", 3.0),
    ("field-edge", "bg", 3.0), ("field-edge", "surface", 3.0),
];

thread_local! {
    // The language being built: its code for the page, and its words for the
    // page's frame, read from CODE/docs/chrome.txt ("English = translation" a
    // line); any not there stay English.
    static CHROME: RefCell<(String, HashMap<String, String>)> = RefCell::new(("en-GB".into(), HashMap::new()));
}

// The page language for a language's folder name.
fn page_lang(code: &str) -> String {
    if code.is_empty() { "en-GB".into() } else { code.replace('_', "-") }
}

// Written right to left.
fn rtl(lang: &str) -> bool {
    matches!(lang.split('-').next().unwrap_or(""), "ar" | "ur" | "fa" | "he" | "ps" | "sd" | "ug" | "yi" | "dv" | "ku")
}

// The frame's word in the language being built.
fn word(english: &str) -> String {
    CHROME.with(|c| c.borrow().1.get(english).cloned().unwrap_or_else(|| english.to_string()))
}

fn read_chrome(text: &str) -> HashMap<String, String> {
    text.lines().filter(|l| !l.trim_start().starts_with('#'))
        .filter_map(|l| l.split_once(" = ")).map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
        .filter(|(a, b)| !a.is_empty() && !b.is_empty()).collect()
}

const GROUPS: [&str; 4] = ["Start", "Using Trindle", "Help", "Reference"];

struct Paths {
    root: PathBuf,
    content: PathBuf,
    dist: PathBuf,
    features: PathBuf,
    statics: PathBuf,
    // A language's docs folder (CODE/docs); None for the English site.
    overlay: Option<PathBuf>,
}

// ------------------------------------------------------------------ helpers

fn read(p: &Path) -> String {
    let text = fs::read_to_string(p).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {e}", p.display());
        std::process::exit(2)
    });
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn write(p: &Path, text: &str) {
    fs::write(p, text).unwrap_or_else(|e| {
        eprintln!("cannot write {}: {e}", p.display());
        std::process::exit(2)
    });
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            '\'' => o.push_str("&#x27;"),
            _ => o.push(c),
        }
    }
    o
}

// The few entities esc() makes, and numbered ones.
fn unescape(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        o.push_str(&rest[..i]);
        rest = &rest[i..];
        let end = rest.find(';').filter(|e| *e <= 10);
        let decoded = end.and_then(|e| {
            let name = &rest[1..e];
            let ch = match name {
                "amp" => Some('&'),
                "lt" => Some('<'),
                "gt" => Some('>'),
                "quot" => Some('"'),
                "apos" => Some('\''),
                "nbsp" => Some('\u{a0}'),
                _ => name
                    .strip_prefix("#x")
                    .and_then(|h| u32::from_str_radix(h, 16).ok())
                    .or_else(|| name.strip_prefix('#').and_then(|d| d.parse().ok()))
                    .and_then(char::from_u32),
            };
            ch.map(|c| (c, e))
        });
        match decoded {
            Some((c, e)) => {
                o.push(c);
                rest = &rest[e + 1..];
            }
            None => {
                o.push('&');
                rest = &rest[1..];
            }
        }
    }
    o.push_str(rest);
    o
}

fn strip_tags(s: &str, with: &str) -> String {
    re(r"<[^>]+>").sub_str(s, with)
}

fn slug(s: &str) -> String {
    let s = strip_tags(s, "").to_lowercase();
    let s = re(r"[^a-z0-9]+").sub_str(&s, "-");
    let s = s.trim_matches('-');
    if s.is_empty() { "section".to_string() } else { s.to_string() }
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn png_size(path: &Path) -> (u32, u32) {
    if path.to_string_lossy().ends_with(".svg") {
        let m = re(r#"viewBox="0 0 (\d+) (\d+)""#).search(&read(path)).expect("an SVG with a viewBox");
        return (m.grp(1).parse().unwrap(), m.grp(2).parse().unwrap());
    }
    let bytes = fs::read(path).expect("a picture that can be read");
    let be = |a: usize| u32::from_be_bytes([bytes[a], bytes[a + 1], bytes[a + 2], bytes[a + 3]]);
    (be(16), be(20))
}

// ----------------------------------------------------------------- markdown

fn inline(text: &str, sentence: bool) -> String {
    let mut keep: Vec<String> = Vec::new();
    let text = re(r"`([^`]+)`").sub(text, &mut |m| {
        keep.push(format!("<code>{}</code>", esc(m.grp(1))));
        format!("\u{0}{}\u{0}", keep.len() - 1)
    });
    let text = esc(&text);
    let text = re(r"\[([^\]]+)\]\(([^)\s]+)\)").sub(&text, &mut |m| {
        format!("<a{} href=\"{}\">{}</a>", if sentence { " class=\"in\"" } else { "" }, m.grp(2), m.grp(1))
    });
    let text = re(r"\*\*([^*]+)\*\*").sub(&text, &mut |m| format!("<strong>{}</strong>", m.grp(1)));
    let text = re(r"(?<![\w*])\*([^*\n]+)\*(?![\w*])").sub(&text, &mut |m| format!("<em>{}</em>", m.grp(1)));
    re(r"\x00(\d+)\x00").sub(&text, &mut |m| keep[m.grp(1).parse::<usize>().unwrap()].clone())
}

// A list item that is only a link stands on its own line, with no underline;
// an optional description sits under it. Any other item is a sentence, so a
// link in it is marked as an in-sentence link.
fn item_html(t: &str) -> String {
    if let Some(m) = re(r"^\[([^\]]+)\]\(([^)\s]+)\)(?::\s+(.*))?$").at_start(t) {
        let why = if m.grp(3).is_empty() {
            String::new()
        } else {
            format!("<span class=\"why\">{}</span>", inline(m.grp(3), false))
        };
        return format!("<li class=\"link\"><a href=\"{}\">{}</a>{}</li>", m.grp(2), m.grp(1), why);
    }
    format!("<li>{}</li>", inline(t, true))
}

type Heads = Vec<(usize, String, String)>;

// The small Markdown the pages use. Returns the HTML and the headings.
fn markdown(p: &Paths, src: &str) -> (String, Heads) {
    let mut out: Vec<String> = Vec::new();
    let mut heads: Heads = Vec::new();
    let lines: Vec<&str> = src.split('\n').collect();
    let mut i = 0;
    let mut para: Vec<String> = Vec::new();
    let mut used: HashSet<String> = HashSet::new();
    let list_start = re(r"^(- |\d+\. )");

    fn flush(p: &Paths, para: &mut Vec<String>, out: &mut Vec<String>) {
        if para.is_empty() {
            return;
        }
        let text = para.join(" ");
        let text = text.trim();
        if let Some(m) = re(r#"^!\[([^\]]*)\]\(([^)\s]+)(?:\s+"([^"]*)")?\)$"#).at_start(text) {
            let path = m.grp(2);
            let file = p.content.join(path);
            let (w, h) = if file.exists() { png_size(&file) } else { (1, 1) };
            let mut fig = format!(
                "<figure><img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\" loading=\"lazy\">",
                esc(path), esc(m.grp(1)), w, h
            );
            if m.has(3) {
                if !m.grp(3).is_empty() {
                    fig.push_str(&format!("<figcaption>{}</figcaption>", inline(m.grp(3), false)));
                }
            }
            out.push(fig + "</figure>");
        } else {
            out.push(format!("<p>{}</p>", inline(text, true)));
        }
        para.clear();
    }

    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            flush(p, &mut para, &mut out);
            i += 1;
            continue;
        }
        // The questions a section answers are for the search (the Help
        // program and the website's), never shown.
        if line.starts_with("Questions:") {
            flush(p, &mut para, &mut out);
            i += 1;
            continue;
        }
        // The area for a more technical reader. In the Help program it is
        // shut until a button opens it; on the website it is always shown,
        // under its own title, because only Help may fold.
        if line.trim_end() == ":::expert" {
            flush(p, &mut para, &mut out);
            out.push("<div class=\"expert\"><p class=\"expert-title\"><strong>More detail for experts</strong></p>".to_string());
            i += 1;
            continue;
        }
        if line.trim_end() == ":::" {
            flush(p, &mut para, &mut out);
            out.push("</div>".to_string());
            i += 1;
            continue;
        }
        if line.starts_with("<!--") {
            flush(p, &mut para, &mut out);
            out.push(line.to_string());
            i += 1;
            continue;
        }
        if line.starts_with("```") {
            flush(p, &mut para, &mut out);
            i += 1;
            let mut code: Vec<&str> = Vec::new();
            while i < lines.len() && !lines[i].starts_with("```") {
                code.push(lines[i]);
                i += 1;
            }
            i += 1;
            out.push(format!("<pre tabindex=\"0\"><code>{}</code></pre>", esc(&code.join("\n"))));
            continue;
        }
        if let Some(m) = re(r"^(#{2,3}) (.+)$").at_start(line) {
            flush(p, &mut para, &mut out);
            let level = m.grp(1).len();
            let text = m.grp(2).trim().to_string();
            let mut sid = slug(&text);
            let mut n = 2;
            while used.contains(&sid) {
                sid = format!("{}-{}", slug(&text), n);
                n += 1;
            }
            used.insert(sid.clone());
            out.push(format!("<h{level} id=\"{sid}\">{}</h{level}>", inline(&text, false)));
            heads.push((level, sid, text));
            i += 1;
            continue;
        }
        if list_start.is_match_start(line) {
            flush(p, &mut para, &mut out);
            let ordered = re(r"^\d+\. ").at_start(line).is_some();
            let mut items: Vec<String> = Vec::new();
            while i < lines.len() && (list_start.is_match_start(lines[i]) || (lines[i].starts_with("  ") && !items.is_empty())) {
                if list_start.is_match_start(lines[i]) {
                    items.push(list_start.sub_str(lines[i], ""));
                } else {
                    let last = items.last_mut().unwrap();
                    last.push(' ');
                    last.push_str(lines[i].trim());
                }
                i += 1;
            }
            let tag = if ordered { "ol" } else { "ul" };
            let inner: String = items.iter().map(|t| item_html(t)).collect();
            out.push(format!("<{tag}>{inner}</{tag}>"));
            continue;
        }
        if line.starts_with("> ") {
            flush(p, &mut para, &mut out);
            let mut quote: Vec<&str> = Vec::new();
            while i < lines.len() && lines[i].starts_with("> ") {
                quote.push(&lines[i][2..]);
                i += 1;
            }
            out.push(format!("<aside class=\"note\">{}</aside>", inline(&quote.join(" "), true)));
            continue;
        }
        para.push(line.trim().to_string());
        i += 1;
    }
    flush(p, &mut para, &mut out);
    (out.join("\n"), heads)
}

impl Re {
    fn is_match_start(&self, s: &str) -> bool {
        self.at_start(s).is_some()
    }
}

fn front(text: &str) -> (HashMap<String, String>, String) {
    try_front(text).unwrap_or_else(|| {
        eprintln!("a content file has no front matter");
        std::process::exit(1)
    })
}

fn try_front(text: &str) -> Option<(HashMap<String, String>, String)> {
    let m = res(r"^---\n(.*?)\n---\n").at_start(text)?;
    let mut meta = HashMap::new();
    for ln in m.grp(1).split('\n') {
        let (k, v) = match ln.find(':') {
            Some(i) => (&ln[..i], &ln[i + 1..]),
            None => (ln, ""),
        };
        meta.insert(k.trim().to_string(), v.trim().to_string());
    }
    Some((meta, text[m.end..].to_string()))
}

// ----------------------------------------------------------------- features

// spec/features.md as a page: a section per group, a block per feature with
// its state said in words.
fn features_page(p: &Paths) -> (String, Heads, String) {
    let raw = read(&p.features);
    let blocks: Vec<String> = re(r"\n\s*\n")
        .split(&raw)
        .into_iter()
        .map(|b| b.trim().to_string())
        .filter(|b| !b.is_empty())
        .collect();
    let mut body: Vec<String> = Vec::new();
    let mut heads: Heads = Vec::new();
    let mut used: HashSet<String> = HashSet::new();
    let (mut now, mut built) = (0, 0);
    for b in &blocks {
        if b.starts_with("# ") {
            continue;
        }
        if let Some(rest) = b.strip_prefix("## ") {
            let text = rest.trim();
            let sid = slug(text);
            used.insert(sid.clone());
            body.push(format!("<h2 id=\"{}\">{}</h2>", sid, inline(text, false)));
            heads.push((2, sid, text.to_string()));
            continue;
        }
        if let Some(m) = res(r"^\*\*(.+?)\.\*\*\s*(.*)$").at_start(b) {
            let title = m.grp(1).to_string();
            let mut rest = m.grp(2).split_whitespace().collect::<Vec<_>>().join(" ");
            let st = re(r"\s*(In Trindle now|Being built)(?: \(([^)]*)\))?\.?\s*$").search(&rest);
            let (state, when) = match &st {
                Some(s) => (Some(s.grp(1).to_string()), if s.has(2) { s.grp(2).to_string() } else { String::new() }),
                None => (None, String::new()),
            };
            if let Some(s) = &st {
                rest = rest[..s.start].trim().to_string();
            }
            let mut sid = slug(&title);
            let mut n = 2;
            while used.contains(&sid) {
                sid = format!("{}-{}", slug(&title), n);
                n += 1;
            }
            used.insert(sid.clone());
            let mut label = String::new();
            if let Some(state) = &state {
                let is_now = state == "In Trindle now";
                if is_now { now += 1 } else { built += 1 }
                label = format!(
                    "<p class=\"state {}\"><span>{}</span>{}</p>",
                    if is_now { "now" } else { "soon" },
                    state,
                    if when.is_empty() { String::new() } else { format!(" {}", esc(&when)) }
                );
            }
            body.push(format!(
                "<section class=\"feature\"><h3 id=\"{}\">{}</h3>{}<p>{}</p></section>",
                sid, inline(&title, false), label, inline(&rest, true)
            ));
            heads.push((3, sid, title));
            continue;
        }
        body.push(markdown(p, b).0);
    }
    let summary = format!(
        "What sets Trindle apart, in plain words. {now} are in Trindle now and {built} are still being built; each one says which."
    );
    (body.join("\n"), heads, summary)
}

// -------------------------------------------------------------------- pages

struct Page {
    slug: String,
    title: String,
    summary: String,
    group: String,
    order: i64,
    body: String,
    heads: Heads,
    file: String,
}

impl Page {
    fn new(slug: &str, meta: &HashMap<String, String>, body: String, heads: Heads) -> Page {
        let get = |k: &str, d: &str| meta.get(k).cloned().unwrap_or_else(|| d.to_string());
        Page {
            slug: slug.to_string(),
            title: meta.get("title").cloned().expect("a title in the front matter"),
            summary: get("summary", ""),
            group: get("group", "Using Trindle"),
            order: get("order", "50").parse().expect("a number for order"),
            body,
            heads,
            file: if slug == "index" { "index.html".to_string() } else { format!("{slug}.html") },
        }
    }
}

// The English front matter with a translation's title and summary laid over
// it. Group and order stay English so no translation moves a page.
fn merge_meta(en: &HashMap<String, String>, tr: &HashMap<String, String>, slug: &str, notes: &mut Vec<String>) -> HashMap<String, String> {
    let mut meta = en.clone();
    for (k, v) in tr {
        if !en.contains_key(k) {
            notes.push(format!("{slug}: front matter key \"{k}\" is not in the English page, ignored"));
        } else if (k == "title" || k == "summary") && !v.is_empty() {
            meta.insert(k.clone(), v.clone());
        }
    }
    for k in ["title", "summary"] {
        if en.contains_key(k) && tr.get(k).map_or(true, |v| v.trim().is_empty()) {
            notes.push(format!("{slug}: front matter key \"{k}\" is missing, English kept"));
        }
    }
    meta
}

fn load_pages(p: &Paths, notes: &mut Vec<String>) -> Vec<Page> {
    let mut files: Vec<PathBuf> = fs::read_dir(&p.content)
        .expect("site/content")
        .flatten()
        .map(|e| e.path())
        .filter(|f| f.extension().map_or(false, |x| x == "md"))
        .collect();
    files.sort();
    let mut pages = Vec::new();
    for f in files {
        let slug = f.file_stem().unwrap().to_str().unwrap();
        let (mut meta, mut body) = front(&read(&f));
        if let Some(dir) = &p.overlay {
            let tf = dir.join(f.file_name().unwrap());
            if !tf.is_file() {
                notes.push(format!("{slug}: no translated page, English used"));
            } else {
                match fs::read_to_string(&tf).ok().and_then(|t| try_front(&t.replace("\r\n", "\n"))) {
                    Some((tm, tb)) => {
                        meta = merge_meta(&meta, &tm, slug, notes);
                        body = tb;
                    }
                    None => notes.push(format!("{slug}: translated page unreadable or without front matter, English used")),
                }
            }
        }
        let (h, heads) = markdown(p, &body);
        pages.push(Page::new(slug, &meta, h, heads));
    }
    let (body, heads, summary) = features_page(p);
    let meta: HashMap<String, String> = [
        ("title", "What makes Trindle different"),
        ("summary", summary.as_str()),
        ("group", "Reference"),
        ("order", "10"),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
    pages.push(Page::new("features", &meta, body, heads));
    pages
}

// The pages of one group, in the order the menu shows them.
fn in_group<'a>(pages: &'a [Page], g: &str) -> Vec<&'a Page> {
    let mut items: Vec<&Page> = pages.iter().filter(|p| p.group == g && p.slug != "index").collect();
    items.sort_by(|a, b| (a.order, &a.title).cmp(&(b.order, &b.title)));
    items
}

fn nav_html(pages: &[Page], current: &str) -> String {
    let mut out = vec!["<nav class=\"side\" aria-label=\"Contents\">".to_string()];
    for g in GROUPS {
        let items = in_group(pages, g);
        if items.is_empty() {
            continue;
        }
        out.push(format!("<p class=\"group\">{}</p><ul>", esc(g)));
        for p in items {
            let cur = if p.slug == current { " aria-current=\"page\"" } else { "" };
            out.push(format!("<li><a href=\"{}\"{}>{}</a></li>", p.file, cur, esc(&p.title)));
        }
        out.push("</ul>".to_string());
    }
    out.push("</nav>".to_string());
    out.join("\n")
}

const FIND: &str = "<form class=\"find\" role=\"search\" action=\"search.html\" method=\"get\">
<label for=\"q\">Search the help</label>
<input id=\"q\" name=\"q\" type=\"search\" autocomplete=\"off\" placeholder=\"Search, such as printer or Wi-Fi\">
<button type=\"submit\">Search</button>
</form>
";

fn shell(title: &str, current: &str, pages: &[Page], main: &str, description: &str, scripts: &str) -> String {
    let find = if current == "index" || current == "search" { String::new() } else {
        FIND.replace("Search the help", &esc(&word("Search the help"))).replace("Search, such as printer or Wi-Fi", &esc(&word("Search, such as printer or Wi-Fi")))
            .replace(">Search<", &format!(">{}<", esc(&word("Search"))))
    };
    let lang = CHROME.with(|c| c.borrow().0.clone());
    let dir = if rtl(&lang) { " dir=\"rtl\"" } else { "" };
    format!(
        "<!doctype html>
<html lang=\"{lang}\"{dir}>
<head>
<meta charset=\"utf-8\">
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
<title>{title}</title>
<meta name=\"description\" content=\"{desc}\">
<meta name=\"color-scheme\" content=\"dark\">
<link rel=\"stylesheet\" href=\"style.css\">
</head>
<body>
<a class=\"skip\" href=\"#main\">{skip}</a>
<header class=\"top\">
<a class=\"brand\" href=\"index.html\">Trindle <span>{help}</span></a>
{find}</header>
<div class=\"frame\">
{nav}
<main id=\"main\" tabindex=\"-1\">
{main}
</main>
</div>
<footer class=\"foot\"><p>{foot}</p></footer>
{scripts}</body>
</html>
",
        title = esc(title),
        desc = esc(description),
        nav = nav_html(pages, current),
        skip = esc(&word("Skip to the page")),
        help = esc(&word("help")),
        foot = esc(&word("Trindle help. No cookies, no tracking, no outside fonts or scripts.")),
    )
}

fn article(page: &Page, pages: &[Page]) -> String {
    let mut toc = String::new();
    let h2: Vec<&(usize, String, String)> = page.heads.iter().filter(|h| h.0 == 2).collect();
    if h2.len() >= 3 {
        let li: String = h2.iter().map(|(_, sid, t)| format!("<li><a href=\"#{}\">{}</a></li>", sid, inline(t, false))).collect();
        let otp = esc(&word("On this page"));
        toc = format!("<nav class=\"toc\" aria-label=\"{otp}\"><h2>{otp}</h2><ul>{li}</ul></nav>");
    }
    let ordered: Vec<&Page> = GROUPS.iter().flat_map(|g| in_group(pages, g)).collect();
    let idx = ordered.iter().position(|q| q.slug == page.slug).expect("the page is in a known group");
    let mut pn = String::new();
    if idx > 0 {
        let q = ordered[idx - 1];
        pn.push_str(&format!("<a class=\"prev\" href=\"{}\"><span>Before</span>{}</a>", q.file, esc(&q.title)));
    }
    if idx + 1 < ordered.len() {
        let q = ordered[idx + 1];
        pn.push_str(&format!("<a class=\"next\" href=\"{}\"><span>Next</span>{}</a>", q.file, esc(&q.title)));
    }
    format!(
        "<article>
<h1>{}</h1>
<p class=\"lede\">{}</p>
{}
{}
</article>
<nav class=\"pn\" aria-label=\"Before and next\">{}</nav>",
        esc(&page.title), inline(&page.summary, true), toc, page.body, pn
    )
}

fn home(p: &Paths, pages: &[Page]) -> String {
    let card = |slugname: &str, text: &str| {
        let q = pages.iter().find(|q| q.slug == slugname).expect("a page for every card");
        format!("<li><a class=\"card\" href=\"{}\"><h3>{}</h3><p>{}</p></a></li>", q.file, esc(&q.title), esc(text))
    };
    let start: String = [
        card("getting-started", "Put Trindle on a computer and find your way round the first day."),
        card("quick-answers", "The twenty questions a new person asks first."),
        card("ring", "Everything lives in one circle. Learn how to open it."),
        card("apps", "Find, install, update and remove apps."),
        card("folders", "Look after your files."),
        card("settings", "Every setting, page by page."),
        card("troubleshooting", "Something is wrong? Start here."),
    ]
    .concat();
    let more: String = [
        card("text", "Write and edit plain text and Markdown."),
        card("lock-screen", "How the screen locks and how you get back in."),
        card("drives", "See and change your drives, with a password every time."),
        card("flow", "One mouse and keyboard for two computers."),
        card("report-a-problem", "Tell people what went wrong, and what is kept."),
        card("features", "What makes Trindle different."),
        card("glossary", "Short answers to the words used here."),
    ]
    .concat();
    let (w, h) = png_size(&p.content.join("img/ring.png"));
    let main = format!(
        "<section class=\"hero\">
<h1>Help for Trindle</h1>
<p class=\"lede\">Plain answers about Trindle, with pictures. Type a few words to find one, or pick a page below.</p>
<form class=\"find big\" role=\"search\" action=\"search.html\" method=\"get\">
<label for=\"q2\">Search the help</label>
<input id=\"q2\" name=\"q\" type=\"search\" autocomplete=\"off\" placeholder=\"Search, such as printer, lock or install\">
<button type=\"submit\">Search</button>
</form>
<figure><img src=\"img/ring.png\" alt=\"Trindle's ring open in the middle of an otherwise empty dark screen: a clock in the centre, small round buttons around it and an app on the outer circle.\" width=\"{w}\" height=\"{h}\"></figure>
</section>
<section class=\"block\"><h2>Start here</h2><ul class=\"cards\">{start}</ul></section>
<section class=\"block\"><h2>More to read</h2><ul class=\"cards\">{more}</ul></section>"
    );
    shell(&word(SITE), "index", pages, &main, "Plain answers about Trindle, with pictures.", "")
}

fn search_page(pages: &[Page]) -> String {
    let mut items = String::new();
    for g in GROUPS {
        for p in in_group(pages, g) {
            items.push_str(&format!("<li><a href=\"{}\">{}</a> <span class=\"quiet\">{}</span></li>", p.file, esc(&p.title), esc(&p.summary)));
        }
    }
    let main = format!(
        "<article>
<h1>Search the help</h1>
<p class=\"lede\" id=\"lede\">Type a few words. Results appear as you type.</p>
<form class=\"find big\" role=\"search\" action=\"search.html\" method=\"get\" id=\"sform\">
<label for=\"sq\">Words to look for</label>
<input id=\"sq\" name=\"q\" type=\"search\" autocomplete=\"off\" placeholder=\"Search, such as printer or Wi-Fi\">
<button type=\"submit\">Search</button>
</form>
<p id=\"status\" role=\"status\" aria-live=\"polite\" class=\"status\"></p>
<ol id=\"results\" class=\"results\" aria-label=\"Search results\"></ol>
<div id=\"all\">
<h2>All pages</h2>
<p>This is every page in the help. The search above needs JavaScript; this list works without it.</p>
<ul class=\"plain\">{items}</ul>
</div>
</article>"
    );
    shell(
        &format!("{} - {}", word("Search"), word(SITE)), "search", pages, &main, "Search the Trindle help.",
        "<script src=\"search-index.js\"></script>\n<script src=\"search.js\"></script>\n",
    )
}

fn not_found(pages: &[Page]) -> String {
    let main = "<article><h1>That page is not here</h1>
<p class=\"lede\">The address may be old or mistyped. Search for what you wanted, or go back to the start.</p>
<p><a class=\"button\" href=\"index.html\">Go to the start</a></p></article>";
    shell(&format!("{} - {}", word("Page not found"), word(SITE)), "404", pages, main, "Page not found.", "")
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            '\u{8}' => o.push_str("\\b"),
            '\u{c}' => o.push_str("\\f"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn build_index(pages: &[Page]) -> String {
    let mut docs: Vec<String> = Vec::new();
    for p in pages {
        let mut sections: Vec<String> = Vec::new();
        // Split the body at each heading, so a result points at the section.
        for part in re(r#"(?=<h[23] id=")"#).split(&p.body) {
            let m = res(r#"<h([23]) id="([^"]+)">(.*?)</h\1>"#).at_start(&part);
            let body = match &m {
                Some(m) => &part[m.end..],
                None => &part[..],
            };
            let text = re(r"</(p|li|h\d|ul|ol|aside|figcaption|section|pre|figure)>").sub_str(body, " ");
            let text = strip_tags(&text, "");
            let text = re(r"\s+").sub_str(&text, " ");
            let text = unescape(&text).trim().to_string();
            if let Some(m) = &m {
                let head = unescape(&strip_tags(m.grp(3), ""));
                sections.push(format!("{{\"i\":{},\"h\":{},\"x\":{}}}", json_str(m.grp(2)), json_str(&head), json_str(&text)));
            } else if !text.is_empty() {
                sections.push(format!("{{\"i\":\"\",\"h\":\"\",\"x\":{}}}", json_str(&text)));
            }
        }
        if p.slug == "index" {
            continue;
        }
        docs.push(format!(
            "{{\"p\":{},\"t\":{},\"s\":{},\"c\":[{}]}}",
            json_str(&p.file), json_str(&p.title), json_str(&p.summary), sections.join(",")
        ));
    }
    format!("window.SEARCH_INDEX=[{}];\n", docs.join(","))
}

// ------------------------------------------------------------------- checks

fn lum(hex: &str) -> f64 {
    let h = hex.trim_start_matches('#');
    let ch = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).unwrap() as f64 / 255.0;
    let f = |c: f64| if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
    0.2126 * f(ch(0)) + 0.7152 * f(ch(2)) + 0.0722 * f(ch(4))
}

fn token(name: &str) -> &'static str {
    TOKENS.iter().find(|t| t.0 == name).expect("a known colour").1
}

fn ratio(a: &str, b: &str) -> f64 {
    let (la, lb) = (lum(token(a)), lum(token(b)));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

fn is_upper(s: &str) -> bool {
    s.chars().any(|c| c.is_alphabetic()) && !s.chars().any(|c| c.is_lowercase())
}

fn check(p: &Paths, pages: &[Page]) -> Vec<String> {
    let mut bad: Vec<String> = Vec::new();
    for (a, b, need) in PAIRS {
        let r = ratio(a, b);
        if r < need {
            bad.push(format!("contrast {a} on {b} is {r:.2}, needs {need:.1}"));
        }
    }
    let mut files: HashSet<String> = pages.iter().map(|q| q.file.clone()).collect();
    files.insert("search.html".to_string());
    files.insert("404.html".to_string());
    // Split so this file does not hold the names itself (own_tests checks every file).
    let banned = rei(&["cos", "mic", "|", "ne", "mo", "|", "light", "morphic"].concat());
    for pg in pages {
        if pg.slug != "features" {
            for (_, _, text) in &pg.heads {
                let plain = re(r"[`*]").sub_str(text, "");
                let ws: Vec<&str> = plain.split_whitespace().collect();
                for w in ws.iter().skip(1) {
                    let w2 = re(r"[^A-Za-z-]").sub_str(w, "");
                    let first_upper = w2.chars().next().map_or(false, |c| c.is_uppercase());
                    if first_upper && !CAPITALS.contains(&w2.as_str()) && !is_upper(&w2) {
                        bad.push(format!("{}: heading not in sentence case: {}", pg.file, text));
                        break;
                    }
                }
            }
        }
        for m in re(r#"<img [^>]*?alt="([^"]*)""#).find_all(&pg.body) {
            if m.grp(1).trim().chars().count() < 12 {
                bad.push(format!("{}: picture alt text too short", pg.file));
            }
        }
        if re(r"<img(?![^>]*alt=)").is_match(&pg.body) {
            bad.push(format!("{}: picture with no alt text", pg.file));
        }
        for m in re(r#"<img [^>]*src="([^"]+)""#).find_all(&pg.body) {
            if !p.content.join(m.grp(1)).exists() {
                bad.push(format!("{}: picture missing: {}", pg.file, m.grp(1)));
            }
        }
        for m in re(r##"href="([^"#]+)(#[^"]*)?""##).find_all(&pg.body) {
            let t = m.grp(1);
            if t.starts_with("http://") || t.starts_with("https://") || t.starts_with("mailto:") {
                continue;
            }
            if !files.contains(t) {
                bad.push(format!("{}: link to a page that is not there: {}", pg.file, t));
            } else if m.has(2) {
                let target = pages.iter().find(|q| q.file == t);
                if let Some(target) = target {
                    if !target.body.contains(&format!("id=\"{}\"", &m.grp(2)[1..])) {
                        bad.push(format!("{}: link to a part that is not there: {}{}", pg.file, t, m.grp(2)));
                    }
                }
            }
        }
        let flat = format!("{} {} {}", strip_tags(&pg.body, " "), pg.title, pg.summary);
        if flat.contains('\u{2014}') {
            bad.push(format!("{}: em dash", pg.file));
        }
        if let Some(mm) = banned.search(&flat) {
            bad.push(format!("{}: banned word: {}", pg.file, &flat[mm.start..mm.end]));
        }
        if rei(r"\b(favorite|color|organize|center)\b").is_match(&flat) {
            bad.push(format!("{}: American spelling", pg.file));
        }
        if re("[\\U0001F300-\\U0001FAFF\u{2600}-\u{27BF}]").is_match(&flat) {
            bad.push(format!("{}: emoji", pg.file));
        }
        if pg.body.matches("<!-- TODO").count() > 0 && !pg.body.contains("-->") {
            bad.push(format!("{}: broken TODO comment", pg.file));
        }
    }
    bad
}

// -------------------------------------------------------- reading checks

const MAX_RUN: usize = 300;
const MAX_SENTENCE: usize = 30;

// For one page: words, pictures, longest run of words with no picture,
// longest sentence in words, words that talk of a screen.
fn reading_numbers(page: &Page) -> (usize, usize, usize, usize, usize) {
    let body = res(r"<pre\b.*?</pre>").sub_str(&page.body, " ");
    let pictures = body.matches("<figure>").count();
    let parts = res(r"<figure>.*?</figure>").split(&body);
    let counts: Vec<usize> = parts.iter().map(|part| words(&unescape(&strip_tags(part, " ")))).collect();
    let without = res(r"<figure>.*?</figure>").sub_str(&body, " ");
    let mut longest = 0;
    for m in res(r"<(?:p|li|aside)\b[^>]*>(.*?)</(?:p|li|aside)>").find_all(&without) {
        let text = unescape(&strip_tags(m.grp(1), " "));
        for sentence in re(r"(?<=[.!?:;])\s+").split(&text) {
            longest = longest.max(words(&sentence));
        }
    }
    let flat = unescape(&strip_tags(&without, " "));
    let screen = rei(r"\b(press|pressed|click|button|buttons|window|screen|tab|tabs)\b").find_all(&flat).len();
    (counts.iter().sum(), pictures, counts.iter().copied().max().unwrap_or(0), longest, screen)
}

// Plain-reading rules for the pages written by hand (the Features page is
// built from spec/features.md, so it is left out): no more than MAX_RUN
// words in a row without a picture or diagram, at least one picture on a
// page that talks of a screen, no sentence over MAX_SENTENCE words. Gives
// the numbers for every page.
fn check_reading(pages: &[Page]) -> (Vec<String>, String) {
    let mut bad = Vec::new();
    let mut lines = Vec::new();
    let mut sorted: Vec<&Page> = pages.iter().collect();
    sorted.sort_by(|a, b| a.slug.cmp(&b.slug));
    for p in sorted {
        if p.slug == "index" || p.slug == "features" {
            continue;
        }
        let (w, pics, run, sentence, screen) = reading_numbers(p);
        lines.push(format!("  {:<18} {:>4} words  {:>2} pictures  longest run {:>3}  longest sentence {:>2}", p.file, w, pics, run, sentence));
        if run > MAX_RUN {
            bad.push(format!("{}: {} words in a row without a picture or diagram (most allowed: {})", p.file, run, MAX_RUN));
        }
        if pics == 0 && screen >= 3 {
            bad.push(format!("{}: describes a screen but has no picture or diagram", p.file));
        }
        for m in res(r"<figure>.*?(?:<figcaption>(.*?)</figcaption>)?</figure>").find_all(&p.body) {
            if !m.grp(1).starts_with("What you will see") {
                bad.push(format!("{}: a picture whose caption does not start with What you will see", p.file));
            }
        }
        if sentence > MAX_SENTENCE {
            bad.push(format!("{}: a sentence of {} words (most allowed: {})", p.file, sentence, MAX_SENTENCE));
        }
    }
    (bad, lines.join("\n"))
}

// ------------------------------------------------------------------- build

// Checks on the built pages: headings in order, a skip link, alt text on
// every picture, the search result list announced, links that share a
// sentence with words carry a cue that is not colour, and the style sheet
// gives every link, button and the search box a visible focus ring.
fn check_dist(p: &Paths) -> (Vec<String>, usize) {
    let mut bad: Vec<String> = Vec::new();
    let css = read(&p.statics.join("style.css"));
    let css_flat = re(r"\s+").sub_str(&css, " ");
    if !re(r":focus-visible\s*\{[^}]*outline:\s*3px solid var\(--focus\)").is_match(&css_flat) {
        bad.push("style.css: no 3px focus ring on :focus-visible".into());
    }
    if re(r"outline:\s*(none|0)").is_match(&re(r"main:focus\s*\{[^}]*\}").sub_str(&css_flat, "")) {
        bad.push("style.css: an outline is switched off".into());
    }
    if !re(r"a\.in\s*\{[^}]*font-weight:\s*[7-9]00").is_match(&css_flat) {
        bad.push("style.css: an in-sentence link is not bold at rest".into());
    }
    if !re(r"a\.in:hover,\s*a\.in:focus-visible\s*\{[^}]*text-decoration:\s*underline").is_match(&css_flat) {
        bad.push("style.css: an in-sentence link is not underlined on hover and on keyboard focus".into());
    }
    if re(r"(^|[}\s])a\s*\{[^}]*text-decoration:\s*underline").is_match(&css_flat) {
        bad.push("style.css: links are underlined at rest".into());
    }
    let mut inline_links = 0;
    let mut names: Vec<PathBuf> = fs::read_dir(&p.dist)
        .expect("site/dist")
        .flatten()
        .map(|e| e.path())
        .filter(|f| f.extension().map_or(false, |x| x == "html"))
        .collect();
    names.sort();
    for f in names {
        let h = read(&f);
        let name = f.file_name().unwrap().to_string_lossy().to_string();
        if !h.contains("<a class=\"skip\" href=\"#main\">") || !h.contains("id=\"main\"") {
            bad.push(format!("{name}: no skip link to the main part"));
        }
        let lang = CHROME.with(|c| c.borrow().0.clone());
        let want = if rtl(&lang) { format!("<html lang=\"{lang}\" dir=\"rtl\">") } else { format!("<html lang=\"{lang}\">") };
        if !h.contains(&want) {
            bad.push(format!("{name}: no page language"));
        }
        let mut last = 0;
        for m in re(r"<h([1-6])[ >]").find_all(&h) {
            let lv: usize = m.grp(1).parse().unwrap();
            if (last == 0 && lv != 1) || lv > last + 1 {
                bad.push(format!("{name}: heading level h{lv} after h{last}"));
            }
            last = lv;
        }
        if h.matches("<h1").count() != 1 {
            bad.push(format!("{name}: needs exactly one h1"));
        }
        for m in re(r"<img\b[^>]*>").find_all(&h) {
            if !re(r#"alt="[^"]{12,}""#).is_match(m.grp(0)) {
                bad.push(format!("{name}: picture without proper alt text"));
            }
        }
        for m in re(r#"<(input|textarea|select)\b[^>]*\bid="([^"]+)""#).find_all(&h) {
            if !h.contains(&format!("for=\"{}\"", m.grp(2))) {
                bad.push(format!("{name}: box {} has no label", m.grp(2)));
            }
        }
        for m in res(r"<(p|aside)\b[^>]*>(.*?)</\1>").find_all(&h) {
            for a in re(r"<a\b([^>]*)>").find_all(m.grp(2)) {
                inline_links += 1;
                if !a.grp(1).contains("class=\"in\"") && !a.grp(1).contains("class=\"button\"") {
                    bad.push(format!("{name}: link in a sentence without the in-sentence cue"));
                }
            }
        }
        for m in res(r"<li>(.*?)</li>").find_all(&h) {
            let inner = m.grp(1);
            let head: String = m.grp(0).chars().take(5).collect();
            if inner.contains("<a ") && !head.contains("class=\"") {
                for a in re(r"<a\b([^>]*)>").find_all(inner) {
                    let attrs = a.grp(1);
                    if !attrs.contains("class=\"in\"") && !attrs.contains("card") && !attrs.contains("title") {
                        let without = strip_tags(&res(r"<a\b.*?</a>").sub_str(inner, ""), "");
                        let txt = without.trim_matches(|c| " .,:;".contains(c));
                        if !txt.is_empty() && !inner.contains("quiet") {
                            bad.push(format!("{name}: link shares a list line with other words, no cue"));
                        }
                    }
                }
            }
        }
        for m in res(r#"<li class="link">(.*?)</li>"#).find_all(&h) {
            if m.grp(1).matches("<a ").count() != 1 {
                bad.push(format!("{name}: link line with more than one link"));
            }
        }
    }
    let sh = read(&p.dist.join("search.html"));
    if !re(r#"id="status"[^>]*aria-live="polite""#).is_match(&sh) && !re(r#"aria-live="polite"[^>]*id="status""#).is_match(&sh) {
        bad.push("search.html: result count is not announced politely".into());
    }
    if !sh.contains("id=\"results\"") {
        bad.push("search.html: no result list".into());
    }
    (bad, inline_links)
}

fn report_contrast() -> String {
    PAIRS
        .iter()
        .map(|(a, b, need)| {
            let r = ratio(a, b);
            format!("  {:<10} on {:<10} {:>5.2}:1 (needs {:.1}) {}", a, b, r, need, if r >= *need { "ok" } else { "FAIL" })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// The messages of an .ftl file: `key = value` lines, `#` comments, and
// indented lines that continue the message above.
fn parse_ftl(text: &str) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    let mut last: Option<String> = None;
    for ln in text.replace("\r\n", "\n").split('\n') {
        if ln.trim().is_empty() || ln.starts_with('#') {
            last = None;
        } else if ln.starts_with(' ') || ln.starts_with('\t') {
            if let Some(k) = &last {
                let v = out.get_mut(k).unwrap();
                v.push(' ');
                v.push_str(ln.trim());
            }
        } else if let Some(i) = ln.find('=') {
            let k = ln[..i].trim().to_string();
            out.insert(k.clone(), ln[i + 1..].trim().to_string());
            last = Some(k);
        }
    }
    out
}

// Every translatable piece of an SVG, in drawing order, with its key:
// `title`, `desc`, then `label-01`, `label-02` for the non-empty <text>.
fn svg_labels(svg: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for tag in ["title", "desc"] {
        if let Some(m) = re(&format!(r"<{tag}>([^<]*)</{tag}>")).search(svg) {
            out.push((tag.to_string(), unescape(m.grp(1))));
        }
    }
    let mut n = 0;
    for m in re(r"<text [^>]*>([^<]+)</text>").find_all(svg) {
        n += 1;
        out.push((format!("label-{n:02}"), unescape(m.grp(1))));
    }
    out
}

// The SVG with each label replaced by its message; a missing or blank
// message keeps the English word.
fn localise_svg(svg: &str, msgs: &HashMap<String, String>) -> String {
    let mut out = String::new();
    let mut at = 0;
    let mut n = 0;
    let pick = |key: &str, en: &str| match msgs.get(key) {
        Some(v) if !v.trim().is_empty() => esc(v),
        _ => en.to_string(),
    };
    let marks = re(r"<(title|desc)>([^<]*)</(?:title|desc)>|<text [^>]*>([^<]+)</text>");
    for m in marks.find_all(svg) {
        let (key, from) = if m.grp(1).is_empty() {
            n += 1;
            (format!("label-{n:02}"), 3)
        } else {
            (m.grp(1).to_string(), 2)
        };
        let whole = &svg[m.start..m.end];
        let inner = m.grp(from);
        let off = whole.rfind("</").unwrap() - inner.len();
        out.push_str(&svg[at..m.start + off]);
        out.push_str(&pick(&key, inner));
        at = m.start + off + inner.len();
    }
    out.push_str(&svg[at..]);
    out
}

// What the checks find wrong in one language's built site, each line said
// with the language it is in.
fn language_faults(code: &str, lp: &Paths, lpages: &[Page]) -> Vec<String> {
    let mut bad = check(lp, lpages);
    bad.extend(check_dist(lp).0);
    bad.into_iter().map(|b| format!("{code}: {b}")).collect()
}

fn build(p: &Paths, notes: &mut Vec<String>) -> Vec<Page> {
    let code = p.overlay.as_ref().and_then(|o| o.parent()).and_then(|d| d.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let words = p.overlay.as_ref().map(|o| o.join("chrome.txt")).filter(|f| f.is_file()).map(|f| read_chrome(&read(&f))).unwrap_or_default();
    CHROME.with(|c| *c.borrow_mut() = (page_lang(&code), words));
    let pages = load_pages(p, notes);
    if p.dist.exists() {
        fs::remove_dir_all(&p.dist).expect("clear site/dist");
    }
    fs::create_dir_all(p.dist.join("img")).expect("make site/dist");
    for pg in &pages {
        if pg.slug == "index" {
            continue;
        }
        write(&p.dist.join(&pg.file), &shell(&format!("{} - {}", pg.title, word(SITE)), &pg.slug, &pages, &article(pg, &pages), &pg.summary, ""));
    }
    write(&p.dist.join("index.html"), &home(p, &pages));
    write(&p.dist.join("search.html"), &search_page(&pages));
    write(&p.dist.join("404.html"), &not_found(&pages));
    write(&p.dist.join("search-index.js"), &build_index(&pages));
    for f in ["style.css", "search.js", "_headers"] {
        let mut text = read(&p.statics.join(f));
        if f == "style.css" {
            let vars: String = TOKENS.iter().map(|(k, v)| format!("  --{k}: {v};\n")).collect();
            text = format!(":root{{\n{vars}}}\n{text}");
        }
        write(&p.dist.join(f), &text);
    }
    let mut imgs: Vec<PathBuf> = fs::read_dir(p.content.join("img")).expect("site/content/img").flatten().map(|e| e.path()).collect();
    imgs.sort();
    for img in imgs {
        let name = img.file_name().unwrap();
        let to = p.dist.join("img").join(name);
        let ftl = p.overlay.as_ref().map(|d| d.join("img").join(Path::new(name).with_extension("ftl")));
        if img.extension().map_or(false, |x| x == "svg") && p.overlay.is_some() {
            let en = read(&img);
            let msgs = match ftl.as_ref().filter(|f| f.is_file()) {
                Some(f) => parse_ftl(&read(f)),
                None => HashMap::new(),
            };
            for (k, _) in svg_labels(&en) {
                if msgs.get(&k).map_or(true, |v| v.trim().is_empty()) {
                    notes.push(format!("img/{}: no message \"{k}\", English kept", name.to_string_lossy()));
                }
            }
            write(&to, &localise_svg(&en, &msgs));
        } else {
            fs::copy(&img, &to).expect("copy a picture");
        }
    }
    pages
}

// The language folders in DIR that have a docs folder and a safe name.
fn languages(dir: &Path) -> Vec<String> {
    let mut out: Vec<String> = fs::read_dir(dir)
        .map(|rd| rd.flatten().filter(|e| e.path().join("docs").is_dir()).filter_map(|e| e.file_name().into_string().ok()).collect())
        .unwrap_or_default();
    out.retain(|c| good_code(c));
    out.sort();
    out
}

fn good_code(c: &str) -> bool {
    let mut parts = c.split('-');
    let first = parts.next().unwrap_or("");
    (2..=3).contains(&first.len())
        && first.chars().all(|ch| ch.is_ascii_lowercase())
        && first != "img"
        && parts.all(|x| !x.is_empty() && x.chars().all(|ch| ch.is_ascii_alphanumeric()))
}

fn run() -> i32 {
    let root = std::env::current_dir().expect("a current folder");
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    // The features list is kept with Trindle itself (spec/features.md there), not in this repository.
    let features = arg("--features").or_else(|| std::env::var("TRINDLE_FEATURES").ok()).map_or_else(|| root.join("spec/features.md"), PathBuf::from);
    let p = Paths {
        content: root.join("site/content"),
        dist: root.join("site/dist"),
        features,
        statics: root.join("site/static"),
        overlay: None,
        root,
    };
    if !p.content.is_dir() {
        eprintln!("run this from the repository root: {} is not there", p.content.display());
        return 2;
    }
    let mut notes = Vec::new();
    let mut language_bad = Vec::new();
    let checking = std::env::args().any(|a| a == "--check");
    let pages = build(&p, &mut notes);
    println!("built {} pages into {}", pages.len() + 2, p.dist.strip_prefix(&p.root).unwrap().display());
    if let Some(dir) = arg("--translations") {
        let only = arg("--lang");
        for code in languages(Path::new(&dir)) {
            if only.as_ref().map_or(false, |o| *o != code) {
                continue;
            }
            let lp = Paths {
                content: p.content.clone(),
                dist: p.dist.join(&code),
                features: p.features.clone(),
                statics: p.statics.clone(),
                overlay: Some(Path::new(&dir).join(&code).join("docs")),
                root: p.root.clone(),
            };
            let mut ln = Vec::new();
            let lpages = build(&lp, &mut ln);
            println!("built {} pages for {code} into {}, {} fallbacks", lpages.len() + 2, lp.dist.strip_prefix(&p.root).unwrap().display(), ln.len());
            notes.extend(ln.into_iter().map(|n| format!("{code}/{n}")));
            if checking {
                language_bad.extend(language_faults(&code, &lp, &lpages));
            }
        }
        for n in &notes {
            println!("fallback: {n}");
        }
    }
    if checking {
        CHROME.with(|c| *c.borrow_mut() = (page_lang(""), HashMap::new()));
        let mut bad = check(&p, &pages);
        let (more, n) = check_dist(&p);
        bad.extend(more);
        bad.extend(language_bad);
        let (reading_bad, numbers) = check_reading(&pages);
        bad.extend(reading_bad);
        println!("reading:\n{numbers}");
        println!("contrast:\n{}", report_contrast());
        println!("links inside sentences: {n}");
        if !bad.is_empty() {
            println!("{}", bad.join("\n"));
            return 1;
        }
        println!("checks passed");
    }
    0
}

fn main() {
    // The matcher goes deep on long pages, so run on a roomy stack.
    let code = std::thread::Builder::new().stack_size(512 << 20).spawn(run).unwrap().join().unwrap_or(2);
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("docs-rs-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    const SVG: &str = "<svg><title>Ring &amp; menu</title><desc>A drawing</desc><text x=\"1\" y=\"2\"></text><text x=\"1\">Open</text><text x=\"3\">Close</text></svg>";

    #[test]
    fn labels_are_numbered_from_the_top() {
        let l = svg_labels(SVG);
        let keys: Vec<&str> = l.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, ["title", "desc", "label-01", "label-02"]);
        assert_eq!(l[0].1, "Ring & menu");
    }

    #[test]
    fn missing_label_keeps_english() {
        let m = parse_ftl("# note\ntitle = Anneau & menu\nlabel-01 = Ouvrir\nlabel-02 =\n");
        let out = localise_svg(SVG, &m);
        assert!(out.contains("<title>Anneau &amp; menu</title>"));
        assert!(out.contains("<desc>A drawing</desc>"));
        assert!(out.contains(">Ouvrir</text>"));
        assert!(out.contains(">Close</text>"));
        assert!(out.contains("<text x=\"1\" y=\"2\"></text>"));
    }

    #[test]
    fn a_hostile_label_cannot_add_markup() {
        let m = parse_ftl("label-01 = <script>x</script>\n");
        assert!(!localise_svg(SVG, &m).contains("<script>"));
    }

    #[test]
    fn multiline_messages_join() {
        let m = parse_ftl("a = one\n  two\n\nb = three\n");
        assert_eq!(m["a"], "one two");
        assert_eq!(m["b"], "three");
    }

    #[test]
    fn language_codes() {
        for ok in ["de", "pt-BR".to_lowercase().as_str(), "zh-Hans".to_lowercase().as_str()] {
            assert!(good_code(ok), "{ok}");
        }
        for bad in ["img", "..", "a/b", "", "d", "ring", "de-", "DE"] {
            assert!(!good_code(bad), "{bad}");
        }
    }

    fn site(name: &str) -> (Paths, PathBuf) {
        let root = tmp(name);
        let content = root.join("content");
        fs::create_dir_all(content.join("img")).unwrap();
        fs::write(content.join("one.md"), "---\ntitle: One\nsummary: First page.\ngroup: Start\norder: 5\n---\n\nEnglish one.\n").unwrap();
        fs::write(content.join("two.md"), "---\ntitle: Two\nsummary: Second page.\ngroup: Help\norder: 7\n---\n\nEnglish two.\n").unwrap();
        fs::write(root.join("features.md"), "# Features\n\n## Big features\n\n**A feature.** What it does. In Trindle now (1 January 2026).\n").unwrap();
        let docs = root.join("fr/docs");
        fs::create_dir_all(docs.join("img")).unwrap();
        let cwd = std::env::current_dir().unwrap();
        let p = Paths {
            content,
            dist: root.join("dist"),
            features: root.join("features.md"),
            statics: cwd.join("site/static"),
            overlay: Some(docs),
            root,
        };
        (p, cwd)
    }

    #[test]
    fn missing_page_falls_back_and_front_matter_differences_are_handled() {
        let (p, _) = site("pages");
        let docs = p.overlay.clone().unwrap();
        fs::write(docs.join("one.md"), "---\ntitle: Un\nsummary:\ngroup: Reference\norder: 99\nextra: x\n---\n\nFrench one.\n").unwrap();
        let mut notes = Vec::new();
        let pages = load_pages(&p, &mut notes);
        let one = pages.iter().find(|q| q.slug == "one").unwrap();
        let two = pages.iter().find(|q| q.slug == "two").unwrap();
        assert_eq!(one.title, "Un");
        assert_eq!(one.summary, "First page.");
        assert!(notes.iter().any(|n| n.starts_with("one: front matter key \"summary\" is missing")), "an empty summary is a gap: {notes:?}");
        assert_eq!((one.group.as_str(), one.order), ("Start", 5));
        assert!(one.body.contains("French one."));
        assert_eq!(two.title, "Two");
        assert!(two.body.contains("English two."));
        assert!(notes.iter().any(|n| n.starts_with("two: no translated page")));
        assert!(notes.iter().any(|n| n.contains("\"extra\"")));
    }

    // A language's own pages are held to the same checks as English.
    #[test]
    fn a_translated_page_with_a_fault_is_found() {
        let (mut p, cwd) = site("faults");
        p.content = cwd.join("site/content");
        p.statics = cwd.join("site/static");
        let docs = p.overlay.clone().unwrap();
        let en = fs::read_to_string(p.content.join("ring.md")).unwrap();
        fs::write(docs.join("ring.md"), format!("{en}\n![x](img/none.svg)\n")).unwrap();
        let mut notes = Vec::new();
        let pages = build(&p, &mut notes);
        let bad = language_faults("fr", &p, &pages);
        assert!(bad.iter().all(|b| b.starts_with("fr: ")));
        assert!(bad.iter().any(|b| b.contains("ring.html") && b.contains("picture")), "{bad:?}");
    }

    #[test]
    fn broken_translated_page_falls_back() {
        let (p, _) = site("broken");
        fs::write(p.overlay.clone().unwrap().join("two.md"), "no front matter at all\n").unwrap();
        let mut notes = Vec::new();
        let pages = load_pages(&p, &mut notes);
        assert!(pages.iter().find(|q| q.slug == "two").unwrap().body.contains("English two."));
        assert!(notes.iter().any(|n| n.starts_with("two: translated page unreadable")));
    }

    #[test]
    fn build_writes_a_site_with_labels_filled() {
        let (mut p, cwd) = site("build");
        p.content = cwd.join("site/content");
        let docs = p.overlay.clone().unwrap();
        let en = fs::read_to_string(p.content.join("ring.md")).unwrap();
        fs::write(docs.join("ring.md"), en.replacen("title: The ring", "title: L'anneau", 1)).unwrap();
        fs::write(docs.join("img/ring-layout.ftl"), "title = Titre\nlabel-01 = Un\n").unwrap();
        let mut notes = Vec::new();
        build(&p, &mut notes);
        let a = fs::read_to_string(p.dist.join("img/ring-layout.svg")).unwrap();
        assert!(a.contains("<title>Titre</title>") && a.contains(">Un</text>"));
        assert!(fs::read_to_string(p.dist.join("ring.html")).unwrap().contains("L&#x27;anneau"));
        assert!(notes.iter().any(|n| n == "img/ring-layout.svg: no message \"label-02\", English kept"));
        assert!(notes.iter().any(|n| n == "img/two-panes.svg: no message \"title\", English kept"));
        assert!(p.dist.join("img/ring.png").is_file() && p.dist.join("style.css").is_file());
        assert!(notes.iter().any(|n| n == "folders: no translated page, English used"));
    }

    // A translated page says its own language, right to left where it is
    // written so, and its frame in its own words where they are given.
    #[test]
    fn a_translated_page_says_its_language_and_direction() {
        assert_eq!(page_lang(""), "en-GB");
        assert_eq!(page_lang("pt_BR"), "pt-BR");
        assert!(rtl("ar") && rtl("ur") && !rtl("fr"));
        CHROME.with(|c| *c.borrow_mut() = ("ar".into(), read_chrome("# a note\nSkip to the page = تخطى\nhelp = مساعدة\nbad line\n")));
        let page = shell("t", "x", &[], "m", "d", "");
        assert!(page.contains("<html lang=\"ar\" dir=\"rtl\">"));
        assert!(page.contains(">تخطى</a>") && page.contains("Trindle <span>مساعدة</span>"));
        assert!(page.contains("No cookies"), "a frame word not given stays English");
        CHROME.with(|c| *c.borrow_mut() = (page_lang(""), HashMap::new()));
        assert!(shell("t", "x", &[], "m", "d", "").contains("<html lang=\"en-GB\">"));
    }
}
