use yyaml::Value;
use std::collections::BTreeMap;

fn main() {
    println!("Testing simple YAML parsing...");
    
    // Very simple test first
    let yaml = "test: 123";
    println!("Parsing: {}", yaml);
    
    match yyaml::parse_str::<BTreeMap<String, i32>>(yaml) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    
    // Test with Value
    match yyaml::parse_str::<Value>(yaml) {
        Ok(result) => println!("Value result: {:?}", result),
        Err(e) => println!("Value error: {:?}", e),
    }
}