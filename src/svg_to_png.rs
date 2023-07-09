use anyhow::anyhow;
use anyhow::Result;
use resvg;
use resvg::tiny_skia;
use resvg::usvg;
use resvg::usvg::TreeParsing;

pub fn svg_to_png(svg_data: &Vec<u8>) -> Result<Vec<u8>> {
    let rtree = resvg::Tree::from_usvg(
        &usvg::Tree::from_data(&svg_data, &usvg::Options::default())
            .map_err(|e| anyhow!("{}", e))?,
    );
    let pixmap_size = rtree.size.to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or(anyhow!("Failed to create a pixmap"))?;
    rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap.encode_png().map_err(|e| e.into())
}
