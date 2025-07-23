use yyaml::YamlLoader;

fn main() {
    env_logger::init();
    
    let yaml = r#"- provider: git
  url: https://github.com/fluent/fluent.git"#;
    
    println!("Input YAML:");
    println!("{}", yaml);
    println!();
    
    match YamlLoader::load_from_str(yaml) {
        Ok(result) => {
            println!("Parsed result: {:#?}", result);
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}