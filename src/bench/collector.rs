use crate::bench::{create_output_file, BenchError, Record, Result};
use csv;
use log::info;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

/// Translate output string into csv style output:
/// '''
/// User time (seconds): 0.05
/// System time (seconds): 0.01
/// Percent of CPU this job got: 144%
/// Elapsed (wall clock) time (h:mm:ss or m:ss): 0:00.04
/// Average shared text size (kbytes): 0
/// Average unshared data size (kbytes): 0
/// Average stack size (kbytes): 0
/// Average total size (kbytes): 0
/// Maximum resident set size (kbytes): 19076
/// Average resident set size (kbytes): 0
/// Major (requiring I/O) page faults: 0
/// Minor (reclaiming a frame) page faults: 5450
/// Voluntary context switches: 0
/// Involuntary context switches: 305
/// Swaps: 0
/// File system inputs: 0
/// File system outputs: 0
/// Socket messages sent: 0
/// Socket messages received: 0
/// Signals delivered: 0
/// Page size (bytes): 4096
/// Exit status: 0
/// ```
///
/// out:
/// ```
/// time, CPU, mem
/// 50, 144, 19076
/// ```
pub fn process_outputs(app: &str, runs: &Vec<String>) -> Result<(Vec<i64>, Vec<i32>, Vec<i32>)> {
    // let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_writer(create_output_file(app, "benchmarks.csv").inner);

    let mut time = vec![];
    let mut cpu = vec![];
    let mut mem = vec![];

    for run in runs {
        let mut r = Record::default();
        for ip in run.split(LINE_ENDING) {
            if ip.contains("User time") {
                let i = ip.find(':').unwrap() + 2;
                let j = ip.len();
                let o = &ip[i..j];
                print!("{},", o);
                r.time = o;
                // time.push( sec_to_i64(o));
                time.push((o.parse::<f64>().unwrap() * 1000.0) as i64);
            }
            if ip.contains("Percent of CPU") {
                let i = ip.find(':').unwrap() + 2;
                let j = ip.len() - 1;
                let o = &ip[i..j];
                print!("{},", o);
                r.cpu = o;
                cpu.push(o.parse::<i32>().unwrap());
            }
            if ip.contains("Maximum resident") {
                let i = ip.find(':').unwrap() + 2;
                let j = ip.len();
                let o = &ip[i..j];
                println!("{},", o);
                r.mem = o;
                mem.push(o.parse::<i32>().unwrap());
            }
        }

        wtr.serialize(r).expect("Error serializing outputs to csv");
    }

    //summary
    info!("SUMMARY:");
    info!(
        "Time [ms]:: min: {}, max: {}, avg: {} ms",
        time.iter().min().unwrap(),
        time.iter().max().unwrap(),
        average_i64(&time)
    );
    info!(
        "CPU [%]:: min: {}, max: {}, avg: {} %",
        cpu.iter().min().unwrap(),
        cpu.iter().max().unwrap(),
        average(&cpu)
    );
    info!(
        "Memory [kB]:: min: {}, max: {}, avg: {} kB",
        mem.iter().min().unwrap(),
        mem.iter().max().unwrap(),
        average(&mem)
    );

    //FIXME!!
    match wtr.flush() {
        Ok(_) => Ok((time, cpu, mem)),
        Err(e) => Err(BenchError::IOError(format!(
            "Could not create output csv: {:?}",
            e
        ))),
    }
}

fn average_i64(numbers: &Vec<i64>) -> f64 {
    numbers.iter().sum::<i64>() as f64 / numbers.len() as f64
}

fn average(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}
