mod benchmark;

use benchmark::Export;
use std::collections::HashMap;
use chrono::Utc;
use clap::{Parser, Subcommand};
use csv::Writer;
use std::io::Write;

use rand::rng;
use rand::seq::SliceRandom;

#[derive(Subcommand, Debug, Clone)]
enum Mode {
    /// Run the benchmarks
    Run {
        /// Number of runs per task
        #[arg(short, long, default_value_t = 1)]
        runs: u64,
        /// Run benchmarks in order
        #[arg(short, long, action)]
        ordered: bool,
        /// How many seconds delay between each benchmark
        #[arg(short,long, default_value_t = 0)]
        cooldown: u64
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
    /// Whether to display task and langauge coverage matrix or not
    #[arg(short,long, action)]
    matrix: bool,
}

fn main() {

    let start_time = Utc::now().time();

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

    if args.matrix { matrix(tasks.clone()); }

    let str = match args.mode {
        Mode::Run{ .. } => "Running",
        Mode::Compile => "Compiling"
    };

    println!("{} {} benchmarks ...", str, tasks.len());

    match args.mode {
        Mode::Run { runs, ordered, cooldown } => run_and_export(tasks, runs, ordered, cooldown),
        Mode::Compile => benchmark::compile(tasks),
    }

    let duration = Utc::now().time() - start_time;
    println!("Process took {} second(s)", duration.num_seconds())
}

// Way too big for a terminal, but whatever
fn matrix(tasks: Vec<benchmark::Task>) {
    let mut map: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut names = HashMap::new();
    for task in tasks {
        let t = task.clone();
        map.entry(t.language).or_insert_with(HashMap::new).insert(t.name.clone(), true);
        names.entry(t.name).or_insert(true);
    }

    print!("{:11}", "");
    for (unique_task, _) in names.clone() {
        print!("{:^18}", unique_task)
    }
    print!("\n");
    for (lang, value) in map {
        print!("{:11}", lang);
        for (unique_task, _) in names.clone() {
            print!("{:^18}", if value.get(&unique_task).is_some() {"x"} else {""});
        }
        print!("\n")
    }
}

fn filter_list(tasks: Vec<benchmark::Task>, lang: Option<&str>, name: Option<&str>) -> Vec<benchmark::Task> {
  
    tasks.into_iter()
        .filter(|t| lang.is_none() || t.language.to_lowercase() == lang.unwrap().to_lowercase())
        .filter(|t| name.is_none() || t.name.to_lowercase() == name.unwrap().to_lowercase())
        .collect()
}

fn run_and_export(unique_tasks: Vec<benchmark::Task>, runs: u64, ordered: bool, cooldown: u64) {
    
    let mut tasks = vec![];
    for ut in unique_tasks {
        for _ in 0..runs {
            tasks.push(ut.clone());
        }
    }

    if !ordered {
        tasks.shuffle(&mut rng());
    }

    let Ok(exports) = benchmark::run(tasks, runs, cooldown) else { panic!("Something has gone terribly wrong :(") };
    let _ = csv(exports);
}

use std::fs::File;
fn csv(data: Vec<Export>) -> Result<(), Box<dyn std::error::Error>> {
    // Serialize to CSV
    let mut writer = Writer::from_writer(vec![]);
    for itt in data {
        writer.serialize(itt)?;
    }

    // Write data to CSV
    let data = String::from_utf8(writer.into_inner()?)?;
    let mut file = File::create("energy.csv")?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
