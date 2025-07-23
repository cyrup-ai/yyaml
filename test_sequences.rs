extern crate yyaml;
use yyaml::YamlLoader;

fn main() {
    println!("=== Testing YAML sequence parsing bugs ===\n");
    
    // Test the exact YAML from test_yaml.rs that shows the bug
    let yaml_content = r#"- provider: openai
  models:
    - name: gpt-4.1
      max_input_tokens: 1047576
- provider: anthropic
  models:
    - name: claude-3"#;

    println!("Testing YAML sequence with nested mappings:");
    println!("{}", yaml_content);
    println!("---");
    
    match YamlLoader::load_from_str(yaml_content) {
        Ok(docs) => {
            println!("✅ Parsed {} documents", docs.len());
            
            if let Some(doc) = docs.first() {
                println!("Document type: {:?}", doc);
                
                if let Some(array) = doc.as_vec() {
                    println!("✅ Successfully parsed as ARRAY with {} items", array.len());
                    for (i, item) in array.iter().enumerate() {
                        println!("  Item {}: {:?}", i, item);
                    }
                } else {
                    println!("❌ BUG: Parsed as mapping instead of array!");
                    println!("    Keys found:");
                    if let Some(hash) = doc.as_hash() {
                        for (k, v) in hash.iter() {
                            println!("      {:?} -> {:?}", k, v);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Parse error: {}", e);
        }
    }
}