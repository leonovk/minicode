use clap::Parser;
mod code_runner;
mod files;
mod parser;

/// My lang
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// file path
    #[arg(short, long)]
    path: Option<String>,

    /// update command
    #[arg(short, long)]
    update: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.update {
        println!("updating");
    } else if cli.path != None {
        code_runner::run(cli.path.unwrap());
    } else {
        println!("enter the command --help or any other command");
    }
}
