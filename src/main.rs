use clap::Parser;
use rand::Rng;

/// A simple passphrase generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of words in the generated passphrase
    #[arg(short, long, default_value_t = 5)]
    words: u8,

    /// Whether to capitalize the first letter in each word
    #[arg(short, long, default_value_t = false)]
    capitalize: bool,

    /// Whether to append a number (0-9) to each word
    #[arg(short, long, default_value_t = false)]
    numbers: bool,

    /// What string to use between words
    #[arg(short, long, default_value = "-")]
    separator: String,

    /// Maximum length of the generated passphrase
    #[arg(short, long, default_value_t = 255)]
    max_length: u8,
}

fn main() {
    let args = Args::parse();
    let words: Vec<&str> = include_str!("words.txt").lines().collect();
    let mut rng = rand::thread_rng();
    let mut passphrase = String::new();

    for count in (0..args.words).rev() {
        let filtered_words: Vec<&str> = words
            .iter()
            .filter(|&&word| {
                let word_length = word.len() as u8
                    + if args.numbers { 1 } else { 0 }
                    + args.separator.len() as u8;
                return word_length * (count + 1)
                    < args.max_length.saturating_sub(passphrase.len() as u8);
            })
            .copied()
            .collect();

        if filtered_words.is_empty() {
            eprintln!("No suitable words left for the remaining passphrase length. Please increase the max length or decrease the number of words.");
            return;
        }

        let mut word = filtered_words[rng.gen_range(0..filtered_words.len())].to_string();

        if args.capitalize {
            word = capitalize(&word);
        }

        if args.numbers {
            let number = rng.gen_range(0..10);
            word = format!("{}{}", word, number);
        }

        if !passphrase.is_empty() {
            passphrase.push_str(&args.separator);
        }

        passphrase.push_str(&word);
    }

    println!("{}", passphrase);
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
