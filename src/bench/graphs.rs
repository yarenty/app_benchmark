use crate::bench::{create_output_file, BenchError, Result};
use fmt::Write;
use itertools::Itertools;
use poloto::prelude::*;
use poloto::simple_theme::SimpleTheme;
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
    scatter(app, y, "time.svg", "Processing time", "time [ms]")
}

pub fn mem_graph(app: &str, y: Vec<i32>) -> Result<()> {
    scatter(
        app,
        y.iter().map(|x| *x as i64).collect_vec(),
        "mem.svg",
        "Memory usage",
        "memory [kB]",
    )
}

pub fn scatter(app: &str, y: Vec<i64>, file: &str, title: &str, y_axis: &str) -> Result<()> {
    let data: _ = y
        .iter()
        .enumerate()
        .map(|(idx, n)| (idx as i128, *n as f64))
        .collect::<Vec<(i128, f64)>>(); //.collect();

    let plotter = poloto::quick_fmt!(
        title,
        "execution no",
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

#[allow(unused)]
pub fn bars() -> fmt::Result {
    let trend = vec![
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.iter().copied());

    let data = poloto::data(plots!(
        it.cloned_plot().histogram(""),
        poloto::build::markers([24], [])
    ));

    let opt = poloto::render::render_opt();
    let (_, by) = poloto::ticks::bounds(&data, &opt);
    let xtick_fmt = poloto::ticks::from_iter((0..).step_by(6));
    let ytick_fmt = poloto::ticks::from_default(by);

    let pp = poloto::plot_with(
        data,
        opt,
        poloto::plot_fmt(
            "Number of rides at theme park hourly",
            "Hour",
            "Number of rides",
            xtick_fmt.with_tick_fmt(|w, v| write!(w, "{} hr", v)),
            ytick_fmt,
        ),
    );

    // print!("{}", poloto::disp(|w| pp.simple_theme(w)));

    // let plotter = poloto::plot_with(s, &opt, fmt);

    let w = create_output_file("test", "bars.svg");

    pp.simple_theme(w)
}
