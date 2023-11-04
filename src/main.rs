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
        .get_matches();

    if let Some(input) = matches.get_one::<String>("input") {
        // Get input epub path
        let input_epub = cwd.as_path().join(input);
        // Extract images
        let mut extractor = extractor::EpubExtractor::new(input_epub.to_str().unwrap()).unwrap();
        extractor.extract(|s| println!("{}", s)).unwrap();
        println!("Done");
    }
}
