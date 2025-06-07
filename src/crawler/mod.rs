use std::sync::Arc;
use rusqlite::Connection;
use tokio::sync::Semaphore;
use crate::storage::sqlite::{init_db, insert_page};
use crate::crawler::{fetch::fetch_html, parser::{extract_links, extract_content}, visited::VisitedUrls};
pub mod fetch;
pub mod parser;
pub mod visited;

pub async fn crawl(start_url: &str, max_depth: usize, concurrency: usize) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Arc::new(Connection::open("crawl_results.db")?);
    init_db(&conn)?;

    let visited = Arc::new(VisitedUrls::new());
    let semaphore = Arc::new(Semaphore::new(concurrency));

    crawl_recursive(
        start_url.to_string(),
        0,
        max_depth,
        visited.clone(),
        semaphore.clone(),
        conn.clone(),
    ).await;

    Ok(())
}

async fn crawl_recursive(
    url: String,
    depth: usize,
    max_depth: usize,
    visited: Arc<VisitedUrls>,
    semaphore: Arc<Semaphore>,
    conn: Arc<Connection>,
) {
    if depth > max_depth || !visited.mark_visited(&url) {
        return;
    }

    let permit = semaphore.clone().acquire_owned().await.unwrap();
    match fetch_html(&url).await {
        Ok(body) => {
            println!("Crawled: {}", url);

            // ✅ Extract content (title and text) from the body
            let (title, content) = extract_content(&body).unwrap_or_default();

            // ✅ Insert into SQLite database
            if let Err(err) = insert_page(&conn, &url, &title, &content) {
                eprintln!("Failed to insert page into DB ({}): {}", url, err);
            }

            // ✅ Release the semaphore before recursion
            drop(permit);

            // ✅ Extract links and recurse
            let links = extract_links(&body, &url);
            let futures = links.into_iter().map(|link| {
                crawl_recursive(
                    link,
                    depth + 1,
                    max_depth,
                    visited.clone(),
                    semaphore.clone(),
                    conn.clone(),
                )
            });
            futures::future::join_all(futures).await;
        }
        Err(err) => {
            eprintln!("Failed to crawl {}: {}", url, err);
        }
    }
}