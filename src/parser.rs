// src/parser.rs
use scraper::{Html, Selector};
use url::Url;
use rayon::prelude::*;

/// Parses the HTML content and extracts the title and meta description.
pub fn get_meta_and_title(html: &str) -> (Option<String>, Option<String>) {
    let document = Html::parse_document(html);

    // Create selectors
    let title_selector = Selector::parse("title").unwrap();
    let meta_selector = Selector::parse(r#"meta[name=\"description\"]"#).unwrap();

    // Extract title
    let title = document
        .select(&title_selector)
        .next()
        .map(|elem| elem.text().collect::<Vec<_>>().concat());

    // Extract meta description
    let description = document
        .select(&meta_selector)
        .next()
        .and_then(|elem| elem.value().attr("content").map(|s| s.to_string()));

    (title, description)
}

pub fn extract_links(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();

    // Collect raw hrefs into a Vec (in single-threaded fashion)
    let hrefs: Vec<&str> = document.select(&selector)
        .filter_map(|element| element.value().attr("href"))
        .collect();

    // Use rayon to iterate over the collected hrefs in parallel and filter them
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


/// Sanitizes and validates a URL string.
pub fn sanitize_link(link: &str) -> Option<String> {
    match Url::parse(link) {
        Ok(url) => match url.scheme() {
            "http" | "https" => Some(url.to_string()),
            _ => None,
        },
        Err(_) => None,
    }
}

/// Finds an HTML element by its `id` and returns its text content.
///
/// # Arguments
///
/// * `html` - A string slice containing the HTML content.
/// * `id` - A string slice that holds the `id` of the desired element.
///
/// # Returns
///
/// * `Option<String>` - Returns `Some(text)` if the element is found, otherwise `None`.
pub fn find_element_by_id(html: &str, id: &str) -> Option<String> {
    // Parse the HTML document
    let document = Html::parse_document(html);

    // Create a CSS selector for the id
    let selector = Selector::parse(&format!("#{}", id)).ok()?;

    // Select the first element that matches the selector
    let element = document.select(&selector).next()?;

    // Return the HTML content of the element
    Some(element.html())
}

/// Retrieves the HTML content of all elements with the specified class from the given HTML.
///
/// # Arguments
///
/// * `html` - A string slice containing the HTML content.
/// * `class` - The class name to search for.
///
/// # Returns
///
/// A vector of strings, each representing the HTML content of an element with the specified class.
/// If the selector is invalid, an empty vector is returned.
pub fn get_elements_by_cls(html: &str, class: &str) -> Vec<String> {
    // Parse the HTML document once
    let document = Html::parse_document(html);

    // Efficiently build the selector string without using `format!`
    // This minimizes allocations by pre-allocating the exact required capacity
    let selector_str = {
        let mut s = String::with_capacity(1 + class.len());
        s.push('.');
        s.push_str(class);
        s
    };

    // Parse the CSS selector, return empty vector if it's invalid
    let selector = match Selector::parse(&selector_str) {
        Ok(sel) => sel,
        Err(e) => {
            eprintln!("Error parsing selector '{}': {}", selector_str, e);
            return Vec::new();
        }
    };

    // Collect the HTML content of all matching elements into a Vec<String>
    let html_contents: Vec<String> = document.select(&selector)
        .map(|element| element.html())
        .collect();

    // If we will be having additional processing to do on each HTML content, you can leverage Rayon here.
    // For demonstration, we'll simply clone the strings in parallel.
    // Replace the following `.map` with your actual processing logic as needed.
    html_contents.par_iter()
        .map(|html_content| html_content.clone())
        .collect()
}


pub fn get_elements_by_tag(html: &str, tag_name: &str) -> Vec<String> {
    // Parse the HTML document
    let document = Html::parse_document(html);

    // Create a CSS selector for the tag name
    let selector_str = tag_name;
    let selector = match Selector::parse(selector_str) {
        Ok(sel) => sel,
        Err(e) => {
            eprintln!("Error parsing selector '{}': {}", selector_str, e);
            return Vec::new();
        }
    };

    // Collect the HTML content of all matching elements into a Vec<String>
    let html_contents: Vec<String> = document.select(&selector)
        .map(|element| element.html())
        .collect();

    // Use rayon to process the elements in parallel
    html_contents.par_iter()
        .map(|html_content| html_content.clone())
        .collect()
}

/// Extracts and returns all the text content from an element, ignoring any nested tags.
pub fn get_text(html: &str, selector: &str) -> Vec<String> {
    // Parse the HTML document
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector).unwrap();

    // Find all elements matching the selector
    let elements: Vec<_> = document.select(&selector).collect();

    // Extract text content from each element in parallel
    elements.iter()
        .map(|element| {
            element.text().collect::<Vec<_>>().join(" ")
        })
        .collect()
}