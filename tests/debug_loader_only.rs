use yyaml::YamlLoader;

#[test]
fn test_loader_only_tagged_sequence() {
    println!("Testing YamlLoader directly with tagged sequence...");
    
    let yaml = "vec: !wat\n  - 0";
    
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("SUCCESS: YamlLoader parsed {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
            }
        }
        Err(e) => {
            println!("FAILED: YamlLoader failed: {:?}", e);
        }
    }
}

#[test] 
fn test_loader_only_simple_sequence() {
    println!("Testing YamlLoader directly with simple sequence...");
    
    let yaml = "- 0";
    
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("SUCCESS: Simple sequence parsed {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
            }
        }
        Err(e) => {
            println!("FAILED: Simple sequence failed: {:?}", e);
        }
    }
}