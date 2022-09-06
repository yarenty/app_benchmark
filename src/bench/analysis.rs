use crate::bench::{get_current_working_dir, BenchError, Result};
use log::{debug, info, trace};
use std::process::{Command, Stdio};

pub fn analyze(app: &str, path: &str, runs: usize) -> Result<Vec<String>> {
    let timer = if cfg!(target_os = "macos") {
        "gtime"
    } else {
        "time"
    };

    debug!("Collector for {}: {}", &app, &timer);

    let mut out: Vec<String> = vec![];

    for i in 0..runs {
        info!("Run {} of {}", i, runs);
        let cmd = Command::new(timer)
            .arg("-v")
            .arg(&path)
            .current_dir(get_current_working_dir())
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output();

        trace!("CMD::{:?}", cmd);

        match cmd {
            Ok(reading) => {
                let proc = String::from_utf8(reading.stderr).unwrap();

                debug!("PROC: {}", proc);
                out.push(proc);
            }
            Err(e) => {
                return Err(BenchError::Unknown(format!(
                    "Error collecting outputs {:?}",
                    e
                )))
            }
        }
    }

    Ok(out)
}
