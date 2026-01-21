// This is responsible for splitting a picture resource into strongly typed actions+args.
// https://www.agidev.com/articles/agispec/agispecs-7.html
// https://github.com/wjp/freesci-archive/blob/master/src/gfx/resource/sci_pic_0.c#L531
// https://github.com/wjp/freesci-archive/blob/master/src/scicore/decompress01.c

#[derive(PartialEq, Debug)]
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
    let mut last_action: Option<Action> = None;
    let mut last_args: Vec<u8> = Vec::new();
    let mut actions: Vec<ActionArguments> = Vec::new();
    for &b in data {
        if b >= 0xf0 { // New action.
            if let Some(action) = last_action {
                actions.push(ActionArguments {
                    action,
                    arguments: last_args.clone(),
                });
            }
            last_action = Some(Action::from_byte(b));
            last_args.clear();
        } else {
            last_args.push(b);
        }
    }
    if let Some(action) = last_action {
        actions.push(ActionArguments {
            action,
            arguments: last_args.clone(),
        });
    }
    actions
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
            1, // Should be ignored.
            0xf0, 1, 2, 3,
            0xf1,
            0xf2,
            0xff,
        ];
        let result = super::split(&resource);
        let expected: Vec<super::ActionArguments> = vec![
            ActionArguments{
                action: Action::SetVisualColour,
                arguments: vec![1, 2, 3],
            },
            ActionArguments{
                action: Action::DisableVisual,
                arguments: vec![],
            },
            ActionArguments{
                action: Action::SetPriorityColour,
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
