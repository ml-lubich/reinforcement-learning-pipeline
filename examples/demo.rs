//! `cargo run --example demo`
//!
//! Same flow as `rlpipe demo`, kept as a library-only example entrypoint.

use reinforcement_learning_pipeline::scenarios::run_demo_report;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("{}", run_demo_report());
    Ok(())
}
