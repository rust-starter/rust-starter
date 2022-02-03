use std::path::PathBuf;
use clap::{AppSettings, Parser, Subcommand};

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;

#[derive(Parser)]
#[clap(
    name = "rust-starter",
    author,
    about,
    long_about = "Rust Starter CLI",
    version
)]
#[clap(setting = AppSettings::SubcommandRequired)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    /// Set a custom config file
    #[clap(short, long,parse(from_os_str), value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        name = "hazard",
        about = "Generate a hazardous occurance",
        long_about = None, 
    )]
    Hazard,
    #[clap(
        name = "error",
        about = "Simulate an error",
        long_about = None, 
    )]
    Error,
    #[clap(
        name = "config",
        about = "Show Configuration",
        long_about = None,
    )]
    Config,
}

pub fn cli_match() -> Result<()> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli.config.as_deref())?;

    // Execute the subcommand
    match &cli.command {
        Commands::Hazard => commands::hazard()?,
        Commands::Error => commands::simulate_error()?,
        Commands::Config => commands::config()?,
    }

    Ok(())
}
