use enum_iterator::Sequence;
use enum_iterator::all;
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value) {
        Ok(ResistorColor::Black) => return "Black".to_string(),
        Ok(ResistorColor::Brown) => return "Brown".to_string(),
        Ok(ResistorColor::Red) => return "Red".to_string(),
        Ok(ResistorColor::Orange) => return "Orange".to_string(),
        Ok(ResistorColor::Yellow) => return "Yellow".to_string(),
        Ok(ResistorColor::Green) => return "Green".to_string(),
        Ok(ResistorColor::Blue) => return "Blue".to_string(),
        Ok(ResistorColor::Violet) => return "Violet".to_string(),
        Ok(ResistorColor::Grey) => return "Grey".to_string(),
        Ok(ResistorColor::White) => return "White".to_string(),
        Err(e) => return "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
