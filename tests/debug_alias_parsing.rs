use yyaml::{YamlLoader, Yaml, Value};
use std::collections::BTreeMap;

#[test]
fn debug_alias_parsing() {
    println!("=== ALIAS PARSING DEBUG ANALYSIS ===");
    
    // The exact YAML from test_alias that should produce {"first": 1, "second": 1, "third": 3}
    let test_yaml = r#"first:
  &alias
  1
second:
  *alias
third: 3"#;
    
    println!("Input YAML:");
    println!("{test_yaml}");
    println!("\n--- PARSING ANALYSIS ---");
    
    // Parse with YamlLoader::load_from_str
    let docs = YamlLoader::load_from_str(test_yaml).expect("Parsing should succeed");
    println!("✅ Parsing succeeded!");
    println!("Document count: {}", docs.len());
    
    for (i, doc) in docs.iter().enumerate() {
        println!("\n📄 Document {i}: {doc:#?}");
        
        // Analyze the structure
        match doc {
            Yaml::Hash(map) => {
                println!("📋 Hash with {} entries:", map.len());
                for (key, value) in map.iter() {
                    println!("  Key: {key:#?}");
                    println!("  Value: {value:#?}");
                    
                    // Check for unresolved aliases or BadValues
                    match value {
                        Yaml::Alias(id) => println!("  ⚠️  UNRESOLVED ALIAS: {id}"),
                        Yaml::BadValue => println!("  ❌ BAD VALUE DETECTED"),
                        _ => {}
                    }
                }
            }
            Yaml::Null => println!("📭 Empty/Null document"),
            Yaml::BadValue => println!("❌ BadValue document"),
            other => println!("📦 Other document type: {other:#?}"),
        }
    }
    
    // Test Value conversion
    println!("\n--- VALUE CONVERSION TEST ---");
    if let Some(first_doc) = docs.first() {
        // This might panic if aliases are unresolved
        let conversion_result = std::panic::catch_unwind(|| {
            Value::from_yaml(first_doc)
        });
        
        match conversion_result {
            Ok(value) => {
                println!("✅ Value conversion succeeded: {value:#?}");
                
                // Test serde deserialization
                println!("\n--- SERDE DESERIALIZATION TEST ---");
                match yyaml::from_value::<BTreeMap<String, i32>>(value) {
                    Ok(map) => {
                        println!("✅ Serde deserialization succeeded!");
                        println!("Final result: {map:?}");
                        
                        // Compare with expected
                        let mut expected = BTreeMap::new();
                        expected.insert("first".to_owned(), 1);
                        expected.insert("second".to_owned(), 1);
                        expected.insert("third".to_owned(), 3);
                        
                        if map == expected {
                            println!("🎉 PERFECT MATCH! Alias resolution working correctly!");
                        } else {
                            println!("❌ MISMATCH!");
                            println!("Expected: {expected:?}");
                            println!("Got:      {map:?}");
                        }
                    }
                    Err(e) => println!("❌ Serde deserialization failed: {e}"),
                }
            }
            Err(_) => {
                println!("❌ Value conversion PANICKED! This indicates unresolved aliases.");
            }
        }
    }
    
    // Additional analysis - test simpler alias cases
    println!("\n--- SIMPLER ALIAS TEST ---");
    let simple_yaml = r#"anchor: &test 42
alias: *test"#;
    
    match YamlLoader::load_from_str(simple_yaml) {
        Ok(docs) => {
            println!("Simple alias parsing result:");
            for doc in &docs {
                println!("  {doc:#?}");
            }
        }
        Err(e) => println!("Simple alias parsing failed: {e}"),
    }
    
    // Test what fast parser excludes
    println!("\n--- FAST PARSER EXCLUSION TEST ---");
    println!("Original YAML contains '&': {}", test_yaml.contains('&'));
    println!("Original YAML contains '*': {}", test_yaml.contains('*'));
    println!("This should exclude it from fast parser and use full parser");
    
    // Direct serde test without Value conversion
    println!("\n--- DIRECT SERDE TEST ---");
    match yyaml::parse_str::<BTreeMap<String, i32>>(test_yaml) {
        Ok(result) => {
            println!("✅ Direct serde parsing succeeded: {result:?}");
        }
        Err(e) => {
            println!("❌ Direct serde parsing failed: {e}");
        }
    }
}