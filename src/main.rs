mod generator;
mod console_args;

use generator::generator_password;



fn main() {
    if let Some(opts) = console_args::read_args() {
        for _ in 0..opts.total {
            generator_password(opts.length,opts.upper,opts.lower,opts.digital,opts.mark);
        }
    }
}

