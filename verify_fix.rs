extern crate yyaml;

fn main() {
    println!("=== Testing YAML Array Fix ===");
    
    let test_cases = vec![
        ("Simple array", "- item1\n- item2"),
        ("Array with colons", "- provider: huggingface\n  model: llama\n- provider: openai\n  model: gpt4"),
        ("Mixed content", "- simple_item\n- key: value\n  nested: content"),
    ];
    
    for (name, yaml) in test_cases {
        println!("\nTesting: {}", name);
        println!("YAML: {:?}", yaml);
        
        match yyaml::YamlLoader::load_from_str(yaml) {
            Ok(docs) => {
                if let Some(doc) = docs.first() {
                    match doc {
                        yyaml::Yaml::Array(arr) => {
                            println!("✅ SUCCESS: Parsed as Array with {} items", arr.len());
                            for (i, item) in arr.iter().enumerate() {
                                println!("  Item {}: {:?}", i, item);
                            }
                        },
                        yyaml::Yaml::Hash(hash) => {
                            println!("❌ PROBLEM: Parsed as Hash instead of Array");
                            println!("   Hash has {} entries", hash.len());
                            for (key, value) in hash.iter() {
                                println!("     {:?} -> {:?}", key, value);
                            }
                        },
                        other => {
                            println!("❓ OTHER: {:?}", other);
                        }
                    }
                } else {
                    println!("❌ No documents parsed");
                }
            },
            Err(e) => {
                println!("❌ Parse error: {:?}", e);
            }
        }
    }
}