use open_crawler::crawler::crawl;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let start_url = args.get(1).expect("Usage: cargo run -- <URL> [depth] [concurrency]");
    let max_depth = args.get(2).and_then(|d| d.parse().ok()).unwrap_or(2);
    let concurrency = args.get(3).and_then(|c| c.parse().ok()).unwrap_or(5);

    if let Err(e) = crawl(start_url, max_depth, concurrency).await {
        eprintln!("Error during crawl: {}", e);
    }
}