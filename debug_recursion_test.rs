use yyaml::Value;
use std::collections::BTreeMap;
use serde::Deserialize;

fn main() {
    println!("Testing for infinite recursion...");
    
    let yaml = "first: 1\nsecond: 2";
    println!("Parsing YAML: {}", yaml);
    
    // First parse to Value
    let value: Value = yyaml::parse_str(yaml).unwrap();
    println!("Parsed to Value: {:?}", value);
    
    // Now try to deserialize from &Value - this might cause infinite recursion
    println!("About to deserialize from &Value...");
    let result: Result<BTreeMap<String, i32>, _> = BTreeMap::deserialize(&value);
    
    match result {
        Ok(map) => println!("Success: {:?}", map),
        Err(e) => println!("Error: {:?}", e),
    }
    
    println!("Test completed without stack overflow!");
}