// This is responsible for parsing 'views' which are bitmap sprites/animations.

use crate::palette::TRANSPARENT;

pub struct View {
    pub loops: Vec<Loop>,
}

impl View {
    pub fn parse(data: &[u8]) -> View {
        let count = (data[0] as usize) + ((data[1] as usize) << 8);
        let mirror_flags = (data[2] as usize) + ((data[3] as usize) << 8);
        // 4-7 is unknown.
        let mut loops: Vec<Loop> = Vec::with_capacity(count as usize);
        for i in 0..count {
            // Read the position.
            let offset = 8 + (i as usize) * 2;
            let position = (data[offset] as usize) + ((data[offset + 1] as usize) << 8);
            // Read the loop.
            let loop_data = &data[position..];
            let is_mirrored = (mirror_flags >> i) & 1 != 0;
            let view_loop = Loop::parse(loop_data, data, is_mirrored);
            loops.push(view_loop);
        }
        View{ loops }
    }
}

pub struct Loop {
    pub cels: Vec<Cel>,
}
impl Loop {
    fn parse(data: &[u8], resource: &[u8], is_mirrored: bool) -> Loop {
        let count = (data[0] as usize) + ((data[1] as usize) << 8);
        // 2-3 is unknown.
        let positions_onwards = &data[4..];
        let positions_data = &positions_onwards[..(count*2)];
        let positions: Vec<usize> = positions_data.chunks_exact(2).map(parse_2_byte_le).collect();
        let cels: Vec<Cel> = positions.iter().map(|&p| {
            Cel::parse(&resource[p..], is_mirrored)
        }).collect();
        Loop { cels }
    }
}

#[derive(Clone)]
pub struct Cel {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // EGA palette indexes.
}
impl Cel {
    fn parse(data: &[u8], is_mirrored: bool) -> Cel {
        let width = (data[0] as usize) + ((data[1] as usize) << 8);
        let height = (data[2] as usize) + ((data[3] as usize) << 8);
        let _x_placement_modifier = data[4];
        let _y_placement_modifier = data[5];
        let transparent_color = data[6];
        let image_source_data = &data[7..];
        let mut pixels: Vec<u8> = Vec::with_capacity(width * height);
        'outer: for b in image_source_data {
            let count = b >> 4;
            let raw_colour = b & 0xf;
            let colour = if raw_colour == transparent_color { TRANSPARENT } else { raw_colour };
            for _ in 0..count {
                pixels.push(colour);
                if pixels.len() >= width * height {
                    break 'outer;
                }
            }
        }
        while pixels.len() < width * height {
            pixels.push(TRANSPARENT);
        }
        if is_mirrored {
            pixels = mirror(&pixels, width);
        }
        Cel { width, height, pixels }
    }
}

fn mirror(pixels: &[u8], width: usize) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(pixels.len());
    for chunk in pixels.chunks_exact(width) {
        for p in chunk.iter().rev() {
            out.push(*p);
        }
    }
    out
}

fn parse_2_byte_le(data: &[u8]) -> usize {
    (data[0] as usize) + ((data[1] as usize) << 8)
}
