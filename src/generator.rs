use rand::prelude::{IteratorRandom, SliceRandom};
use password_generator::{DefaultValue, DIGITS, LOWERCASE_LETTERS, MARKS, UPPERCASE_LETTERS};

pub fn  generator_password<'a>(length: u8, upper: u8, lower : u8, digital: u8, mark: u8) ->  String {
    let mut password = String::new();
    if upper >DefaultValue::Zero.as_u8() {
        password.extend(UPPERCASE_LETTERS.iter().choose_multiple(&mut rand::thread_rng(), upper as usize));
    }
    if lower > DefaultValue::Zero.as_u8()  {
        password.extend(LOWERCASE_LETTERS.iter().choose_multiple(&mut rand::thread_rng(), lower as usize));
    }
    if digital > DefaultValue::Zero.as_u8()  {
        password.extend(DIGITS.iter().choose_multiple(&mut rand::thread_rng(), digital as usize));
    }
    if mark > DefaultValue::Zero.as_u8()  {
        password.extend(MARKS.iter().choose_multiple(&mut rand::thread_rng(), mark as usize));
    }
    if mark + digital + lower + upper < length {
        let all_chars = UPPERCASE_LETTERS.iter().chain(LOWERCASE_LETTERS.iter()).chain(DIGITS.iter()).chain(MARKS.iter());
        password.extend(all_chars.choose_multiple(&mut rand::thread_rng(), (length - mark - digital - lower - upper) as usize));
    }
    let mut pass: Vec<char> = Vec::new();
    pass.extend(password.chars());
    pass.shuffle(&mut rand::thread_rng());
    println!("{}", pass.iter().collect::<String>());
    pass.iter().collect()
}
