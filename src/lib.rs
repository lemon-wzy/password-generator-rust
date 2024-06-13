use crate::DefaultValue::{DefaultGenerateTotal, DefaultLength, Zero};

pub static UPPERCASE_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];
pub static LOWERCASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];
pub static DIGITS: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];
pub static MARKS: [char; 10] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')'
];


#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DefaultValue {
    DefaultLength = 16,
    Zero = 0,
    DefaultCut = 4,
    DefaultGenerateTotal = 5
}

impl DefaultValue {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

pub struct  Argument {
    pub length: u8,
    pub upper: u8,
    pub lower : u8,
    pub digital: u8,
    pub mark: u8,
    pub total: u8
}

impl Argument {
    pub fn default()-> Self {
        Self{
            length: DefaultLength.as_u8(),
            upper: Zero.as_u8(),
            lower : Zero.as_u8(),
            digital: Zero.as_u8(),
            mark: Zero.as_u8(),
            total: DefaultGenerateTotal.as_u8()
        }
    }
}
