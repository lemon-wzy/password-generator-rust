use rand::prelude::{IteratorRandom, SliceRandom};
use password_generator::{ DIGITS, LOWERCASE_LETTERS, MARKS, UPPERCASE_LETTERS};
use password_generator::DefaultValue::Zero;

pub fn  generator_password<'a>(length: u8, upper: u8, lower : u8, digital: u8, mark: u8) ->  String {
    let mut password = String::new();
    if upper.gt (&Zero.as_u8()) {
        password.extend(UPPERCASE_LETTERS.iter().choose_multiple(&mut rand::thread_rng(), upper as usize));
    }
    if lower.gt(&Zero.as_u8())  {
        password.extend(LOWERCASE_LETTERS.iter().choose_multiple(&mut rand::thread_rng(), lower as usize));
    }
    if digital.gt( &Zero.as_u8())  {
        password.extend(DIGITS.iter().choose_multiple(&mut rand::thread_rng(), digital as usize));
    }
    if mark.gt(&Zero.as_u8())  {
        password.extend(MARKS.iter().choose_multiple(&mut rand::thread_rng(), mark as usize));
    }
    let mut surplus_chars: Vec<char> = Vec::new();
    if mark.eq( &Zero.as_u8()) {
        surplus_chars.extend(MARKS.iter());
    }
    if digital.eq( &Zero.as_u8()) {
        surplus_chars.extend(DIGITS.iter());
    }
    if upper.eq(&Zero.as_u8())  {
        surplus_chars.extend(UPPERCASE_LETTERS.iter());
    }
    if lower.eq(&Zero.as_u8()) {
        surplus_chars.extend(LOWERCASE_LETTERS.iter());
    }
    if (surplus_chars.len() as u8).gt( &Zero.as_u8()) {
        password.extend(surplus_chars.iter().choose_multiple(&mut rand::thread_rng(), length as usize - password.len()));
    }
    if password.len().le (&(length as usize)) {
        let all_chars = UPPERCASE_LETTERS.iter().chain(LOWERCASE_LETTERS.iter()).chain(DIGITS.iter()).chain(MARKS.iter());
        password.extend(all_chars.choose_multiple(&mut rand::thread_rng(), length as usize - password.len()));
    }
    let mut pass: Vec<char> = Vec::new();
    pass.extend(password.chars());
    pass.shuffle(&mut rand::thread_rng());
    println!("{}", pass.iter().collect::<String>());
    pass.iter().collect()
}
