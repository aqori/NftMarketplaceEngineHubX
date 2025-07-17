// src/main.rs
/*
 * Main executable for NftMarketplaceEngineHubX
 */

use clap::Parser;
use nftmarketplaceenginehubx::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceEngineHubX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
