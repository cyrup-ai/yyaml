# yaml_sugar

Serde support for [yaml-rust2](https://github.com/Ethiraric/yaml-rust2), providing seamless serialization and deserialization of Rust data structures to and from YAML.

## Why yaml_sugar?

The original `yyaml` crate has been unmaintained since 2021. While `yaml-rust2` is a well-maintained YAML 1.2 parser, it doesn't provide serde integration out of the box. `yaml_sugar` bridges this gap by adding serde support to yaml-rust2.

## Features

- Full serde integration for yaml-rust2
- Serialize Rust types to YAML strings
- Deserialize YAML strings to Rust types
- Support for all common YAML data types
- Clean error handling
- Zero unsafe code

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
yaml_sugar = "0.1"
serde = { version = "1.0", features = ["derive"] }
```

## Examples

### Basic Usage

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    port: u16,
    enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        name: "server".to_string(),
        port: 8080,
        enabled: true,
    };

    // Serialize to YAML string
    let yaml_str = yaml_sugar::to_string(&config)?;
    println!("YAML output:\n{}", yaml_str);

    // Deserialize from YAML string
    let parsed: Config = yaml_sugar::from_str(&yaml_str)?;
    println!("Parsed: {:?}", parsed);

    Ok(())
}
```

### Working with Collections

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    connections: HashMap<String, Connection>,
    default: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Connection {
    host: String,
    port: u16,
    credentials: Credentials,
}

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = r#"
connections:
  primary:
    host: localhost
    port: 5432
    credentials:
      username: admin
      password: secret
  secondary:
    host: backup.example.com
    port: 5433
    credentials:
      username: readonly
      password: readonly123
default: primary
"#;

    let db: Database = yaml_sugar::from_str(yaml)?;
    println!("Loaded {} connections", db.connections.len());
    println!("Default connection: {}", db.default);

    Ok(())
}
```

### Configuration File Example

```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    pool_size: u32,
    timeout: u64,
}

fn load_config(path: &str) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config = yaml_sugar::from_str(&contents)?;
    Ok(config)
}

fn save_config(config: &AppConfig, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let yaml = yaml_sugar::to_string(config)?;
    fs::write(path, yaml)?;
    Ok(())
}
```

## Comparison with yyaml

`yaml_sugar` provides a compatible API with `yyaml`, making migration straightforward:

```rust
// Before (yyaml)
let value: MyType = yyaml::from_str(&yaml_string)?;
let yaml_string = yyaml::to_string(&value)?;

// After (yaml_sugar)
let value: MyType = yaml_sugar::from_str(&yaml_string)?;
let yaml_string = yaml_sugar::to_string(&value)?;
```

## Supported Types

yaml_sugar supports all types that implement serde's `Serialize` and `Deserialize` traits:

- Primitives: bool, integers, floats, strings
- Collections: Vec, HashMap, BTreeMap, HashSet
- Options and Results
- Tuples and arrays
- Custom structs and enums with serde derives

## Error Handling

The crate provides a custom `Error` type that covers:

- YAML parsing errors
- Serialization/deserialization errors
- Type conversion errors
- I/O errors

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.