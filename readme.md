# Comprehensive Web Scraper with Rust, Perl, and Python

## Overview

This repository contains a web scraper implemented across three different languages: Rust, Perl, and Python. Each component performs specific tasks within the web scraping pipeline and communicates via encrypted IPC (Inter-Process Communication).

- **Rust**: Manages the HTTP requests and initial data storage.
- **Perl**: Parses the HTML content and saves reply logs.
- **Python**: Handles additional data processing and saves to Parquet files.

## File Structure

```plaintext
.
├── rust_part
│   ├── Cargo.toml
│   └── main.rs
├── perl_part
│   └── scraper.pl
└── python_part
    └── scraper.py
```

## Requirements

### Rust

- [Reqwest](https://docs.rs/reqwest)
- [UUID](https://docs.rs/uuid)
- [Serde JSON](https://docs.rs/serde_json)
- [Tokio](https://docs.rs/tokio)
- [SHA2](https://docs.rs/sha2)

### Perl

- [JSON](https://metacpan.org/pod/JSON)
- [File::Slurp](https://metacpan.org/pod/File::Slurp)
- [Data::UUID](https://metacpan.org/pod/Data::UUID)
- [DateTime](https://metacpan.org/pod/DateTime)
- [AnyEvent::Socket](https://metacpan.org/pod/AnyEvent::Socket)
- [Digest::SHA](https://metacpan.org/pod/Digest::SHA)

### Python

- [Pandas](https://pandas.pydata.org)
- [Cryptography](https://cryptography.io/en/latest/)
- [PyArrow](https://arrow.apache.org/docs/python/)

## Setup and Installation

### Rust

1. Navigate to `rust_part`.
2. Run `cargo build` to compile the code.
3. Execute with `cargo run`.

### Perl

1. Navigate to `perl_part`.
2. Install Perl modules via CPAN or your package manager.
3. Run the script with `perl main.pl`.

### Python

1. Navigate to `python_part`.
2. Install the required Python packages with `pip install -r requirements.txt`.
3. Execute the script with `python main.py`.

## Workflow

1. **Rust**: The Rust component makes HTTP requests to fetch web pages and saves the request logs. It also saves the fetched data temporarily. This component initiates the IPC to send encrypted data to the Perl component.
  
2. **Perl**: The Perl component listens for incoming IPC data. Upon receiving, it decrypts the data and parses the HTML content to extract links, images, and text. It also saves a reply log.

3. **Python**: This component listens for incoming IPC data from the Perl component. It decrypts and processes the data, saves it in a Parquet file, and logs the operation.

## Security

All data transferred between components is encrypted using SHA-256. Python and Perl use additional Fernet symmetric encryption for added security.

## Error Handling

- Rust: Utilizes Rust's built-in error-handling mechanisms.
- Perl: Utilizes Perl's built-in error-handling mechanisms.
- Python: Has explicit try-except blocks with retries for robustness.

## Future Enhancements

[] Implement a handshake protocol between components to ensure data integrity.
[] Extend the scraper to include more advanced data parsing capabilities.
[] Add a UI/terminal for the user to enter in a website 
[] Have a JSON be accepted for multiple URLs to be scraped at once
[] Add a database to store the data


## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
