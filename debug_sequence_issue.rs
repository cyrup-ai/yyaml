use yyaml::YamlLoader;

fn main() {
    println!("=== Testing YAML sequence parsing issue ===");
    
    // Test YAML content that should be parsed as a sequence
    let yaml_content = r#"- provider: openai
  models:
    - name: gpt-4
      max_tokens: 4096
- provider: anthropic
  models:
    - name: claude-3
      max_tokens: 8192"#;
    
    println!("Input YAML:");
    println!("{}", yaml_content);
    println!();
    
    match YamlLoader::load_from_str(yaml_content) {
        Ok(docs) => {
            println!("✅ Parsing succeeded with {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
                
                // Check if it's parsed as an array (sequence)
                if let Some(array) = doc.as_vec() {
                    println!("  ✅ Correctly parsed as array with {} items", array.len());
                    for (j, item) in array.iter().enumerate() {
                        println!("    Item {}: {:?}", j, item);
                    }
                } else if let Some(hash) = doc.as_hash() {
                    println!("  ❌ INCORRECTLY parsed as hash with {} keys", hash.len());
                    for (key, value) in hash.iter() {
                        println!("    Key: {:?} -> Value: {:?}", key, value);
                    }
                } else {
                    println!("  ❓ Parsed as other type: {:?}", doc);
                }
            }
        },
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
        }
    }
}