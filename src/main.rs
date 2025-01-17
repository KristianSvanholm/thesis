mod benchmark;

use benchmark::Exports;
use clap::{Parser, Subcommand};
use csv::Writer;
use std::io::Write;

#[derive(Subcommand, Debug, Clone)]
enum Mode {
    /// Run the benchmarks
    Run {
        /// Number of runs per task
        #[arg(short, long, default_value_t = 1)]
        runs: u64,
    },
    /// Compile the benchmarks
    Compile,
}

#[derive(Debug, Parser)]
#[command(version, verbatim_doc_comment)]
struct CLI {
    #[command(subcommand)]
    mode: Mode,
    /// Set path to benchmarks directory
    #[arg(short, long, default_value = "./benchmarks")]
    path: String,
    /// Set language filter
    #[arg(short, long)]
    language: Option<String>,
    /// Set task filter
    #[arg(short, long)]
    task: Option<String>,
}

fn main() {
    let args = CLI::parse();

    let mut tasks = match benchmark::list_all(args.path) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Filter out unwanted tasks
    tasks = filter_list(tasks, args.language.as_deref(), args.task.as_deref());

    match args.mode {
        Mode::Run { runs } => run_and_export(tasks, runs),
        Mode::Compile => benchmark::compile(tasks),
    }
}

fn filter_list(tasks: Vec<benchmark::Task>, lang: Option<&str>, name: Option<&str>) -> Vec<benchmark::Task> {
  
    tasks.into_iter()
        .filter(|t| lang.is_none() || t.language.to_lowercase() == lang.unwrap().to_lowercase())
        .filter(|t| name.is_none() || t.name.to_lowercase() == name.unwrap().to_lowercase())
        .collect()
}

fn run_and_export(tasks: Vec<benchmark::Task>, runs: u64) {
    let Ok(exports) = benchmark::run(tasks, runs) else { panic!("AAAA") };
    let _ = csv(exports);
}

use std::fs::File;
fn csv(data: Vec<Exports>) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize to CSV
    let mut writer = Writer::from_writer(vec![]);
    for lang in data {
        for itt in lang.0 {
            writer.serialize(itt)?;
        }
    }

    // Write data to CSV
    let data = String::from_utf8(writer.into_inner()?)?;
    let mut file = File::create("energy.csv")?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
