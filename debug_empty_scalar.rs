use yyaml::{YamlLoader, Yaml};

fn main() {
    let yaml = "thing:\n";
    println!("Parsing YAML: {:?}", yaml);
    
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Successfully parsed {} document(s)", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
                if let Some(thing_value) = doc.as_hash() {
                    for (key, value) in thing_value.iter() {
                        println!("  Key: {:?}, Value: {:?}", key, value);
                    }
                }
            }
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}