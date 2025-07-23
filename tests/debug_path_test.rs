use serde::{Deserialize, Serialize};
use yyaml;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct<T> {
    thing: T,
}

#[test]
fn debug_deserialization_paths() {
    let yaml = "thing:\n";
    println!("Testing YAML: {:?}", yaml);
    
    // Test each path separately to isolate the issue
    
    // Path 1: Direct parse_str (should work - uses YamlDeserializer)
    println!("\n=== Path 1: Direct parse_str ===");
    match yyaml::parse_str::<TestStruct<yyaml::Sequence>>(yaml) {
        Ok(result) => println!("✓ Success: {:?}", result),
        Err(e) => println!("✗ Error: {}", e),
    }
    
    // Path 2: Parse to Value, then T::deserialize(&value) (uses &'de Value deserializer)
    println!("\n=== Path 2: Value then &value deserialize ===");
    match yyaml::parse_str::<yyaml::Value>(yaml) {
        Ok(value) => {
            println!("Value parsed: {:?}", value);
            
            // Also test direct null deserialization
            println!("\n--- Testing direct null deserialization ---");
            let null_value = yyaml::Value::Null;
            match yyaml::Sequence::deserialize(&null_value) {
                Ok(result) => println!("✓ Direct null->Sequence works: {:?}", result),
                Err(e) => println!("✗ Direct null->Sequence fails: {}", e),
            }
            
            match TestStruct::<yyaml::Sequence>::deserialize(&value) {
                Ok(result) => println!("✓ Success: {:?}", result),
                Err(e) => println!("✗ Error: {}", e),
            }
        },
        Err(e) => println!("✗ Parse error: {}", e),
    }
    
    // Path 3: Parse to Value, then yyaml::from_value (uses ValueDeserializerOwned)
    println!("\n=== Path 3: Value then from_value ===");
    match yyaml::parse_str::<yyaml::Value>(yaml) {
        Ok(value) => {
            match yyaml::from_value::<TestStruct<yyaml::Sequence>>(value) {
                Ok(result) => println!("✓ Success: {:?}", result),
                Err(e) => println!("✗ Error: {}", e),
            }
        },
        Err(e) => println!("✗ Parse error: {}", e),
    }
}