use pdf_splitter::{Config, build_extracted_pdf, load_pdf};

fn main() {
    let config = Config::build();
    match config {
        Ok(_) => println!("Successfully built config"),
        Err(e) => {
            println!("Error with building config: {e}");
            std::process::exit(1);
        }
    }

    let config = config.unwrap();

    match load_pdf(&config.path) {
        Ok(doc) => {
            let page_count = doc.get_pages().len();
            println!("PDF loaded with {} pages", page_count);

            build_extracted_pdf(&doc, &config);
        },
        Err(e) => {
            eprintln!("Error with loading file: {}", e);
        }
    }

}