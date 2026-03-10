use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tool")]
#[command(about = "Example CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(value_parser = validate_input)]
        input: String,
    },
    Decode {
        #[arg(value_parser = validate_input)]
        input: String,
    },
}

fn validate_input(s: &str) -> Result<String, String> {
    if s.chars().count() <= 10 {
        Ok(s.to_string())
    } else {
        Err("input must be at most 10 characters".into())
    }
}

fn encode(s: &str) {
    println!("Encoding: {}", s);
}

fn decode(s: &str) {
    println!("Decoding: {}", s);
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input } => {
            validate(&input);
            encode(&input);
        }
        Commands::Decode { input } => {
            validate(&input);
            decode(&input);
        }
    }
}

fn validate(s: &str) {
    if s.chars().count() > 10 {
        eprintln!("Error: input must be at most 10 characters");
        std::process::exit(1);
    }
}

