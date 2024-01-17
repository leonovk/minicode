use clap::Parser;

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
        self_update();
        return;
    }

    if cli.path != None {
        println!("run code");
    } else {
        println!("enter the command --help or any other command");
    }
}

fn self_update() {
    println!("updating");
}
