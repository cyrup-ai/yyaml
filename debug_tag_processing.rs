use yyaml::YamlLoader;

fn main() {
    // Test what gets processed as scalars with tags
    let simple_tag = "test: !wat 0";
    let complex_tag = r#"test: !wat
  x: 0"#;
    
    println!("=== Simple tagged scalar ===");
    match YamlLoader::load_from_str(simple_tag) {
        Ok(docs) => {
            println!("Parsed docs: {:#?}", docs);
            if let Some(doc) = docs.get(0) {
                println!("test value: {:?}", doc["test"]);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Complex tagged structure ===");
    match YamlLoader::load_from_str(complex_tag) {
        Ok(docs) => {
            println!("Parsed docs: {:#?}", docs);
            if let Some(doc) = docs.get(0) {
                println!("test value: {:?}", doc["test"]);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test the specific failing case
    println!("\n=== Full test_ignore_tag YAML ===");
    let full_yaml = r#"struc: !wat
  x: 0
tuple: !wat
  - 0
  - 0
newtype: !wat 0
map: !wat
  x: 0
vec: !wat
  - 0"#;
    
    match YamlLoader::load_from_str(full_yaml) {
        Ok(docs) => {
            println!("Parsed docs: {:#?}", docs);
            if let Some(doc) = docs.get(0) {
                println!("struc: {:?}", doc["struc"]);
                println!("tuple: {:?}", doc["tuple"]);
                println!("newtype: {:?}", doc["newtype"]);
                println!("map: {:?}", doc["map"]);
                println!("vec: {:?}", doc["vec"]);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}