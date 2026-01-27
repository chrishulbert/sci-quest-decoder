// This is responsible for drawing pictures from the vector instructions.
// https://www.agidev.com/articles/agispec/agispecs-7.html
// https://github.com/wjp/freesci-archive/blob/master/src/gfx/resource/sci_pic_0.c#L531
// https://github.com/wjp/freesci-archive/blob/master/src/scicore/decompress01.c

use crate::palette;
use crate::palette::PALETTE;
use crate::picture_splitter;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 190;

pub struct Picture {
    pub picture: Vec<u8>,
}

impl Picture {
    pub fn parse(data: &[u8]) -> Picture {
        let actions = picture_splitter::split(data);
        let picture = draw(&actions);
        Picture { picture }
    }
}

fn draw(actions: &[picture_splitter::ActionArguments]) -> Vec<u8> {
    let mut picture: Vec<u8> = vec![palette::WHITE; WIDTH * HEIGHT];
    let mut is_drawing = false;
    let mut colour = palette::WHITE;
    let mut is_rectangle = false;
    let mut is_pattern = false;
    let mut pattern_size: u8 = 0;
    for a in actions {
        match a.action {
            // Colour:
            picture_splitter::Action::SetVisualColour => {
                is_drawing = true;
                // Sometimes this is oddly empty. Sometimes it has >1 args, i have no idea why!
                if !a.arguments.is_empty() {
                    colour = a.arguments[0];
                    if colour as usize >= PALETTE.len() {
                        colour = 0;
                    }
                }
            }
            picture_splitter::Action::DisableVisual => {
                is_drawing = false;
            }
            // Lines:
            picture_splitter::Action::LongLines => {
                if is_drawing {
                    draw_long_lines(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::MediumRelativeLines => {
                if is_drawing {
                    draw_medium_relative_lines(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::ShortRelativeLines => {
                if is_drawing {
                    draw_short_relative_lines(&mut picture, colour, &a.arguments);
                }
            }            
            // Patterns:
            picture_splitter::Action::SetPattern => {
                if a.arguments.len() == 0 {
                    //println!("SetPattern has no argument!");
                } else {
                    if a.arguments.len() > 1 {
                        //println!("SetPattern has extra arguments, len: {}!", a.arguments.len());
                    }
                    is_rectangle = a.arguments[0] & 0x10 != 0; // vs circle.
                    is_pattern = a.arguments[0] & 0x20 != 0; // vs solid.
                    pattern_size = a.arguments[0] & 7; // 0-7.
                    //println!("SetPattern: {:02x} rect {}, pattern {}, size {}", a.arguments[0], is_rectangle, is_pattern, pattern_size);
                }
            }
            picture_splitter::Action::LongPatterns => {
                if is_drawing {
                    let chunk_size = if is_pattern { 4 } else { 3 };
                    for chunk in a.arguments.chunks_exact(chunk_size) {
                        let pattern_number = if is_pattern { chunk[0] } else { 0 };
                        let after_pattern_number = if is_pattern { &chunk[1..] } else { chunk };
                        let (x, y) = xy_from_triple(after_pattern_number);
                        draw_pattern(&mut picture, colour, x, y, pattern_number as usize, pattern_size as usize, is_pattern, is_rectangle);
                    }
                }
            }
            picture_splitter::Action::MediumRelativePatterns => {
                if is_drawing {
                    if a.arguments.is_empty() { continue }
                    // Pattern number byte is only there if is_pattern is set:
                    let pattern_number = if is_pattern { a.arguments[0] } else { 0 };
                    let after_pattern_number = if is_pattern { &a.arguments[1..] } else { &a.arguments };
                    // Starting position:
                    if after_pattern_number.len() < 3 { continue }
                    let (mut x, mut y) = xy_from_triple(after_pattern_number);
                    draw_pattern(&mut picture, colour, x, y, pattern_number as usize, pattern_size as usize, is_pattern, is_rectangle);
                    // Remaining ones that are deltas:
                    let remaining_arguments = &after_pattern_number[3..];
                    let chunk_size = if is_pattern { 3 } else { 2 };
                    let chunks = remaining_arguments.chunks_exact(chunk_size);
                    for chunk in chunks {
                        let pattern_number = if is_pattern { chunk[0] } else { 0 };
                        let after_pattern_number = if is_pattern { &chunk[1..] } else { chunk };
                        // Y uses sign-magnitude:
                        let y_raw = (after_pattern_number[0] & 0x7f) as usize; 
                        let y_is_minus = after_pattern_number[0] & 0x80 > 0;
                        if y_is_minus && y_raw > y {
                            //println!("Medium relative pattern going into negative Y! {} - {}", y, y_raw);
                            continue
                        }
                        y = if y_is_minus { y - y_raw } else { y + y_raw };
                        // X uses 2s complement:
                        let x_delta = after_pattern_number[1] as i8;
                        x = ((x as isize) + (x_delta as isize)) as usize;
                        draw_pattern(&mut picture, colour, x, y, pattern_number as usize, pattern_size as usize, is_pattern, is_rectangle);
                    }
                }
            }
            picture_splitter::Action::ShortRelativePatterns => {
                if is_drawing {
                    draw_short_relative_patterns(&mut picture, &a.arguments, colour, pattern_size as usize, is_pattern, is_rectangle);
                }                
            }
            // Etc:
            picture_splitter::Action::FloodFill => {
                if is_drawing {
                    fill(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::CommandExtensions => {
                if a.arguments.is_empty() { continue }
                let command = a.arguments[0];
                match command {
                    0 => { // Set palette entry.
                        println!("Set palette entry! {}", describe_buf(&a.arguments))
                    }
                    1 => { // Set entire palette.
                        println!("Set entire palette! {}", describe_buf(&a.arguments))
                    }
                    _ => {} // Ignore monochrome / sci01 stuff.
                }                
            }
            // Unused:
            picture_splitter::Action::SetPriorityColour => {}
            picture_splitter::Action::DisablePriority => {}
            picture_splitter::Action::SetControlColour => {}
            picture_splitter::Action::DisableControl => {}
            picture_splitter::Action::End => {} // Done!
        }
    }
    picture
}

fn draw_short_relative_patterns(picture: &mut [u8], arguments: &[u8], colour: u8, pattern_size: usize, is_pattern: bool, is_rectangle: bool) {
    if arguments.is_empty() { return }
    // Pattern number byte is only there if is_pattern is set:
    let pattern_number = if is_pattern { arguments[0] } else { 0 };
    let after_pattern_number = if is_pattern { &arguments[1..] } else { &arguments };
    // Starting position:
    if after_pattern_number.len() < 3 { return }
    let (mut x, mut y) = xy_from_triple(after_pattern_number);
    draw_pattern(picture, colour, x, y, pattern_number as usize, pattern_size as usize, is_pattern, is_rectangle);
    // Remaining ones that are deltas:
    let remaining_arguments = &after_pattern_number[3..];
    let chunk_size = if is_pattern { 2 } else { 1 };
    let chunks = remaining_arguments.chunks_exact(chunk_size);
    for chunk in chunks {
        let pattern_number = if is_pattern { chunk[0] } else { 0 };
        let xy = if is_pattern { chunk[1] } else { chunk[0] };

        // X:
        let x_nibble = xy >> 4;
        let x_raw = (x_nibble & 0x7) as usize; 
        let x_is_minus = x_nibble & 8 != 0;
        if x_is_minus && x_raw > x {
            //println!("Short relative line going into negative X! {} - {}", x, x_raw);
            return
        }
        x = if x_is_minus { x - x_raw } else { x + x_raw };

        // Y:
        let y_raw = (xy & 0x7) as usize; 
        let y_is_minus = xy & 8 != 0;
        if y_is_minus && y_raw > y {
            //println!("Short relative line going into negative Y! {} - {}", y, y_raw);
            return
        }
        y = if y_is_minus { y - y_raw } else { y + y_raw };

        draw_pattern(picture, colour, x, y, pattern_number as usize, pattern_size, is_pattern, is_rectangle);
    }
}

// Hard to find SCI specs, so i'm assuming this is much like AGI:
// https://www.agidev.com/articles/agispec/agispecs-7.html
fn draw_pattern(picture: &mut [u8], colour: u8, x: usize, y: usize, pattern_number: usize, pattern_size: usize, is_pattern: bool, is_rectangle: bool) {
    let circle_0: Vec<&str> = vec![
        "X",
    ];
    let circle_1: Vec<&str> = vec![
        "XXX",
        "XXX",
        "XXX",
    ];
    let circle_2: Vec<&str> = vec![
        " XXX ",
        "XXXXX",
        "XXXXX",
        "XXXXX",
        " XXX ",
    ];
    let circle_3: Vec<&str> = vec![
        "  XXX  ",
        " XXXXX ",
        "XXXXXXX",
        "XXXXXXX",
        "XXXXXXX",
        " XXXXX ",
        "  XXX  ",
    ];
    let circle_4: Vec<&str> = vec![
        "  XXXXX  ",
        " XXXXXXX ",
        "XXXXXXXXX",
        "XXXXXXXXX",
        "XXXXXXXXX",
        "XXXXXXXXX",
        "XXXXXXXXX",
        " XXXXXXX ",
        "  XXXXX  ",
    ];
    let circle_5: Vec<&str> = vec![
        "    XXX    ",
        " XXXXXXXXX ",
        "XXXXXXXXXXX",
        "XXXXXXXXXXX",
        "XXXXXXXXXXX",
        "XXXXXXXXXXX",
        "XXXXXXXXXXX",
        "XXXXXXXXXXX",
        "XXXXXXXXXXX",
        " XXXXXXXXX ",
        "    XXX    ",
    ];
    let circle_6: Vec<&str> = vec![
        "    XXXXX    ",
        " XXXXXXXXXXX ",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        "XXXXXXXXXXXXX",
        " XXXXXXXXXXX ",
        "    XXXXX    ",
    ];
    let circle_7: Vec<&str> = vec![
        "     XXXXX     ",
        "  XXXXXXXXXXX  ",
        " XXXXXXXXXXXXX ",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        "XXXXXXXXXXXXXXX",
        " XXXXXXXXXXXXX ",
        "  XXXXXXXXXXX  ",
        "     XXXXX     ",
    ];
    let circles = vec![
        circle_0,
        circle_1,
        circle_2,
        circle_3,
        circle_4,
        circle_5,
        circle_6,
        circle_7,
    ];
    let texture: Vec<u8> = vec![ // https://sciwiki.sierrahelp.com/index.php/Picture_Resource#Patterns
        0x20, 0x94, 0x02, 0x24, 0x90, 0x82, 0xa4, 0xa2,
        0x82, 0x09, 0x0a, 0x22, 0x12, 0x10, 0x42, 0x14,
        0x91, 0x4a, 0x91, 0x11, 0x08, 0x12, 0x25, 0x10,
        0x22, 0xa8, 0x14, 0x24, 0x00, 0x50, 0x24, 0x04,
    ];
    let patterns: Vec<u8> = vec![ // Indices into the texture table.
        0x00, 0x18, 0x30, 0xc4, 0xdc, 0x65, 0xeb, 0x48,
        0x60, 0xbd, 0x89, 0x04, 0x0a, 0xf4, 0x7d, 0x6d,
        0x85, 0xb0, 0x8e, 0x95, 0x1f, 0x22, 0x0d, 0xdf,
        0x2a, 0x78, 0xd5, 0x73, 0x1c, 0xb4, 0x40, 0xa1,
        0xb9, 0x3c, 0xca, 0x58, 0x92, 0x34, 0xcc, 0xce,
        0xd7, 0x42, 0x90, 0x0f, 0x8b, 0x7f, 0x32, 0xed,
        0x5c, 0x9d, 0xc8, 0x99, 0xad, 0x4e, 0x56, 0xa6,
        0xf7, 0x68, 0xb7, 0x25, 0x82, 0x37, 0x3a, 0x51,
        0x69, 0x26, 0x38, 0x52, 0x9e, 0x9a, 0x4f, 0xa7,
        0x43, 0x10, 0x80, 0xee, 0x3d, 0x59, 0x35, 0xcf,
        0x79, 0x74, 0xb5, 0xa2, 0xb1, 0x96, 0x23, 0xe0,
        0xbe, 0x05, 0xf5, 0x6e, 0x19, 0xc5, 0x66, 0x49,
        0xf0, 0xd1, 0x54, 0xa9, 0x70, 0x4b, 0xa4, 0xe2,
        0xe6, 0xe5, 0xab, 0xe4, 0xd2, 0xaa, 0x4c, 0xe3,
        0x06, 0x6f, 0xc6, 0x4a, 0x75, 0xa3, 0x97, 0xe1,
    ];
    let texture_index = patterns[pattern_number % patterns.len()] as usize;
    let mut texture_bits = LoopingPatternBitStream::new(&texture, texture_index);
    let circle = &circles[pattern_size];
    let pixel_size = (pattern_size as usize) * 2 + 1;
    let start_x: isize = (x as isize) - (pattern_size as isize);
    let start_y: isize = (y as isize) - (pattern_size as isize);
    for yi in 0..pixel_size {
        let y = start_y + (yi as isize);
        for xi in 0..pixel_size {
            if is_pattern {
                if !texture_bits.next() {
                    continue
                }
            }
            let x = start_x + (xi as isize);
            if 0 <= x && x < (WIDTH as isize) && 0 <= y && y < (HEIGHT as isize) {
                let offset = (y as usize) * WIDTH + (x as usize);
                if is_rectangle {
                    picture[offset] = colour;
                } else { // Circle.
                    let is_part_of_circle = circle[yi].as_bytes()[xi] == b'X';
                    if is_part_of_circle {
                        picture[offset] = colour;
                    }
                }
            }
        }
    }
}

// This starts at the high bit.
pub struct LoopingPatternBitStream<'a> {
    data: &'a [u8],
    index: usize,
    bit: usize,
}
impl<'a> LoopingPatternBitStream<'a> {
    pub fn new(data: &'a [u8], index: usize) -> Self {
        Self { data, index, bit: 7 }
    }
    pub fn next(&mut self) -> bool {
        let byte = self.data[self.index % self.data.len()];
        let is_set = ((byte >> self.bit) & 1) != 0;
        if self.bit == 0 {
            self.bit = 7;
            self.index += 1;
        } else {
            self.bit -= 1;
        }
        is_set
    }
}

// Converts 3 bytes XY,XX,YY to x,y.
fn xy_from_triple(data: &[u8]) -> (usize, usize) {
    let x = (((data[0] >> 4) as usize) << 8) + (data[1] as usize);
    let y = (((data[0] & 0xf) as usize) << 8) + (data[2] as usize);
    (x, y)
}

fn draw_long_lines(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    if arguments.len() < 3 { return } // Occasionally happens.
    let (mut x, mut y) = xy_from_triple(&arguments);
    let remaining_arguments = &arguments[3..];
    let moves = remaining_arguments.chunks_exact(3);
    for coordinate in moves {
        let (new_x, new_y) = xy_from_triple(coordinate);
        draw_sierra_line(picture, colour, x, y, new_x, new_y);
        x = new_x;
        y = new_y;
    }
}

fn draw_medium_relative_lines(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    if arguments.len() < 3 { return } // Occasionally happens.
    let (mut x, mut y) = xy_from_triple(&arguments);
    let remaining_arguments = &arguments[3..];
    let moves = remaining_arguments.chunks_exact(2);
    for coordinate in moves {
        // Y uses sign-magnitude:
        let y_raw = (coordinate[0] & 0x7f) as usize; 
        let y_is_minus = coordinate[0] & 0x80 > 0;
        if y_is_minus && y_raw > y {
            //println!("Medium relative line going into negative Y! {} - {}", y, y_raw);
            return
        }
        let new_y = if y_is_minus { y - y_raw } else { y + y_raw };
        // X uses 2s complement:
        let x_delta = coordinate[1] as i8;
        let new_x = ((x as isize) + (x_delta as isize)) as usize;
        draw_sierra_line(picture, colour, x, y, new_x, new_y);
        x = new_x;
        y = new_y;
    }
}

fn draw_short_relative_lines(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    if arguments.len() < 3 { return } // Occasionally happens.
    let (mut x, mut y) = xy_from_triple(&arguments);
    let remaining_arguments = &arguments[3..];
    for coordinate in remaining_arguments {
        let x_nibble = coordinate >> 4;
        let x_raw = (x_nibble & 0x7) as usize; 
        let x_is_minus = x_nibble & 8 != 0;
        if x_is_minus && x_raw > x {
            //println!("Short relative line going into negative X! {} - {}", x, x_raw);
            return
        }
        let new_x = if x_is_minus { x - x_raw } else { x + x_raw };

        let y_raw = (coordinate & 0x7) as usize; 
        let y_is_minus = coordinate & 8 != 0;
        if y_is_minus && y_raw > y {
            //println!("Short relative line going into negative Y! {} - {}", y, y_raw);
            return
        }
        let new_y = if y_is_minus { y - y_raw } else { y + y_raw };

        draw_sierra_line(picture, colour, x, y, new_x, new_y);

        x = new_x;
        y = new_y;
    }
}

fn fill(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    //println!("Fill args: {}", arguments.len());
    for chunk in arguments.chunks_exact(3) {
        let (x, y) = xy_from_triple(chunk);
        if x >= WIDTH || y >= HEIGHT {
            //println!("Fill weird xy! {} {}", x, y);
            continue
        }
        let mut queue: Vec<(usize, usize)> = vec![(x, y)];
        while let Some(xy) = queue.pop() {
            let x = xy.0; 
            let y = xy.1; 
            let offset = y * WIDTH + x;
            if picture[offset] != palette::WHITE { continue }
            picture[offset] = colour;
            if x > 0 { queue.push((x-1, y)); } // Left.
            if x < WIDTH-1 { queue.push((x+1, y)); } // Right.
            if y > 0 { queue.push((x, y-1)); } // Up.
            if y < HEIGHT-1 { queue.push((x, y+1)); } // Down.
        }
    }
    //println!("Done fill");
}

fn draw_sierra_line(picture: &mut [u8], colour: u8, x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 >= WIDTH || y1 >= HEIGHT || x2 >= WIDTH || y2 >= HEIGHT {
        //println!("Draw line out of bounds! {},{} -> {},{}", x1, y1, x2, y2);
        return
    }
    // https://www.agidev.com/articles/agispec/agispecs-7.html
    fn agi_round(value: f32, direction: f32) -> usize {
        if direction < 0. {
            if value - value.floor() <= 0.501 {
                value.floor() as usize
            } else {
                value.ceil() as usize
            }
        } else {
            if value - value.floor() < 0.499 {
                value.floor() as usize
            } else {
                value.ceil() as usize
            }
        }
    }
    let height = (y2 as isize) - (y1 as isize);
    let width = (x2 as isize) - (x1 as isize);
    let add_x: f32 = if height == 0 { 0. } else { (width as f32) / (height.abs() as f32) };
    let add_y: f32 = if width == 0 { 0. } else { (height as f32) / (width.abs() as f32) };
    let mut x = x1 as f32;
    let mut y = y1 as f32;
    if width.abs() > height.abs() {
      let add_x: f32 = if width == 0 { 0. } else { if width > 0 { 1. } else { -1. } };
      if x2 > x1 {
        while x < (x2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      } else {
        while x > (x2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      }
   } else {
      let add_y: f32 = if height == 0 { 0. } else { if height > 0 { 1. } else { -1. } };
      if y2 > y1 {
        while y < (y2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      } else {
        while y > (y2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      }
   }
    picture[y2 * WIDTH + x2] = colour;
}

fn describe_buf(buf: &[u8]) -> String {
    let mut s = format!("Len: ({}) [", buf.len());
    for (i, x) in buf.iter().enumerate() {
        if i != 0 {
            s.push(' ');
        }
        s.push_str(&format!("{:02x}", x));
    }
    s.push(']');
    s
}