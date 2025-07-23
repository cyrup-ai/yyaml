use yyaml::YamlLoader;

fn main() {
    let yaml = r#"
substructure:
  a: 'foo'
  b: 'bar'
"#;
    
    let docs = YamlLoader::load_from_str(yaml).unwrap();
    println!("Parsed YAML structure:");
    for (i, doc) in docs.iter().enumerate() {
        println!("Document {}: {:#?}", i, doc);
    }
    
    if let Some(doc) = docs.first() {
        if let yyaml::Yaml::Hash(ref hash) = doc {
            for (key, value) in hash {
                println!("Key: {:#?}", key);
                println!("Value: {:#?}", value);
                if let yyaml::Yaml::Hash(ref subhash) = value {
                    println!("Subhash has {} entries:", subhash.len());
                    for (subkey, subvalue) in subhash {
                        println!("  {:#?} -> {:#?}", subkey, subvalue);
                    }
                }
            }
        }
    }
}