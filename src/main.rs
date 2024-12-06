use anyhow::Result;
use clap::Parser;
use log::LevelFilter;
use owo_colors::OwoColorize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod solutions;

#[derive(Parser)]
struct Cli {
    /// Day to run. If not provided, all days will be run.
    day: Option<u32>,
    /// Log level (off, error, warn, info, debug, trace)
    #[arg(long, default_value = "warn")]
    log_level: LevelFilter,
    /// Show timing information
    #[arg(long)]
    timing: bool,
    /// Input file (defaults to inputs/dayN.txt)
    #[arg(long)]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(cli.log_level)
        .format_timestamp(None)
        .init();

    log::trace!("Enabled trace logging");
    log::debug!("Enabled debug logging");
    log::info!("Enabled info logging");

    let registry = solutions::get_registry();

    match (cli.day, cli.input) {
        (Some(day), Some(path)) => {
            println!("{} {}", "Running day".bright_green(), day);
            let file = File::open(path)?;
            let mut reader = BufReader::new(file);
            registry.run_day(day, &mut reader, cli.timing)
        }
        (Some(day), None) => {
            println!("{} {}", "Running day".bright_green(), day);
            let file = File::open(format!("inputs/day{}.txt", day))?;
            let mut reader = BufReader::new(file);
            registry.run_day(day, &mut reader, cli.timing)
        }
        (None, _) => registry.run_all(cli.timing),
    }
}
