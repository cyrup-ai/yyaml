use yyaml::{parse_str, YamlLoader, Value};

fn main() {
    println!("=== Testing .inf deserialization before fix ===");
    
    // Test current behavior - should show it deserializes as String
    let yaml_doc = YamlLoader::load_from_str(".inf").unwrap();
    println!("Raw YAML parsed: {:?}", yaml_doc[0]);
    
    // Test serde_yaml::Value deserialization
    let parsed: Result<Value, _> = parse_str(".inf");
    match parsed {
        Ok(value) => {
            println!("Parsed successfully as: {:?}", value);
            match value {
                Value::Number(n) => println!("  It's a Number: {}", n),
                Value::String(s) => println!("  It's a String: '{}'", s),
                other => println!("  It's something else: {:?}", other),
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }
    
    // Test direct f64 deserialization
    let parsed_f64: Result<f64, _> = parse_str(".inf");
    match parsed_f64 {
        Ok(f) => println!("Direct f64 parse: {}", f),
        Err(e) => println!("Direct f64 parse error: {}", e),
    }
}