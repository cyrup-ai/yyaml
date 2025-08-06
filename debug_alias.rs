use yyaml::YamlLoader;

fn main() {
    // Simple circular alias test
    let yaml = r#"
aref: &aref *aref
"#;
    
    println!("About to parse simple circular alias...");
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Parsed successfully: {:?}", docs);
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}