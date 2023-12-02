use std::{
    fmt::{Display, Formatter, Result},
    fs, io,
};

pub enum InputType<'a> {
    Real,
    Test,
    Other(&'a str),
}

impl<'a> Display for InputType<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InputType::Real => write!(f, "real"),
            InputType::Test => write!(f, "test"),
            InputType::Other(other) => write!(f, "{}", other),
        }
    }
}

pub fn read_input(day: u8, input_type: InputType) -> io::Result<String> {
    fs::read_to_string(format!("./inputs/day{day:02}/{input_type}.txt"))
}
