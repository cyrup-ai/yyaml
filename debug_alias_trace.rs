use std::collections::BTreeMap;

fn main() {
    let yaml = "first:
  &alias
  1
second:
  *alias
third: 3";
    
    println!("=== YAML INPUT ===");
    println!("{}", yaml);
    println!();
    
    println!("=== STEP 1: YamlLoader::load_from_str ===");
    match yyaml::YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Successfully loaded {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
            }
        }
        Err(e) => {
            println!("Failed to load YAML: {:?}", e);
            return;
        }
    }
    
    println!();
    println!("=== STEP 2: yyaml::parse_str as BTreeMap ===");
    match yyaml::parse_str::<BTreeMap<String, i32>>(yaml) {
        Ok(result) => {
            println!("Successfully parsed as BTreeMap: {:?}", result);
        }
        Err(e) => {
            println!("Failed to parse as BTreeMap: {:?}", e);
        }
    }
    
    println!();
    println!("=== STEP 3: yyaml::parse_str as Value ===");
    match yyaml::parse_str::<yyaml::Value>(yaml) {
        Ok(result) => {
            println!("Successfully parsed as Value: {:?}", result);
        }
        Err(e) => {
            println!("Failed to parse as Value: {:?}", e);
        }
    }
}