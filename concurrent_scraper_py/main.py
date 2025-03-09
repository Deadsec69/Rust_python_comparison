import time
from scraper.fetcher import Fetcher
from scraper.processor import Processor

def main():
    print("Python Concurrent Scraper Demo")
    print("-----------------------------")
    print("Note: This implementation is limited by Python's Global Interpreter Lock (GIL)")
    print("CPU-bound tasks like HTML processing won't achieve true parallelism\n")

    # List of URLs to scrape (same as Rust version for comparison)
    urls = [
        "https://www.rust-lang.org",
        "https://doc.rust-lang.org",
        "https://crates.io",
        "https://blog.rust-lang.org",
        "https://foundation.rust-lang.org",
    ]

    print(f"Starting concurrent fetch of {len(urls)} URLs...")
    
    # Create fetcher with same concurrency limit as Rust version
    fetcher = Fetcher(max_concurrent=3)
    start_time = time.time()

    # Fetch URLs "concurrently" (I/O bound, so GIL impact is minimal)
    results = fetcher.fetch_urls(urls)
    
    print("\nFetch Metrics:")
    print(fetcher.get_metrics())
    print(f"Total fetch time: {time.time() - start_time:.2f} seconds")

    # Filter successful results
    successful_contents = [
        content for content in results 
        if not isinstance(content, Exception)
    ]

    print("\nStarting parallel content processing...")
    print("Note: Due to GIL, processing won't achieve true parallelism")
    processor = Processor()
    process_start = time.time()

    # Process content with threads (but limited by GIL)
    processed_results = processor.process_contents(successful_contents)
    
    print("\nProcessing Metrics:")
    print(processor.get_metrics())
    print(f"Total processing time: {time.time() - process_start:.2f} seconds")

    # Display results summary
    print("\nResults Summary:")
    print("---------------")
    for i, result in enumerate(processed_results):
        if isinstance(result, Exception):
            print(f"Page {i + 1}: Error processing content: {result}")
        else:
            print(
                f"Page {i + 1}: "
                f"Title: '{result.title}', "
                f"{len(result.links)} links found, "
                f"Content length: {len(result.text_content)} chars"
            )

    print(f"\nTotal execution time: {time.time() - start_time:.2f} seconds")
    print("\nComparison with Rust version:")
    print("- Python's GIL prevents true parallel processing of CPU-bound tasks")
    print("- Thread switching overhead impacts performance")
    print("- Locks needed for shared resources due to GIL limitations")
    print("- I/O operations (URL fetching) less impacted due to GIL release during I/O")

if __name__ == "__main__":
    main()
