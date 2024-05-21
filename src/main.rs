
use std::{collections::HashSet, borrow::BorrowMut};
use std::env;
use scraper::{Html, Selector};
use reqwest::Url;
use async_recursion::async_recursion;
use petgraph::Graph;
use clap::Parser;

/// Create a dot graph of a site's links
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// URL of a site to scan
    url: String,

    /// Regex to exclude certain links
    #[arg(short, long)]
    filter: Option<String>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let url = args.url;
    let mut visited_urls: HashSet<String> = HashSet::new();
    dfs(&url, &mut visited_urls).await;
    println!("{visited_urls:?}");
    Ok(())
}

#[async_recursion]
async fn dfs(url: &str, visited: &mut HashSet<String>) {
    if visited.contains(url) {
        return;
    }
    visited.insert(String::from(url));
    let page_content = read_page(&url).await.unwrap();
    let links = collect_links(&url, page_content).await.unwrap();
    for link in links {
        dfs(&link, visited).await;
        println!("\"{url}\" -> \"{link}\";")
    }
}

async fn read_page(url: &str) -> Result<String, reqwest::Error>  {
    reqwest::get(url)
        .await?
        .text()
        .await
}

// TODO: Make this iter
async fn collect_links(url: &str, content: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let domain = get_domain(url.to_string());
    let page = Html::parse_document(&content);
    let tags_selector = Selector::parse("a")?;
    let links = page
        .select(&tags_selector)
        .filter_map(|e| e.value().attr("href"))
        .filter(|href| {
            get_domain(href.to_string()) == domain
        })
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
