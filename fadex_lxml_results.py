from fadex import get_elements_by_tag

def test_parse_html():
    # Sample HTML content
    html_content = """
    <html>
        <head>
            <title>Test Page</title>
        </head>
        <body>
            <h1>Header</h1>
            <p class="intro">This is a paragraph.</p>
            <a href="https://example.com">Link</a>
        </body>
    </html>
    """
    
    # Call the parse_html function
    results = get_elements_by_tag(html_content, "p")
    
    print(results)

# Run the test function
test_parse_html()