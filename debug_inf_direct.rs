use yyaml::{Yaml, YamlLoader};

fn main() {
    // Test what happens when we parse .inf
    println!("=== Testing .inf parsing ===");
    
    let docs = YamlLoader::load_from_str(".inf").unwrap();
    let yaml_value = &docs[0];
    println!("Parsed Yaml value: {:?}", yaml_value);
    
    // Test the parse_f64 function directly
    match yaml_value {
        Yaml::Real(s) => {
            println!("It's a Real with string: '{}'", s);
            match s.parse::<f64>() {
                Ok(f) => println!("String parses to f64: {}", f),
                Err(e) => println!("String parse error: {}", e),
            }
        }
        other => println!("Not a Real variant: {:?}", other),
    }
    
    // Test as_f64 method
    println!("yaml_value.as_f64(): {:?}", yaml_value.as_f64());
}