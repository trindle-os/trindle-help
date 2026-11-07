# Changelog

## Unreleased

Help is a program and a repository of its own, split out of Trindle's single program.

- The window opens on a front page with a card for each page on its shelf; reading shows the page's sections down the side, the article beside them, and Previous and Next buttons; links in the writing say where they lead.
- Every picture of the pages (screenshots and drawings) shows in the article, drawn from the program itself with no network; the real error text of a failure shows at the top of its section, above any picture.
- `trindle-help` is built on the Trindle Toolkit from its GitHub repository at a fixed commit, so a clean clone builds.
- One set of pages, `site/content`, makes both the program and the website (`tools/site/docs.rs`).
- Installed as a Debian package that the main Trindle package depends on; `packaging/build-deb.sh` makes it. It is one of the programs a person cannot uninstall.
- `trindle-help --classify TEXT` prints the section a failure's words lead to, for Trindle's own tests.
- The website's features page is read from a file named by `--features` or `TRINDLE_FEATURES`, as Trindle keeps that list.
