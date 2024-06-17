use std::env;

use crate::DefaultValue::{DefaultGenerateTotal, DefaultLength, MinLength, Zero};

pub const UPPERCASE_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
pub const LOWERCASE_LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
pub const MARKS: [char; 10] = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')'];

pub const LENGTH_ARG: char = 'l';
pub const UPPER_ARG: char = 'u';
pub const LOWER_ARG: char = 'o';
pub const DIGITAL_ARG: char = 'd';
pub const MARK_ARG: char = 'm';
pub const TOTAL_ARG: char = 't';
pub const HELP_ARG: char = 'h';
pub const VERSION_ARG: char = 'v';

pub const PROGRAM: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const USAGE: &str = "Usage: password-generator [options]";
pub const UNKNOWN: &str = "Unknown argument\n";

pub const THREADPOOLNUM: usize = 5;

pub struct DefaultArgs {
    pub length: (String, String, String, String),
    pub upper: (String, String, String, String),
    pub lower: (String, String, String, String),
    pub digital: (String, String, String, String),
    pub mark: (String, String, String, String),
    pub total: (String, String, String, String),
    pub help: (String, String, String),
    pub version: (String, String, String),
}

impl DefaultArgs {
    pub fn default_init() -> Self {
        Self {
            length: (
                String::from("l"),
                String::from("length"),
                String::from("Set the length of the password"),
                String::from("LENGTH"),
            ),
            upper: (
                String::from("u"),
                String::from("upper"),
                String::from("Set the number of uppercase letters"),
                String::from("UPPER"),
            ),
            lower: (
                String::from("o"),
                String::from("lower"),
                String::from("Set the number of lowercase letters"),
                String::from("LOWER"),
            ),
            digital: (
                String::from("d"),
                String::from("digital"),
                String::from("Set the number of digital"),
                String::from("DIGITAL"),
            ),
            mark: (
                String::from("m"),
                String::from("mark"),
                String::from("Set the number of mark"),
                String::from("MARK"),
            ),
            total: (
                String::from("t"),
                String::from("total"),
                String::from("Set the number of total"),
                String::from("TOTAL"),
            ),
            help: (
                String::from("h"),
                String::from("help"),
                String::from("Print usage information"),
            ),
            version: (
                String::from("v"),
                String::from("version"),
                String::from("Print version information"),
            ),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DefaultValue {
    DefaultLength = 16,
    Zero = 0,
    DefaultCut = 4,
    DefaultGenerateTotal = 5,
    MinLength = 8,
}

impl DefaultValue {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        self as usize
    }
}

pub enum DefaultArgChar {
    UpperChar(char),
    LowerChar(char),
    LengthChar(char),
    MarkChar(char),
    DigtalChar(char),
    TotalChar(char),
    VersionChar(char),
    HelpChar(char),
}

pub struct Argument {
    pub length: u8,
    pub upper: u8,
    pub lower: u8,
    pub digital: u8,
    pub mark: u8,
    pub total: u8,
}

impl Argument {
    pub fn default_init() -> Self {
        Self {
            length: DefaultLength.as_u8(),
            upper: Zero.as_u8(),
            lower: Zero.as_u8(),
            digital: Zero.as_u8(),
            mark: Zero.as_u8(),
            total: DefaultGenerateTotal.as_u8(),
        }
    }

    pub fn check(&self) -> bool {
        (self.upper + self.lower + self.digital + self.mark).le(&self.length)
    }

    pub fn modify_arg(&mut self, letter: char, value: u8) {
        match letter {
            UPPER_ARG => self.upper += value,
            LOWER_ARG => self.lower += value,
            DIGITAL_ARG => self.digital += value,
            MARK_ARG => self.mark += value,
            TOTAL_ARG => self.total = value,
            LENGTH_ARG => {
                if value.ge(&MinLength.as_u8()) {
                    self.length = value
                } else {
                    self.length = DefaultLength.as_u8();
                }
            }
            _ => {}
        }
    }

    pub fn modify(&mut self, char_enum: DefaultArgChar, value: u8) {
        match char_enum {
            DefaultArgChar::UpperChar(UPPER_ARG) => self.upper += value,
            DefaultArgChar::DigtalChar(DIGITAL_ARG) => self.digital += value,
            DefaultArgChar::LowerChar(LOWER_ARG) => self.lower += value,
            DefaultArgChar::MarkChar(MARK_ARG) => self.mark += value,
            DefaultArgChar::TotalChar(TOTAL_ARG) => self.total = value,
            DefaultArgChar::LengthChar(LENGTH_ARG) => {
                if value.ge(&MinLength.as_u8()) {
                    self.length = value
                } else {
                    self.length = DefaultLength.as_u8()
                }
            }
            _ => {}
        }
    }
}
