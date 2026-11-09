//! Implementation of the `init` subcommand.

use crate::{cli::InitArgs, config::AppConfig, error::Result};

/// Initialize a sovereign `kompress-super-ultra` profile and print its canonical configuration.
pub async fn execute(args: InitArgs) -> Result<()> {
    if args.name.trim().is_empty() {
        return Err(crate::error::AppError::InvalidInput(
            "profile name must not be empty".to_string(),
        ));
    }

    let cfg = AppConfig::default();
    println!(
        "Initialized sovereign Kompress-Super-Ultra profile '{}' (bind={}, bonobo=3/2, techno_zen=9/8)",
        args.name, cfg.bind_addr
    );
    Ok(())
}
