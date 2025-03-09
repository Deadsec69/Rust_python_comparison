from setuptools import setup, find_packages

setup(
    name="concurrent_scraper_py",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        "requests==2.31.0",
        "beautifulsoup4==4.12.2",
        "aiohttp==3.7.4",
        "asyncio==3.4.3",
    ],
)
