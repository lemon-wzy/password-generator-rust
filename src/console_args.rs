use getopts::Options;
use password_generator::{
    Argument, DefaultArgs, DIGITAL_ARG, LENGTH_ARG, LOWER_ARG, MARK_ARG, PROGRAM, TOTAL_ARG,
    UNKNOWN, UPPER_ARG, USAGE, VERSION,
};

pub fn menu() -> Options {
    let default_args = DefaultArgs::default_init();
    let mut opts = Options::new();
    opts.optflag(
        &default_args.help.0,
        &default_args.help.1,
        &default_args.help.2,
    );
    opts.optflag(
        &default_args.version.0,
        &default_args.version.1,
        &default_args.version.2,
    );
    opts.optopt(
        &default_args.length.0,
        &default_args.length.1,
        &default_args.length.2,
        &default_args.length.3,
    );
    opts.optopt(
        &default_args.upper.0,
        &default_args.upper.1,
        &default_args.upper.2,
        &default_args.upper.3,
    );
    opts.optopt(
        &default_args.lower.0,
        &default_args.lower.1,
        &default_args.lower.2,
        &default_args.lower.3,
    );
    opts.optopt(
        &default_args.digital.0,
        &default_args.digital.1,
        &default_args.digital.2,
        &default_args.digital.3,
    );
    opts.optopt(
        &default_args.mark.0,
        &default_args.mark.1,
        &default_args.mark.2,
        &default_args.mark.3,
    );
    opts.optopt(
        &default_args.total.0,
        &default_args.total.1,
        &default_args.total.2,
        &default_args.total.3,
    );
    opts
}

pub fn read_args() -> Option<Argument> {
    let default_args = DefaultArgs::default_init();
    let mut argument = Argument::default_init();
    let opts = menu();
    let matches = match opts.parse(std::env::args().skip(1)) {
        Ok(success) => success,
        Err(_) => {
            println!("{} {} {}", UNKNOWN, PROGRAM, opts.usage(USAGE));
            std::process::exit(0)
        }
    };
    if matches.opt_present(&default_args.help.0) {
        println!("{} {}", PROGRAM, opts.usage(USAGE));
        std::process::exit(0);
    }
    if matches.opt_present(&default_args.version.0) {
        println!("{} {}", PROGRAM, VERSION);
        std::process::exit(0);
    }
    if matches.opt_present(&default_args.length.0) {
        let length: u8 = matches
            .opt_str(&default_args.length.0)
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        argument.modify_arg(LENGTH_ARG, length);
    }
    if matches.opt_present(&default_args.upper.0) {
        let upper: u8 = matches
            .opt_str(&default_args.upper.0)
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        argument.modify_arg(UPPER_ARG, upper);
    }
    if matches.opt_present(&default_args.lower.0) {
        let lower: u8 = matches
            .opt_str(&default_args.lower.0)
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        argument.modify_arg(LOWER_ARG, lower);
    }
    if matches.opt_present(&default_args.digital.0) {
        let digital: u8 = matches
            .opt_str(&default_args.digital.0)
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        argument.modify_arg(DIGITAL_ARG, digital);
    }
    if matches.opt_present(&default_args.mark.0) {
        let mark: u8 = matches
            .opt_str(&default_args.mark.0)
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        argument.modify_arg(MARK_ARG, mark);
    }
    if matches.opt_present(&default_args.total.0) {
        let total: u8 = matches
            .opt_str(&default_args.total.0)
            .unwrap()
            .parse()
            .unwrap_or_else(|_| {
                arg_value_err();
                std::process::exit(0)
            });
        argument.modify_arg(TOTAL_ARG, total);
    }
    if argument.check().eq(&false) {
        pass_length_err();
        std::process::exit(0)
    };
    Some(argument)
}

fn arg_value_err() {
    println!("参数错误,请输入无符号整数");
}

fn pass_length_err() {
    println!("给定字符长度总和超出密码长度");
}
