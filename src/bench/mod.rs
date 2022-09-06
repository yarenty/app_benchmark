use serde_derive::{Deserialize, Serialize};
use std::env;
use std::process::{Command, Stdio};
use thiserror::Error;
// use core::fmt::Error;

pub mod analysis;
pub mod collector;
pub mod graphs;

pub type Result<T> = std::result::Result<T, BenchError>;

#[derive(Error, Debug)]
pub enum BenchError {
    /// benchmarking related errors
    #[error("{0}")]
    Unknown(String),
    /// app not found
    #[error("{0}")]
    AppNotFound(String),
    // /// number of runs >2
    // #[error("{0}")]
    // TooSmallRuns(String),
    /// IO error -
    #[error("{0}")]
    IOError(String),
    /// visualisation error
    #[error("{0}")]
    Visualization(String),
    // /// Error wrapper for plotter
    // #[error("{0}")]
    // PlotterError(#[from] dyn std::error::Error),
}
/*
impl From<BenchError> for dyn  std::error::Error {
    fn from(error: BenchError) -> Self {
        match error {
            BenchError::PlotterError(e) => Error(e),
        }
    }
}
*/

// Note that structs can derive both Serialize and Deserialize!
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Record<'a> {
    time: &'a str,
    cpu: &'a str,
    mem: &'a str,
}

pub fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

/// Checking if application is in current dir or is the full path.
/// Returns full paths and short name of app.
/// Or error other wise.
pub fn check_in_current_dir(app: &str) -> Result<(String, String)> {
    let (full, short) = if app.contains(std::path::MAIN_SEPARATOR) {
        (
            app.to_string(),
            app.split(std::path::MAIN_SEPARATOR)
                .last()
                .unwrap()
                .to_string(),
        )
    } else {
        (
            format!(
                "{}{}{}",
                get_current_working_dir(),
                std::path::MAIN_SEPARATOR,
                app
            ),
            app.to_string(),
        )
    };

    let checker = if cfg!(target_os = "windows") {
        "dir"
    } else {
        "ls"
    };

    let cmd = Command::new(checker)
        .arg(&full)
        .current_dir(get_current_working_dir())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output();

    match cmd {
        Ok(out) => {
            if out.status.code() == Some(0) {
                Ok((full, short))
            } else {
                Err(BenchError::AppNotFound(format!(
                    "Could not find application: {}.",
                    short
                )))
            }
        }
        Err(e) => Err(BenchError::Unknown(format!(
            "Wrong system utils - are you on windows? {:?}",
            e
        ))),
    }
}

pub fn create_output_file(app: &str, filename: &str) -> tagger::Adaptor<std::fs::File> {
    std::fs::create_dir_all(format!("bench_{}", app)).expect("Cannot create output directory");
    let file = std::fs::File::create(format!(
        "bench_{}{}{}",
        app,
        std::path::MAIN_SEPARATOR,
        filename
    ))
    .unwrap_or_else(|_| panic!("Cannot create output file: {}", filename));
    tagger::upgrade_write(file)
}
