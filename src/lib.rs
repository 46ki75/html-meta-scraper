pub struct MetaScraper {
    document: scraper::Html,
}

impl MetaScraper {
    pub fn new(html: &str) -> Self {
        let document = scraper::Html::parse_document(html);

        MetaScraper { document }
    }

    /// Expected Output: `"Page Title"`
    /// ```html
    /// <title>Page Title</title>
    /// ```
    pub fn extract_title(&self) -> Option<String> {
        self.document
            .select(&scraper::Selector::parse("title").unwrap())
            .next()
            .map(|element| element.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty())
    }

    /// Expected Output: `"Page Title"`
    /// ```html
    /// <meta property="og:title" content="Page Title" />
    /// ```
    pub fn extract_og_title(&self) -> Option<String> {
        let og_title_selector =
            scraper::Selector::parse("meta[property='og:title'], meta[name='og:title']").unwrap();

        let og_title = self
            .document
            .select(&og_title_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string());

        og_title
    }

    /// Expected Output: `"Page Title"`
    /// ```html
    /// <meta name="twitter:title" content="Page Title" />
    /// ```
    pub fn extract_twitter_title(&self) -> Option<String> {
        let twitter_title_selector =
            scraper::Selector::parse("meta[name='twitter:title'], meta[property='twitter:title']")
                .unwrap();

        let twitter_title = self
            .document
            .select(&twitter_title_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string());

        twitter_title
    }

    /// Retrieves the page title.
    ///
    /// Priority order:
    /// 1. `<meta property="og:title">`
    /// 2. `<meta name="twitter:title">`
    /// 3. `<title>`
    ///
    /// Returns the first one found.
    pub fn title(&self) -> Option<String> {
        self.extract_og_title()
            .or_else(|| self.extract_twitter_title())
            .or_else(|| self.extract_title())
    }

    /// Expected Output: `"My Description"`
    /// ```html
    /// <meta name="description" content="My Description" />
    /// ```
    pub fn extract_description(&self) -> Option<String> {
        let description_selector = scraper::Selector::parse("meta[name='description']").unwrap();

        let description = self
            .document
            .select(&description_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string());

        description
    }

    /// Expected Output: `"My Description"`
    /// ```html
    /// <meta property="og:description" content="My Description" />
    /// ```
    pub fn extract_og_description(&self) -> Option<String> {
        let og_description_selector = scraper::Selector::parse(
            "meta[property='og:description'], meta[name='og:description']",
        )
        .unwrap();

        let og_description = self
            .document
            .select(&og_description_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string());

        og_description
    }

    /// Expected Output: `"My Description"`
    /// ```html
    /// <meta name="twitter:description" content="My Description" />
    /// ```
    pub fn extract_twitter_description(&self) -> Option<String> {
        let twitter_description_selector = scraper::Selector::parse(
            "meta[name='twitter:description'], meta[property='twitter:description']",
        )
        .unwrap();

        let twitter_description = self
            .document
            .select(&twitter_description_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string());

        twitter_description
    }

    /// Retrieves the page description.
    ///
    /// Priority order:
    /// 1. `<meta property="og:description">`
    /// 2. `<meta name="twitter:description">`
    /// 3. `<meta name="description">`
    ///
    /// Returns the first one found.
    pub fn description(&self) -> Option<String> {
        self.extract_og_description()
            .or_else(|| self.extract_twitter_description())
            .or_else(|| self.extract_description())
    }

    /// Expected Output: `"/favicon.ico"`
    /// ```html
    /// <link rel="icon" href="/favicon.ico" />
    /// ```
    pub fn favicon(&self) -> Option<String> {
        let favicon_selector = scraper::Selector::parse("link[rel~='icon']").unwrap();

        let favicon = self
            .document
            .select(&favicon_selector)
            .next()
            .and_then(|element| element.value().attr("href").map(|href| href.to_string()));

        favicon
    }

    /// Expected Output: `"https://example.com/image.jpg"`
    /// ```html
    /// <meta property="og:image" content="https://example.com/image.jpg" />
    /// ```
    pub fn extract_og_image(&self) -> Option<String> {
        let og_image_selector =
            scraper::Selector::parse("meta[property='og:image'], meta[name='og:image']").unwrap();

        let og_image = self
            .document
            .select(&og_image_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string());

        og_image
    }

    /// Expected Output: `["https://example.com/image.jpg", "https://example.com/image.png"]`
    /// ```html
    /// <meta property="og:image" content="https://example.com/image.jpg" />
    /// <meta property="og:image" content="https://example.com/image.png" />
    /// ```
    pub fn extract_og_images(&self) -> Vec<String> {
        let og_image_selector =
            scraper::Selector::parse("meta[property='og:image'], meta[name='og:image']").unwrap();

        let og_images = self
            .document
            .select(&og_image_selector)
            .filter_map(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string())
            .collect::<Vec<String>>();

        og_images
    }

    /// Expected Output: `"https://example.com/image.jpg"`
    /// ```html
    /// <meta name="twitter:image" content="https://example.com/image.jpg" />
    /// <meta name="twitter:image:alt" content="Image description" />
    /// ```
    pub fn extract_twitter_image(&self) -> Option<String> {
        let twitter_image_selector =
            scraper::Selector::parse("meta[name='twitter:image'], meta[property='twitter:image']")
                .unwrap();

        let twitter_image = self
            .document
            .select(&twitter_image_selector)
            .next()
            .and_then(|element| element.value().attr("content"))
            .filter(|content| !content.is_empty())
            .map(|content| content.to_string());

        twitter_image
    }

    /// Retrieves the page image URL.
    ///
    /// Priority order:
    /// 1. `<meta property="og:image">`
    /// 2. `<meta name="twitter:image">`
    ///
    /// Returns the first one found.
    pub fn image(&self) -> Option<String> {
        self.extract_og_image()
            .or_else(|| self.extract_twitter_image())
    }

    /// Expected Output: `"en"`
    /// ```html
    /// <html lang="en">
    /// ...
    /// </html>
    /// ```
    pub fn lang(&self) -> Option<String> {
        let html_selector = scraper::Selector::parse("html").unwrap();

        let lang = self
            .document
            .select(&html_selector)
            .next()
            .and_then(|element| {
                element
                    .value()
                    .attr("lang")
                    .map(|content| content.to_string())
            });

        lang
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_title() {
        let scraper = MetaScraper::new(r#"<title>Page Title</title>"#);

        let title = scraper.extract_title();

        assert_eq!(title, Some("Page Title".to_string()));
    }

    #[test]
    fn extract_og_title() {
        let scraper = MetaScraper::new(r#"<meta property="og:title" content="Page Title" />"#);

        let og_title = scraper.extract_og_title();

        assert_eq!(og_title, Some("Page Title".to_string()));
    }

    #[test]
    fn extract_twitter_title() {
        let scraper = MetaScraper::new(r#"<meta name="twitter:title" content="Page Title" />"#);

        let og_title = scraper.extract_twitter_title();

        assert_eq!(og_title, Some("Page Title".to_string()));
    }

    #[test]
    fn extract_description() {
        let scraper = MetaScraper::new(r#"<meta name="description" content="My Description" />"#);

        let description = scraper.extract_description();

        assert_eq!(description, Some("My Description".to_string()));
    }

    #[test]
    fn extract_og_description() {
        let scraper =
            MetaScraper::new(r#"<meta property="og:description" content="My Description" />"#);

        let og_description = scraper.extract_og_description();

        assert_eq!(og_description, Some("My Description".to_string()));
    }

    #[test]
    fn extract_twitter_description() {
        let scraper =
            MetaScraper::new(r#"<meta name="twitter:description" content="My Description" />"#);

        let twitter_description = scraper.extract_twitter_description();

        assert_eq!(twitter_description, Some("My Description".to_string()));
    }

    #[test]
    fn favicon() {
        let scraper = MetaScraper::new(r#"<link rel="icon" href="/favicon.ico" />"#);

        let favicon = scraper.favicon();

        assert_eq!(favicon, Some("/favicon.ico".to_string()));
    }

    #[test]
    fn extract_og_image() {
        let scraper = MetaScraper::new(
            r#"<meta property="og:image" content="https://example.com/image.jpg" />"#,
        );

        let og_image = scraper.extract_og_image();

        assert_eq!(og_image, Some("https://example.com/image.jpg".to_string()));
    }

    #[test]
    fn extract_og_images() {
        let scraper = MetaScraper::new(
            r#"
            <meta property="og:image" content="https://example.com/image.jpg" />
            <meta property="og:image" content="https://example.com/image.png" />"#,
        );

        let og_image = scraper.extract_og_images();

        assert_eq!(
            og_image,
            vec![
                "https://example.com/image.jpg".to_string(),
                "https://example.com/image.png".to_string()
            ]
        );
    }

    #[test]
    fn extract_twitter_image() {
        let scraper = MetaScraper::new(
            r#"<meta name="twitter:image" content="https://example.com/image.jpg" />"#,
        );

        let twitter_image = scraper.extract_twitter_image();

        assert_eq!(
            twitter_image,
            Some("https://example.com/image.jpg".to_string())
        );
    }

    #[test]
    fn lang() {
        let scraper = MetaScraper::new(
            r#"
            <html lang="en">
            ...
            </html>
        "#,
        );

        let lang = scraper.lang();

        assert_eq!(lang, Some("en".to_owned()));
    }

    // ---------------------------------------------------------------------
    // Bug-reproduction tests. These encode the desired behavior and are
    // expected to FAIL against the current implementation.
    // ---------------------------------------------------------------------

    #[test]
    fn empty_title_tag_returns_none() {
        let scraper = MetaScraper::new(r#"<title></title>"#);
        assert_eq!(scraper.extract_title(), None);
    }

    #[test]
    fn title_whitespace_is_trimmed() {
        let scraper = MetaScraper::new("<title>\n  Page Title\n</title>");
        assert_eq!(scraper.extract_title(), Some("Page Title".to_string()));
    }

    #[test]
    fn empty_og_title_content_returns_none() {
        let scraper = MetaScraper::new(r#"<meta property="og:title" content="" />"#);
        assert_eq!(scraper.extract_og_title(), None);
    }

    #[test]
    fn empty_description_content_returns_none() {
        let scraper = MetaScraper::new(r#"<meta name="description" content="" />"#);
        assert_eq!(scraper.extract_description(), None);
    }

    #[test]
    fn og_title_with_name_attribute_is_recognized() {
        // Some CMSes emit `name="og:..."` instead of `property="og:..."`.
        let scraper = MetaScraper::new(r#"<meta name="og:title" content="Page Title" />"#);
        assert_eq!(scraper.extract_og_title(), Some("Page Title".to_string()));
    }

    #[test]
    fn og_description_with_name_attribute_is_recognized() {
        let scraper =
            MetaScraper::new(r#"<meta name="og:description" content="My Description" />"#);
        assert_eq!(
            scraper.extract_og_description(),
            Some("My Description".to_string())
        );
    }

    #[test]
    fn og_image_with_name_attribute_is_recognized() {
        let scraper =
            MetaScraper::new(r#"<meta name="og:image" content="https://example.com/i.jpg" />"#);
        assert_eq!(
            scraper.extract_og_image(),
            Some("https://example.com/i.jpg".to_string())
        );
    }

    #[test]
    fn twitter_title_with_property_attribute_is_recognized() {
        // Mirror case: Twitter tags sometimes appear as `property=`.
        let scraper = MetaScraper::new(r#"<meta property="twitter:title" content="Page Title" />"#);
        assert_eq!(
            scraper.extract_twitter_title(),
            Some("Page Title".to_string())
        );
    }

    #[test]
    fn twitter_description_with_property_attribute_is_recognized() {
        let scraper =
            MetaScraper::new(r#"<meta property="twitter:description" content="My Description" />"#);
        assert_eq!(
            scraper.extract_twitter_description(),
            Some("My Description".to_string())
        );
    }

    #[test]
    fn twitter_image_with_property_attribute_is_recognized() {
        let scraper = MetaScraper::new(
            r#"<meta property="twitter:image" content="https://example.com/i.jpg" />"#,
        );
        assert_eq!(
            scraper.extract_twitter_image(),
            Some("https://example.com/i.jpg".to_string())
        );
    }

    #[test]
    fn favicon_matches_shortcut_icon() {
        let scraper = MetaScraper::new(r#"<link rel="shortcut icon" href="/favicon.ico" />"#);
        assert_eq!(scraper.favicon(), Some("/favicon.ico".to_string()));
    }

    #[test]
    fn favicon_matches_multi_token_rel() {
        let scraper = MetaScraper::new(r#"<link rel="icon shortcut" href="/favicon.ico" />"#);
        assert_eq!(scraper.favicon(), Some("/favicon.ico".to_string()));
    }

    #[test]
    fn title_fallback_prefers_og_over_twitter_over_native() {
        let scraper = MetaScraper::new(
            r#"
            <title>Native Title</title>
            <meta property="og:title" content="OG Title" />
            <meta name="twitter:title" content="Twitter Title" />
            "#,
        );
        assert_eq!(scraper.title(), Some("OG Title".to_string()));

        let scraper = MetaScraper::new(
            r#"
            <title>Native Title</title>
            <meta name="twitter:title" content="Twitter Title" />
            "#,
        );
        assert_eq!(scraper.title(), Some("Twitter Title".to_string()));

        let scraper = MetaScraper::new(r#"<title>Native Title</title>"#);
        assert_eq!(scraper.title(), Some("Native Title".to_string()));
    }

    #[test]
    fn title_returns_none_when_no_source_present() {
        let scraper = MetaScraper::new(r#"<html><head></head><body></body></html>"#);
        assert_eq!(scraper.title(), None);
    }
}
