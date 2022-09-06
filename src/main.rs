use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Simple Japanese text analysis tool
#[derive(Parser, Debug)]
#[clap(version, arg_required_else_help(true))]
struct Args {
    /// The file to analyze or stdin by default
    #[clap(value_parser)]
    file: Option<String>,

    /// Show a frequency table of all the characters specified by the character type
    #[clap(short, long, value_enum)]
    frequency: Option<jpaz::CharType>,

    /// Prints unique number of characters for all character types
    #[clap(short, long, action)]
    unique: bool,

    /// Prints number of characters for all character types
    #[clap(short, long, action)]
    count: bool,

    /// Character types to exclude from counts (not frequencies)
    #[clap(short, long, value_enum, multiple_values(true))]
    exclude: Option<Vec<jpaz::CharType>>,
}

fn main() {
    let args = Args::parse();

    let analyzer = parse_input(&args.file).unwrap_or_else(|err| {
        eprintln!("Failed to read: {err}");
        std::process::exit(1);
    });

    if let Some(char_type) = &args.frequency {
        let freqs = analyzer.char_freqs(char_type);
        for (char, count) in freqs {
            println!("{char} {count}");
        }
    }

    let exclude_list = args.exclude.unwrap_or(Vec::new());
    macro_rules! print_count {
        ($count_func: ident) => {
            let mut total_count = 0;
            let mut counts = Vec::new();
            for char_type in jpaz::ALL_CHAR_TYPES {
                if !exclude_list.iter().any(|&i| i == *char_type) {
                    let count = analyzer.$count_func(char_type);
                    total_count += count;
                    counts.push((char_type, count));
                }
            }

            for (char_type, count) in counts {
                println!(
                    "{char_type} {count} {:.2}%",
                    (count as f32 / total_count as f32) * 100.0
                );
            }

            println!("Total {total_count}");
        };
    }

    if args.count {
        print_count!(get_total_count);
    }

    if args.unique {
        print_count!(get_unique_count);
    }
}

fn parse_input(filename: &Option<String>) -> Result<jpaz::Analyzer, std::io::Error> {
    let mut analyzer = jpaz::Analyzer::default();
    if let Some(file_name) = filename {
        let file = File::open(file_name)?;
        for line in BufReader::new(file).lines() {
            analyzer.read_str(&line?);
        }
    } else {
        for line in std::io::stdin().lines() {
            analyzer.read_str(&line?);
        }
    }

    Ok(analyzer)
}
