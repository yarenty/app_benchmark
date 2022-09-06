use std::io::Write;
use std::thread;

use chrono::prelude::*;
use env_logger::fmt::{Color, Formatter};
use env_logger::{Builder, WriteStyle};
use log::{Level, LevelFilter, Record};

pub fn setup_logger(log_thread: bool, rust_log: Option<&str>) {
    let output_format = move |formatter: &mut Formatter, record: &Record| {
        let thread_name = if log_thread {
            format!("(t: {}) ", thread::current().name().unwrap_or("unknown"))
        } else {
            "".to_string()
        };

        let mut thread_style = formatter.style();
        let mut level_style = formatter.style();

        match record.level() {
            Level::Error => level_style.set_color(Color::Red).set_bold(true),
            Level::Warn => level_style.set_color(Color::Red),
            Level::Info => level_style.set_color(Color::Green).set_intense(true),
            Level::Debug => level_style.set_color(Color::Blue),
            Level::Trace => level_style.set_color(Color::Magenta),
        };
        thread_style.set_color(Color::Magenta).set_intense(true);

        let local_time: DateTime<Local> = Local::now();
        let time_str = local_time.format("%H:%M:%S%.3f").to_string();
        writeln!(
            formatter,
            "{} {}{} - {} - {}",
            time_str,
            thread_style.value(thread_name),
            level_style.value(record.level()),
            record.target(),
            record.args()
        )
    };

    let mut builder = Builder::new();
    builder
        .format(output_format)
        .filter(None, LevelFilter::Info);
    builder.write_style(WriteStyle::Always);

    rust_log.map(|conf| builder.parse_filters(conf));

    builder.init();
}

