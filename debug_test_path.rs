extern crate yyaml;
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct<T> {
    thing: T,
}

fn main() {
    let yaml = "thing:\n";
    println!("Testing exact test path for YAML: {:?}", yaml);
    
    // Step 1: Parse as Value (like line 22 in test)
    println!("\nStep 1: Parse as Value");
    let value: yyaml::Value = match yyaml::parse_str(yaml) {
        Ok(v) => {
            println!("Success: {:?}", v);
            v
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    
    // Step 2: Deserialize from &Value reference (like line 23 in test)
    println!("\nStep 2: Deserialize Sequence from &Value reference");
    match TestStruct::<yyaml::Sequence>::deserialize(&value) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\nStep 3: Deserialize Mapping from &Value reference");
    match TestStruct::<yyaml::Mapping>::deserialize(&value) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}