// This is responsible for rendering views to PNGs.
// This deals with the aspect ratio issues and expanding to a visible size.

use crate::view::{Loop, Cel};
use crate::png;
use crate::palette::{PALETTE, TRANSPARENT};
use crate::xbrz;

// The game is originally rendered at 320x200 on a 4:3 screen, so pixels are 1.2x higher than wide.
// Resizing at 5w x 6h preserves this ratio.
const WIDTH_MULTIPLIER: usize = 5;
const HEIGHT_MULTIPLIER: usize = 6;

// It's eligible to be an animation even if sizes are different.
// Padding is added to the top and right, which seems to align cels nicely on space quest.
pub fn is_animation(viewloop: &Loop) -> bool {
    viewloop.cels.len() >= 2
}

pub fn apng_from_loop(viewloop: &Loop) -> Vec<u8> {
    // Get max height.
    let width = viewloop.cels.iter().map(|c| c.width).max().unwrap();
    let height = viewloop.cels.iter().map(|c| c.height).max().unwrap();
    let frames: Vec<Vec<u32>> = viewloop.cels.iter()
        .map(|c| pad_cel(c, width, height))
        .map(|c| scaled_rgbas_from_cel(&c))
        .collect();
    png::apng_data(
        width * WIDTH_MULTIPLIER,
        height * HEIGHT_MULTIPLIER,
        &frames)
}

pub fn png_from_cel(cel: &Cel) -> Vec<u8> {
    png::png_data(
        cel.width * WIDTH_MULTIPLIER,
        cel.height * HEIGHT_MULTIPLIER,
        &scaled_rgbas_from_cel(cel))
}

// pub fn png_from_picture(picture: &picture::Picture) -> Vec<u8> {
//     let cel: Cel = Cel {
//         width: picture::WIDTH,
//         height: picture::HEIGHT,
//         pixels: picture.picture.clone(),
//     };
//     png_from_cel(&cel)
// }

// Increase the width/height of a cel.
fn pad_cel(cel: &Cel, width: usize, height: usize) -> Cel {
    if cel.width == width && cel.height == height { return cel.clone(); }
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);
    // Pad the height:
    let extra_height = height - cel.height;
    let extra_pixels_top = extra_height * width;
    pixels.extend(vec![TRANSPARENT; extra_pixels_top]);
    // Pad the width:
    let extra_width = width - cel.width;
    for row in cel.pixels.chunks_exact(cel.width) {
        pixels.extend_from_slice(row);
        for _ in 0..extra_width {
            pixels.push(TRANSPARENT);
        }
    }
    Cel { width, height, pixels }
}

// This converts an unscaled cel to scaled rgbas.
fn scaled_rgbas_from_cel(cel: &Cel) -> Vec<u32> {
    let unscaled_rgbas: Vec<u32> = cel.pixels.iter().map(|p| PALETTE[*p as usize]).collect();
    // Scale using xbrz - this only really works if the multipliers are 5 and 3:
    let bigger_dimension = HEIGHT_MULTIPLIER.max(WIDTH_MULTIPLIER);
    let scaled_square = xbrz::scale(bigger_dimension as u8, &unscaled_rgbas, cel.width as u32, cel.height as u32);
    // Now descale to the aspect we want:
    // This will only work for 5x6.
    let mut scaled_aspect: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * cel.height * HEIGHT_MULTIPLIER);
    for chunk in scaled_square.chunks_exact(bigger_dimension) {
        scaled_aspect.push(chunk[0]);
        scaled_aspect.push(chunk[1]);
        scaled_aspect.push(interpolate_rgba(chunk[2], chunk[3]));
        scaled_aspect.push(chunk[4]);
        scaled_aspect.push(chunk[5]);
    }
    scaled_aspect

    // Nearest-neighbour scaling:
    // let mut rgbas: Vec<u32> = Vec::with_capacity(cel.width * cel.height * WIDTH_MULTIPLIER * HEIGHT_MULTIPLIER);
    // for row in cel.pixels.chunks_exact(cel.width) {
    //     for _ in 0..HEIGHT_MULTIPLIER {
    //         for p in row {
    //             let rgba = PALETTE[*p as usize];
    //             for _ in 0..WIDTH_MULTIPLIER {
    //                 rgbas.push(rgba);
    //             }
    //         }
    //     }
    // }
    // rgbas
}

fn interpolate_rgba(x: u32, y: u32) -> u32 {
    let r1 = x >> 24;
    let g1 = (x >> 16) & 0xff;
    let b1 = (x >> 8) & 0xff;
    let a1 = x & 0xff;
    let r2 = y >> 24;
    let g2 = (y >> 16) & 0xff;
    let b2 = (y >> 8) & 0xff;
    let a2 = y & 0xff;
    let r = (r1 + r2) / 2;
    let g = (g1 + g2) / 2;
    let b = (b1 + b2) / 2;
    let a = (a1 + a2) / 2;
    (r << 24) + (g << 16) + (b << 8) + a
}