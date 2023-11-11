use std::{path::PathBuf, str::FromStr};
use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, shells::{Bash, Fish, Zsh}};

use core::commands;
use utils::app_config::AppConfig;
use utils::error::Result;
use utils::types::LogLevel;


#[derive(Parser, Debug)]
#[command(
    name = "rust-starter",
    author,
    about,
    long_about = "Rust Starter CLI",
    version
)]
//TODO: #[clap(setting = AppSettings::SubcommandRequired)]
//TODO: #[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    /// Set a custom config file
    /// TODO: parse(from_os_str)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Set a custom config file
    #[arg(name="debug", short, long="debug", value_name = "DEBUG")]
    pub debug: Option<bool>,

    /// Set Log Level 
    #[arg(name="log_level", short, long="log-level", value_name = "LOG_LEVEL")]
    pub log_level: Option<LogLevel>,

    /// Subcommands
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
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
        name = "completion",
        about = "Generate completion scripts",
        long_about = None,
        )]
        Completion {
            #[clap(subcommand)]
            subcommand: CompletionSubcommand,
        },
    #[clap(
        name = "config",
        about = "Show Configuration",
        long_about = None,
    )]
    Config,
}

#[derive(Subcommand, PartialEq, Debug)]
enum CompletionSubcommand {
    #[clap(about = "generate the autocompletion script for bash")]
    Bash,
    #[clap(about = "generate the autocompletion script for zsh")]
    Zsh,
    #[clap(about = "generate the autocompletion script for fish")]
    Fish,
}

pub fn cli_match() -> Result<()> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Merge clap config file if the value is set
    AppConfig::merge_config(cli.config.as_deref())?;

    let app = Cli::command();
    let matches = app.get_matches();
    
    AppConfig::merge_args(matches)?;

    // Execute the subcommand
    match &cli.command {
        Commands::Hazard => commands::hazard()?,
        Commands::Error => commands::simulate_error()?,
        Commands::Completion {subcommand} => {
            let mut app = Cli::command();
            match subcommand {
                CompletionSubcommand::Bash => {
                    generate(Bash, &mut app, "rust-starter", &mut std::io::stdout());
                }
                CompletionSubcommand::Zsh => {
                    generate(Zsh, &mut app, "rust-starter", &mut std::io::stdout());
                }
                CompletionSubcommand::Fish => {
                    generate(Fish, &mut app, "rust-starter", &mut std::io::stdout());
                }
            }
        }
        Commands::Config => commands::config()?,
    }

    Ok(())
}
