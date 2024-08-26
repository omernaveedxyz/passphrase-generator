use clap::Parser;
use rand::Rng;
use std::collections::HashMap;

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
    #[arg(short, long, default_value = None)]
    max_length: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let nums = match args.max_length {
        Some(max_length) => {
            generate_nums_with_max_length(args.words, max_length, args.numbers, &args.separator)
        }
        None => generate_nums(args.words),
    };

    println!(
        "{}",
        convert_nums_to_passphrase(nums, args.capitalize, args.numbers, &args.separator)
    );
}

fn generate_nums_with_max_length(
    words: u8,
    max_length: u8,
    numbers: bool,
    separator: &str,
) -> Vec<u32> {
    let mut sum = if numbers { words as usize } else { 0 } + separator.len() * (words - 1) as usize;
    let mut nums: Vec<u32> = Vec::new();

    if sum + 13 * (words as usize) < (max_length as usize) {
        eprintln!("Specified max length is too long");
        std::process::exit(1);
    } else if sum + 4 * (words as usize) > (max_length as usize) {
        eprintln!("Specified max length is too short");
        std::process::exit(1);
    }

    for _ in 0..words {
        let num = rand::thread_rng().gen_range(4..=13);
        nums.push(num);
        sum += num as usize;
    }

    while sum < max_length as usize {
        let rnd = rand::thread_rng().gen_range(0..nums.len());
        if nums[rnd] < 13 {
            nums[rnd] += 1;
            sum += 1;
        }
    }

    while sum > max_length as usize {
        let rnd = rand::thread_rng().gen_range(0..nums.len());
        if nums[rnd] > 4 {
            nums[rnd] -= 1;
            sum -= 1;
        }
    }

    return nums;
}

fn generate_nums(words: u8) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();

    for _ in 0..words {
        let num = rand::thread_rng().gen_range(4..=13);
        nums.push(num);
    }

    return nums;
}

fn convert_nums_to_passphrase(
    nums: Vec<u32>,
    capitalize: bool,
    numbers: bool,
    separator: &str,
) -> String {
    let mut words: HashMap<u32, Vec<&str>> = HashMap::new();
    for word in include_str!("words.txt").lines() {
        match word.len() {
            4 => words.entry(4).or_insert(Vec::new()).push(word),
            5 => words.entry(5).or_insert(Vec::new()).push(word),
            6 => words.entry(6).or_insert(Vec::new()).push(word),
            7 => words.entry(7).or_insert(Vec::new()).push(word),
            8 => words.entry(8).or_insert(Vec::new()).push(word),
            9 => words.entry(9).or_insert(Vec::new()).push(word),
            10 => words.entry(10).or_insert(Vec::new()).push(word),
            11 => words.entry(11).or_insert(Vec::new()).push(word),
            12 => words.entry(12).or_insert(Vec::new()).push(word),
            13 => words.entry(13).or_insert(Vec::new()).push(word),
            _ => {
                eprintln!("Found word with unexpected length");
                std::process::exit(1)
            }
        }
    }

    let passphrase = nums
        .iter()
        .enumerate()
        .map(|(idx, &num)| {
            let rnd = rand::thread_rng().gen_range(0..words.get(&num).unwrap().len());
            let mut word: String = words.get(&num).unwrap()[rnd].to_string();

            if capitalize {
                word = titlecase(&word);
            }

            if numbers {
                let number = rand::thread_rng().gen_range(0..10);
                word = format!("{}{}", word, number);
            }

            if idx != nums.len() - 1 {
                word = format!("{}{}", word, separator);
            }

            return word;
        })
        .collect();

    return passphrase;
}

fn titlecase(s: &str) -> String {
    let mut c = s.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
