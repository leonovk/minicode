use clap::Parser;
mod code_runner;
mod files;
mod interpreter;
mod network;
mod opcode;
mod parser;
mod self_update;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// file path
    #[arg(short, long)]
    path: Option<String>,

    /// command line arguments
    args: Option<Vec<String>>,

    /// update command
    #[arg(short, long)]
    update: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.update {
        if let Err(e) = self_update::update() {
            println!("[ERROR] {}", e);
            ::std::process::exit(1);
        }
    } else if cli.path != None {
        let path = cli.path.unwrap();
        let args = cli.args.unwrap_or(vec![]);
        code_runner::run(path, args);
    } else {
        println!("enter the command --help or any other command");
    }
}
