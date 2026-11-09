//! Throughput benchmark for `kompress-super-ultra` (`cargo bench`).

use kompress_super_ultra::{cli::BenchArgs, commands::run::execute_bench};

fn main() {
    execute_bench(BenchArgs { iterations: 10_000 }).expect("benchmark failed");
}
