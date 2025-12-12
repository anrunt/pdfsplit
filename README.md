# pdfsplit - PDF Page Extractor

A command-line tool written in Rust for extracting specific page ranges from PDF files.

**Note:** This is my first Rust CLI project, built for personal use. It may contain bugs.

## Features

- Extract specific page ranges from PDF files
- Simple command-line interface
- Automatic output file naming
- Built with Rust

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo installed on your system

### Build from Source

1. Clone the repository:
```bash
git clone https://github.com/yourusername/pdf-splitter.git
cd pdf-splitter
```

2. Build the project using Cargo:
```bash
cargo build --release
```

3. (Optional) Install globally:
```bash
cargo install --path .
```

After installation, you can use `pdfsplit` from anywhere in your terminal.

## Usage

Run the command with your PDF filename:

```bash
pdfsplit myfile.pdf
```

The program will prompt you to enter the page range:

```
Give start and end range separated by space: 1 5
```

This will extract pages 1 through 5 from `myfile.pdf` and save them as `myfile-1-5.pdf` in the current directory.

### Example

```bash
$ pdfsplit document.pdf
Give start and end range separated by space: 3 10
Extracted 8 pages
```

Output file: `document-3-10.pdf`

### Requirements

- The PDF file must be in the current working directory
- Filename should not contain spaces, slashes, or special characters
- Page numbers must be within the document's page count
- Start page must be less than or equal to end page

## Contributing

This is a personal project and may contain bugs. If you encounter any issues or bugs:

1. Feel free to open an issue describing the problem
2. **Pull requests with bug fixes are welcome!**
3. Fork the repository, make your changes, and submit a PR

All contributions are appreciated!

## License

This project is licensed under the MIT License - you are free to use, modify, and distribute this software as you wish.

```
MIT License

Copyright (c) 2025

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Acknowledgments

Built with:
- [pdfcat](https://crates.io/crates/pdfcat) - PDF manipulation library
- [tokio](https://tokio.rs/) - Asynchronous runtime for Rust
