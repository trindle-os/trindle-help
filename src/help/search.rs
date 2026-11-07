//! The Help search: fuzzy, offline, no model. Finds the section that answers
//! a question and shows the one sentence in it that answers best. It never
//! writes words of its own: the snippet is cut from the section's text.

use std::collections::HashMap;

pub struct Page {
    pub id: String,
    pub title: String,
    pub sections: Vec<Section>,
}

pub struct Section {
    pub id: String,
    pub heading: String,
    pub text: String,
    pub questions: Vec<String>,
}

pub struct Hit {
    pub page: String,
    pub section: String,
    pub snippet: String,
}

/// Plain-word alternatives. Each row is words that mean the same to a person;
/// the first is the one all the others are turned into. Add a row to extend.
const ALTERNATIVES: &[&[&str]] = &[
    &["nas", "network drive", "network storage", "network share", "shared folder", "file server"],
    &["wifi", "wi-fi", "wireless", "wlan"],
    &["wallpaper", "background", "desktop picture"],
    &["size", "big", "bigger", "larger", "large", "smaller", "small", "zoom", "scale", "enlarge", "magnify"],
    &["sound", "audio", "volume", "speaker"],
    &["microphone", "mic"],
    &["screen", "display", "monitor"],
    &["restart", "reboot"],
    &["delete", "remove", "erase"],
    &["folder", "directory"],
    &["app", "application", "program", "software"],
    &["internet", "online", "web"],
    &["usb", "flash drive", "memory stick", "pen drive", "thumb drive"],
    &["freeze", "hang", "unresponsive", "stuck"],
    &["disconnect", "drop", "dropout", "cut out"],
    &["screenshot", "screen capture", "screen grab"],
    &["update", "upgrade"],
    &["colour", "color"],
    &["bluetooth", "blue tooth"],
    &["password", "passcode"],
    &["turn off", "shut down", "power off"],
];

/// Words that only make a sentence a question.
const QUESTION_WORDS: &[&str] = &[
    "how", "do", "does", "did", "i", "me", "my", "we", "you", "your", "can", "could", "would",
    "should", "will", "wont", "cant", "dont", "doesnt", "isnt", "arent", "didnt", "why", "what",
    "whats", "when", "where", "which", "who", "is", "are", "am", "was", "be", "to", "a", "an",
    "the", "of", "in", "on", "it", "its", "for", "and", "or", "if", "there", "this", "that",
    "with", "get", "not", "no", "about", "please", "way", "help", "need", "want", "so", "at",
];

const SNIPPET_CHARS: usize = 160;
const W_HEADING: f32 = 3.0;
const W_QUESTION: f32 = 3.0;
const W_TITLE: f32 = 1.0;
const W_BODY: f32 = 1.0;

const F_HEADING: u8 = 1;
const F_QUESTION: u8 = 2;
const F_BODY: u8 = 4;
const F_TITLE: u8 = 8;

fn words(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in s.chars() {
        if c.is_alphanumeric() {
            cur.extend(c.to_lowercase());
        } else if c == '\'' || c == '\u{2019}' {
            continue;
        } else if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn undouble(s: &mut String) {
    let b = s.as_bytes();
    let n = b.len();
    if n >= 3 && b[n - 1] == b[n - 2] && b[n - 1].is_ascii_lowercase() && !b"lsz".contains(&b[n - 1]) {
        s.pop();
    }
}

/// Simple English stemming: plurals, -ing, -ed, doubled-consonant -er, and a
/// trailing e, so make, makes and making meet.
fn stem(w: &str) -> String {
    if !w.is_ascii() || w.len() <= 3 || w.bytes().all(|b| b.is_ascii_digit()) {
        return w.to_string();
    }
    let mut s = w.to_string();
    if s.ends_with("ies") && s.len() > 4 {
        s.truncate(s.len() - 3);
        s.push('y');
        return s;
    } else if s.ends_with("sses") {
        s.truncate(s.len() - 2);
    } else if s.ends_with("ing") && s.len() > 5 {
        s.truncate(s.len() - 3);
        undouble(&mut s);
    } else if s.ends_with("ed") && s.len() > 4 {
        s.truncate(s.len() - 2);
        undouble(&mut s);
    } else if s.ends_with("er") && s.len() > 4 {
        let b = s.as_bytes();
        if b[b.len() - 3] == b[b.len() - 4] {
            s.truncate(s.len() - 2);
            undouble(&mut s);
        }
    } else if s.ends_with('s') && !s.ends_with("ss") && !s.ends_with("us") && !s.ends_with("is") {
        s.pop();
    }
    if s.len() > 3 && s.ends_with('e') {
        s.pop();
    }
    s
}

fn stems(s: &str) -> Vec<String> {
    words(s)
        .into_iter()
        .filter(|w| !QUESTION_WORDS.contains(&w.as_str()))
        .map(|w| stem(&w))
        .collect()
}

/// Longest alternative first, at each position, so "network drive" is one
/// idea before "network" is looked at alone.
struct Alternatives {
    rows: Vec<(Vec<String>, String)>,
}

impl Alternatives {
    fn new() -> Alternatives {
        let mut rows = Vec::new();
        for row in ALTERNATIVES {
            let canon = stems(row[0]).join("_");
            for phrase in *row {
                rows.push((stems(phrase), canon.clone()));
            }
        }
        rows.sort_by_key(|r| std::cmp::Reverse(r.0.len()));
        Alternatives { rows }
    }

    fn apply(&self, toks: Vec<String>) -> Vec<String> {
        let mut out = Vec::with_capacity(toks.len());
        let mut i = 0;
        'next: while i < toks.len() {
            for (phrase, canon) in &self.rows {
                if !phrase.is_empty() && toks[i..].starts_with(phrase) {
                    out.push(canon.clone());
                    i += phrase.len();
                    continue 'next;
                }
            }
            out.push(toks[i].clone());
            i += 1;
        }
        out
    }
}

fn terms(alt: &Alternatives, s: &str) -> Vec<String> {
    alt.apply(stems(s))
}

/// Distance with swaps of neighbours counted as one slip, giving up past `max`.
fn distance(a: &[char], b: &[char], max: usize) -> Option<usize> {
    if a.len().abs_diff(b.len()) > max {
        return None;
    }
    let (n, m) = (a.len(), b.len());
    let mut rows = vec![vec![0usize; m + 1]; 3];
    for (j, v) in rows[0].iter_mut().enumerate() {
        *v = j;
    }
    for i in 1..=n {
        let (p2, p1, cur) = ((i + 1) % 3, (i + 2) % 3, i % 3);
        rows[cur][0] = i;
        let mut best = i;
        for j in 1..=m {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut v = (rows[p1][j] + 1).min(rows[cur][j - 1] + 1).min(rows[p1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                v = v.min(rows[p2][j - 2] + 1);
            }
            rows[cur][j] = v;
            best = best.min(v);
        }
        if best > max {
            return None;
        }
    }
    let d = rows[n % 3][m];
    (d <= max).then_some(d)
}

/// How well a typed term stands for a word in the pages: 0 for not at all.
fn similarity(q: &str, t: &str) -> f32 {
    if q == t {
        return 1.0;
    }
    let (ql, tl) = (q.chars().count(), t.chars().count());
    if ql >= 4 && t.starts_with(q) {
        return 0.7;
    }
    let max = match ql {
        0..=3 => 0,
        4..=6 => 1,
        _ => 2,
    };
    if max == 0 || q.chars().next() != t.chars().next() || ql.abs_diff(tl) > max {
        return 0.0;
    }
    let (a, b): (Vec<char>, Vec<char>) = (q.chars().collect(), t.chars().collect());
    match distance(&a, &b, max) {
        Some(1) => 0.75,
        Some(_) => 0.5,
        None => 0.0,
    }
}

struct Posting {
    section: u32,
    fields: u8,
    body_count: u8,
}

struct SectionData {
    page: u32,
    id: String,
    text: String,
}

pub struct Index {
    pages: Vec<String>,
    sections: Vec<SectionData>,
    vocab: Vec<String>,
    postings: Vec<Vec<Posting>>,
    alternatives: Alternatives,
}

impl Index {
    pub fn build(pages: &[Page]) -> Index {
        let alternatives = Alternatives::new();
        let mut ids: HashMap<String, usize> = HashMap::new();
        let mut vocab: Vec<String> = Vec::new();
        let mut postings: Vec<Vec<Posting>> = Vec::new();
        let mut page_ids = Vec::new();
        let mut sections = Vec::new();

        for (pi, page) in pages.iter().enumerate() {
            page_ids.push(page.id.clone());
            let title = terms(&alternatives, &page.title);
            for sec in &page.sections {
                let si = sections.len() as u32;
                let mut here: HashMap<usize, (u8, u8)> = HashMap::new();
                let mut note = |toks: Vec<String>, field: u8| {
                    for t in toks {
                        let id = *ids.entry(t.clone()).or_insert_with(|| {
                            vocab.push(t);
                            postings.push(Vec::new());
                            vocab.len() - 1
                        });
                        let e = here.entry(id).or_insert((0, 0));
                        e.0 |= field;
                        if field == F_BODY {
                            e.1 = e.1.saturating_add(1);
                        }
                    }
                };
                note(title.clone(), F_TITLE);
                note(terms(&alternatives, &sec.heading), F_HEADING);
                for q in &sec.questions {
                    note(terms(&alternatives, q), F_QUESTION);
                }
                note(terms(&alternatives, &sec.text), F_BODY);
                for (id, (fields, body_count)) in here {
                    postings[id].push(Posting { section: si, fields, body_count });
                }
                sections.push(SectionData { page: pi as u32, id: sec.id.clone(), text: sec.text.clone() });
            }
        }
        for p in &mut postings {
            p.sort_by_key(|x| x.section);
            p.shrink_to_fit();
        }
        Index { pages: page_ids, sections, vocab, postings, alternatives }
    }

    pub fn query(&self, words: &str, limit: usize) -> Vec<Hit> {
        let mut q = terms(&self.alternatives, words);
        q.sort();
        q.dedup();
        if q.is_empty() || limit == 0 || self.sections.is_empty() {
            return Vec::new();
        }
        let (total, matched) = self.scores(&q);

        let mut ranked: Vec<(usize, f32)> = (0..self.sections.len())
            .filter(|&i| matched[i] > 0)
            .map(|i| {
                let coverage = matched[i] as f32 / q.len() as f32;
                (i, total[i] * (0.5 + 0.5 * coverage))
            })
            .collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        ranked.truncate(limit);
        ranked
            .into_iter()
            .map(|(i, _)| {
                let s = &self.sections[i];
                Hit {
                    page: self.pages[s.page as usize].clone(),
                    section: s.id.clone(),
                    snippet: self.snippet(&s.text, &q),
                }
            })
            .collect()
    }

    /// For each section, how well the terms match it in all, and how many of the terms match it at all.
    fn scores(&self, q: &[String]) -> (Vec<f32>, Vec<u8>) {
        let n = self.sections.len() as f32;
        let mut total = vec![0f32; self.sections.len()];
        let mut matched = vec![0u8; self.sections.len()];

        for term in q {
            let mut best: HashMap<u32, f32> = HashMap::new();
            for (vi, v) in self.vocab.iter().enumerate() {
                let sim = similarity(term, v);
                if sim == 0.0 {
                    continue;
                }
                let posts = &self.postings[vi];
                let idf = (1.0 + n / posts.len() as f32).ln();
                for p in posts {
                    let field = field_weight(p);
                    let e = best.entry(p.section).or_insert(0.0);
                    *e = e.max(sim * idf * field);
                }
            }
            for (s, v) in best {
                total[s as usize] += v;
                matched[s as usize] = matched[s as usize].saturating_add(1);
            }
        }
        (total, matched)
    }

    /// The one sentence of the text that holds most of what was asked.
    fn snippet(&self, text: &str, q: &[String]) -> String {
        let mut best: Option<(&str, f32)> = None;
        let mut first: Option<&str> = None;
        for sentence in sentences(text) {
            first.get_or_insert(sentence);
            let toks = terms(&self.alternatives, sentence);
            let score: f32 = q
                .iter()
                .map(|t| toks.iter().map(|w| similarity(t, w)).fold(0.0, f32::max))
                .sum();
            if score > 0.0 && best.is_none_or(|b| score > b.1) {
                best = Some((sentence, score));
            }
        }
        cut(best.map(|b| b.0).or(first).unwrap_or(""))
    }
}

/// How much a word's place in a section counts: the heading, a question, the title or the body, whichever counts most.
fn field_weight(p: &Posting) -> f32 {
    let mut field = 0f32;
    if p.fields & F_HEADING != 0 {
        field = field.max(W_HEADING);
    }
    if p.fields & F_QUESTION != 0 {
        field = field.max(W_QUESTION);
    }
    if p.fields & F_TITLE != 0 {
        field = field.max(W_TITLE);
    }
    if p.fields & F_BODY != 0 {
        field = field.max(W_BODY * (1.0 + (p.body_count as f32).ln() * 0.3));
    }
    field
}

fn sentences(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let bytes = text.as_bytes();
    let mut push = |a: usize, b: usize| {
        let s = text[a..b].trim_matches(|c: char| c.is_whitespace() || "#*->|".contains(c));
        if !s.is_empty() {
            out.push(s);
        }
    };
    for (i, c) in text.char_indices() {
        let end = i + c.len_utf8();
        let stop = match c {
            '\n' => true,
            '.' | '!' | '?' => end == bytes.len() || bytes[end].is_ascii_whitespace(),
            _ => false,
        };
        if stop {
            push(start, end);
            start = end;
        }
    }
    push(start, text.len());
    out
}

fn cut(s: &str) -> String {
    let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() <= SNIPPET_CHARS {
        return s;
    }
    let head: String = s.chars().take(SNIPPET_CHARS - 1).collect();
    let head = match head.rfind(' ') {
        Some(i) if i > SNIPPET_CHARS / 2 => &head[..i],
        _ => &head,
    };
    format!("{}\u{2026}", head.trim_end_matches(|c: char| !c.is_alphanumeric()))
}

