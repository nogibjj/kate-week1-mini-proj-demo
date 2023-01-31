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
    Hello {
        #[clap(short, long)]
        firstname: String,
        #[clap(short, long)]
        lastname: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Hello {
            firstname,
            lastname,
        }) => {
            let result = cli::hello(&firstname, &lastname);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
