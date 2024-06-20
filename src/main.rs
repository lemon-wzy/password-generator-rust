mod config_read;
mod console_args;
mod generator;
#[macro_use]
extern crate lazy_static;

use password_generator::THREADPOOLNUM;
use threadpool::ThreadPool;

use generator::generator_password_with_config;

fn main() {
    let config = config_read::ConfigRead::get();

    if let Some(opts) = console_args::read_args() {
        let thread_pool = ThreadPool::new(THREADPOOLNUM);
        for _ in 0..opts.total {
            thread_pool.execute(move || {
                println!(
                    "{}",
                    generator_password_with_config(
                        config,
                        opts.length,
                        opts.upper,
                        opts.lower,
                        opts.digital,
                        opts.mark
                    )
                )
            })
        }
        thread_pool.join()
    }
}
