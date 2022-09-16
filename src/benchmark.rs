//! Full benchmarking solution for separate end-to-end applications.

use clap::Parser;
use log::info;

mod bench;
mod error;
mod utils;

use crate::bench::analysis::analyze;
use crate::bench::collector::process_outputs;
use crate::bench::graphs::*;
use error::Result;
use utils::check_in_current_dir;
use utils::setup_logger;

#[derive(Parser, Debug)]
#[clap(version)]
#[clap(about = "Benchmarking data collector.", long_about = None)]
struct Args {
    /// Application path (just name if it is in the same directory).
    #[clap(value_parser)]
    application: String,

    /// Number of runs to be executed.
    #[clap(short, long)]
    #[clap(default_value_t = 10)]
    runs: usize,

    ///Set custom log level: info, debug, trace
    #[clap(short, long, default_value = "info")]
    log: String,
}

/// Full benchmarking solution for separate end-to-end applications.
fn main() -> Result<()> {
    let args = Args::parse();
    setup_logger(true, Some(&args.log));

    info!("Application to be benchmark is: {}", &args.application);
    info!("Number of runs: {}", &args.runs);

    let (path, app) = check_in_current_dir(&args.application)?;

    info!("Collecting data::{}", &app);
    let out = analyze(&app, &path, args.runs)?;

    info!("Processing outputs");
    let (time, cpu, mem) = process_outputs(&app, &out)?;

    cpu_graph(&app, cpu)?;
    time_graph(&app, time)?;
    mem_graph(&app, mem)?;

    Ok(())
}
