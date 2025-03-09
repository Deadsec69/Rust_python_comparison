use std::sync::Arc;
use tokio::sync::Semaphore;
use parking_lot::RwLock;
use reqwest::Client;
use std::time::Instant;
use futures::future::join_all;
use thiserror::Error;
use crate::utils::metrics::Metrics;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Too many requests")]
    TooManyRequests,
}

pub struct Fetcher {
    client: Client,
    metrics: Arc<RwLock<Metrics>>,
    semaphore: Arc<Semaphore>,
}

impl Fetcher {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            client: Client::new(),
            metrics: Arc::new(RwLock::new(Metrics::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn fetch_urls(&self, urls: Vec<String>) -> Vec<Result<String, FetchError>> {
        self.metrics.write().start_operation();
        
        let futures = urls.into_iter().map(|url| {
            let client = self.client.clone();
            let semaphore = Arc::clone(&self.semaphore);
            let metrics = Arc::clone(&self.metrics);

            async move {
                // Acquire semaphore permit for concurrent limiting
                let _permit = semaphore.acquire().await.map_err(|_| FetchError::TooManyRequests)?;
                let start = Instant::now();

                match client.get(&url).send().await {
                    Ok(response) => {
                        let bytes = response.bytes().await?;
                        let duration = start.elapsed();
                        
                        metrics.write().record_success(
                            url,
                            duration,
                            bytes.len()
                        );

                        Ok(String::from_utf8_lossy(&bytes).into_owned())
                    }
                    Err(e) => {
                        metrics.write().record_failure(url, start.elapsed());
                        Err(FetchError::RequestFailed(e))
                    }
                }
            }
        });

        join_all(futures).await
    }

    pub fn get_metrics(&self) -> String {
        self.metrics.read().get_summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_concurrent_fetching() {
        let fetcher = Fetcher::new(5);
        let urls = vec![
            "https://www.rust-lang.org".to_string(),
            "https://www.python.org".to_string(),
        ];

        let results = fetcher.fetch_urls(urls).await;
        assert_eq!(results.len(), 2);
        
        // Print metrics after fetching
        println!("{}", fetcher.get_metrics());
    }
}
