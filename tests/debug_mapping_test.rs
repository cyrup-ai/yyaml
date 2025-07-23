#[cfg(test)]
mod tests {
    use yyaml::{YamlLoader, from_str};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Data {
        pub substructure: yyaml::Mapping,
    }

    #[test]
    fn debug_mapping_parse() {
        let yaml = r#"
substructure:
  a: 'foo'
  b: 'bar'
"#;
        
        println!("Raw YAML:");
        println!("{yaml}");
        
        // Parse with YamlLoader first to see raw structure
        let docs = YamlLoader::load_from_str(yaml).unwrap();
        println!("\nParsed with YamlLoader:");
        for (i, doc) in docs.iter().enumerate() {
            println!("Document {i}: {doc:#?}");
        }
        
        // Try to deserialize
        println!("\nTrying serde deserialization:");
        match from_str::<Data>(yaml) {
            Ok(data) => {
                println!("Successfully deserialized: {data:#?}");
                println!("Mapping has {} entries", data.substructure.len());
                for (key, value) in &data.substructure {
                    println!("  {key:#?} -> {value:#?}");
                }
            }
            Err(e) => {
                println!("Deserialization failed: {e:#?}");
            }
        }
    }
}