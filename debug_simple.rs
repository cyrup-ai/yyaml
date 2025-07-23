use yyaml::YamlLoader;

fn main() {
    println!("=== Testing simple YAML sequence parsing ===");
    
    // Test 1: Simple sequence
    let yaml1 = "- item1\n- item2";
    println!("\nTest 1 - Simple sequence:");
    println!("Input: {:?}", yaml1);
    match YamlLoader::load_from_str(yaml1) {
        Ok(docs) => {
            println!("✅ Parsed {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
                if let Some(seq) = doc.as_vec() {
                    println!("  - As sequence: {} items", seq.len());
                    for (j, item) in seq.iter().enumerate() {
                        println!("    [{}]: {:?}", j, item);
                    }
                } else {
                    println!("  - NOT a sequence! Type: {:?}", doc);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
    
    // Test 2: Sequence with mapping
    let yaml2 = "- key: value\n- key2: value2";
    println!("\nTest 2 - Sequence with mappings:");
    println!("Input: {:?}", yaml2);
    match YamlLoader::load_from_str(yaml2) {
        Ok(docs) => {
            println!("✅ Parsed {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
                if let Some(seq) = doc.as_vec() {
                    println!("  - As sequence: {} items", seq.len());
                } else {
                    println!("  - NOT a sequence!");
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}