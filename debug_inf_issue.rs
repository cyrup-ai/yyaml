use yyaml::{parse_str, YamlLoader};
use serde_yaml::Value;

fn main() {
    // First, let's see what the YAML parser produces for .inf
    let yaml_doc = YamlLoader::load_from_str(".inf").unwrap();
    println!("Raw YAML parsed: {:?}", yaml_doc[0]);
    
    // Now let's see what happens when we try to deserialize into serde_yaml::Value
    let parsed: Result<Value, _> = parse_str(".inf");
    println!("Serde parsed result: {:?}", parsed);
    
    // And let's try deserializing into f64 directly
    let parsed_f64: Result<f64, _> = parse_str(".inf");
    println!("Direct f64 parsed result: {:?}", parsed_f64);
}