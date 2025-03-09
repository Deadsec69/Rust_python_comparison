import threading
from concurrent.futures import ThreadPoolExecutor
import requests
import time
from typing import List, Dict, Union, Tuple
from utils.metrics import Metrics

class FetchError(Exception):
    """Custom exception for fetch operations."""
    pass

class Fetcher:
    def __init__(self, max_concurrent: int):
        """Initialize fetcher with concurrency limit."""
        self.max_concurrent = max_concurrent
        self.metrics = Metrics()
        self._lock = threading.Lock()  # Needed due to GIL limitations
        
    def fetch_urls(self, urls: List[str]) -> List[Union[str, Exception]]:
        """
        Fetch multiple URLs concurrently using threads.
        Note: Due to Python's GIL, this won't achieve true parallelism for CPU-bound tasks.
        """
        self.metrics.start_operation()
        results = []

        with ThreadPoolExecutor(max_workers=self.max_concurrent) as executor:
            # Submit all tasks
            future_to_url = {
                executor.submit(self._fetch_url, url): url 
                for url in urls
            }
            
            # Collect results as they complete
            for future in future_to_url:
                url = future_to_url[future]
                try:
                    content = future.result()
                    results.append(content)
                except Exception as e:
                    results.append(e)
                    
        return results

    def _fetch_url(self, url: str) -> str:
        """
        Fetch a single URL.
        Note: The GIL will be released during I/O operations (requests.get),
        but any CPU-intensive processing will be limited by the GIL.
        """
        start_time = time.time()
        
        try:
            response = requests.get(url)
            response.raise_for_status()
            content = response.text
            duration = time.time() - start_time
            
            # Need lock due to shared metrics object
            with self._lock:
                self.metrics.record_success(
                    url=url,
                    duration=duration,
                    bytes_count=len(content.encode())
                )
            
            return content
            
        except Exception as e:
            duration = time.time() - start_time
            with self._lock:
                self.metrics.record_failure(url=url, duration=duration)
            raise FetchError(f"Failed to fetch {url}: {str(e)}")

    def get_metrics(self) -> str:
        """Get metrics summary."""
        return self.metrics.get_summary()


# For testing
if __name__ == "__main__":
    urls = [
        "https://www.python.org",
        "https://www.rust-lang.org",
        "https://invalid.invalid",  # This will fail
    ]
    
    fetcher = Fetcher(max_concurrent=2)
    results = fetcher.fetch_urls(urls)
    
    print("\nResults:")
    for i, result in enumerate(results):
        if isinstance(result, Exception):
            print(f"URL {i+1}: Error - {result}")
        else:
            print(f"URL {i+1}: Success - {len(result)} bytes")
    
    print("\nMetrics:")
    print(fetcher.get_metrics())
