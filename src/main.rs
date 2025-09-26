use clap::{Arg, Command};

mod extractor;

fn main() {
    // Get current working directory
    let cwd = std::env::current_dir().unwrap();

    let matches = Command::new("Epub Image Extractor")
        .version("0.1.0")
        .author("Taiyuuki<taiyuuki@qq.com>")
        .about("Extract images from ePub file and rename them in order.")
        .arg(
            Arg::new("input")
                .value_name("EPUB File")
                .required(true)
                .num_args(1..)
                .help("One or more EPUB files to extract images from."),
        )
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .value_name("Ignore Size")
                .help("Ignore images smaller than the specified size in KB."),
        )
        .get_matches();
    if let Some(inputs) = matches.get_many::<String>("input") {
        let ignore_size = matches.get_one::<String>("ignore");
        for input in inputs {
            let input_epub = cwd.as_path().join(input);
            println!("\nProcessing: {}", input_epub.display());
            let extractor = extractor::EpubExtractor::new(input_epub.to_str().unwrap());
            if extractor.is_err() {
                eprintln!("Not exist or not a valid epub file: {}", input);
                continue;
            }
            let mut extractor = extractor.unwrap();
            const BAR_CHARS: &str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠇";
            match ignore_size {
                Some(size) => {
                    match size.parse::<usize>() {
                        Ok(size) => {
                            extractor.set_ignore_size(size);
                        }
                        Err(_) => {
                            eprintln!("Invalid ignore size: {}", size);
                            continue;
                        }
                    };
                }
                None => {}
            }
            println!("Extracting...");
            extractor
                .extract(|s| {
                    // Print progress
                    print!(
                        "\r {} {}% \u{1b}[42m{}\u{1b}[0m",
                        BAR_CHARS.chars().nth(s % 10).unwrap(),
                        s,
                        " ".repeat(s / 4),
                    );
                })
                .unwrap();
            print!("\r Done \u{1b}[42m{}\u{1b}[0m\n", " ".repeat(20));
        }
    }
}
