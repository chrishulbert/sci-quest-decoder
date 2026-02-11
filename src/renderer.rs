// This is responsible for rendering views to PNGs.
// This deals with the aspect ratio issues and expanding to a visible size.
// A 'dither double' is where both nibbles contain an EGA palette index.

use crate::view::{Loop, Cel};
use crate::png;
use crate::picture;
use crate::palette;
use crate::scalefx;

// The game is originally rendered at 320x200 on a 4:3 screen, so pixels are 1.2x higher than wide.
// Resizing at 5w x 6h preserves this ratio.
// Since ScaleFX natively scales to 9x, some tricky code exists to take it down to 5x6, so if you
// want to use other ratios, you'll need to disable ScaleFX.
const WIDTH_MULTIPLIER: usize = 5;
const HEIGHT_MULTIPLIER: usize = 6;
const USE_SCALEFX: bool = true;

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
    if USE_SCALEFX {
        scaled_rgbas_from_cel_scalefx(cel, is_dither_double)
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

fn scaled_rgbas_from_cel_scalefx(cel: &Cel, is_dither_double: bool) -> Vec<u32> {
    assert!(WIDTH_MULTIPLIER == 5 && HEIGHT_MULTIPLIER == 6);

    // Scale to 9x:
    let unscaled_rgbas: Vec<u32> = cel.pixels.iter().map(|p| rgba_from_indexed_colour(*p, is_dither_double)).collect();
    let (_, sfx_height, sfx_pixels) = scalefx::scale9x(cel.width, cel.height, &unscaled_rgbas);

    // Shrink horizontally to 5x:
    let mut scaled_horizontal: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * sfx_height);
    for chunk in sfx_pixels.chunks_exact(9) {
        scaled_horizontal.push(interpolate_rgba(chunk[0], chunk[1]));
        scaled_horizontal.push(interpolate_rgba(chunk[2], chunk[3]));
        scaled_horizontal.push(chunk[4]); // Would it look nicer if 4 was interpolated with both 3 and 5?
        scaled_horizontal.push(interpolate_rgba(chunk[5], chunk[6]));
        scaled_horizontal.push(interpolate_rgba(chunk[7], chunk[8]));
    }

    // Scale vertically from 9x to 6x, by splitting into each 3 row triplet, and outputting 2 rows:
    let mut scaled_aspect: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * cel.height * HEIGHT_MULTIPLIER);
    let scaled_row_size = cel.width * WIDTH_MULTIPLIER;
    for triplet in scaled_horizontal.chunks_exact(scaled_row_size * 3) {
        // Split this triplet of scaled rows into each of the scaled rows:
        let row_top = &triplet[..scaled_row_size];
        let row_mid = &triplet[scaled_row_size..scaled_row_size*2];
        let row_bot = &triplet[scaled_row_size*2..];
        // Interpolate them, so the middle row gets interpolated with both the top and bottom, but half-weighted each time.
        for (top, mid) in row_top.iter().zip(row_mid) {
            scaled_aspect.push(interpolate_rgba_weighted(*top, *mid));
        }
        for (bot, mid) in row_bot.iter().zip(row_mid) {
            scaled_aspect.push(interpolate_rgba_weighted(*bot, *mid));
        }
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

// Interpolates the two colours, giving the first one double weight.
fn interpolate_rgba_weighted(x: u32, y: u32) -> u32 {
    let r1 = x >> 24;
    let g1 = (x >> 16) & 0xff;
    let b1 = (x >> 8) & 0xff;
    let a1 = x & 0xff;
    let r2 = y >> 24;
    let g2 = (y >> 16) & 0xff;
    let b2 = (y >> 8) & 0xff;
    let a2 = y & 0xff;
    let r = (r1 * 2 + r2) / 3;
    let g = (g1 * 2 + g2) / 3;
    let b = (b1 * 2 + b2) / 3;
    let a = (a1 * 2 + a2) / 3;
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
