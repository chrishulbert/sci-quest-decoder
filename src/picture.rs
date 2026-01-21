use crate::decode::decode;
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
                    println!("SetPattern has no argument!");
                } else {
                    if a.arguments.len() > 1 {
                        println!("SetPattern has extra arguments, len: {}!", a.arguments.len());
                    }
                    let is_rectangle = a.arguments[0] & 0x10 != 0; // vs circle.
                    let is_pattern = a.arguments[0] & 0x20 != 0; // vs solid.
                    let size = a.arguments[0] & 0xf;
                    println!("SetPattern: {:02x} rect {}, pattern {}, size {}", a.arguments[0], is_rectangle, is_pattern, size);
                }
            }
            picture_splitter::Action::LongPatterns => {

            }
            picture_splitter::Action::MediumRelativePatterns => {
                
            }
            picture_splitter::Action::ShortRelativePatterns => {
                
            }
            // Etc:
            picture_splitter::Action::FloodFill => {
                if is_drawing {
                    fill(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::CommandExtensions => {
                
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
            println!("Medium relative line going into negative Y! {} - {}", y, y_raw);
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
            println!("Short relative line going into negative X! {} - {}", x, x_raw);
            return
        }
        let new_x = if x_is_minus { x - x_raw } else { x + x_raw };

        let y_raw = (coordinate & 0x7) as usize; 
        let y_is_minus = coordinate & 8 != 0;
        if y_is_minus && y_raw > y {
            println!("Short relative line going into negative Y! {} - {}", y, y_raw);
            return
        }
        let new_y = if y_is_minus { y - y_raw } else { y + y_raw };

        draw_sierra_line(picture, colour, x, y, new_x, new_y);

        x = new_x;
        y = new_y;
    }
}

fn fill(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    println!("Fill args: {}", arguments.len());
    for chunk in arguments.chunks_exact(3) {
        let (x, y) = xy_from_triple(chunk);
        if x >= WIDTH || y >= HEIGHT {
            println!("Fill weird xy! {} {}", x, y);
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
}

fn plot(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    for chunk in arguments.chunks_exact(2) {
        let x = chunk[0] as usize;
        let y = chunk[1] as usize;
        if x >= WIDTH || y >= HEIGHT { continue } // Out of bounds.
        let offset = y * WIDTH + x;
        picture[offset] = colour;
    }
}

fn draw_sierra_line(picture: &mut [u8], colour: u8, x1: usize, y1: usize, x2: usize, y2: usize) {
    if x1 >= WIDTH || y1 >= HEIGHT || x2 >= WIDTH || y2 >= HEIGHT {
        println!("Draw line out of bounds! {},{} -> {},{}", x1, y1, x2, y2);
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
