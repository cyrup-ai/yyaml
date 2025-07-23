extern crate yyaml;
use yyaml::YamlLoader;

fn main() {
    println!("=== Simple sequence test ===");
    
    // Test simple sequence that should work
    let yaml = "- item1\n- item2";
    println!("Testing: {:?}", yaml);
    
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("✅ Success: {} docs", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Doc {}: {:?}", i, doc);
                if let Some(arr) = doc.as_vec() {
                    println!("  Array with {} items", arr.len());
                }
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    println!("\n=== Complex sequence test ===");
    
    // Test complex sequence that's failing
    let yaml2 = "- key: value";
    println!("Testing: {:?}", yaml2);
    
    match YamlLoader::load_from_str(yaml2) {
        Ok(docs) => {
            println!("✅ Success: {} docs", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Doc {}: {:?}", i, doc);
                if let Some(arr) = doc.as_vec() {
                    println!("  Array with {} items", arr.len());
                } else {
                    println!("  NOT an array!");
                }
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
}