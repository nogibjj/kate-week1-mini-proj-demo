use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Kate Feng", about = "A Marco Polo game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Kate Feng")]
    Play {
        #[clap(short, long)]
        first_name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { first_name }) => {
            let result = cli::marco_polo(&first_name);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
