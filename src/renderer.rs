// This is responsible for rendering views to PNGs.
// This deals with the aspect ratio issues and expanding to a visible size.
// A 'dither double' is where both nibbles contain an EGA palette index.

use crate::view::{Loop, Cel};
use crate::png;
use crate::picture;
use crate::palette;
use crate::xbrz;

// The game is originally rendered at 320x200 on a 4:3 screen, so pixels are 1.2x higher than wide.
// Resizing at 5w x 6h preserves this ratio.
const WIDTH_MULTIPLIER: usize = 5;
const HEIGHT_MULTIPLIER: usize = 6;
const USE_XBRZ: bool = true;

// It's eligible to be an animation even if sizes are different.
// Padding is added to the top and right, which seems to align cels nicely on space quest.
pub fn is_animation(viewloop: &Loop) -> bool {
    viewloop.cels.len() >= 2
}

// This assumes it's normal pixels, not dither-doubles.
pub fn apng_from_loop(viewloop: &Loop) -> Vec<u8> {
    // Get max height.
    let width = viewloop.cels.iter().map(|c| c.width).max().unwrap();
    let height = viewloop.cels.iter().map(|c| c.height).max().unwrap();
    let frames: Vec<Vec<u32>> = viewloop.cels.iter()
        .map(|c| pad_cel(c, width, height))
        .map(|c| scaled_rgbas_from_cel(&c, false))
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
        &scaled_rgbas_from_cel(cel, false))
}

pub fn png_from_picture(picture: &picture::Picture) -> Vec<u8> {
    let cel: Cel = Cel {
        width: picture::WIDTH,
        height: picture::HEIGHT,
        pixels: picture.picture.clone(),
    };
    png::png_data(
        cel.width * WIDTH_MULTIPLIER,
        cel.height * HEIGHT_MULTIPLIER,
        &scaled_rgbas_from_cel(&cel, true))
}

// Increase the width/height of a cel.
// This assumes it's not using 'dither double' pixels.
fn pad_cel(cel: &Cel, width: usize, height: usize) -> Cel {
    if cel.width == width && cel.height == height { return cel.clone(); }
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);
    // Pad the height:
    let extra_height = height - cel.height;
    let extra_pixels_top = extra_height * width;
    pixels.extend(vec![palette::TRANSPARENT; extra_pixels_top]);
    // Pad the width:
    let extra_width = width - cel.width;
    for row in cel.pixels.chunks_exact(cel.width) {
        pixels.extend_from_slice(row);
        for _ in 0..extra_width {
            pixels.push(palette::TRANSPARENT);
        }
    }
    Cel { width, height, pixels }
}

// This converts an unscaled cel to scaled rgbas.
fn scaled_rgbas_from_cel(cel: &Cel, is_dither_double: bool) -> Vec<u32> {
    if USE_XBRZ {
        scaled_rgbas_from_cel_xbrz(cel, is_dither_double)
    } else {
        scaled_rgbas_from_cel_nearest_neighbour(cel, is_dither_double)
    }
}

fn scaled_rgbas_from_cel_nearest_neighbour(cel: &Cel, is_dither_double: bool) -> Vec<u32> {
    let mut rgbas: Vec<u32> = Vec::with_capacity(cel.width * cel.height * WIDTH_MULTIPLIER * HEIGHT_MULTIPLIER);
    for row in cel.pixels.chunks_exact(cel.width) {
        for _ in 0..HEIGHT_MULTIPLIER {
            for p in row {
                let rgba = rgba_from_indexed_colour(*p, is_dither_double);
                for _ in 0..WIDTH_MULTIPLIER {
                    rgbas.push(rgba);
                }
            }
        }
    }
    rgbas
}

fn scaled_rgbas_from_cel_crt(cel: &Cel, is_dither_double: bool) -> Vec<u32> {
    if WIDTH_MULTIPLIER == 6 && HEIGHT_MULTIPLIER == 6 {
        scaled_rgbas_from_cel_crt_6(cel, is_dither_double)
    } else if WIDTH_MULTIPLIER == 3 && HEIGHT_MULTIPLIER == 3 {
        scaled_rgbas_from_cel_crt_3(cel, is_dither_double)
    } else {
        panic!("CRT scaling only supports 6x6 or 3x3!");
    }
}

fn scaled_rgbas_from_cel_crt_6(cel: &Cel, is_dither_double: bool) -> Vec<u32> {
    // Convert into this pattern:
    // rrggbb
    // rrggbb
    // gbbrrg
    // gbbrrg
    // rrggbb
    // rrggbb
    assert!(WIDTH_MULTIPLIER == 6);
    assert!(HEIGHT_MULTIPLIER == 6);
    let mut rgbas: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * cel.height * HEIGHT_MULTIPLIER);
    let mut line_rgb: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER);
    let mut line_gbr: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER);
    for (i, row) in cel.pixels.chunks_exact(cel.width).enumerate() {
        line_rgb.clear();
        line_gbr.clear();
        for pixel in row {
            let rgba = rgba_from_indexed_colour(*pixel, is_dither_double);
            let r = rgba & 0xff0000ff;
            let g = rgba & 0xff00ff;
            let b = rgba & 0xffff;
            line_rgb.push(r);
            line_rgb.push(r);
            line_rgb.push(g);
            line_rgb.push(g);
            line_rgb.push(b);
            line_rgb.push(b);
            line_gbr.push(g);
            line_gbr.push(b);
            line_gbr.push(b);
            line_gbr.push(r);
            line_gbr.push(r);
            line_gbr.push(g);
        }
        let is_odd = i & 1 != 0;
        if is_odd {
            rgbas.extend_from_slice(&line_rgb);
            rgbas.extend_from_slice(&line_rgb);
            rgbas.extend_from_slice(&line_gbr);
            rgbas.extend_from_slice(&line_gbr);
            rgbas.extend_from_slice(&line_rgb);
            rgbas.extend_from_slice(&line_rgb);
        } else {
            rgbas.extend_from_slice(&line_gbr);
            rgbas.extend_from_slice(&line_gbr);
            rgbas.extend_from_slice(&line_rgb);
            rgbas.extend_from_slice(&line_rgb);
            rgbas.extend_from_slice(&line_gbr);
            rgbas.extend_from_slice(&line_gbr);
        }
    }
    rgbas
}

fn scaled_rgbas_from_cel_crt_3(cel: &Cel, is_dither_double: bool) -> Vec<u32> {
    // Convert into this pattern:
    // rgb
    // brg
    // gbr
    assert!(WIDTH_MULTIPLIER == 3);
    assert!(HEIGHT_MULTIPLIER == 3);
    let mut rgbas: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * cel.height * HEIGHT_MULTIPLIER);
    let mut line_rgb: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER);
    let mut line_brg: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER);
    let mut line_gbr: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER);
    for row in cel.pixels.chunks_exact(cel.width) {
        line_rgb.clear();
        line_brg.clear();
        line_gbr.clear();
        for pixel in row {
            let rgba = rgba_from_indexed_colour(*pixel, is_dither_double);
            let r = rgba & 0xff0000ff;
            let g = rgba & 0xff00ff;
            let b = rgba & 0xffff;
            line_rgb.push(r);
            line_rgb.push(g);
            line_rgb.push(b);
            line_brg.push(b);
            line_brg.push(r);
            line_brg.push(g);
            line_gbr.push(g);
            line_gbr.push(b);
            line_gbr.push(r);
        }
        rgbas.extend_from_slice(&line_rgb);
        rgbas.extend_from_slice(&line_brg);
        rgbas.extend_from_slice(&line_gbr);
    }
    rgbas
}

fn scaled_rgbas_from_cel_xbrz(cel: &Cel, is_dither_double: bool) -> Vec<u32> {
    // Scale up using xbrz:
    let unscaled_rgbas: Vec<u32> = cel.pixels.iter().map(|p| rgba_from_indexed_colour(*p, is_dither_double)).collect();
    let bigger_dimension = HEIGHT_MULTIPLIER.max(WIDTH_MULTIPLIER);
    let scaled_square = xbrz::scale(bigger_dimension as u8, &unscaled_rgbas, cel.width as u32, cel.height as u32);
    if WIDTH_MULTIPLIER == HEIGHT_MULTIPLIER {
        return scaled_square
    }
    // For 5x6, we can tweak to get the perfect aspect ratio:
    assert!(WIDTH_MULTIPLIER == 5);
    assert!(HEIGHT_MULTIPLIER == 6);
    let mut scaled_aspect: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * cel.height * HEIGHT_MULTIPLIER);
    for chunk in scaled_square.chunks_exact(bigger_dimension) {
        scaled_aspect.push(chunk[0]);
        scaled_aspect.push(chunk[1]);
        scaled_aspect.push(interpolate_rgba(chunk[2], chunk[3]));
        scaled_aspect.push(chunk[4]);
        scaled_aspect.push(chunk[5]);
    }
    scaled_aspect
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

// This supports both cels (no dither doubles) and pics (dither doubles - each nibble has a colour).
fn rgba_from_indexed_colour(index: u8, is_dither_double: bool) -> u32 {
    if is_dither_double {
        let index_a = index & 0xf;
        let index_b = index >> 4;
        if index_a == index_b { // No dithering.
            palette::PALETTE[index_a as usize]
        } else { // Average them.
            let a = palette::PALETTE[index_a as usize];
            let b = palette::PALETTE[index_b as usize];
            interpolate_rgba(a, b)
        }
    } else {
        palette::PALETTE[index as usize]
    }
}
