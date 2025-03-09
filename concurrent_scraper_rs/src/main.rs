mod scraper;
mod utils;

use scraper::{Fetcher, Processor};
use std::time::Instant;

#[tokio::main]
async fn main() {
    // Initialize tracing for better async debugging
    tracing_subscriber::fmt::init();

    println!("Rust Concurrent Scraper Demo");
    println!("----------------------------");
    
    // List of URLs to scrape
    let urls = vec![
        "https://www.rust-lang.org".to_string(),
        "https://doc.rust-lang.org".to_string(),
        "https://crates.io".to_string(),
        "https://blog.rust-lang.org".to_string(),
        "https://foundation.rust-lang.org".to_string(),
    ];

    println!("Starting concurrent fetch of {} URLs...", urls.len());
    
    // Create fetcher with concurrency limit
    let fetcher = Fetcher::new(3); // Limit to 3 concurrent requests
    let start = Instant::now();

    // Fetch URLs concurrently
    let results = fetcher.fetch_urls(urls).await;
    
    println!("\nFetch Metrics:");
    println!("{}", fetcher.get_metrics());
    println!("Total fetch time: {:?}", start.elapsed());

    // Process successful results
    let successful_contents: Vec<String> = results
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    println!("\nStarting parallel content processing...");
    let processor = Processor::new();
    let process_start = Instant::now();

    // Process content in parallel using Rayon
    let processed_results = processor.process_contents(successful_contents);
    
    println!("\nProcessing Metrics:");
    println!("{}", processor.get_metrics());
    println!("Total processing time: {:?}", process_start.elapsed());

    // Display results summary
    println!("\nResults Summary:");
    println!("---------------");
    for (i, result) in processed_results.iter().enumerate() {
        match result {
            Ok(content) => println!(
                "Page {}: Title: '{}', {} links found, Content length: {} chars",
                i + 1,
                content.title,
                content.links.len(),
                content.text_content.len()
            ),
            Err(e) => println!("Page {}: Error processing content: {}", i + 1, e),
        }
    }

    println!("\nTotal execution time: {:?}", start.elapsed());
}
