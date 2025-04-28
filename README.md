# html-meta-scraper

Scrape and extract metadata like title, description, images, and favicon from HTML documents.

## Features

- Extract `<title>`, OGP metadata (`og:title`, `og:description`, `og:image`)
- Extract Twitter Card metadata (`twitter:title`, `twitter:description`, `twitter:image`)
- Extract favicon (`<link rel="icon" href="...">`)
- Prioritized fallback (e.g., `og:title` → `twitter:title` → `<title>`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
html-meta-scraper = "0.1.0"
```

## Example

```rust
use html_meta_scraper::MetaScraper;

let html = r#"
    <html>
        <head>
            <meta property="og:title" content="Example Title" />
            <meta name="twitter:description" content="Example Description" />
            <link rel="icon" href="/favicon.ico" />
        </head>
    </html>
"#;

let scraper = MetaScraper::new(html);

assert_eq!(scraper.title(), Some("Example Title".to_string()));
assert_eq!(scraper.description(), Some("Example Description".to_string()));
assert_eq!(scraper.favicon(), Some("/favicon.ico".to_string()));
```

## API Overview

| Method              | Description                                                                           |
| :------------------ | :------------------------------------------------------------------------------------ |
| `title()`           | Retrieves page title (`og:title` → `twitter:title` → `<title>`)                       |
| `description()`     | Retrieves page description (`og:description` → `twitter:description` → `description`) |
| `image()`           | Retrieves page image URL (`og:image` → `twitter:image`)                               |
| `favicon()`         | Retrieves favicon URL (`<link rel="icon">`)                                           |
| `extract_*` methods | Low-level methods to extract specific metadata                                        |
