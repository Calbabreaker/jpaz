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
    frequency: Option<jpaz::CharKind>,

    /// Prints unique number of characters for all character types
    #[clap(short, long, action)]
    unique: bool,

    /// Prints number of characters for all character types
    #[clap(short, long, action)]
    count: bool,

    /// Character types to exclude from counts (not frequencies)
    #[clap(short, long, value_enum, num_args(1..))]
    exclude: Option<Vec<jpaz::CharKind>>,
}

fn main() {
    let args = Args::parse();

    let analyzer = parse_input(&args.file).unwrap_or_else(|err| {
        eprintln!("Failed to read: {err}");
        std::process::exit(1);
    });

    if let Some(kind) = &args.frequency {
        let freqs = analyzer.char_freqs(*kind);
        for (char, count) in freqs {
            println!("{char} {count}");
        }
    }

    let exclude_list = args.exclude.unwrap_or(Vec::new());
    if args.count {
        print_count(&exclude_list, |char| analyzer.get_total_count(char));
    }

    if args.unique {
        print_count(&exclude_list, |char| analyzer.get_unique_count(char));
    }
}

fn parse_input(filename: &Option<String>) -> Result<jpaz::Analyzer, std::io::Error> {
    let mut analyzer = jpaz::Analyzer::default();
    if let Some(file_name) = filename {
        let file = File::open(file_name)?;
        for line in BufReader::new(file).lines() {
            analyzer.parse_str(&line?);
        }
    } else {
        for line in std::io::stdin().lines() {
            analyzer.parse_str(&line?);
        }
    }

    Ok(analyzer)
}

fn print_count(exclude_list: &[jpaz::CharKind], get_count_func: impl Fn(jpaz::CharKind) -> u32) {
    let mut total_count = 0;
    let mut counts = Vec::new();
    for kind in jpaz::CharKind::ALL {
        if !exclude_list.iter().any(|&i| i == *kind) {
            let count = get_count_func(*kind);
            total_count += count;
            counts.push((kind, count));
        }
    }

    for (kind, count) in counts {
        let percentage = (count as f32 / total_count as f32) * 100.0;
        println!(
            "{kind} {count} {:.2}%",
            if percentage.is_nan() { 0.0 } else { percentage }
        );
    }

    println!("Total {total_count}");
}
