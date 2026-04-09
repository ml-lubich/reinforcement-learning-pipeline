//! `rlpipe` CLI — in-process harness for the bandit env and agents.

use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};
use reinforcement_learning_pipeline::art::{HELP_LONG, TAGLINE};
use reinforcement_learning_pipeline::env::BanditEnv;
use reinforcement_learning_pipeline::scenarios;
use reinforcement_learning_pipeline::train::{run_greedy_session, run_random_episode};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(
    name = "rlpipe",
    version,
    about = TAGLINE,
    long_about = HELP_LONG
)]
struct Cli {
    /// Enable `tracing` spans/events (honors `RUST_LOG`, default `info` when unset).
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the scripted random vs ε-greedy report.
    Demo,
    /// Random policy baseline for `pulls` on a `BanditEnv`.
    Random {
        /// Number of arms.
        #[arg(short = 'k', long, default_value_t = 5)]
        arms: usize,
        /// Index of the best arm.
        #[arg(short = 'b', long, default_value_t = 2)]
        best: usize,
        /// How many pulls.
        #[arg(short = 'n', long, default_value_t = 120)]
        pulls: usize,
        #[arg(short, long, default_value_t = 99)]
        seed: u64,
    },
    /// ε-greedy policy for `pulls`.
    Greedy {
        #[arg(short = 'k', long, default_value_t = 5)]
        arms: usize,
        #[arg(short = 'b', long, default_value_t = 2)]
        best: usize,
        #[arg(short = 'n', long, default_value_t = 120)]
        pulls: usize,
        #[arg(short, long, default_value_t = 99)]
        seed: u64,
        #[arg(short = 'e', long, default_value_t = 0.1)]
        epsilon: f32,
    },
}

fn init_tracing(verbose: bool) -> Result<()> {
    if !verbose {
        return Ok(());
    }

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .try_init()
        .map_err(|e| anyhow::anyhow!("failed to install tracing subscriber: {e}"))?;

    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Demo => {
            print!("{}", scenarios::run_demo_report());
        }
        Command::Random {
            arms,
            best,
            pulls,
            seed,
        } => {
            let k = arms.max(1);
            let b = best.min(k - 1);
            let env = BanditEnv::new(k, b, 1.0, 0.12).map_err(|e| anyhow::anyhow!("{e}"))?;
            let out = run_random_episode(&env, pulls, seed).map_err(|e| anyhow::anyhow!("{e}"))?;
            println!(
                "arms={} mean_reward={:.4} return={:.2}",
                env.num_arms(),
                out.mean_reward,
                out.return_sum
            );
        }
        Command::Greedy {
            arms,
            best,
            pulls,
            seed,
            epsilon,
        } => {
            let k = arms.max(1);
            let b = best.min(k - 1);
            let env = BanditEnv::new(k, b, 1.0, 0.12).map_err(|e| anyhow::anyhow!("{e}"))?;
            let out = run_greedy_session(&env, pulls, epsilon, seed)
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            println!(
                "arms={} mean_reward={:.4} return={:.2}",
                env.num_arms(),
                out.mean_reward,
                out.return_sum
            );
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    if let Err(err) = init_tracing(cli.verbose) {
        eprintln!("{err:#}");
        return ExitCode::FAILURE;
    }
    if let Err(err) = run(cli) {
        tracing::error!(error = %err, "rlpipe failed");
        eprintln!("{err:#}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
