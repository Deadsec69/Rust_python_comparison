"""
Main entry point for the Python downloader comparison.
"""
from pathlib import Path
from typing import List
import sys
from .downloader import compare_approaches

def main():
    # Example URLs to download
    urls: List[str] = [
        "https://raw.githubusercontent.com/rust-lang/rust/master/README.md",
        "https://raw.githubusercontent.com/python/cpython/main/README.rst",
        "https://raw.githubusercontent.com/rust-lang/rust/master/CONTRIBUTING.md",
        "https://raw.githubusercontent.com/python/cpython/main/CONTRIBUTING.rst",
        "https://raw.githubusercontent.com/rust-lang/rust/master/LICENSE-MIT",
        "https://raw.githubusercontent.com/python/cpython/main/LICENSE",
    ] * 5  # Multiply list to create more downloads for better comparison

    # Create output directory
    output_dir = Path("downloads")
    
    try:
        results = compare_approaches(urls, output_dir)
        
        print("\nDownload Results:")
        print("-" * 50)
        print(f"Threaded approach took: {results['threaded']:.2f} seconds")
        print(f"Async approach took: {results['async']:.2f} seconds")
        print("\nNote: Python's GIL prevents true parallel execution in the threaded approach.")
        print("The async approach generally performs better for I/O-bound tasks.")
        
    except Exception as e:
        print(f"Error during download comparison: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
