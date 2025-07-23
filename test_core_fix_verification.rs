extern crate yyaml;
use yyaml::YamlLoader;

fn main() {
    println!("🧪 CORE INFINITE RECURSION BUG FIX VERIFICATION");
    println!("==================================================");
    
    // The EXACT case that was broken before the fix
    let test_case = "- key: value";
    println!("Testing: {:?}", test_case);
    
    match YamlLoader::load_from_str(test_case) {
        Ok(docs) => {
            if docs.len() != 1 {
                panic!("❌ FAILED: Expected 1 document, got {}", docs.len());
            }
            
            let doc = &docs[0];
            
            // Before fix: This would be Null
            // After fix: This should be Array([Hash({"key": "value"})])
            match doc.as_vec() {
                Some(array) => {
                    if array.len() != 1 {
                        panic!("❌ FAILED: Expected array with 1 item, got {}", array.len());
                    }
                    
                    let item = &array[0];
                    match item.as_hash() {
                        Some(hash) => {
                            if hash.len() != 1 {
                                panic!("❌ FAILED: Expected hash with 1 pair, got {}", hash.len());
                            }
                            
                            let key = yyaml::Yaml::String("key".to_string());
                            match hash.get(&key) {
                                Some(value) => {
                                    if value.as_str() != Some("value") {
                                        panic!("❌ FAILED: Expected 'value', got {:?}", value);
                                    }
                                    println!("✅ SUCCESS: Infinite recursion bug FIXED!");
                                    println!("   Input: \"- key: value\"");
                                    println!("   Output: Array([Hash({{\"key\": \"value\"}})])");
                                }
                                None => panic!("❌ FAILED: Key 'key' not found in hash"),
                            }
                        }
                        None => panic!("❌ FAILED: Array item is not a hash: {:?}", item),
                    }
                }
                None => {
                    panic!("❌ FAILED: Document is not an array (this was the original bug): {:?}", doc);
                }
            }
        }
        Err(e) => {
            panic!("❌ FAILED: Parse error: {}", e);
        }
    }
    
    println!("🎉 VERIFICATION COMPLETE: Core bug fix is working correctly!");
}