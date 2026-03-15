use clap::{Parser, Subcommand};

const BASE23: &str = "0123456789ABCDEFGHIJKLM";

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
        input: u64,
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

fn get_offset(s: char) -> Option<usize> {
    BASE23.find(s)
}

fn decode(s: &str) {
    let mut sum: u64 = 0;
    for c in s.chars() {
        let offset = get_offset(c).expect("Bad String") as u64;
        sum = sum *23 + offset;
    }
    println!("Base10: {}", sum);
}

fn encode(s: &u64) {
    let encode_vector = BASE23.chars().collect::<Vec<char>>();
    let mut acc = s.clone();
    let mut sum = String::new();

    if acc == 0 {
        sum = String::from("0");
    }

    while acc > 0 {
        let m = acc % 23;
        acc /= 23;
        sum.push(encode_vector[m as usize]);
    }
    println!("Base23: {}", sum.chars().rev().collect::<String>());
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input } => {
            encode(&input);
        }
        Commands::Decode { input } => {
            decode(&input.to_uppercase());
        }
    }
}


#[cfg(test)]
mod tests {
   use super::*;

    #[test]
    fn test_get_offset() {
        assert_eq!(get_offset('A'), Some(10));
    }
    #[test]
    fn test_get_offset_2() {
        assert_eq!(get_offset('x'), None);
    }
    #[test]
    fn test_get_offset_3() {
        assert_eq!(get_offset('M'), Some(22));
    }

    /*
    #[test]
    fn test_encode() {
        assert_eq!();
    }
     */


}