use clap::{Arg, Command};

mod extractor;

fn main() {
    // Get current working directory
    let cwd = std::env::current_dir().unwrap();

    let matches = Command::new("Epub Image Extractor")
        .version("0.1.0")
        .author("Taiyuuki<taiyuuki@qq.com>")
        .about("Extract images from epub, and save to output directory")
        .arg(Arg::new("input").value_name("EPUB File").required(true))
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .value_name("Ignore Size")
                .help("Ignore images smaller than the specified size in KB."),
        )
        .get_matches();

    if let Some(input) = matches.get_one::<String>("input") {
        // Get input epub path
        let input_epub = cwd.as_path().join(input);
        // Extract images
        let extractor = extractor::EpubExtractor::new(input_epub.to_str().unwrap());
        if extractor.is_none() {
            println!(
                "Not exist or not a valid epub file: {}",
                input_epub.display()
            );
            return;
        }
        let mut extractor = extractor.unwrap();
        println!("Extracting...");
        const BAR_CHARS: &str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠇";
        let ignore_size = matches.get_one::<String>("ignore");
        match ignore_size {
            Some(size) => {
                match size.parse::<usize>() {
                    Ok(size) => {
                        extractor.set_ignore_size(size);
                    }
                    Err(_) => {
                        println!("Invalid ignore size: {}", size);
                        return;
                    }
                };
            }
            None => println!(" "),
        }
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
        print!("\r Done \u{1b}[42m{}\u{1b}[0m", " ".repeat(20));
    }
}
