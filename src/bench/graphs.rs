use crate::error::{BenchError, Result};
use crate::utils::create_output_file;
use fmt::Write;
use itertools::Itertools;
use poloto::prelude::*;
use std::fmt;

pub fn cpu_graph(app: &str, y: Vec<i32>) -> Result<()> {
    scatter(
        app,
        y.iter().map(|x| *x as i64).collect_vec(),
        "cpu.svg",
        "CPU occupancy",
        "cpus [%]",
    )
}

pub fn time_graph(app: &str, y: Vec<i64>) -> Result<()> {
    if y.iter().max().unwrap() > &9999 {
        // over 10 sec - move scale to sec
        scatter(
            app,
            y.iter().map(|x| (*x / 1000) as i64).collect_vec(),
            "time.svg",
            "Processing time",
            "time [s]",
        )
    } else {
        scatter(app, y, "time.svg", "Processing time", "time [ms]")
    }
}

pub fn mem_graph(app: &str, y: Vec<i32>) -> Result<()> {
    if y.iter().max().unwrap() > &2048 {
        scatter(
            app,
            y.iter().map(|x| (*x / 1024) as i64).collect_vec(),
            "mem.svg",
            "Memory usage",
            "memory [MB]",
        )
    } else {
        scatter(
            app,
            y.iter().map(|x| *x as i64).collect_vec(),
            "mem.svg",
            "Memory usage",
            "memory [kB]",
        )
    }
}

pub fn scatter(app: &str, y: Vec<i64>, file: &str, title: &str, y_axis: &str) -> Result<()> {
    let data: _ = y
        .iter()
        .enumerate()
        .map(|(idx, n)| (idx as i128, *n as f64))
        .collect::<Vec<(i128, f64)>>(); //.collect();

    let plotter = poloto::quick_fmt!(
        title,
        "runs",
        y_axis,
        poloto::build::markers([], [0.0]),
        data.iter().cloned_plot().line("")
    );
    let mut w = create_output_file(app, file);

    match write!(
        w,
        "{}<style>{}.poloto_scatter{{stroke-width:33;}}.poloto_scatter.poloto_legend_icon{{stroke-width:10}}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(BenchError::Visualization(format!("{:?}", e))),
    }
}
