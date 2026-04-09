//! ASCII banner and help copy for the CLI.

/// RL “loop” motif (pure ASCII).
pub const BANNER: &str = r"
  (•) → (•) → (•)
    \   |   /        R L   P I P E L I N E
      \ | /          bandit env + agents
       (•)
";

/// Short tagline for `--help` headers.
pub const TAGLINE: &str = "Bandit environment, random & ε-greedy agents, `rlpipe` CLI.";

/// Long-form body for `clap` (`long_about`): banner + tagline + tips.
pub const HELP_LONG: &str = r"

  (•) → (•) → (•)
    \   |   /        R L   P I P E L I N E
      \ | /          bandit env + agents
       (•)

Bandit environment, random & ε-greedy agents, `rlpipe` CLI.

Tips
  RUST_LOG=info (or debug, trace) works with `rlpipe --verbose …`.

Scope
  Toy RL scaffold for structure parity with `pub-sub-pipeline`. Extend `env`/`agent` for MDPs.
";
