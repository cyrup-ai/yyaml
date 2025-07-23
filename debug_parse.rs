use yyaml::{YamlLoader, Yaml};

fn main() {
    let yaml = "- plain nonàscii\n- 'single quoted'\n- \"double quoted\"";
    println!("Input YAML:\n{}", yaml);
    
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Parsed {} documents:", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {}: {:?}", i, doc);
            }
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}