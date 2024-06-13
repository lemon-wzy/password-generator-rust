use getopts::Options;
use password_generator::Argument;
use password_generator::DefaultValue::{DefaultGenerateTotal, DefaultLength, Zero};

const PROGRAM: &str = "pg";
const VERSION: &str = "0.1.0";
const USAGE: &str = "Usage: password-generator [options]";

pub fn menu() ->Options  {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Print usage information");
    opts.optflag("v", "version", "Print version information");
    opts.optopt("l", "length", "Set the length of the password", "LENGTH");
    opts.optopt("u", "upper", "Set the number of uppercase letters", "UPPER");
    opts.optopt("o","lower", "Set the number of lowercase letters", "LOWER");
    opts.optopt("d", "digital", "Set the number of digital", "DIGITAL");
    opts.optopt("m", "mark", "Set the number of mark", "MARK");
    opts.optopt("t", "total", "Set the number of total", "TOTAL");
    opts
}

pub fn read_args() -> Option<Argument> {
    let mut argument = Argument::default();
    let opts = menu();
    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    if matches.opt_present("h") {
        println!("{} {}", PROGRAM,opts.usage(USAGE));
        std::process::exit(0);
    }
    if matches.opt_present("v") {
        println!("{} {}", PROGRAM, VERSION);
        std::process::exit(0);
    }
    if matches.opt_present("l") {
        let mut length :u8 = matches.opt_str("l").unwrap().parse().unwrap();
        if length < Zero.as_u8() {
            length = DefaultLength.as_u8();
        }
        argument.length = length;
    }
    if matches.opt_present("u") {
        let mut upper :u8 = matches.opt_str("u").unwrap().parse().unwrap();
        if upper < Zero.as_u8() {
            upper = Zero.as_u8();
        }
        argument.upper = upper;
    }
    if matches.opt_present("o") {
        let mut lower :u8 = matches.opt_str("o").unwrap().parse().unwrap();
        if lower < Zero.as_u8() {
            lower = Zero.as_u8();
        }
        argument.lower = lower;
    }
    if matches.opt_present("d") {
        let mut digital :u8 = matches.opt_str("d").unwrap().parse().unwrap();
        if digital < Zero.as_u8() {
            digital = Zero.as_u8();
        }
        argument.digital = digital;
    }
    if matches.opt_present("m") {
        let mut mark :u8 = matches.opt_str("m").unwrap().parse().unwrap();
        if mark < Zero.as_u8() {
            mark = Zero.as_u8();
        }
        argument.mark = mark;
    }
    if matches.opt_present("t") {
        let mut total :u8 = matches.opt_str("t").unwrap().parse().unwrap();
        if total < Zero.as_u8() {
            total = DefaultGenerateTotal.as_u8();
        }
        argument.total = total;
    }
    Some(argument)
}
