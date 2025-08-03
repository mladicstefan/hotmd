use clap::Parser;
use notify::{RecursiveMode, Watcher};
use std::error::Error;
use std::path::Path;
use std::sync::mpsc;

#[derive(Parser, Debug)]
#[command(name = "hotmd")]
#[command(about = "Live Markdown to HTML transpiler")]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn watch_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("File changed: {:?}", event),
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();
    println!("{:?}", cli);
    watch_file(Path::new(&cli.file))?;
    Ok(())
}
