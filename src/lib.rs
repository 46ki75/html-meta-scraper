pub struct MetaScraper {
    document: scraper::Html,
}

impl MetaScraper {
    pub fn new(html: &str) -> Self {
        let document = scraper::Html::parse_document(html);

        MetaScraper { document }
    }

    pub fn extract_title(&self) -> Option<String> {
        self.document
            .select(&scraper::Selector::parse("title").unwrap())
            .next()
            .map(|element| element.text().collect::<String>())
    }

    pub fn extract_og_title(&self) -> Option<String> {
        let og_title_selector = scraper::Selector::parse("meta[property='og:title']").unwrap();

        let og_title = self
            .document
            .select(&og_title_selector)
            .next()
            .and_then(|element| {
                element
                    .value()
                    .attr("content")
                    .map(|content| content.to_string())
            });

        og_title
    }

    pub fn extract_twitter_title(&self) -> Option<String> {
        let twitter_title_selector =
            scraper::Selector::parse("meta[name='twitter:title']").unwrap();

        let twitter_title = self
            .document
            .select(&twitter_title_selector)
            .next()
            .and_then(|element| {
                element
                    .value()
                    .attr("content")
                    .map(|content| content.to_string())
            });

        twitter_title
    }

    pub fn title(&self) -> Option<String> {
        self.extract_og_title()
            .or_else(|| self.extract_twitter_title())
            .or_else(|| self.extract_title())
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
}
