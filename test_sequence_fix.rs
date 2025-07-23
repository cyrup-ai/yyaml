extern crate yyaml;
use yyaml::YamlLoader;

fn main() {
    println!("=== Testing Sequence Bug Fix ===");
    
    // Test case that was previously broken
    let yaml = "- key: value";
    println!("Input: {:?}", yaml);
    
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            assert_eq!(docs.len(), 1, "Should have exactly 1 document");
            
            let doc = &docs[0];
            println!("Parsed as: {:?}", doc);
            
            // Verify it's an array
            let array = doc.as_vec().expect("Should be an array");
            assert_eq!(array.len(), 1, "Array should have 1 item");
            
            // Verify the item is a hash
            let item = &array[0];
            let hash = item.as_hash().expect("Item should be a hash");
            assert_eq!(hash.len(), 1, "Hash should have 1 key-value pair");
            
            // Verify the key-value pair
            let key = yyaml::Yaml::String("key".to_string());
            let value = hash.get(&key).expect("Should have 'key' key");
            assert_eq!(value.as_str().unwrap(), "value");
            
            println!("✅ SUCCESS: Sequence with mapping parsed correctly!");
            println!("   Array[0] = Hash{{\"key\": \"value\"}}");
        }
        Err(e) => {
            panic!("❌ FAILED: Parse error: {}", e);
        }
    }
    
    // Test additional cases
    println!("\n=== Testing Additional Cases ===");
    
    // Multiple items
    let yaml2 = "- key1: value1\n- key2: value2";
    match YamlLoader::load_from_str(yaml2) {
        Ok(docs) => {
            let array = docs[0].as_vec().expect("Should be array");
            assert_eq!(array.len(), 2, "Should have 2 items");
            println!("✅ Multi-item sequence: {} items", array.len());
        }
        Err(e) => println!("❌ Multi-item failed: {}", e),
    }
    
    // Mixed values
    let yaml3 = "- simple\n- key: value";
    match YamlLoader::load_from_str(yaml3) {
        Ok(docs) => {
            let array = docs[0].as_vec().expect("Should be array");
            assert_eq!(array.len(), 2, "Should have 2 items");
            println!("✅ Mixed sequence: {} items", array.len());
        }
        Err(e) => println!("❌ Mixed sequence failed: {}", e),
    }
}