use std::collections::HashMap;
use std::env;
use scraper::{Html, Selector};
use reqwest::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    let links = collect_links(url.to_string()).await?;
    for link in links {
        println!("{link}");
    }
    Ok(())
}


async fn collect_links(url: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url.clone())
        .await?
        .text()
        .await?;
    let domain = get_domain(url.to_string());
    let page = Html::parse_document(&resp);
    let tags_selector = Selector::parse("a")?;
    let links = page
        .select(&tags_selector)
        .filter_map(|e| e.value().attr("href"))
        .filter(|href| get_domain(href.to_string()) == domain)
        .map(String::from)
        .collect();
    Ok(links)
}

fn get_domain(url: String) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(&url) {
        parsed_url.domain().map(|s| s.to_string())
    } else {
        None
    }
    
}
