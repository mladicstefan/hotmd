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

    loop {
        match rx.recv() {
            Ok(event) => println!("File changed: {:?}", event),
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Args::parse();

    let filepath = Path::new(&cli.file);
    if !filepath.exists() {
        eprintln!("file not found {:?}", filepath);
        std::process::exit(1);
    }

    watch_file(filepath)?;
    Ok(())
}
