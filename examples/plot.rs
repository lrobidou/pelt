// cargo run --example plot -- --data 36,42,37,42,41,39,40,39,40,42,37,37,8,9,12,8,12,11,9,10,9,8,14,80,80,78,76,76,78,82,76,78,80,82,81,81,77,79 --penalties 0.0,1.0,3.0

use std::collections::HashSet;

use clap::Parser;
use pelt_reindeer2::{pelt, score};

/// Simple program to parse a list of integers
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[clap(long, value_delimiter = ',')]
    data: Vec<u64>,
    #[clap(long, value_delimiter = ',')]
    penalties: Vec<f64>,
}

#[derive(serde::Serialize)]
struct DataOfASinglePlot {
    penalty: f64,
    breakpoints: HashSet<usize>,
}

#[derive(serde::Serialize)]
struct DataToPlot {
    input: Vec<u64>,
    data: Vec<DataOfASinglePlot>,
}

fn main() {
    let args = Args::parse();

    let data_plots = args
        .penalties
        .iter()
        .map(|penalty| DataOfASinglePlot {
            penalty: *penalty,
            breakpoints: pelt(&args.data, score, *penalty),
        })
        .collect();

    let data = DataToPlot {
        input: args.data,
        data: data_plots,
    };

    let path = "data.json";
    let s = serde_json::to_string(&data).unwrap();
    std::fs::write(path, s).unwrap();
    println!("Written to {}", path);
}
