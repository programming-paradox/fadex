// src/parser.rs
use scraper::{Html, Selector};
use url::Url;
use rayon::prelude::*;

pub fn get_meta_and_title(html: &str) -> (Option<String>, Option<String>) {
    let document = Html::parse_document(html);

    let title_selector = Selector::parse("title").unwrap();
    let meta_selector = Selector::parse(r#"meta[name=\"description\"]"#).unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .map(|elem| elem.text().collect::<Vec<_>>().concat());

    let description = document
        .select(&meta_selector)
        .next()
        .and_then(|elem| elem.value().attr("content").map(|s| s.to_string()));

    (title, description)
}

pub fn extract_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();

    let hrefs: Vec<&str> = document.select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .collect();

    hrefs.par_iter()
        .filter_map(|&href| {
            if href.starts_with("http://") || href.starts_with("https://") {
                Some(href.to_string())
            } else {
                None
            }
        })
        .collect()
}


pub fn sanitize_link(link: &str) -> Option<String> {
    match Url::parse(link) {
        Ok(url) => match url.scheme() {
            "http" | "https" => Some(url.to_string()),
            _ => None,
        },
        Err(_) => None,
    }
}


pub fn find_element_by_id(html: &str, id: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(&format!("#{}", id)).ok()?;
    let element = document.select(&selector).next()?;

    Some(element.html())
}


pub fn get_elements_by_cls(html: &str, class: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    let selector_str = {
        let mut s = String::with_capacity(1 + class.len());
        s.push('.');
        s.push_str(class);
        s
    };

    let selector = match Selector::parse(&selector_str) {
        Ok(sel) => sel,
        Err(e) => {
            eprintln!("Error parsing selector '{}': {}", selector_str, e);
            return Vec::new();
        }
    };

    let html_contents: Vec<String> = document.select(&selector)
        .map(|element| element.html())
        .collect();

    html_contents.par_iter()
        .map(|html_content| html_content.clone())
        .collect()
}


pub fn get_elements_by_tag(html: &str, tag_name: &str) -> Vec<String> {
    let document = Html::parse_document(html);

    let selector_str = tag_name;
    let selector = match Selector::parse(selector_str) {
        Ok(sel) => sel,
        Err(e) => {
            eprintln!("Error parsing selector '{}': {}", selector_str, e);
            return Vec::new();
        }
    };

    let html_contents: Vec<String> = document.select(&selector)
        .map(|element| element.html())
        .collect();

    html_contents.par_iter()
        .map(|html_content| html_content.clone())
        .collect()
}

pub fn get_text(html: &str, selector: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector).unwrap();

    let elements: Vec<_> = document.select(&selector).collect();
    elements.iter()
        .map(|element| {
            element.text().collect::<Vec<_>>().join(" ")
        })
        .collect()
}