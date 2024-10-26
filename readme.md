## :dart: About ##
**Fadex** is a powerful Python module that provides robust web scraping functionalities, including fetching web pages, extracting metadata, and parsing HTML content. Built with a Rust backend using PyO3, it is optimized for performance and ease of use in web scraping tasks.

## :sparkles: Features ##

:heavy_check_mark: Fetch web pages asynchronously;\
:heavy_check_mark: Extract metadata including title and description;\
:heavy_check_mark: Sanitize and extract all href links from HTML;\
:heavy_check_mark: Fetch elements by ID and class efficiently;

## Installing

Use the following command in your terminal to install the module.
```bash
$ pip install fadex
```

## :rocket: Technologies ##

The following tools were used in this project:

- [Python](https://python.org)
- [Rust](https://www.rust-lang.org/)
- [PyO3](https://pyo3.rs/v0.15.0/)

## :white_check_mark: Requirements ##

Before starting :checkered_flag:, ensure you have [Python](https://python.org) installed.


## :test_tube: How To Use ##

```python
import asyncio
from fadex import fetch_page_py

async def fetch_page(url):
    try:
        content = await fetch_page_py(url)
        print("Page content fetched successfully:")
        print(content)
    except Exception as e:
        print(f"Failed to fetch page: {e}")

# Example usage
url = "http://example.com"
asyncio.run(fetch_page(url))
```

## :hammer_and_wrench: Functionalities

- Fetch metadata (title and description):
  ```python
  title, description = get_meta_and_title(html_content)
  ```

- Extract links from HTML:
  ```python
  links = extract_links(html_content)
  ```

- Fetch elements by ID:
  ```python
  elements = find_element_by_id(html_content, "your-id")
  ```

- Fetch elements by class:
  ```python
  elements = get_elements_by_cls(html_content, "your-class")
  ```

## :memo: License ##

This project is licensed under the MIT License. For more details, see the [LICENSE](LICENSE.md) file.

Made with :heart: by <a href="https://github.com/fahad-programmer" target="_blank">Fahad Malik</a>

&#xa0;

<a href="#top">Back to top</a>
```

# Fadex: A Powerful Web Scraper With Unmatched Performance

## Overview

**Fadex** is a Python module that provides powerful web scraping functionalities, including fetching web pages, extracting metadata, and parsing HTML content. Built with a Rust backend using PyO3, it aims to provide high performance and ease of use for web scraping tasks.

## Installation

You can easily install Fadex using pip:

```bash
pip install fadex
```

## Usage

### Basic Example

To fetch the content of a web page asynchronously, you can use the `fetch_page` function:

```python
import asyncio
from fadex import fetch_page

async def fetch_page_py(url):
    try:
        content = await fetch_page(url)
        print("Page content fetched successfully:")
        print(content)
    except Exception as e:
        print(f"Failed to fetch page: {e}")

# Example usage
url = "http://gigmasters.it"
asyncio.run(fetch_page_py(url))
```

## API Reference

### Functions

#### `get_meta_and_title(html: str) -> Tuple[Optional[str], Optional[str]]`

Parses the HTML content and extracts the title and meta description.

- **Parameters:**
  - `html`: A string containing the HTML content.
- **Returns:**
  - A tuple containing:
    - `title`: An optional string representing the page title.
    - `description`: An optional string representing the meta description.

#### `extract_links(html: str) -> List[str]`

Extracts and sanitizes all href links from the HTML content.

- **Parameters:**
  - `html`: A string containing the HTML content.
- **Returns:**
  - A list of sanitized URLs extracted from the HTML.

#### `fetch_page(url: str) -> Awaitable[str]`

Asynchronously fetches the content of a web page.

- **Parameters:**
  - `url`: A string containing the URL of the page to fetch.
- **Returns:**
  - A string containing the content of the fetched page.

#### `find_element_by_id(html: str, id: str) -> List[str]`

Fetches the elements that have the specified id in the html content.

- **Parameters:**
  - `html`: A string containing the html content.
  - `id` : The id of which u want elements for.
- **Returns:**
  - A list of elements usually one that have the same id as given in param.

#### `get_elements_by_tag(html: str, tag: str) -> List[str]`

Fetches all the elements that have the specified tag in the html content.

- **Parameters:**
  - `html`: A string containing the html content.
  - `tag` : The tag of which u want elements for like <p> or <span>.
- **Returns:**
  - A list of elements usually one that have the same id as given in param.

#### `get_elements_by_cls(html: str, class: str) -> List[str]`

Fetches the elements that have the specified class in the html content.

- **Parameters:**
  - `html`: A string containing the html content.
  - `class` : The class of which you want elements for.
- **Returns:**
  - A list of elements that have the same class as given in param.


## Performance Comparison

We conducted a performance comparison between **Fadex**, **BeautifulSoup**, and **lxml** by extracting the metadata (title and description) and extracting all links from 10 popular websites. The results are as follows:

### Metadata Extraction Performance

```
Fadex Metadata Extraction Average Time: 0.56 seconds (Successful Extracts: 100)
BeautifulSoup Metadata Extraction Average Time: 0.78 seconds (Successful Extracts: 100)
lxml Metadata Extraction Average Time: 0.69 seconds (Successful Extracts: 100)

Performance Comparison for Metadata Extraction:
Fadex Time: 0.56 seconds
BeautifulSoup Time: 0.78 seconds
lxml Time: 0.69 seconds

Winner for Metadata Extraction: Fadex
```

### Link Extraction Performance

```
Fadex Link Extraction Average Time: 0.62 seconds (Successful Extracts: 100)
BeautifulSoup Link Extraction Average Time: 0.81 seconds (Successful Extracts: 100)
lxml Link Extraction Average Time: 0.65 seconds (Successful Extracts: 100)

Performance Comparison for Link Extraction:
Fadex Time: 0.62 seconds
BeautifulSoup Time: 0.81 seconds
lxml Time: 0.65 seconds

Winner for Link Extraction: Fadex
```

These results show that **Fadex** outperforms both **BeautifulSoup** and **lxml** in terms of average response time for extracting metadata and links. However, the performance of each library can also depend on factors such as the complexity of the HTML content and the internet connection stability.

## Example Code for Performance Comparison

Below is the code used for the performance comparison:

```python
import asyncio
import time
from fadex import fetch_page_py, get_meta_and_title_py, extract_links_py
from bs4 import BeautifulSoup
from lxml import html as lxml_html
from urllib.parse import urljoin, urlparse

# Function to extract metadata using Fadex
def extract_metadata_with_fadex(html_content):
    try:
        title, description = get_meta_and_title_py(html_content)
        return True, title, description
    except Exception as e:
        return False, None, None

# Function to extract metadata using BeautifulSoup
def extract_metadata_with_beautifulsoup(html_content):
    try:
        soup = BeautifulSoup(html_content, 'html.parser')
        title = soup.title.string if soup.title else None
        description = None
        meta_tag = soup.find('meta', attrs={'name': 'description'})
        if meta_tag:
            description = meta_tag.get('content')
        return True, title, description
    except Exception as e:
        return False, None, None

# Function to extract metadata using lxml
def extract_metadata_with_lxml(html_content):
    try:
        tree = lxml_html.fromstring(html_content)
        title = tree.find('.//title').text if tree.find('.//title') is not None else None
        description = None
        meta = tree.xpath('//meta[@name="description"]')
        if meta and 'content' in meta[0].attrib:
            description = meta[0].attrib['content']
        return True, title, description
    except Exception as e:
        return False, None, None

# Function to extract links using Fadex
def extract_links_with_fadex(html_content, base_url):
    try:
        links = extract_links_py(html_content, base_url)
        return True, links
    except Exception as e:
        return False, []

# Function to extract links using BeautifulSoup
def extract_links_with_beautifulsoup(html_content, base_url):
    try:
        soup = BeautifulSoup(html_content, 'html.parser')
        links = [urljoin(base_url, a['href']) for a in soup.find_all('a', href=True)]
        return True, [link for link in links if urlparse(link).scheme in ["http", "https"]]
    except Exception as e:
        return False, []

# Function to extract links using lxml
def extract_links_with_lxml(html_content, base_url):
    try:
        tree = lxml_html.fromstring(html_content)
        links = [urljoin(base_url, link) for link in tree.xpath('//a/@href')]
        return True, [link for link in links if urlparse(link).scheme in ["http", "https"]]
    except Exception as e:
        return False, []

# Function to measure average performance for each library
def measure_metadata_performance(html_contents, extract_func, iterations=5):
    total_time = 0
    successful_extracts = 0
    for _ in range(iterations):
        for html_content in html_contents:
            start_time = time.time()
            success, title, description = extract_func(html_content)
            total_time += time.time() - start_time
            if success:
                successful_extracts += 1
    average_time = total_time / (len(html_contents) * iterations)
    return average_time, successful_extracts

# Function to measure link extraction performance for each library
def measure_link_extraction_performance(html_contents, base_urls, extract_func, iterations=5):
    total_time = 0
    successful_extracts = 0
    for _ in range(iterations):
        for html_content, base_url in zip(html_contents, base_urls):
            start_time = time.time()
            success, links = extract_func(html_content, base_url)
            total_time += time.time() - start_time
            if success:
                successful_extracts += 1
    average_time = total_time / (len(html_contents) * iterations)
    return average_time, successful_extracts

# Main function to run the tests
async def main():
    # List of popular URLs for testing
    urls = [
        "https://www.google.com",
        "https://www.wikipedia.org",
        "https://www.github.com",
        "https://www.reddit.com",
        "https://www.stackoverflow.com",
        "https://www.nytimes.com",
        "https://www.bbc.com",
        "https://www.amazon.com",
        "https://www.apple.com",
        "https://www.microsoft.com"
    ]

    # Fetch page content using Fadex
    html_contents = []
    for url in urls:
        try:
            content = await fetch_page_py(url)
            html_contents.append(content)
        except Exception as e:
            print(f"Failed to fetch page from {url}: {e}")

    # Define number of iterations for performance measurement
    iterations = 10

    # Measure performance for Fadex (metadata extraction)
    fadex_meta_average_time, fadex_meta_success = measure_metadata_performance(
        html_contents, extract_metadata_with_fadex, iterations
    )

    # Measure performance for BeautifulSoup (metadata extraction)
    bs_meta_average_time, bs_meta_success = measure_metadata_performance(
        html_contents, extract_metadata_with_beautifulsoup, iterations
    )

    # Measure performance for lxml (metadata extraction)
    lxml_meta_average_time, lxml_meta_success = measure_metadata_performance(
       