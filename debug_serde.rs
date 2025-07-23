use yyaml;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct<T> {
    thing: T,
}

fn main() {
    let yaml = "thing:\n";
    println!("Testing YAML: {:?}", yaml);
    
    // Try to deserialize as sequence
    println!("\nTrying to deserialize as Sequence:");
    match yyaml::parse_str::<TestStruct<yyaml::Sequence>>(yaml) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Try to deserialize as mapping
    println!("\nTrying to deserialize as Mapping:");
    match yyaml::parse_str::<TestStruct<yyaml::Mapping>>(yaml) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Try to deserialize as Option<Sequence>
    println!("\nTrying to deserialize as Option<Sequence>:");
    match yyaml::parse_str::<TestStruct<Option<yyaml::Sequence>>>(yaml) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}