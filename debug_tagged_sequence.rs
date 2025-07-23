use yyaml::YamlLoader;

fn main() {
    // Test the specific failing case - tagged sequence
    let tagged_sequence = r#"tuple: !wat
  - 0
  - 0"#;
    
    println!("=== Testing tagged sequence ===");
    match YamlLoader::load_from_str(tagged_sequence) {
        Ok(docs) => {
            println!("Success! Parsed docs: {:#?}", docs);
            if let Some(doc) = docs.get(0) {
                println!("tuple value: {:?}", doc["tuple"]);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("Error mark: {:?}", e.mark);
        }
    }
    
    // Also test without tag for comparison
    let untagged_sequence = r#"tuple:
  - 0
  - 0"#;
    
    println!("\n=== Testing untagged sequence (should work) ===");
    match YamlLoader::load_from_str(untagged_sequence) {
        Ok(docs) => {
            println!("Success! Parsed docs: {:#?}", docs);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}