use yyaml;

fn main() {
    let yaml = r#"- provider: huggingface
  model: llama"#;
    
    println!("Testing YAML array parsing:");
    println!("Input: {:?}", yaml);
    
    match yyaml::YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Parsed successfully:");
            for (i, doc) in docs.iter().enumerate() {
                println!("  Document {}: {:?}", i, doc);
                match doc {
                    yyaml::Yaml::Array(arr) => {
                        println!("  Found Array with {} items", arr.len());
                        for (j, item) in arr.iter().enumerate() {
                            println!("    Item {}: {:?}", j, item);
                        }
                    },
                    yyaml::Yaml::Hash(hash) => {
                        println!("  Found Hash with {} items", hash.len());
                        for (key, value) in hash.iter() {
                            println!("    Key: {:?} -> Value: {:?}", key, value);
                        }
                    },
                    other => println!("  Other type: {:?}", other),
                }
            }
        },
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}