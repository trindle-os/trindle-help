//! trindle-help: Trindle's own Help. One small window with a search box on
//! top, the answers below it, and the section chosen to read. The pages are
//! site/content/*.md, the same writing as the website (see `pages`). It
//! works with no network and holds no more than the pages while open.
//! `trindle-help <page>#<section>` opens straight at a section; that is what
//! every "More" beside a failure message uses.

use std::collections::HashMap;
use std::rc::Rc;

use crate::toolkit::bus::{Asked, Bus, Offer, Props};
use crate::toolkit::event::Loop;
use std::cell::{Cell, RefCell};
use std::time::Duration;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, card, column, entry, even_grid, heading, icon_circle, icon_image, image_view, aspect, item, label, list, note, page_title, quiet_action, row, scroll, section_rule, side_heading, Length, Node, Tone};
use crate::toolkit::window::{Display, Kind};

use trindle_help::{clean, errors};
pub mod pages;
mod search;
mod svg;
#[cfg(test)]
mod search_tests;

use pages::{Block, Doc, Kind as BlockKind, Link};

const SIZE: (u32, u32) = (760, 640);
const NAME: &str = "org.trindle.Help";
const PATH: &str = "/org/trindle/Help";
const XML: &str = r#"<interface name="org.trindle.Help"><method name="Open"><arg type="s" direction="in"/><arg type="s" direction="in"/></method></interface>"#;
const BACK_ICON: &str = "go-previous-symbolic";
const LINK_ICON: &str = "emblem-symbolic-link";
const ALL_ICON: &str = "go-home-symbolic";
const BEFORE_ICON: &str = "go-previous-symbolic";
const AFTER_ICON: &str = "go-next-symbolic";
const NOTE_ICON: &str = "dialog-information-symbolic";
const COPY_ICON: &str = "edit-copy-symbolic";
/// The most answers the search is asked for.
const ANSWERS: usize = 12;
/// How wide the column at the side of the article is: the answers to a search, or the sections of the page.
/// The pictures whose file names start so are of this window itself, made from it: they are for the website, and Help does not draw a picture of itself in itself.
const OWN_PICTURES: &str = "errors-";
const SIDE: f32 = 260.0;
/// The most a picture drawn from a vector file is drawn, along its longer side, in pixels.
const PICTURE_MOST: u32 = 1400;
/// How wide a page's card on the front page is, at least.
const SHELF_CARD: f32 = 300.0;
/// The groups of pages in the order the front page shows them; any other follows.
const SHELVES: [&str; 3] = ["Start", "Using Trindle", "Help"];

/// The icon a page's card carries on the front page.
fn page_icon(id: &str) -> &'static str {
    match id {
        "getting-started" => "media-playback-start-symbolic",
        "quick-answers" => "help-about-symbolic",
        "ring" => "view-refresh-symbolic",
        "apps" => "folder-download-symbolic",
        "folders" => "folder-symbolic",
        "settings" => "window-new-symbolic",
        "text" => "text-x-generic-symbolic",
        "drives" => "media-floppy-symbolic",
        "games" => "media-seek-forward-symbolic",
        "lock-screen" => "changes-prevent-symbolic",
        "in-use-dots" => NOTE_ICON,
        "own-words" => "edit-select-all-symbolic",
        "plugins" => "list-add-symbolic",
        "flow" => "go-next-symbolic",
        "errors" => "dialog-warning-symbolic",
        "troubleshooting" => "dialog-error-symbolic",
        "glossary" => "document-open-symbolic",
        "report-a-problem" => "document-new-symbolic",
        _ => "text-x-generic-symbolic",
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    Typed(String),
    /// A row of the answers was chosen, by its place.
    Pick(usize),
    /// A section of the page being read was chosen, by its place in the page.
    Part(usize),
    /// Back to the front page, with every page on its shelf.
    Shelf,
    /// A link in the writing, to a page and perhaps a section.
    Go(String, Option<String>),
    Back,
    /// The button of the area written for experts: opens it, or shuts it.
    Expert,
    /// The button that brings the whole of Help back after More.
    All,
    /// Puts the real error text on the clipboard.
    CopyError,
    Close,
    /// Another copy of Help was started: the address and the failure's words.
    Aim(String, String),
}

/// Where an address given on starting leads.
#[derive(Debug, PartialEq)]
pub struct Opened {
    pub page: usize,
    pub part: usize,
    /// False when the address named something that is not in Help, so the
    /// window starts at the top page and says so.
    pub found: bool,
}

/// The page and section an address such as `folders#other-computers` names;
/// an empty address is the first page, quietly, and an unknown one is the
/// first page too, saying so.
pub fn resolve(docs: &[Doc], address: &str) -> Opened {
    let top = docs.iter().position(|d| d.id == pages::FIRST_PAGE).unwrap_or(0);
    let address = address.trim();
    if address.is_empty() {
        return Opened { page: top, part: 0, found: true };
    }
    let (page, section) = address.split_once('#').unwrap_or((address, ""));
    let page = page.strip_suffix(".html").unwrap_or(page);
    let Some(p) = docs.iter().position(|d| d.id == page) else { return Opened { page: top, part: 0, found: false } };
    if section.is_empty() {
        return Opened { page: p, part: 0, found: true };
    }
    match docs[p].parts.iter().position(|s| s.id == section) {
        Some(s) => Opened { page: p, part: s, found: true },
        None => Opened { page: top, part: 0, found: false },
    }
}

struct Helper {
    docs: Vec<Doc>,
    index: search::Index,
    typed: String,
    hits: Vec<search::Hit>,
    /// The section being read: its page and its place in the page.
    here: (usize, usize),
    /// Where the person has been, newest last.
    before: Vec<(usize, usize)>,
    /// The address given on starting, when Help does not have it.
    unknown: Option<String>,
    /// The expert area of the section read is open. Shut again on going to
    /// another section.
    expert: bool,
    /// The real words of the failure Help was opened for, cleaned.
    error: Option<String>,
    /// Opened from a failure's More: only the answer shows, with no search or
    /// list, until All of Help is pressed.
    focus: bool,
    /// The front page shows, with a card for every page, and not a section.
    shelf: bool,
    /// The section the error was opened at; the text is shown there only.
    opened_at: (usize, usize),
    copy: Option<String>,
    events: Option<Loop>,
    /// The pictures of the section being read, drawn from the file the first time they show and let go of on turning to another section; None for one that would not read.
    seen: RefCell<HashMap<String, Option<Rc<image::DynamicImage>>>>,
}

impl Helper {
    fn new(docs: Vec<Doc>, address: &str, error: &str) -> Helper {
        let index = search::Index::build(&pages::for_search(&docs));
        let mut helper = Helper {
            docs,
            index,
            typed: String::new(),
            hits: Vec::new(),
            here: (0, 0),
            before: Vec::new(),
            unknown: None,
            expert: false,
            focus: false,
            shelf: false,
            opened_at: (0, 0),
            error: None,
            copy: None,
            events: None,
            seen: RefCell::new(HashMap::new()),
        };
        helper.aim(address, error);
        helper
    }

    /// Turns to the page an address and a failure's words lead to, as on
    /// starting; also when another copy of Help is started and hands them
    /// to this one.
    /// An `Open` call from another copy: a body that does not read leaves the
    /// person's place and search as they are, and the window is only raised.
    fn asked_to_aim(&mut self, body: Option<(String, String)>) {
        if let Some((address, error)) = body {
            self.update(Msg::Aim(address, error));
        }
    }

    fn aim(&mut self, address: &str, error: &str) {
        let error = clean::error(error);
        let address = errors::address_for(address, &error);
        let mut opened = resolve(&self.docs, &address);
        if !opened.found && !error.is_empty() {
            opened = resolve(&self.docs, errors::GENERAL);
        }
        self.typed = String::new();
        self.hits = Vec::new();
        self.seen.borrow_mut().clear();
        self.here = (opened.page, opened.part);
        self.before = Vec::new();
        self.unknown = (!opened.found).then(|| address.trim().to_string());
        self.expert = false;
        self.focus = opened.found && !address.trim().is_empty();
        self.shelf = !self.focus;
        self.opened_at = (opened.page, opened.part);
        self.error = (!error.is_empty()).then_some(error);
    }

    /// A picture of the pages, drawn from its bytes once for as long as the section stays.
    fn picture(&self, name: &str) -> Option<Rc<image::DynamicImage>> {
        self.seen.borrow_mut().entry(name.to_string()).or_insert_with(|| {
            let bytes = pages::image_bytes(name)?;
            let img = if name.ends_with(".svg") {
                svg::draw(bytes, PICTURE_MOST)
            } else {
                image::load_from_memory_with_format(bytes, image::ImageFormat::Png).ok()
            };
            img.map(Rc::new)
        }).clone()
    }

    fn go(&mut self, page: usize, part: usize) {
        if (page, part) != self.here {
            self.seen.borrow_mut().clear();
            self.before.push(self.here);
            self.here = (page, part);
            self.expert = false;
        }
        self.unknown = None;
        self.shelf = false;
    }

    /// The place of a page and section by their ids.
    fn find(&self, page: &str, section: Option<&str>) -> Option<(usize, usize)> {
        let p = self.docs.iter().position(|d| d.id == page)?;
        let s = match section {
            Some(s) => self.docs[p].parts.iter().position(|x| x.id == s)?,
            None => 0,
        };
        Some((p, s))
    }

    /// What the answers show: the sections that answer what was typed.
    fn rows(&self) -> Vec<(usize, usize)> {
        if self.typed.trim().is_empty() {
            return Vec::new();
        }
        self.hits.iter().filter_map(|h| self.find(&h.page, Some(&h.section))).collect()
    }

    fn block(&self, b: &Block) -> Node<Msg> {
        let mut parts = Vec::new();
        match b.kind {
            BlockKind::Paragraph => parts.push(label(b.lines.join(" ")).width(Length::Fill)),
            BlockKind::Note => parts.push(
                card(row(vec![icon_image(NOTE_ICON, theme::sheet().icon, Tone::Accent), label(b.lines.join(" ")).width(Length::Fill)]).gap(12.0).width(Length::Fill)).width(Length::Fill),
            ),
            BlockKind::Code => parts.push(card(label(b.lines.join("\n")).width(Length::Fill)).width(Length::Fill)),
            BlockKind::Picture if b.picture.starts_with(OWN_PICTURES) => {}
            BlockKind::Picture => match self.picture(&b.picture) {
                Some(img) => {
                    let (w, h) = image::GenericImageView::dimensions(img.as_ref());
                    let ratio = w as f32 / h.max(1) as f32;
                    parts.push(card(aspect(ratio, image_view(img, b.alt.clone(), (w as f32, h as f32)))).flat().width(Length::Fill));
                    parts.push(note(b.lines.join(" ")).width(Length::Fill));
                }
                None => parts.push(note(t!("help-picture", caption = b.lines.join(" "))).width(Length::Fill)),
            },
            BlockKind::Bullets | BlockKind::Steps => {
                for (n, line) in b.lines.iter().enumerate() {
                    let mark = if b.kind == BlockKind::Steps { format!("{}.", n + 1) } else { "-".to_string() };
                    parts.push(row(vec![label(mark), label(line.clone()).width(Length::Fill)]).gap(8.0).width(Length::Fill));
                }
            }
        }
        if !b.links.is_empty() {
            parts.push(crate::toolkit::widget::flow(b.links.iter().filter_map(|l| self.link(l)).collect()));
        }
        column(parts).gap(8.0).width(Length::Fill)
    }

    /// A worded button to a link's page or section, saying where it leads.
    fn link(&self, l: &Link) -> Option<Node<Msg>> {
        let (p, s) = self.find(&l.page, l.section.as_deref())?;
        let name = if s == 0 { self.docs[p].title.clone() } else { format!("{}, {}", self.docs[p].title, self.docs[p].parts[s].heading) };
        Some(quiet_action(LINK_ICON, l.words.clone()).called(t!("help-go", title = name)).on_press(Msg::Go(l.page.clone(), l.section.clone())))
    }

    /// The section being read, as an article: where it stands, its name, and
    /// its writing, with the way to the sections on either side of it.
    fn reading(&self) -> Node<Msg> {
        let (p, s) = self.here;
        let doc = &self.docs[p];
        let part = &doc.parts[s];
        let eyebrow = if s == 0 { doc.group.clone() } else { format!("{}  \u{203a}  {}", doc.group, doc.title) };
        let mut body = vec![note(eyebrow), page_title(part.heading.clone())];
        if s == 0 && !doc.summary.is_empty() {
            body.push(heading(doc.summary.clone()).width(Length::Fill));
        }
        body.push(section_rule());
        if let Some(error) = self.error.as_ref().filter(|_| self.here == self.opened_at) {
            body.push(card(column(vec![
                heading(t!("help-error")),
                row(vec![label(error.clone()).width(Length::Fill), button(COPY_ICON, t!("help-error-copy")).on_press(Msg::CopyError)]).gap(8.0).width(Length::Fill),
            ]).gap(8.0).width(Length::Fill)).width(Length::Fill));
        }
        body.extend(part.blocks.iter().map(|b| self.block(b)));
        if !part.expert.is_empty() {
            let tip = if self.expert { t!("help-expert-shut") } else { t!("help-expert-open") };
            let icon = if self.expert { "go-up-symbolic" } else { "go-down-symbolic" };
            body.push(row(vec![label(t!("help-expert")).width(Length::Fill), button(icon, tip).on(self.expert).on_press(Msg::Expert)]).gap(8.0).width(Length::Fill));
            if self.expert {
                body.extend(part.expert.iter().map(|b| self.block(b)));
            }
        }
        let mut ways = Vec::new();
        if s > 0 {
            ways.push(self.way(BEFORE_ICON, p, s - 1));
        }
        if s + 1 < doc.parts.len() {
            ways.push(self.way(AFTER_ICON, p, s + 1));
        }
        if !ways.is_empty() && !self.focus {
            body.push(section_rule());
            body.push(row(ways).gap(12.0));
        }
        scroll(format!("help-{}-{}", doc.id, part.id), column(body).gap(12.0).width(Length::Fill).padding(theme::sheet().gap))
    }

    /// A button to the section before or after the one read, named by it.
    fn way(&self, icon: &str, page: usize, part: usize) -> Node<Msg> {
        let doc = &self.docs[page];
        let words = doc.parts[part].heading.clone();
        quiet_action(icon, words.clone()).called(t!("help-go", title = words)).on_press(Msg::Go(doc.id.clone(), Some(doc.parts[part].id.clone())))
    }

    /// The pages, on shelves by their group: a card for each with its icon, name, what it is for and how much there is.
    fn shelves(&self) -> Node<Msg> {
        let mut groups: Vec<&str> = SHELVES.iter().copied().filter(|g| self.docs.iter().any(|d| d.group == *g)).collect();
        for d in &self.docs {
            if !groups.contains(&d.group.as_str()) {
                groups.push(&d.group);
            }
        }
        let mut out = Vec::new();
        for g in groups {
            let cards = self.docs.iter().enumerate().filter(|(_, d)| d.group == g).map(|(p, d)| {
                let words = column(vec![heading(d.title.clone()), note(d.summary.clone()).width(Length::Fill), note(t!("help-sections", count = d.parts.len()))]).gap(4.0).width(Length::Fill);
                card(row(vec![icon_circle(page_icon(&d.id)), words]).gap(12.0).width(Length::Fill))
                    .width(Length::Fill)
                    .called(format!("{}. {}", d.title, d.summary))
                    .on_press(Msg::Go(d.id.clone(), None))
                    .key(format!("shelf-{p}"))
            }).collect();
            out.push(column(vec![side_heading(if g.is_empty() { t!("help-title").to_string() } else { g.to_string() }), even_grid(SHELF_CARD, cards)]).gap(8.0).width(Length::Fill));
        }
        scroll("help-shelves", column(out).gap(20.0).width(Length::Fill).padding(theme::sheet().gap))
    }

    /// Down the side while reading: the answers to what was typed, or the sections of the page.
    fn side(&self) -> Node<Msg> {
        if !self.typed.trim().is_empty() {
            return self.answers();
        }
        let doc = &self.docs[self.here.0];
        let items = doc.parts.iter().enumerate().map(|(n, part)| item(if n == 0 { t!("help-overview").to_string() } else { part.heading.clone() })).collect();
        column(vec![
            side_heading(doc.title.clone()),
            list(items, Some(self.here.1), Msg::Part).called(t!("help-in-this-page")).key(format!("help-parts-{}", doc.id)).width(Length::Fill).height(Length::Fill),
        ]).gap(8.0).width(Length::Fixed(SIDE)).height(Length::Fill)
    }

    fn answers(&self) -> Node<Msg> {
        let rows = self.rows();
        let chosen = rows.iter().position(|r| *r == self.here);
        let items = rows.iter().enumerate().map(|(n, (p, s))| {
            let doc = &self.docs[*p];
            let mut it = item(doc.parts[*s].heading.clone()).detail(doc.title.clone());
            if let Some(h) = self.hits.get(n) {
                it = it.under(h.snippet.clone());
            }
            it
        }).collect();
        let l = list(items, chosen, Msg::Pick).called(t!("help-contents")).key("help-answers").width(Length::Fill).row_height(theme::sheet().rich_row).height(Length::Fill);
        column(vec![side_heading(t!("help-answers")), l]).gap(8.0).width(Length::Fixed(SIDE + 50.0)).height(Length::Fill)
    }
}

impl App for Helper {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Typed(text) => {
                self.hits = self.index.query(&text, ANSWERS);
                self.typed = text;
                self.unknown = None;
                if let Some(first) = self.rows().first().filter(|_| !self.typed.trim().is_empty()) {
                    let first = *first;
                    self.go(first.0, first.1);
                }
                if !self.typed.trim().is_empty() {
                    self.shelf = false;
                }
            }
            Msg::Part(n) => {
                if n < self.docs[self.here.0].parts.len() {
                    self.go(self.here.0, n);
                }
            }
            Msg::Shelf => {
                self.shelf = true;
                self.typed = String::new();
                self.hits = Vec::new();
            }
            Msg::Pick(n) => {
                if let Some((p, s)) = self.rows().get(n).copied() {
                    self.go(p, s);
                }
            }
            Msg::Go(page, section) => {
                if let Some((p, s)) = self.find(&page, section.as_deref()) {
                    self.go(p, s);
                }
            }
            Msg::Back => {
                if let Some(at) = self.before.pop() {
                    self.here = at;
                    self.expert = false;
                }
            }
            Msg::Expert => self.expert = !self.expert,
            Msg::All => {
                self.focus = false;
                self.shelf = true;
            }
            Msg::CopyError => self.copy = self.error.clone(),
            Msg::Aim(address, error) => self.aim(&address, &error),
            Msg::Close => {
                if let Some(events) = &self.events {
                    events.quit();
                }
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let mut back = button(BACK_ICON, t!("help-back"));
        if !self.before.is_empty() {
            back = back.on_press(Msg::Back);
        }
        let pad = theme::sheet().page_padding;
        if self.focus {
            let all = button(ALL_ICON, t!("help-all")).on_press(Msg::All);
            return column(vec![row(vec![back, all]).gap(12.0), self.reading()]).gap(12.0).padding(pad);
        }
        let mut top = vec![back];
        if !self.shelf {
            top.push(button(ALL_ICON, t!("help-all")).on_press(Msg::Shelf));
        }
        top.push(entry(self.typed.clone(), t!("help-search"), Msg::Typed).first().width(Length::Fill));
        let top = row(top).gap(12.0);
        let searching = !self.typed.trim().is_empty();
        let under = if let Some(address) = &self.unknown {
            Some(note(t!("help-address-unknown", address = address.as_str())).tone(Tone::Warn))
        } else if searching && self.hits.is_empty() {
            Some(note(t!("help-found-none", typed = self.typed.trim())))
        } else {
            None
        };
        let mut page = Vec::new();
        if self.shelf && !searching {
            page.extend([page_title(t!("help-title")), top, label(t!("help-lead")).width(Length::Fill)]);
            page.extend(under);
            page.push(self.shelves());
        } else {
            page.push(top);
            page.extend(under);
            page.push(row(vec![self.side(), self.reading()]).gap(16.0).width(Length::Fill).height(Length::Fill));
        }
        column(page).gap(12.0).padding(pad)
    }

    fn to_clipboard(&mut self) -> Option<String> {
        self.copy.take()
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        match input {
            Input::Key { sym: Keysym::Escape, .. } => Some(Msg::Close),
            _ => None,
        }
    }
}

pub fn main(args: Vec<String>) -> i32 {
    // Trindle's tests ask which section a failure's words lead to: it is printed, nothing opens.
    if args.get(1).map(String::as_str) == Some("--classify") {
        let text = args.get(2).map(String::as_str).unwrap_or_default();
        return match trindle_help::classify(text) {
            Some(address) => {
                println!("{address}");
                0
            }
            None => 1,
        };
    }
    let events = Loop::new();
    let gone = events.clone();
    let address = args.get(1).cloned().unwrap_or_default();
    let error = args.get(2).cloned().unwrap_or_default();
    let Ok(bus) = Bus::session(&events) else {
        let Some(_shown) = window(&events, &gone, &address, &error) else { return 1 };
        events.run();
        return 0;
    };
    let code = Rc::new(Cell::new(0));
    let kept = Rc::new(RefCell::new(None));
    let (ev, b, k, c) = (events.clone(), bus.clone(), kept.clone(), code.clone());
    let (ev2, b2, ev3) = (events.clone(), bus.clone(), events.clone());
    let (a2, e2) = (address.clone(), error.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        let Some(shown) = window(&ev, &gone, &address, &error) else {
            c.set(1);
            ev.quit();
            return;
        };
        let w = Rc::downgrade(&shown);
        let served = b.serve(PATH, Offer {
            name: NAME.into(),
            xml: XML.into(),
            call: Box::new(move |asked: Asked| {
                if let Some(shown) = w.upgrade() {
                    let body = asked.message().body().deserialize::<(String, String)>().ok();
                    shown.change(|h| h.asked_to_aim(body));
                    shown.window.raise();
                }
                asked.reply(&());
            }),
            props: Box::new(Props::new),
            set: None,
        }).ok();
        let raise = crate::toolkit::once::serve_window(&b, PATH, &shown.window).ok();
        *k.borrow_mut() = Some((shown, served, raise));
    }, move || {
        b2.call(NAME, PATH, NAME).method("Open", &(a2, e2), false, Some(Duration::from_secs(2)), &ev2, move |_| ev3.quit());
    });
    events.run();
    drop(kept);
    code.get()
}

/// Opens the one window of Help, at the page the address leads to.
fn window(events: &Loop, gone: &Loop, address: &str, error: &str) -> Option<Rc<Shown<Helper>>> {
    let display = Display::connect(events, { let gone = gone.clone(); move || gone.quit() })?;
    let mut app = Helper::new(pages::all(), address, error);
    app.events = Some(events.clone());
    let kind = Kind::Window { title: t!("help-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());
    Some(shown)
}

#[cfg(test)]
mod rules;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod page_shots_test;
