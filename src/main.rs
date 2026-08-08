use civic_data_forge::{load, summarize};
use std::env;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: civic-data-forge OBSERVATIONS.csv");
        std::process::exit(2);
    };
    match load(path) {
        Ok(rows) => {
            for (region, summary) in summarize(&rows) {
                println!(
                    "{region}: {} observations, total {}, peak {}, average {:.2}",
                    summary.observations,
                    summary.total_count,
                    summary.peak_count,
                    summary.average_count
                );
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(1);
        }
    }
}
