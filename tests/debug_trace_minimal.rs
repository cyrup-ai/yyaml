use yyaml::Value;

#[test]
fn test_trace_untagged_sequence() {
    env_logger::init();
    println!("Testing simple untagged sequence...");
    
    let yaml = "- 0";
    
    match yyaml::parse_str::<Value>(yaml) {
        Ok(value) => {
            println!("SUCCESS: Parsed untagged sequence: {:?}", value);
        }
        Err(e) => {
            println!("FAILED: Untagged sequence failed: {:?}", e);
        }
    }
}

#[test] 
fn test_trace_tagged_sequence() {
    env_logger::init();
    println!("Testing tagged sequence...");
    
    let yaml = "vec: !wat\n  - 0";
    
    match yyaml::parse_str::<Value>(yaml) {
        Ok(value) => {
            println!("SUCCESS: Parsed tagged sequence: {:?}", value);
        }
        Err(e) => {
            println!("FAILED: Tagged sequence failed: {:?}", e);
        }
    }
}