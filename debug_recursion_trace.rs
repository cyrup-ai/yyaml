use serde::Deserialize;
use std::collections::BTreeMap;
use yyaml::Value;

#[derive(Debug, Deserialize, PartialEq)]
struct Data {
    struc: Struc,
    tuple: Tuple,
    newtype: Newtype,
    map: BTreeMap<char, usize>,
    vec: Vec<usize>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Struc {
    x: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Tuple(usize, usize);

#[derive(Debug, Deserialize, PartialEq)]
struct Newtype(usize);

fn main() {
    println!("Testing recursion elimination with simple tagged value...");
    
    // Test simple tagged value first
    let simple_yaml = "test: !wat 42";
    match yyaml::parse_str::<Value>(simple_yaml) {
        Ok(value) => {
            println!("Successfully parsed simple tagged value: {:?}", value);
            
            // Try to deserialize a simple struct from this
            #[derive(Debug, Deserialize)]
            struct Simple { test: usize }
            
            match Simple::deserialize(&value) {
                Ok(result) => println!("Successfully deserialized simple: {:?}", result),
                Err(e) => println!("Failed to deserialize simple: {}", e),
            }
        }
        Err(e) => println!("Failed to parse simple tagged value: {}", e),
    }
    
    println!("\nTesting complex tagged structure...");
    
    // This is the problematic YAML from test_ignore_tag
    let yaml = r#"
struc: !wat
  x: 0
tuple: !wat
  - 0
  - 0  
newtype: !wat 0
map: !wat
  x: 0
vec: !wat
  - 0
"#;

    match yyaml::parse_str::<Value>(yaml) {
        Ok(value) => {
            println!("Successfully parsed complex tagged YAML");
            println!("Value structure: {:#?}", value);
            
            println!("\nAttempting deserialization...");
            match Data::deserialize(&value) {
                Ok(result) => println!("SUCCESS: Deserialized without stack overflow: {:?}", result),
                Err(e) => println!("ERROR: Deserialization failed: {}", e),
            }
        }
        Err(e) => println!("Failed to parse YAML: {}", e),
    }
}