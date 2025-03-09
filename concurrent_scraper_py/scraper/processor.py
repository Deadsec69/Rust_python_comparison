from bs4 import BeautifulSoup
from concurrent.futures import ThreadPoolExecutor
import threading
from typing import List, Dict, Optional, NamedTuple
import time
from utils.metrics import Metrics

class ProcessedContent(NamedTuple):
    """Container for processed content."""
    title: str
    links: List[str]
    text_content: str

class ProcessError(Exception):
    """Custom exception for processing errors."""
    pass

class Processor:
    def __init__(self):
        """Initialize processor with metrics."""
        self.metrics = Metrics()
        self._lock = threading.Lock()  # Required due to GIL limitations

    def process_contents(self, contents: List[str]) -> List[ProcessedContent]:
        """
        Process multiple HTML contents concurrently using threads.
        Note: Due to Python's GIL, this won't achieve true parallelism
        since HTML parsing is CPU-bound.
        """
        self.metrics.start_operation()
        results = []

        # Using ThreadPoolExecutor, but due to GIL, this won't provide
        # true parallelism for CPU-bound tasks
        with ThreadPoolExecutor() as executor:
            future_to_content = {
                executor.submit(self._process_content, content): i 
                for i, content in enumerate(contents)
            }

            for future in future_to_content:
                try:
                    result = future.result()
                    results.append(result)
                except Exception as e:
                    results.append(e)

        return results

    def _process_content(self, html: str) -> ProcessedContent:
        """
        Process a single HTML content.
        Note: This is CPU-bound and will be limited by the GIL.
        """
        start_time = time.time()

        try:
            # Parse HTML - This is CPU-bound and will be affected by GIL
            soup = BeautifulSoup(html, 'html.parser')

            # Extract title
            title = soup.title.string if soup.title else "No title"

            # Extract links - More CPU-bound operations
            links = [
                a.get('href') 
                for a in soup.find_all('a', href=True)
            ]

            # Extract text content - More CPU-bound operations
            text_content = ' '.join(
                p.get_text() 
                for p in soup.find_all(['p', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6'])
            )

            duration = time.time() - start_time
            
            # Need lock due to shared metrics object
            with self._lock:
                self.metrics.record_success(
                    url="processing",
                    duration=duration,
                    bytes_count=len(html.encode())
                )

            return ProcessedContent(
                title=title,
                links=links,
                text_content=text_content
            )

        except Exception as e:
            duration = time.time() - start_time
            with self._lock:
                self.metrics.record_failure(
                    url="processing",
                    duration=duration
                )
            raise ProcessError(f"Failed to process content: {str(e)}")

    def get_metrics(self) -> str:
        """Get metrics summary."""
        return self.metrics.get_summary()


# For testing
if __name__ == "__main__":
    test_html = """
        <html>
            <head><title>Test Page</title></head>
            <body>
                <h1>Hello World</h1>
                <p>Test content</p>
                <a href="https://example.com">Link</a>
            </body>
        </html>
    """
    
    processor = Processor()
    results = processor.process_contents([test_html])
    
    print("\nResults:")
    for i, result in enumerate(results):
        if isinstance(result, Exception):
            print(f"Content {i+1}: Error - {result}")
        else:
            print(f"Content {i+1}:")
            print(f"  Title: {result.title}")
            print(f"  Links: {len(result.links)}")
            print(f"  Content length: {len(result.text_content)} chars")
    
    print("\nMetrics:")
    print(processor.get_metrics())
