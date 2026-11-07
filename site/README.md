# Trindle help site

The documentation website, and the pages Help shows (one set of pages for both): plain static files, no framework, no tracking, no cookies, no outside fonts or scripts.

## Folders

| Folder | What is in it |
| --- | --- |
| `site/content/` | One Markdown file per page, with a few lines of front matter (`title`, `summary`, `group`, `order`). Pictures are in `site/content/img/`. |
| `site/static/` | `style.css` (the colours are written into it from the build script), `search.js` and `_headers`. |
| `tools/site/docs.rs` | The one generator. One Rust file, standard library only, built with `rustc` on its own (it is not part of the Trindle program). |
| `site/dist/` | The output. Not committed. |

The Features page is not written by hand: it is built from Trindle's features list, which the maintainers keep with Trindle (`spec/features.md` there; `--features FILE` or `TRINDLE_FEATURES` says where) each time, so a feature added there shows up on the next build, with its state (In Trindle now, or Being built).

## Build

```
rustc --edition 2021 -O tools/site/docs.rs -o target/site-docs
target/site-docs --check --features ../trindle/spec/features.md
```

Run both from the repository root.

Links: menus, the contents, cards and link lists have no underline. A link never sits inside a sentence; write it on a line of its own, as a list item (`- [Read about Folders](folders.html)`, or `- [Folders](folders.html): why you would read it`). If one must stay in a sentence it is marked `class="in"` automatically: bold at rest and underlined on hover and keyboard focus, never colour alone. `--check` prints every colour pair with its ratio and fails if a pair is below 4.5:1 (3:1 for the focus ring and the search box border), if a heading level is skipped, if a page has no skip link, if a picture has no alt text, if the search result count is not announced, or if an in-sentence link has no cue other than colour.

`--check` also fails the build for: a picture with no alt text, a missing picture or page, a banned word, an em dash, a heading not in sentence case, a colour pair below WCAG 2.2 AA contrast, American spellings and emoji. Leave it off for a quick build.

To look at it, open `site/dist/index.html` in a browser. All links are relative, so it works from a folder on disk.

Other scripts in `tools/site/`:

- `sh tools/site/search-test.sh` runs the built search page in a headless browser for a few questions.
- `sh tools/site/preview-shots.sh OUTDIR` takes pictures of the home page, a page, the search and a narrow page with a headless browser.

## Other languages

```
target/site-docs --translations ../trindle-translations [--lang fr]
```

Builds one more site per language into `site/dist/CODE/`. A page comes from `CODE/docs/PAGE.md` when it is there, else from English; the diagrams get their words from `CODE/docs/img/NAME.ftl` (messages `title`, `desc`, `label-01`, `label-02`, ...). Every fallback is listed in the output. Tests: `rustc --edition 2021 --test tools/site/docs.rs -o target/docs-test && target/docs-test`.

## Search

`search-index.js` is built with the pages (title, summary, headings and text) and read by `search.js` on the search page. The header form sends everything to `search.html?q=...`. Without JavaScript the search page shows a contents list of every page. With it, results appear as you type. Down from the box moves to the first result, Up and Down move between results, and `/` jumps to the box from anywhere on a page.

## Pictures

Two kinds, and each is said in its caption.

- **Real windows**, drawn without a screen by the programs' own picture tests (never on a real screen). Help's own are in this repository (`shoot_help_for_errors`, run with `--ignored` and `TRINDLE_SHOTS` naming a folder). The pictures of other programs' screens are made by those programs' own tests in the Trindle repository, run with `--ignored` and `TRINDLE_SHOTS` (or `SITE_SHOTS`, for the ring) naming a folder, and copied here: `shoot_every_apps_page` (Apps), `shoot_ring_pages` (the ring), `shoot_every_page` (Settings), `shoot_the_window` in `disks` (Drives), `shoot_the_report_window` and `shoot_a_found_word_that_is_selected` (Text). Copy the ones you want into `site/content/img/`.
- **Diagrams**, drawn as SVG by `python3 tools/site/diagrams.py` (it rewrites every `site/content/img/*.svg`). They are narrow (480 wide) so the words stay readable on a phone, use the site's dark colours, and each has a title and description inside it. Use a diagram, and say so in the caption, where a window cannot be pictured without a screen (the lock screen, which the compositor draws) or does not exist yet (Flow). Never fake a screenshot.

Use a picture in a page like this. The alt text says what the picture shows, and the caption starts with "What you will see":

```
![What the picture shows, in a sentence.](img/name.png "What you will see: one short line.")
```

## Writing for the pages

The help is for a person who is not technical. Short sentences, everyday words, one idea to a paragraph, steps as numbered lists, and a one-line explanation of any word that needs one (add it to the Glossary). `--check` holds this: it fails a page with more than 300 words in a row without a picture or diagram, a page that talks of a screen but has no picture, a caption that does not start with "What you will see", and a sentence over 30 words. It prints the numbers for every page. Write about 250 words or fewer between pictures.

## Cloudflare Pages

Nothing here sets up or changes Cloudflare. When someone does, the project needs:

- **Build command:** `rustc --edition 2021 -O tools/site/docs.rs -o target/site-docs && target/site-docs --check`
- **Build output directory:** `site/dist`
- **Root directory:** the repository root (the build reads `spec/features.md`, which the maintainers keep outside this repository)
- **Environment:** nothing beyond Python 3 (any recent version), which the Pages build image already has.

`site/dist/_headers` carries a strict Content Security Policy (only this site's own files, no inline script or style) and other plain headers. `404.html` is used for pages that are not found.
