//! Command-line argument parser and subcommands for `kompress-super-ultra`.

use crate::config::FoldMode;

/// Top-level CLI structure following `/systems-programming-rust-project`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cli {
    /// Selected subcommand. Defaults to `Serve` when invoked without arguments.
    pub command: Commands,
}

/// Supported CLI subcommands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Commands {
    /// Initialize a new sovereign compression profile.
    Init(InitArgs),
    /// Run an offline 4-role semantic fold pass with exact p/q accounting.
    Run(RunArgs),
    /// Start the Axum + Live WebSocket WebUI server (default).
    Serve(ServeArgs),
    /// Execute the deterministic throughput benchmark.
    Bench(BenchArgs),
}

/// Arguments for the `init` subcommand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitArgs {
    /// Profile or project name.
    pub name: String,
}

/// Arguments for the `run` subcommand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunArgs {
    /// Enable verbose diagnostic output.
    pub verbose: bool,
    /// Optional raw payload to fold; uses built-in sovereign diagnostic sample when omitted.
    pub input: Option<String>,
    /// Semantic fold mode (`Standard`, `Aggressive`, `JsonOnly`).
    pub mode: FoldMode,
}

/// Arguments for the `serve` subcommand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServeArgs {
    /// Socket address to bind (`0.0.0.0:8799` by default).
    pub bind: String,
}

/// Arguments for the `bench` subcommand.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BenchArgs {
    /// Number of fold iterations to execute.
    pub iterations: u64,
}

impl Cli {
    /// Parse command-line arguments from `std::env::args()`.
    pub fn parse() -> Self {
        Self::parse_from(std::env::args())
    }

    /// Parse command-line arguments from an arbitrary iterator.
    pub fn parse_from<I>(args: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let raw: Vec<String> = args.into_iter().collect();
        let default_port = std::env::var("PORT").unwrap_or_else(|_| "8799".to_string());
        let default_bind = format!("0.0.0.0:{default_port}");

        if raw.len() <= 1 {
            return Self { command: Commands::Serve(ServeArgs { bind: default_bind }) };
        }

        match raw[1].as_str() {
            "init" => {
                let mut name = "bonobo-r0-profile".to_string();
                let mut i = 2;
                while i < raw.len() {
                    if (raw[i] == "-n" || raw[i] == "--name") && i + 1 < raw.len() {
                        name = raw[i + 1].clone();
                        i += 1;
                    }
                    i += 1;
                }
                Self { command: Commands::Init(InitArgs { name }) }
            }
            "run" => {
                let mut verbose = false;
                let mut input = None;
                let mut mode = FoldMode::Standard;
                let mut i = 2;
                while i < raw.len() {
                    match raw[i].as_str() {
                        "-v" | "--verbose" => verbose = true,
                        "-i" | "--input" if i + 1 < raw.len() => {
                            input = Some(raw[i + 1].clone());
                            i += 1;
                        }
                        "-m" | "--mode" if i + 1 < raw.len() => {
                            mode = FoldMode::parse_opt(Some(&raw[i + 1]));
                            i += 1;
                        }
                        _ => {}
                    }
                    i += 1;
                }
                Self { command: Commands::Run(RunArgs { verbose, input, mode }) }
            }
            "bench" | "--bench" => {
                let mut iterations = 10_000;
                let mut i = 2;
                while i < raw.len() {
                    if (raw[i] == "-n" || raw[i] == "--iterations") && i + 1 < raw.len() {
                        iterations = raw[i + 1].parse().unwrap_or(10_000);
                        i += 1;
                    } else if let Ok(parsed) = raw[i].parse::<u64>() {
                        iterations = parsed;
                    }
                    i += 1;
                }
                Self { command: Commands::Bench(BenchArgs { iterations }) }
            }
            _ => {
                let mut bind = default_bind;
                let mut i = 1;
                while i < raw.len() {
                    if (raw[i] == "-b" || raw[i] == "--bind") && i + 1 < raw.len() {
                        bind = raw[i + 1].clone();
                        i += 1;
                    }
                    i += 1;
                }
                Self { command: Commands::Serve(ServeArgs { bind }) }
            }
        }
    }
}
