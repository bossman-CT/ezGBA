use std::path::Path;

use slot_gfx::{Draw, TexId, OUT_H, OUT_W};

use crate::art;
use crate::slot_chrome::scrim;

/// How much of the picture is taken back out again. 0.62 was tuned for a full-height
/// row sitting directly on top of the picture, needing heavy contrast to read over it;
/// the row now stands in its own clear band below the picture (see SHELF_ROW_LOWER in
/// app.rs) with nothing drawn over the artwork itself, so that much dimming only muted
/// a picture nothing was competing with. Kept low rather than at 0.0 so the theme's own
/// scrim colour still ties the picture into the rest of the card's palette.
const SCRIM: f32 = 0.18;

/// Cover the whole panel, centre cropped. PNG only, as the labels are.
pub fn wallpaper_face(path: &Path) -> Option<Vec<u8>> {
    art::cover(path, OUT_W, OUT_H)
}

/// The picture and the scrim over it, as the first two things on the screen. Nothing at all
/// when the card carries no wallpaper: the clear colour is already the ground.
pub fn draw_backdrop(face: Option<TexId>, out: &mut Vec<Draw>) {
    let Some(tex) = face else {
        return;
    };
    out.push(Draw::Tex {
        x: 0.0,
        y: 0.0,
        w: OUT_W as f32,
        h: OUT_H as f32,
        tex,
        alpha: 1.0,
    });
    let [r, g, b] = scrim();
    out.push(Draw::Rect {
        x: 0.0,
        y: 0.0,
        w: OUT_W as f32,
        h: OUT_H as f32,
        colour: [r, g, b, SCRIM],
    });
}
