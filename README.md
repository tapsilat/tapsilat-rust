# Tapsilat Rust SDK

The Tapsilat SDK for Rust - A simple payment and fintech library.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tapsilat = "2025.9.28"
```

## Usage

```rust
use tapsilat::hello;

fn main() {
    let greeting = hello();
    println!("{}", greeting); // Prints: hello
}
```

## Examples

This package includes several examples demonstrating different usage patterns:

```bash
# Run basic example
cargo run --example basic_usage

# Run advanced example  
cargo run --example advanced_usage
```

## API Documentation

### `hello() -> String`

Returns a simple greeting message.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a Pull Request

### Automated CI/CD
This project uses GitHub Actions for:
- **Continuous Integration**: Automated testing on multiple platforms
- **Automated Publishing**: Release to crates.io when tags are pushed
- **Documentation**: Auto-generated docs deployed to GitHub Pages
- **Release Management**: Automated version bumping and changelog generation

See [`.github/README.md`](.github/README.md) for detailed workflow documentation.
