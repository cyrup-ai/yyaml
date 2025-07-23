extern crate yyaml;
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Data {
    a: Option<f64>,
    d: Option<f64>,
}

fn main() {
    let yaml = "none_f:\n  &none_f\n  ~\nsome_f:\n  &some_f\n  1.0\na: *none_f\nd: *some_f\n";
    println!("Testing YAML alias with Option: {:?}", yaml);
    
    // Step 1: Parse as Value to see what we get
    println!("\nStep 1: Parse as Value");
    let value: yyaml::Value = match yyaml::parse_str(yaml) {
        Ok(v) => {
            println!("Success: {:#?}", v);
            v
        },
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    
    // Step 2: Deserialize from Value
    println!("\nStep 2: Deserialize from Value");
    match Data::deserialize(&value) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Step 3: Direct parse
    println!("\nStep 3: Direct parse");
    match yyaml::parse_str::<Data>(yaml) {
        Ok(result) => println!("Success: {:?}", result),
        Err(e) => println!("Error: {}", e),
    }
}