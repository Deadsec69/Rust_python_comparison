use rayon::prelude::*;
use scraper::{Html, Selector};
use std::sync::Arc;
use parking_lot::RwLock;
use thiserror::Error;
use crate::utils::metrics::Metrics;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Failed to parse HTML")]
    ParseError,
    #[error("Failed to find content")]
    ContentNotFound,
}

pub struct Processor {
    metrics: Arc<RwLock<Metrics>>,
}

#[derive(Debug)]
pub struct ProcessedContent {
    pub title: String,
    pub links: Vec<String>,
    pub text_content: String,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(Metrics::new())),
        }
    }

    pub fn process_contents(&self, contents: Vec<String>) -> Vec<Result<ProcessedContent, ProcessError>> {
        self.metrics.write().start_operation();

        contents.par_iter() // Using Rayon's parallel iterator
            .map(|content| {
                let start = std::time::Instant::now();
                
                match self.extract_content(content) {
                    Ok(processed) => {
                        self.metrics.write().record_success(
                            "processing".to_string(),
                            start.elapsed(),
                            content.len()
                        );
                        Ok(processed)
                    }
                    Err(e) => {
                        self.metrics.write().record_failure(
                            "processing".to_string(),
                            start.elapsed()
                        );
                        Err(e)
                    }
                }
            })
            .collect()
    }

    fn extract_content(&self, html: &str) -> Result<ProcessedContent, ProcessError> {
        let document = Html::parse_document(html);

        // Extract title
        let title_selector = Selector::parse("title").map_err(|_| ProcessError::ParseError)?;
        let title = document
            .select(&title_selector)
            .next()
            .ok_or(ProcessError::ContentNotFound)?
            .text()
            .collect::<String>();

        // Extract links
        let link_selector = Selector::parse("a[href]").map_err(|_| ProcessError::ParseError)?;
        let links: Vec<String> = document
            .select(&link_selector)
            .filter_map(|element| element.value().attr("href"))
            .map(String::from)
            .collect();

        // Extract text content
        let text_selector = Selector::parse("p, h1, h2, h3, h4, h5, h6")
            .map_err(|_| ProcessError::ParseError)?;
        let text_content: String = document
            .select(&text_selector)
            .map(|element| element.text().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        Ok(ProcessedContent {
            title,
            links,
            text_content,
        })
    }

    pub fn get_metrics(&self) -> String {
        self.metrics.read().get_summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_processing() {
        let processor = Processor::new();
        let html_content = r#"
            <html>
                <head><title>Test Page</title></head>
                <body>
                    <h1>Hello World</h1>
                    <p>Test content</p>
                    <a href="https://example.com">Link</a>
                </body>
            </html>
        "#.to_string();

        let results = processor.process_contents(vec![html_content]);
        assert_eq!(results.len(), 1);
        
        if let Ok(content) = &results[0] {
            assert_eq!(content.title, "Test Page");
            assert_eq!(content.links.len(), 1);
            assert!(content.text_content.contains("Hello World"));
        }

        println!("{}", processor.get_metrics());
    }
}
