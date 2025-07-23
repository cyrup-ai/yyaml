use serde::Deserialize;
use yyaml::{from_str, Mapping, Sequence};

#[derive(Debug, Deserialize, PartialEq)]
struct TestStruct {
    thing: Sequence,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestMapping {
    thing: Mapping,
}

fn main() {
    println!("Testing null-to-collection conversion...");
    
    // Test null value deserializing to empty sequence
    let yaml = "thing:\n";  // This parses as { thing: null }
    println!("YAML: {:?}", yaml);
    
    match from_str::<TestStruct>(yaml) {
        Ok(result) => {
            println!("✅ Successfully deserialized null to empty sequence: {:?}", result);
            assert_eq!(result.thing.len(), 0);
        }
        Err(e) => {
            println!("❌ Failed to deserialize null to sequence: {}", e);
        }
    }
    
    // Test null value deserializing to empty mapping
    match from_str::<TestMapping>(yaml) {
        Ok(result) => {
            println!("✅ Successfully deserialized null to empty mapping: {:?}", result);
            assert_eq!(result.thing.len(), 0);
        }
        Err(e) => {
            println!("❌ Failed to deserialize null to mapping: {}", e);
        }
    }
    
    println!("🎯 All null-to-collection tests completed!");
}