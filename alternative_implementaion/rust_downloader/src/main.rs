use anyhow::Result;
use rust_downloader::compare_approaches;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    // Example URLs to download - same as Python for fair comparison
    let base_urls = vec![
        "https://raw.githubusercontent.com/rust-lang/rust/master/README.md".to_string(),
        "https://raw.githubusercontent.com/python/cpython/main/README.rst".to_string(),
        "https://raw.githubusercontent.com/rust-lang/rust/master/CONTRIBUTING.md".to_string(),
        "https://raw.githubusercontent.com/python/cpython/main/CONTRIBUTING.rst".to_string(),
        "https://raw.githubusercontent.com/rust-lang/rust/master/LICENSE-MIT".to_string(),
        "https://raw.githubusercontent.com/python/cpython/main/LICENSE".to_string(),
    ];
    
    // Multiply URLs for better comparison
    let urls: Vec<String> = base_urls.iter()
        .cycle()
        .take(base_urls.len() * 5)
        .cloned()
        .collect();

    println!("\nStarting Rust async downloader demonstration");
    println!("This shows Rust's superior concurrency model");
    println!("-------------------------------------------");

    let output_dir = Path::new("downloads_rust");
    
    match compare_approaches(urls, output_dir).await {
        Ok(results) => {
            println!("\nDownload Results:");
            println!("-----------------");
            println!("Async approach took: {:.2} seconds", results["async"]);
            println!("\nNote: Rust's async runtime (tokio) provides true concurrency");
            println!("without the limitations of Python's GIL.");
            println!("The downloads are processed in parallel using a thread pool,");
            println!("with each download running in its own task.");
        }
        Err(e) => eprintln!("Error during download: {}", e),
    }

    Ok(())
}
