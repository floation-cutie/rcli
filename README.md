# RCLI

A Rust-based command-line interface toolkit demonstrating idiomatic Rust patterns, extensible architecture, and production-ready code practices.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Features

RCLI provides a collection of commonly-used CLI utilities built with modern Rust:

- **CSV Processing** - Convert CSV files to JSON/YAML formats
- **Password Generation** - Generate secure random passwords with customizable requirements
- **Base64 Encoding** - Encode/decode data using Base64 (standard or URL-safe)
- **Text Signing** - Sign and verify text using Blake3 or Ed25519 algorithms
- **HTTP File Server** - Serve static files with automatic directory listing

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
git clone <repository-url>
cd rcli
cargo build --release
```

The binary will be available at `target/release/rcli`.

### Install Globally

```bash
cargo install --path .
```

## Usage

### Quick Start

```bash
# Show help
rcli --help

# Get help for a specific command
rcli csv --help
```

### CSV Processing

Convert CSV files to JSON or YAML format:

```bash
# Convert CSV to JSON
rcli csv -i input.csv -o output.json

# Convert with custom delimiter
rcli csv -i data.csv -o data.json -d ';' --header

# Read from stdin
cat data.csv | rcli csv -i - -o output.json
```

**Options:**
- `-i, --input <FILE>` - Input CSV file (use `-` for stdin)
- `-o, --output <FILE>` - Output file path
- `--format <FORMAT>` - Output format: `json` (default) or `yaml`
- `--header` - Treat first row as header
- `-d, --delimiter <CHAR>` - Field delimiter (default: `,`)

### Password Generation

Generate secure random passwords:

```bash
# Generate a 16-character password with all character types
rcli genpass -l 16 --uppercase --lowercase --number --special

# Generate simple alphanumeric password
rcli genpass -l 12 --lowercase --number
```

**Options:**
- `-l, --length <LENGTH>` - Password length (default: 12)
- `--uppercase` - Include uppercase letters (A-Z)
- `--lowercase` - Include lowercase letters (a-z)
- `--number` - Include numbers (0-9)
- `--special` - Include special characters (!@#$%^&*)

### Base64 Encoding/Decoding

Encode or decode data using Base64:

```bash
# Encode a file
rcli base64 encode -i input.txt --format standard

# Decode a file
rcli base64 decode -i encoded.txt --format standard

# URL-safe encoding
rcli base64 encode -i data.bin --format urlsafe
```

**Subcommands:**
- `encode` - Encode data to Base64
- `decode` - Decode Base64 data

**Options:**
- `-i, --input <FILE>` - Input file (use `-` for stdin)
- `--format <FORMAT>` - Encoding format: `standard` or `urlsafe`

### Text Signing and Verification

Sign and verify text data using cryptographic algorithms:

#### Generate Keys

```bash
# Generate Blake3 key (symmetric)
rcli text generate --format blake3 -o ./keys

# Generate Ed25519 keypair (asymmetric)
rcli text generate --format ed25519 -o ./keys
```

#### Sign Data

```bash
# Sign with Blake3
rcli text sign -i message.txt -k keys/blake3.txt --format blake3

# Sign with Ed25519 private key
rcli text sign -i message.txt -k keys/ed25519.sk --format ed25519
```

#### Verify Signature

```bash
# Verify Blake3 signature
rcli text verify -i message.txt -k keys/blake3.txt --format blake3 -s <signature>

# Verify Ed25519 signature
rcli text verify -i message.txt -k keys/ed25519.pk --format ed25519 -s <signature>
```

**Supported Formats:**
- `blake3` - BLAKE3 hash-based authentication (symmetric)
- `ed25519` - Ed25519 digital signatures (asymmetric)

### HTTP File Server

Start a simple HTTP server to serve files from a directory:

```bash
# Serve current directory on port 8080
rcli http serve -d . --port 8080

# Serve specific directory
rcli http serve -d /var/www/html --port 3000
```

**Features:**
- Automatic directory listing
- File serving with proper MIME types
- Support for both text and binary files

**Options:**
- `-d, --dir <DIR>` - Directory to serve (default: `.`)
- `-p, --port <PORT>` - Server port (default: 8080)

## Examples

### CSV to JSON Conversion

```bash
# Input: data.csv
# name,age,city
# Alice,30,NYC
# Bob,25,LA

rcli csv -i data.csv -o data.json --header

# Output: data.json
# [{"name":"Alice","age":"30","city":"NYC"},{"name":"Bob","age":"25","city":"LA"}]
```

### Secure Password Generation

```bash
rcli genpass -l 20 --uppercase --lowercase --number --special
# Output: aB3$xY9!mN7&qR2@pL5%
```

### Text Signing Workflow

```bash
# 1. Generate keypair
rcli text generate --format ed25519 -o ./keys

# 2. Sign a message
echo "Hello, World!" > message.txt
SIGNATURE=$(rcli text sign -i message.txt -k keys/ed25519.sk --format ed25519)

# 3. Verify signature
rcli text verify -i message.txt -k keys/ed25519.pk --format ed25519 -s "$SIGNATURE"
# Output: true
```

## Project Structure

```
rcli/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library root with trait definitions
│   ├── cli.rs               # CLI argument parsing (using clap)
│   ├── cli/                 # Subcommand definitions
│   │   ├── base64.rs        # Base64 encode/decode commands
│   │   ├── csv.rs           # CSV processing commands
│   │   ├── genpass.rs       # Password generation commands
│   │   ├── http.rs          # HTTP server commands
│   │   └── text.rs          # Text signing commands
│   └── process/             # Business logic implementations
│       ├── b64.rs           # Base64 encoding logic
│       ├── csv_convert.rs   # CSV conversion logic
│       ├── gen_pass.rs      # Password generation logic
│       ├── http_serve.rs    # HTTP server logic
│       └── text.rs          # Text signing/verification logic
├── fixtures/                # Test fixtures and example files
├── Cargo.toml               # Project dependencies
└── README.md                # This file
```

## Technology Stack

- **[clap](https://github.com/clap-rs/clap)** - Command-line argument parsing
- **[serde](https://serde.rs/)** - Serialization/deserialization
- **[tokio](https://tokio.rs/)** - Async runtime
- **[axum](https://github.com/tokio-rs/axum)** - Web framework for HTTP server
- **[csv](https://github.com/BurntSushi/rust-csv)** - CSV parsing
- **[base64](https://github.com/marshallpierce/rust-base64)** - Base64 encoding
- **[blake3](https://github.com/BLAKE3-team/BLAKE3)** - BLAKE3 hashing
- **[ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek)** - Ed25519 signatures
- **[anyhow](https://github.com/dtolnay/anyhow)** - Error handling
- **[enum_dispatch](https://gitlab.com/antonok/enum_dispatch)** - Trait dispatch optimization

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with nextest (if installed)
cargo nextest run

# Run clippy
cargo clippy -- -D warnings
```

### Code Formatting

```bash
cargo fmt
```

### Pre-commit Hooks

This project uses [pre-commit](https://pre-commit.com/) for code quality checks. Install hooks:

```bash
pre-commit install
```

## Architecture Highlights

This project demonstrates several Rust best practices:

- **Trait-based design** - Using `enum_dispatch` for zero-cost abstraction
- **Async/await** - Tokio runtime for async operations
- **Error handling** - Comprehensive error handling with `anyhow`
- **Type safety** - Strong typing with custom types and validation
- **Module organization** - Clean separation of CLI, business logic, and utilities

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Author

cutie <wzxfloation_cutie@bupt.edu.cn>
