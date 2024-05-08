
use std::{collections::HashSet, borrow::BorrowMut};
use std::env;
use scraper::{Html, Selector};
use reqwest::Url;
use async_recursion::async_recursion;
use log::debug;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    // let set = links_set(url.clone(), HashSet::new()).await?;
    // println!("{:?}", set);
    let mut visited_urls: HashSet<String> = HashSet::new();
    dfs(url, &mut visited_urls).await;
    println!("{visited_urls:?}");
    Ok(())
}

// async fn links_set(url: String, graph: HashSet<(String, String)>) -> Result<HashSet<(String, String)>, Box<dyn std::error::Error>> {
//     let original_url = url.clone();
//     let links = collect_links(&url, ).await?;
//     let set: HashSet<(String, String)> = links.into_iter().map(|link| (original_url.clone(), link)).collect();
//     Ok(set)
// }

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

async fn collect_links(url: &str, content: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let domain = get_domain(url.to_string());
    let page = Html::parse_document(&content);
    let tags_selector = Selector::parse("a")?;
    let links = page
        .select(&tags_selector)
        .filter_map(|e| e.value().attr("href"))
        .filter(|href| {
            // println!("{:?} == {:?} = {}", get_domain(href.to_string()), domain, get_domain(href.to_string()) == domain);
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
