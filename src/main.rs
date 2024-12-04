use clap::Parser;
use minicode_core::code_runner;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// file path
    #[arg(short, long)]
    path: Option<String>,

    /// command line arguments
    args: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();

    if cli.path.is_some() {
        let path = cli.path.unwrap();
        let args = cli.args.unwrap_or(vec![]);
        code_runner::run(path, args);
    } else {
        println!("enter the command --help or any other command");
    }
}
