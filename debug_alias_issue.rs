use std::collections::BTreeMap;

fn main() {
    let yaml = indoc::indoc! {"
        first:
          &alias
          1
        second:
          *alias
        third: 3
    "};
    
    println!("YAML to parse:");
    println!("{}", yaml);
    println!("---");
    
    // Try to parse as YAML documents first
    match yyaml::YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Successfully loaded {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
            }
        }
        Err(e) => {
            println!("Failed to load YAML: {:?}", e);
        }
    }
    
    println!("---");
    
    // Try to parse as serde BTreeMap
    match yyaml::parse_str::<BTreeMap<String, i32>>(yaml) {
        Ok(result) => {
            println!("Successfully parsed as BTreeMap: {:?}", result);
        }
        Err(e) => {
            println!("Failed to parse as BTreeMap: {:?}", e);
        }
    }
}