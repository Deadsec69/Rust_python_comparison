use std::path::Path;
use std::time::Instant;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Mutex;

/// A thread-safe counter that demonstrates Rust's safe concurrency
#[derive(Debug)]
pub struct SharedCounter {
    count: Arc<Mutex<u32>>,
}

impl SharedCounter {
    pub fn new() -> Self {
        Self {
            count: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn increment(&self) {
        let mut count = self.count.lock().await;
        *count += 1;
    }

    pub async fn value(&self) -> u32 {
        *self.count.lock().await
    }
}

/// Downloads multiple URLs concurrently using Rust's async/await and tokio
pub struct AsyncDownloader {
    urls: Vec<String>,
    output_dir: std::path::PathBuf,
    counter: SharedCounter,
}

impl AsyncDownloader {
    pub fn new<P: AsRef<Path>>(urls: Vec<String>, output_dir: P) -> Self {
        Self {
            urls,
            output_dir: output_dir.as_ref().to_path_buf(),
            counter: SharedCounter::new(),
        }
    }

    async fn download_url(&self, client: &reqwest::Client, url: &str) -> Result<()> {
        let response = client.get(url).send().await?;
        
        // Extract filename from URL or use default
        let filename = url.split('/').last().unwrap_or("index.html");
        let output_path = self.output_dir.join(filename);
        
        // Create file and write content
        let content = response.bytes().await?;
        let mut file = File::create(&output_path).await?;
        file.write_all(&content).await?;
        
        self.counter.increment().await;
        Ok(())
    }

    pub async fn download_all(&self) -> Result<f64> {
        // Create output directory if it doesn't exist
        tokio::fs::create_dir_all(&self.output_dir).await?;
        
        let start = Instant::now();
        let client = reqwest::Client::new();
        
        // Create progress bar
        let pb = ProgressBar::new(self.urls.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?);

        // Create futures for all downloads
        let mut handles = Vec::new();
        for url in &self.urls {
            let url = url.clone();
            let client = client.clone();
            let pb = pb.clone();
            let self_ref = self.clone();
            
            let handle = tokio::spawn(async move {
                let result = self_ref.download_url(&client, &url).await;
                pb.inc(1);
                result
            });
            handles.push(handle);
        }

        // Wait for all downloads to complete
        for handle in handles {
            handle.await??;
        }

        let downloaded = self.counter.value().await;
        pb.finish_with_message(format!("{} downloads completed", downloaded));
        
        Ok(start.elapsed().as_secs_f64())
    }
}

// Implement Clone for AsyncDownloader
impl Clone for AsyncDownloader {
    fn clone(&self) -> Self {
        Self {
            urls: self.urls.clone(),
            output_dir: self.output_dir.clone(),
            counter: SharedCounter::new(),
        }
    }
}

/// Compares different download approaches and returns timing results
pub async fn compare_approaches(urls: Vec<String>, output_dir: &Path) -> Result<std::collections::HashMap<String, f64>> {
    let mut results = std::collections::HashMap::new();
    
    // Async approach using tokio
    let async_dir = output_dir.join("async");
    let downloader = AsyncDownloader::new(urls.clone(), async_dir);
    let async_time = downloader.download_all().await?;
    results.insert("async".to_string(), async_time);

    Ok(results)
}
