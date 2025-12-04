use pdf_splitter::{Config};
use lopdf::Document;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn load_pdf<P: AsRef<Path>>(path: P) -> Result<Document, lopdf::Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    Document::load_from(reader)
}

fn main() {
    let config = Config::build();
    match config {
        Ok(_) => println!("Successfully built config"),
        Err(e) => {
            println!("Error with building config: {e}");
            std::process::exit(1);
        }
    }

    match load_pdf(config.unwrap().path) {
        Ok(doc) => {
            let page_count = doc.get_pages().len();
            println!("PDF loaded with {} pages", page_count);
        },
        Err(e) => {
            eprintln!("Error with loading file: {}", e);
        }
    }
}