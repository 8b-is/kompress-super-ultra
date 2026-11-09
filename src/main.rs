//! Binary entry point for `kompress-super-ultra`.

use kompress_super_ultra::{
    cli::{Cli, Commands},
    commands,
    error::Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => commands::init::execute(args).await?,
        Commands::Run(args) => commands::run::execute(args).await?,
        Commands::Serve(args) => commands::run::serve(args).await?,
        Commands::Bench(args) => commands::run::execute_bench(args)?,
    }

    Ok(())
}
