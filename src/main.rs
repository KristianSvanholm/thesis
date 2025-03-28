mod benchmark;

use benchmark::Export;
use clap::{Parser, Subcommand};
use csv::Writer;
use std::io::Write;

#[derive(Subcommand, Debug, Clone)]
pub enum Mode {
    /// Run the
    Run {
        /// Number of runs per task
        #[arg(short, long, default_value_t = 1)]
        runs: u64,
        /// Run benchmarks in order
        #[arg(short, long, action)]
        ordered: bool,
        /// How many seconds delay between each benchmark
        #[arg(short, long, default_value_t = 0)]
        cooldown: u64,
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
    /// Set language filter (comma separated list)
    #[arg(short, long)]
    language: Option<String>,
    /// Set task filter (comma separated list)
    #[arg(short, long)]
    task: Option<String>,
    /// Whether to display task and langauge coverage matrix or not
    #[arg(short, long, action)]
    matrix: bool,
}

use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

//TODO:: Redo with Clap instead
fn spl(inp: Option<String>) -> Vec<String> {
    if let Some(l) = inp {
        return l.split(",").map(|e| e.to_string().to_lowercase()).collect();
    } else {
        return vec![]
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    // Intitialize terminal user interface
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut events = EventHandler::new(250);

    let args = CLI::parse();

    let lang = spl(args.language);
    let task = spl(args.task);

    // Fetch all tasks and filter out unwanted tasks
    let tasks = benchmark::list_all(args.path)?
        .into_iter()
        .filter(|t| {
            lang.len()==0 || lang.contains(&t.language.to_lowercase())
        })
        .filter(|t| task.len() ==0 || task.contains(&t.name.to_lowercase()))
        .collect();

    // Create App
    let mut app = App::new(tasks, args.mode, events.new_sender());

    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    app.iterate();

    while app.running {
        // Render UI
        tui.draw(&mut app)?;
        // Handle events
        match tui.events.next().await? {
            Event::CompileDone(task) => app.next((task.language, task.name)),
            Event::Status(msg) => app.status(msg),
            Event::TaskDone(data) => app.done(data),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Tick => app.tick(),
        }
    }

    if app.results.len() != 0 {
        let _ = csv(app.results);
    }

    // Exit the UI
    tui.exit()?;
    Ok(())
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
