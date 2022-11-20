use clap::Parser;
use scraper::{Html, Selector};
use std::{collections::HashSet, error::Error};

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

    let mut found_urls = HashSet::new();

    for element in document.select(&selector) {
        assert_eq!("a", element.value().name());

        for (_, url) in element.value().attrs().filter(|&(name, _)| name == "href") {
            found_urls.insert(url.to_string());
        }
    }

    println!("{:#?}", found_urls);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _ = process_url(&args.url);

    Ok(())
}
