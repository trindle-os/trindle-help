//! Pictures of Help opened by a More beside a failure, for the errors page: the real window at the real section with the real error text.

use super::*;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::Ui;

/// The pictures of Help at an error that the errors page shows: the file name's end, and the failure's words.
const ERRORS: [(&str, &str); 8] = [
    ("package-configure", "dpkg: error processing package example-driver (--configure):\n installed example-driver package post-installation script subprocess returned error exit status 1"),
    ("driver-build", "Error! Bad return status for module build on kernel: 6.12.0-example-amd64 (x86_64)\nConsult /var/lib/dkms/example/1.0/build/make.log for more information.\ndkms autoinstall on 6.12.0-example-amd64 failed for example(10)"),
    ("no-space", "E: Write error - write (28: No space left on device)"),
    ("locked", "E: Could not get lock /var/lib/dpkg/lock-frontend. It is held by process 1234 (apt)"),
    ("download", "E: Failed to fetch http://deb.example.org/pool/main/e/example/example_1.0_amd64.deb  404  Not Found [IP: 192.0.2.1 80]"),
    ("kept-back", "The following packages have been kept back:\n  example-app example-tools"),
    ("dependencies", "The following packages have unmet dependencies:\n example-app : Depends: libexample1 (>= 2.0) but 1.5 is installed"),
    ("unlisted", "example-tool: the update stopped with code 42"),
];

/// Drawn in the built-in English only, so an installed copy of Help's words or another language never changes the pictures.
fn errors_picture(error: &str) -> tiny_skia::Pixmap {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    with_catalogue(Catalogue::new(Bundle::from_files("en", &[]), english()), || {
        let help = Helper::new(pages::all(), "", error);
        let mut ui = Ui::new(help.view(), Theme::new(false, "#fbc711"));
        let w = SIZE.0;
        let h = (ui.natural_height(w as f32).ceil() as u32).clamp(SIZE.1, 1500);
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix
    })
}

/// The pictures on the errors page are of this very window, so they are made from it and kept the same as it draws.
#[test]
fn the_pictures_of_help_at_an_error_are_what_the_window_draws_now() {
    let img = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("site/content/img");
    let stale: Vec<String> = ERRORS.iter().filter_map(|(name, error)| {
        let kept = tiny_skia::Pixmap::load_png(img.join(format!("errors-{name}.png"))).ok();
        (kept.as_ref().map(|k| k.data()) != Some(errors_picture(error).data())).then(|| format!("errors-{name}.png"))
    }).collect();
    assert!(stale.is_empty(), "out of date: {stale:?}. Make them again with: TRINDLE_SHOTS=site/content/img cargo test --release shoot_help_for_errors -- --ignored");
}

#[test]
#[ignore = "makes the pictures of Help for the errors page"]
fn shoot_help_for_errors() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-help".into()));
    std::fs::create_dir_all(&out).unwrap();
    for (name, error) in ERRORS {
        errors_picture(error).save_png(out.join(format!("errors-{name}.png"))).unwrap();
    }
}

#[test]
#[ignore = "makes pictures of Help's front page, a page being read and a search"]
fn shoot_help_screens() {
    let shot = |name: &str, address: &str, typed: &str| {
        let mut help = Helper::new(pages::all(), address, "");
        if !typed.is_empty() {
            help.update(Msg::Typed(typed.into()));
        }
        let mut ui = Ui::new(help.view(), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-help".into()));
        std::fs::create_dir_all(&out).unwrap();
        pix.save_png(out.join(format!("screen-{name}.png"))).unwrap();
    };
    shot("home", "", "");
    let mut help = Helper::new(pages::all(), "", "");
    help.update(Msg::Go("getting-started".into(), None));
    let mut ui = Ui::new(help.view(), Theme::new(false, "#fbc711"));
    let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    pix.save_png("target/shots-help/screen-reading.png").unwrap();
    shot("search", "", "why won't my NAS open");
    shot("picture", "folders#the-right-click-menu", "");
    shot("drawn", "in-use-dots", "");
    shot("steps", "getting-started#five-steps-from-download-to-desktop", "");
}
