from dataclasses import dataclass, field
from typing import Dict
import time
from datetime import timedelta

@dataclass
class Metrics:
    start_time: float = None
    url_timings: Dict[str, float] = field(default_factory=dict)
    successful_requests: int = 0
    failed_requests: int = 0
    total_bytes: int = 0

    def start_operation(self):
        """Start timing an operation."""
        self.start_time = time.time()

    def record_success(self, url: str, duration: float, bytes_count: int):
        """Record a successful operation."""
        self.url_timings[url] = duration
        self.successful_requests += 1
        self.total_bytes += bytes_count

    def record_failure(self, url: str, duration: float):
        """Record a failed operation."""
        self.url_timings[url] = duration
        self.failed_requests += 1

    def get_summary(self) -> str:
        """Get a formatted summary of the metrics."""
        total_requests = self.successful_requests + self.failed_requests
        if total_requests == 0:
            return "No requests processed"

        avg_duration = sum(self.url_timings.values()) / total_requests

        return (
            f"Performance Summary:\n"
            f"Total Requests: {total_requests}\n"
            f"Successful: {self.successful_requests}\n"
            f"Failed: {self.failed_requests}\n"
            f"Average Duration: {timedelta(seconds=avg_duration)}\n"
            f"Total Data: {self.total_bytes} bytes"
        )


# For testing
if __name__ == "__main__":
    metrics = Metrics()
    metrics.start_operation()
    
    # Simulate some operations
    metrics.record_success("http://example.com", 1.5, 1000)
    metrics.record_failure("http://failed.com", 2.0)
    
    print(metrics.get_summary())
