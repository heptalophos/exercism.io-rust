use enum_iterator::IntoEnumIterator;
// use int_enum::IntEnum;

#[derive(Debug, PartialEq, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    if value >= 10 { return String::from("value out of range"); }
    let some_color = ResistorColor::into_enum_iter().nth(value);
    format!("{:?}", some_color.unwrap())
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect::<Vec<ResistorColor>>()
}