use clap::Parser;
use scraper::{Html, Selector};
use std::error::Error;

/// Simple webcrawler

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL to crawl
    #[clap(short, long, value_parser)]
    url: String,
}

fn process_url(url: &str) -> Result<(), Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").expect("Parser error");

    for element in document.select(&selector) {
        assert_eq!("a", element.value().name());

        for attr in element.value().attrs().filter(|&(name, _)| name == "href") {
            println!("{:?}", attr);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _ = process_url(&args.url);

    Ok(())
}
