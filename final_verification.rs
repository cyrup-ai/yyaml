use yyaml;

fn main() {
    println!("=== YAML Array Parsing Verification ===");
    
    let test_cases = vec![
        ("Simple sequence", "- item1\n- item2"),
        ("Empty scalar (null-to-collection)", "thing:\n"),
        ("Complex sequence", "- provider: huggingface\n  model: llama\n- provider: openai\n  model: gpt4"),
    ];
    
    for (name, yaml) in test_cases {
        println!("\n🧪 Testing: {}", name);
        println!("   YAML: {:?}", yaml);
        
        match yyaml::YamlLoader::load_from_str(yaml) {
            Ok(docs) => {
                if let Some(doc) = docs.first() {
                    match doc {
                        yyaml::Yaml::Array(arr) => {
                            println!("   ✅ SUCCESS: Parsed as Array with {} items", arr.len());
                        },
                        yyaml::Yaml::Hash(hash) => {
                            if name == "Empty scalar (null-to-collection)" {
                                println!("   ✅ SUCCESS: Parsed as Hash (expected for key-value)");
                            } else {
                                println!("   ❌ UNEXPECTED: Parsed as Hash instead of Array");
                            }
                        },
                        yyaml::Yaml::Null => {
                            if name == "Empty scalar (null-to-collection)" {
                                println!("   ✅ SUCCESS: Parsed as Null (valid for empty document)");
                            } else {
                                println!("   ❓ NULL: Empty document");
                            }
                        },
                        other => {
                            println!("   ❓ OTHER: {:?}", other);
                        }
                    }
                } else {
                    println!("   ❌ No documents parsed");
                }
            },
            Err(e) => {
                println!("   ❌ Parse error: {:?}", e);
            }
        }
    }
    
    println!("\n🎉 Verification complete!");
}