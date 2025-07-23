use yyaml::parser::loader::YamlReceiver;

fn main() {
    let yaml = r#"- provider"#;
    
    println!("Input YAML:");
    println!("{}", yaml);
    println!();
    
    // Test the direct YamlReceiver approach
    let mut parser = yyaml::parser::Parser::new(yaml.chars());
    let mut loader = YamlReceiver::new();
    
    println!("Loading with YamlReceiver...");
    match parser.load(&mut loader, false) {
        Ok(()) => {
            println!("Load completed successfully");
            println!("Documents: {:#?}", loader.docs);
        }
        Err(e) => {
            println!("Load failed: {}", e);
        }
    }
}