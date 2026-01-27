// This is responsible for splitting a picture resource into strongly typed actions+args.
// Unlike AGI, where all bytes >= 0xf0 are actions and all other bytes are low, in SCI
// the arguments can be >= 0xf0. Thus this needs to use the action to define the length.
// https://www.agidev.com/articles/agispec/agispecs-7.html
// https://github.com/wjp/freesci-archive/blob/master/src/gfx/resource/sci_pic_0.c#L531
// https://github.com/wjp/freesci-archive/blob/master/src/scicore/decompress01.c

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Action {
    SetVisualColour, // 0xF0
    DisableVisual, // 0xF1
    SetPriorityColour, // 0xF2
    DisablePriority, // 0xF3
    ShortRelativePatterns, // 0xF4
    MediumRelativeLines, // 0xF5
    LongLines, // 0xF6
    ShortRelativeLines, // 0xF7
    FloodFill, // 0xF8
    SetPattern, // 0xF9
    LongPatterns, // 0xFA
    SetControlColour, // 0xFB
    DisableControl, // 0xFC
    MediumRelativePatterns, // 0xFD
    CommandExtensions, // 0xFE
    End, // 0xFF
}

#[derive(PartialEq, Debug)]
pub struct ActionArguments {
    pub action: Action,
    pub arguments: Vec<u8>,
}

pub fn split(data: &[u8]) -> Vec<ActionArguments> {
    let mut remaining = data;
    let mut actions: Vec<ActionArguments> = Vec::new();
    let mut is_pattern = false;
    loop {
        if remaining.is_empty() { break }
        let action = Action::from_byte(remaining[0]);
        let args_onwards = &remaining[1..];
        let args_len = desired_arguments_length(action, args_onwards, is_pattern);
        let args = &args_onwards[..args_len];
        if action == Action::SetPattern {
            is_pattern = args[0] & 0x20 != 0;
        }
        actions.push(ActionArguments {
            action,
            arguments: args.to_vec(),
        });
        remaining = &remaining[(1 + args_len)..];
    }
    actions
}

// How many argument bytes are desired for this code.
fn desired_arguments_length(action: Action, args: &[u8], is_pattern: bool) -> usize {
    match action {
        Action::SetVisualColour => { 1 }
        Action::DisableVisual => { 0 }
        Action::SetPriorityColour => { 1 }
        Action::DisablePriority => { 0 }
        Action::SetPattern => { 1 }
        Action::ShortRelativePatterns => { desired_arguments_length_short_patterns(args, is_pattern) }
        Action::MediumRelativePatterns => { desired_arguments_length_medium_patterns(args, is_pattern) }
        Action::LongPatterns => { desired_arguments_length_long_patterns(args, is_pattern) }
        Action::ShortRelativeLines => { desired_arguments_length_short_lines(args) }
        Action::MediumRelativeLines => { desired_arguments_length_medium_lines(args) }
        Action::LongLines => { desired_arguments_length_long_lines(args) }
        Action::FloodFill => { desired_arguments_length_fills(args) }
        Action::SetControlColour => { 1 }
        Action::DisableControl => { 0 }
        Action::CommandExtensions => { desired_arguments_length_extensions(args) }
        Action::End => { 0 }
    }
}

fn desired_arguments_length_extensions(args: &[u8]) -> usize {
    let command = args[0];
    match command {
        0 => { // Set palette entries.
            let mut bytes = 1;
            while args[bytes] < 0xf0 {
                bytes += 2;
            }
            bytes
        },
        1 => { // Set entire palette.
            42 // Command + Palette number + 40 palette entries.
        },
        2 => { 42 }, // Monochrome 0: set palette.
        3 => { 2 }, // Monochrome 1: set visual.
        4 => { 1 }, // Monochrome 2: disable visual.
        5 => { 2 }, // Monochrome 3: set direct visual.
        6 => { 1 }, // Monochrome 4: disable direct visual.
        7 => { // Embed cel (SCI01).
            let size = (args[4] as usize) + ((args[5] as usize) << 8);
            6 + size // Command (1) + XY (3) + cel size (2) + cel (n).
        },
        8 => { // Set priority bands (SCI01).
            15 // Command (1) + Priority table (14).
        },
        _ => {
            panic!("Unrecognised extended operation! Command: {}", command);
        },
    }
}

fn desired_arguments_length_short_lines(args: &[u8]) -> usize {
    let mut bytes = 3;
    while args[bytes] < 0xf0 {
        bytes += 1;
    }
    return bytes;
}

fn desired_arguments_length_medium_lines(args: &[u8]) -> usize {
    let mut bytes = 3;
    while args[bytes] < 0xf0 {
        bytes += 2;
    }
    return bytes;
}

// Long lines are at least 1 multiple of 3.
fn desired_arguments_length_long_lines(args: &[u8]) -> usize {
    let mut bytes = 3;
    loop {
        if args[bytes] >= 0xf0 { break }
        bytes += 3;
    }
    return bytes;
}

// Fills are 0 or more multiples of 3.
fn desired_arguments_length_fills(args: &[u8]) -> usize {
    let mut bytes = 0;
    loop {
        if args[bytes] >= 0xf0 { break }
        bytes += 3;
    }
    return bytes;
}

// Long patterns are chunks of lengths + a pattern byte if pattern mode.
fn desired_arguments_length_long_patterns(args: &[u8], is_pattern: bool) -> usize {
    let mut bytes = 0;
    let chunk_size = if is_pattern { 4 } else { 3 };
    while args[bytes] < 0xf0 {
        bytes += chunk_size;
    }
    return bytes;
}

fn desired_arguments_length_medium_patterns(args: &[u8], is_pattern: bool) -> usize {
    let mut bytes = if is_pattern { 4 } else { 3 };
    let chunk_size = if is_pattern { 3 } else { 2 };
    while args[bytes] < 0xf0 {
        bytes += chunk_size;
    }
    return bytes;
}

fn desired_arguments_length_short_patterns(args: &[u8], is_pattern: bool) -> usize {
    let mut bytes = if is_pattern { 4 } else { 3 };
    let chunk_size = if is_pattern { 2 } else { 1 };
    while args[bytes] < 0xf0 {
        bytes += chunk_size;
    }
    return bytes;
}

impl Action {
    fn from_byte(b: u8) -> Self {
        match b {
            0xF0 => Self::SetVisualColour, 
            0xF1 => Self::DisableVisual, 
            0xF2 => Self::SetPriorityColour, 
            0xF3 => Self::DisablePriority, 
            0xF4 => Self::ShortRelativePatterns, 
            0xF5 => Self::MediumRelativeLines, 
            0xF6 => Self::LongLines, 
            0xF7 => Self::ShortRelativeLines, 
            0xF8 => Self::FloodFill, 
            0xF9 => Self::SetPattern, 
            0xFA => Self::LongPatterns, 
            0xFB => Self::SetControlColour, 
            0xFC => Self::DisableControl, 
            0xFD => Self::MediumRelativePatterns, 
            0xFE => Self::CommandExtensions, 
            0xFF => Self::End, 
            _ => panic!("Unrecognised action type!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_splits() {
        let resource: Vec<u8> = vec![
            0xf0, 1,
            0xf1,
            0xff,
        ];
        let result = super::split(&resource);
        let expected: Vec<super::ActionArguments> = vec![
            ActionArguments{
                action: Action::SetVisualColour,
                arguments: vec![1],
            },
            ActionArguments{
                action: Action::DisableVisual,
                arguments: vec![],
            },
            ActionArguments{
                action: Action::End,
                arguments: vec![],
            },
        ];
        assert_eq!(result, expected);
    }
}
