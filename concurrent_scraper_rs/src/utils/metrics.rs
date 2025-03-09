use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Metrics {
    start_time: Option<Instant>,
    url_timings: HashMap<String, Duration>,
    successful_requests: usize,
    failed_requests: usize,
    total_bytes: usize,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            start_time: None,
            url_timings: HashMap::new(),
            successful_requests: 0,
            failed_requests: 0,
            total_bytes: 0,
        }
    }

    pub fn start_operation(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn record_success(&mut self, url: String, duration: Duration, bytes: usize) {
        self.url_timings.insert(url, duration);
        self.successful_requests += 1;
        self.total_bytes += bytes;
    }

    pub fn record_failure(&mut self, url: String, duration: Duration) {
        self.url_timings.insert(url, duration);
        self.failed_requests += 1;
    }

    pub fn get_summary(&self) -> String {
        let total_requests = self.successful_requests + self.failed_requests;
        let avg_duration = self.url_timings.values()
            .fold(Duration::default(), |acc, &x| acc + x)
            .div_f64(total_requests as f64);

        format!(
            "Performance Summary:\n\
             Total Requests: {}\n\
             Successful: {}\n\
             Failed: {}\n\
             Average Duration: {:.2?}\n\
             Total Data: {} bytes",
            total_requests,
            self.successful_requests,
            self.failed_requests,
            avg_duration,
            self.total_bytes
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_recording() {
        let mut metrics = Metrics::new();
        metrics.start_operation();
        
        metrics.record_success("http://example.com".to_string(), Duration::from_secs(1), 1000);
        metrics.record_failure("http://failed.com".to_string(), Duration::from_secs(2));

        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.failed_requests, 1);
        assert_eq!(metrics.total_bytes, 1000);
    }
}
