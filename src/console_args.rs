use getopts::Options;
use password_generator::{Argument};
use password_generator::DefaultValue::{ DefaultLength};

const PROGRAM: &str = "pg";
const VERSION: &str = "0.1.0";
const USAGE: &str = "Usage: password-generator [options]";
const UNKNOWN: &str = "Unknown argument\n";

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
    let matches = match opts.parse(std::env::args().skip(1)) {
        Ok(success) => success,
        Err(_) => {
            println!("{} {} {}",UNKNOWN, PROGRAM,opts.usage(USAGE));
            std::process::exit(0)
        }
    };
    if matches.opt_present("h") {
        println!("{} {}", PROGRAM,opts.usage(USAGE));
        std::process::exit(0);
    }
    if matches.opt_present("v") {
        println!("{} {}", PROGRAM, VERSION);
        std::process::exit(0);
    }
    if matches.opt_present("l") {
        let length :u8 = matches.opt_str("l").
            unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        modify_args(&mut argument, 'l', length);
    }
    if matches.opt_present("u") {
        let upper :u8 = matches.opt_str("u")
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        modify_args(&mut argument, 'u', upper);
    }
    if matches.opt_present("o") {
        let lower :u8 = matches.opt_str("o")
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
            arg_value_err();
            std::process::exit(0)
        });
        modify_args(&mut argument, 'o', lower);
    }
    if matches.opt_present("d") {
        let digital :u8 = matches.opt_str("d")
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        modify_args(&mut argument, 'd', digital);
    }
    if matches.opt_present("m") {
        let mark :u8 = matches.opt_str("m")
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        modify_args(&mut argument, 'm', mark);
    }
    if matches.opt_present("t") {
        let total :u8 = matches.opt_str("t")
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        modify_args(&mut argument, 't', total);
    }
    Some(argument)
}


fn modify_args(argument: &mut Argument, letter : char, value : u8 ) {
    match letter {
        'u' => argument.upper += value,
        'o' => argument.lower += value,
        'd' => argument.digital += value,
        'm' => argument.mark += value,
        't' => argument.total = value,
        'l' => if value >= 8 { argument.length = value } else { argument.length = DefaultLength.as_u8(); },
        _ => {}
    }
}

fn arg_value_err() {
    println!("参数错误,请输入无符号整数");
}

