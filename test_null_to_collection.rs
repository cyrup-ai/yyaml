use serde::{Deserialize, Serialize};
use yyaml;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    thing: yyaml::Sequence,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStructMapping {
    thing: yyaml::Mapping,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = "thing:\n";  // This parses as { thing: null }
    
    println!("Testing null-to-sequence conversion:");
    println!("YAML: {:?}", yaml);
    
    // First, let's see what the raw value looks like
    let value: yyaml::Value = yyaml::parse_str(yaml)?;
    println!("Parsed value: {:?}", value);
    
    // Try to deserialize to Sequence
    println!("\nTrying to deserialize to Sequence:");
    match yyaml::parse_str::<TestStruct>(yaml) {
        Ok(result) => println!("SUCCESS: {:?}", result),
        Err(e) => println!("ERROR: {:?}", e),
    }
    
    // Try to deserialize to Mapping  
    println!("\nTrying to deserialize to Mapping:");
    match yyaml::parse_str::<TestStructMapping>(yaml) {
        Ok(result) => println!("SUCCESS: {:?}", result),
        Err(e) => println!("ERROR: {:?}", e),
    }
    
    Ok(())
}