//! The drawn pictures of the pages (the SVG files), with their words: the toolkit's own reader draws no words, so Help reads them here, with Trindle Sans.

use std::sync::{Arc, OnceLock};

use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{fontdb::Database, ImageHrefResolver, Options, Tree};

/// The fonts of the computer, found once.
fn fonts() -> Arc<Database> {
    static FONTS: OnceLock<Arc<Database>> = OnceLock::new();
    FONTS.get_or_init(|| {
        let mut db = Database::new();
        db.load_system_fonts();
        db.set_sans_serif_family("Trindle Sans");
        Arc::new(db)
    }).clone()
}

/// An SVG drawn at its own size, or smaller to fit `most` pixels along its longer side. Nothing it names is followed: a picture of the pages holds all it shows.
pub fn draw(data: &[u8], most: u32) -> Option<image::DynamicImage> {
    let opt = Options {
        resources_dir: None,
        image_href_resolver: ImageHrefResolver { resolve_data: Box::new(|_, _, _| None), resolve_string: Box::new(|_, _| None) },
        fontdb: fonts(),
        ..Options::default()
    };
    let tree = Tree::from_data(data, &opt).ok()?;
    let size = tree.size();
    let k = (most as f32 / size.width().max(size.height())).clamp(f32::MIN_POSITIVE, 1.0);
    let (w, h) = ((size.width() * k).round().max(1.0) as u32, (size.height() * k).round().max(1.0) as u32);
    let mut px = Pixmap::new(w, h)?;
    resvg::render(&tree, Transform::from_scale(k, k), &mut px.as_mut());
    let rgba = px.pixels().iter().flat_map(|p| {
        let c = p.demultiply();
        [c.red(), c.green(), c.blue(), c.alpha()]
    }).collect();
    image::RgbaImage::from_raw(w, h, rgba).map(image::DynamicImage::ImageRgba8)
}
