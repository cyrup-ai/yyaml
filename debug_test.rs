// Temporary debug test to verify fast-path parsing
use yyaml::YamlLoader;

fn main() {
    let test_yaml = "hello: world
int: 42
bool: true
nulltest: ~";
    
    println!("Testing YAML parsing...");
    
    match YamlLoader::load_from_str(test_yaml) {
        Ok(docs) => {
            println!("SUCCESS! Parsed {} documents", docs.len());
            if let Some(doc) = docs.first() {
                println!("Document content: {:?}", doc);
            }
        }
        Err(e) => {
            println!("FAILED: {}", e);
        }
    }
}