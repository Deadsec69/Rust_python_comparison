"""
Parallel downloader implementation in Python using both threading and asyncio.
This demonstrates the limitations of Python's GIL in CPU-bound tasks.
"""
from concurrent.futures import ThreadPoolExecutor
import threading
import time
import asyncio
from typing import List, Dict
import aiohttp
import requests
from pathlib import Path

class SharedCounter:
    """
    A thread-safe counter to demonstrate Python's thread safety mechanisms.
    """
    def __init__(self):
        self._value = 0
        self._lock = threading.Lock()
    
    def increment(self):
        with self._lock:
            self._value += 1
    
    @property
    def value(self) -> int:
        return self._value

class ThreadedDownloader:
    """
    Traditional threaded implementation using Python's threading.
    This will demonstrate GIL limitations.
    """
    def __init__(self, urls: List[str], output_dir: Path):
        self.urls = urls
        self.output_dir = output_dir
        self.counter = SharedCounter()
        self.output_dir.mkdir(exist_ok=True, parents=True)
    
    def download_url(self, url: str) -> None:
        """Download a single URL using requests."""
        try:
            response = requests.get(url, timeout=30)
            filename = url.split('/')[-1] or 'index.html'
            output_path = self.output_dir / filename
            
            with open(output_path, 'wb') as f:
                f.write(response.content)
            
            self.counter.increment()
            print(f"Downloaded: {url}")
        except Exception as e:
            print(f"Error downloading {url}: {e}")
    
    def download_all(self, max_workers: int = 10) -> float:
        """
        Download all URLs using a thread pool.
        Returns the total time taken.
        """
        start_time = time.time()
        
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            executor.map(self.download_url, self.urls)
        
        return time.time() - start_time

class AsyncDownloader:
    """
    Asynchronous implementation using aiohttp.
    This will demonstrate better I/O performance compared to threading.
    """
    def __init__(self, urls: List[str], output_dir: Path):
        self.urls = urls
        self.output_dir = output_dir
        self.counter = 0
        self.output_dir.mkdir(exist_ok=True, parents=True)
    
    async def download_url(self, session: aiohttp.ClientSession, url: str) -> None:
        """Download a single URL using aiohttp."""
        try:
            async with session.get(url) as response:
                filename = url.split('/')[-1] or 'index.html'
                output_path = self.output_dir / filename
                
                content = await response.read()
                with open(output_path, 'wb') as f:
                    f.write(content)
                
                self.counter += 1
                print(f"Downloaded: {url}")
        except Exception as e:
            print(f"Error downloading {url}: {e}")
    
    async def download_all(self) -> float:
        """
        Download all URLs asynchronously.
        Returns the total time taken.
        """
        start_time = time.time()
        
        async with aiohttp.ClientSession() as session:
            tasks = [
                self.download_url(session, url)
                for url in self.urls
            ]
            await asyncio.gather(*tasks)
        
        return time.time() - start_time

def compare_approaches(urls: List[str], output_dir: Path) -> Dict[str, float]:
    """
    Compare threaded vs async approaches.
    Returns timing results for both approaches.
    """
    # Create base output directory
    output_dir.mkdir(exist_ok=True, parents=True)
    
    # Threaded approach
    threaded_dir = output_dir / "threaded"
    threaded = ThreadedDownloader(urls, threaded_dir)
    threaded_time = threaded.download_all()
    
    # Async approach
    async_dir = output_dir / "async"
    async_downloader = AsyncDownloader(urls, async_dir)
    async_time = asyncio.run(async_downloader.download_all())
    
    return {
        "threaded": threaded_time,
        "async": async_time
    }
