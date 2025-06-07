use scraper::{Html, Selector};
use url::Url;

pub fn extract_links(body: &str, base_url: &str) -> Vec<String> {
    let document = Html::parse_document(body);
    let selector = Selector::parse("a").unwrap();

    // Use `expect` here only if base_url is guaranteed valid; otherwise, return Result<Vec<String>, _>
    let base = match Url::parse(base_url) {
        Ok(url) => url,
        Err(_) => return vec![], // fallback: return no links if base_url is invalid
    };

    let base_origin = base.origin();

    document
        .select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .filter_map(|href| base.join(href).ok())
        .filter(|url| url.origin() == base_origin)
        .map(|url| url.into()) // uses Into<String>, avoids deprecation warning
        .collect()
}

pub fn extract_content(body: &str) -> Option<(String, String)> {
    let document = Html::parse_document(body);
    let title = Selector::parse("title")
        .ok()
        .and_then(|sel| document.select(&sel).next())
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default();

    const CONTENT_SELECTORS: &[&str] = &[
        "article", "main", "div.content", "div.post", "div.article", "div.entry-content",
    ];

    for selector in CONTENT_SELECTORS {
        if let Some(content) = extract_text_from_selector(&document, selector) {
            return Some((title.clone(), content));
        }
    }

    let p_selector = Selector::parse("p").unwrap();
    if let Some(content) = find_best_fallback(&document, &p_selector) {
        return Some((title, content));
    }

    None
}

fn extract_text_from_selector(document: &Html, selector_str: &str) -> Option<String> {
    let selector = Selector::parse(selector_str).ok()?;
    let el = document.select(&selector).next()?;
    let text = el
        .text()
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

fn find_best_fallback(document: &Html, p_selector: &Selector) -> Option<String> {
    let mut best: Option<(usize, String)> = None;
    for tag in ["div", "section"] {
        if let Ok(selector) = Selector::parse(tag) {
            for el in document.select(&selector) {
                let paragraphs: Vec<String> = el
                    .select(p_selector)
                    .map(|p| p.text().collect::<String>().trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect();

                let length = paragraphs.iter().map(|p| p.len()).sum::<usize>();
                if length > best.as_ref().map(|(l, _)| *l).unwrap_or(0) {
                    best = Some((length, paragraphs.join(" ")));
                }
            }
        }
    }

    best.map(|(_, content)| content).filter(|c| !c.is_empty())
}