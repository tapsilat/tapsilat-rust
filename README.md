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
