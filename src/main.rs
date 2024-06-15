mod console_args;
mod generator;

use password_generator::THREADPOOLNUM;
use threadpool::ThreadPool;

use generator::generator_password;

fn main() {
    if let Some(opts) = console_args::read_args() {
        let thread_pool = ThreadPool::new(THREADPOOLNUM);
        for _ in 0..opts.total {
            thread_pool.execute(move || {
                println!(
                    "{}",
                    generator_password(
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
