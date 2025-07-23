use serde::Deserialize;
use yyaml::Value;

#[derive(Debug, Deserialize)]
struct SimpleStruct {
    x: usize,
}

fn main() {
    println!("Testing simple tagged struct deserialization...");
    
    // Create a simple tagged value manually
    let tag = yyaml::value::Tag::new(None, "wat".to_string());
    let inner_value = yyaml::Value::Mapping({
        let mut map = yyaml::Mapping::new();
        map.insert(
            yyaml::Value::String("x".to_string()),
            yyaml::Value::Number(yyaml::Number::from(42)),
        );
        map
    });
    let tagged_value = yyaml::value::TaggedValue::new(tag, inner_value);
    let value = yyaml::Value::Tagged(Box::new(tagged_value));
    
    println!("Created tagged value: {:?}", value);
    
    println!("Attempting to deserialize...");
    match SimpleStruct::deserialize(&value) {
        Ok(result) => println!("SUCCESS: {:?}", result),
        Err(e) => println!("ERROR: {}", e),
    }
}