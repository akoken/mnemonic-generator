# Mnemonic Generator

A lightweight Rust library for generating Docker-like mnemonics.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
mnemonic-generator = "0.1.0"
```

## Usage

### Basic Usage

```rust
use mnemonic_generator::MnemonicGenerator;

fn main() {
    // Create a generator with default words
    let generator = MnemonicGenerator::new();

    // Generate a mnemonic with default underscore separator
    match generator.generate() {
        Ok(mnemonic) => println!("Generated mnemonic: {}", mnemonic),
        Err(e) => eprintln!("Error: {}", e)
    }
}
```

### Custom Word Lists

```rust
use mnemonic_generator::MnemonicGenerator;

fn main() {
    // Create a generator with custom words
    let generator = MnemonicGenerator::with_words(
        vec!["amazing".to_string(), "legend".to_string()],
        vec!["jordan".to_string(), "larry".to_string()]
    );

    // Generate a mnemonic with a custom separator
    match generator.generate_with_separator("-") {
        Ok(mnemonic) => println!("Custom mnemonic: {}", mnemonic),
        Err(e) => eprintln!("Error: {}", e)
    }
}
```

## Error Handling

The library provides a `MnemonicError` enum to handle potential generation errors:

- `EmptyWordList`: Occurs when no words are available for generating a mnemonic

## License

[MIT License]

