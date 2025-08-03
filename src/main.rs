use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "hotmd")]
#[command(about = "Live Markdown to HTML transpiler")]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let cli = Args::parse();
    println!("{:?}", cli);
}
