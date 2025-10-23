use yyaml::YamlLoader;

#[test]
fn test_bom_loader_only() {
    println!("=== Testing YamlLoader directly ===");

    // Test without BOM
    let yaml_no_bom = "- 0";
    println!("Without BOM: {:?}", yaml_no_bom);
    match YamlLoader::load_from_str(yaml_no_bom) {
        Ok(docs) => {
            println!("  Success: {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("  Doc {}: {:?}", i, doc);
            }
            assert_eq!(docs.len(), 1);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
            panic!("Expected success for non-BOM case");
        }
    }

    // Test with BOM
    let yaml_with_bom = "\u{feff}- 0";
    println!("With BOM: {:?}", yaml_with_bom);
    match YamlLoader::load_from_str(yaml_with_bom) {
        Ok(docs) => {
            println!("  Success: {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("  Doc {}: {:?}", i, doc);
            }
            assert_eq!(docs.len(), 1);
        }
        Err(e) => {
            println!("  Error: {:?}", e);
            panic!("BOM case should succeed after fix: {:?}", e);
        }
    }
}