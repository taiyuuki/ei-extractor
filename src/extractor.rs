use std::collections::HashMap;
use std::io::BufReader;
use std::path::Path;
use std::{fs, io::Write};

use epub::doc::EpubDoc;
use scraper::{Html, Selector};

pub struct EpubExtractor {
    epub: EpubDoc<BufReader<fs::File>>,
    spine: Vec<String>,
    image_paths: HashMap<String, String>,
    output_dir: String,
    ignore_size: usize,
}

impl EpubExtractor {
    pub fn new(full_path: &str) -> Result<EpubExtractor, Box<dyn std::error::Error>> {
        let epub = EpubDoc::new(full_path)?;
        let path = Path::new(full_path);
        let mut spine = Vec::with_capacity(epub.spine.len());
        let title = match epub.mdata("title") {
            Some(t) => t,
            None => path.file_stem().unwrap().to_str().unwrap().to_string(),
        };
        let dir = path.parent().unwrap();
        let output_dir = format!(
            "{}{}{}{}{}",
            dir.to_str().unwrap(),
            std::path::MAIN_SEPARATOR,
            "output",
            std::path::MAIN_SEPARATOR,
            title
        );
        for item in epub.spine.iter() {
            spine.push(item.clone());
        }

        let mut image_paths = HashMap::new();
        for (_id, (path, mime)) in epub.resources.iter() {
            if mime.starts_with("image") {
                if let Some(image_file_name) = path.file_name() {
                    let image_file_name = image_file_name.to_str();
                    if let Some(file_name) = image_file_name {
                        image_paths.insert(
                            String::from(file_name),
                            String::from(path.to_str().unwrap()),
                        );
                    }
                }
            }
        }

        Ok(EpubExtractor {
            epub,
            spine,
            image_paths,
            output_dir,
            ignore_size: 0,
        })
    }

    pub fn set_ignore_size(&mut self, size: usize) {
        self.ignore_size = size;
    }

    pub fn extract<F>(&mut self, on_progress: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(usize) -> (),
    {
        let mut count = 0;
        let len = self.spine.len();
        for s in self.spine.iter() {
            let (html, _) = self.epub.get_resource_str(s).unwrap();
            let document = Html::parse_document(&html);
            let img_el = Selector::parse("img")?;
            for el in document.select(&img_el) {
                match el.value().attr("src") {
                    Some(src) => {
                        if !Path::new(&self.output_dir).exists() {
                            fs::create_dir_all(&self.output_dir)?;
                        }
                        let file_name = src.split("/").last().unwrap();
                        let ext = src.split(".").last().unwrap();
                        let path = self.image_paths.get(file_name).unwrap();
                        let resource = self.epub.get_resource_by_path(path);
                        match resource {
                            Some(data) => {
                                // Ignore images smaller than the specified size.
                                if data.len() < self.ignore_size * 1024 {
                                    continue;
                                }
                                let output_path = format!(
                                    "{}{}{}.{}",
                                    &self.output_dir,
                                    std::path::MAIN_SEPARATOR,
                                    count,
                                    ext
                                );
                                let mut f = fs::File::create(&output_path)?;
                                f.write_all(&data)?;
                            }
                            None => continue,
                        }
                    }
                    None => continue,
                }
            }
            count += 1;
            on_progress(count * 100 / len);
        }
        Ok(())
    }
}
