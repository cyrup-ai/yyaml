use yyaml::Value;

#[test]
fn test_minimal_tagged_sequence() {
    println!("Testing minimal tagged sequence...");
    
    // Test the simplest tagged sequence that might cause recursion
    let yaml = "vec: !wat\n  - 0";
    
    match yyaml::parse_str::<Value>(yaml) {
        Ok(value) => {
            println!("Successfully parsed tagged sequence: {value:?}");
        }
        Err(e) => {
            println!("Failed to parse tagged sequence: {e}");
            panic!("Should parse successfully");
        }
    }
}

#[test]
fn test_simple_untagged_sequence() {
    println!("Testing simple untagged sequence...");
    
    // Test the same structure without tags  
    let yaml = "vec:\n  - 0";
    
    match yyaml::parse_str::<Value>(yaml) {
        Ok(value) => {
            println!("Successfully parsed untagged sequence: {value:?}");
        }
        Err(e) => {
            println!("Failed to parse untagged sequence: {e}");
            panic!("Should parse successfully");
        }
    }
}