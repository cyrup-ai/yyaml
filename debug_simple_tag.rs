use yyaml::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
struct TestStruct {
    x: usize,
}

fn main() {
    // Create a simple tagged value
    let yaml_str = "x: \!wat 0";
    let value: Value = yyaml::parse_str(yaml_str).unwrap();
    println\!("Parsed value: {:?}", value);
    
    // Try to deserialize it - this should cause stack overflow if recursion exists
    let result: TestStruct = TestStruct::deserialize(&value).unwrap();
    println\!("Deserialized: {:?}", result);
}
EOF < /dev/null