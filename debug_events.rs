extern crate yyaml;
use yyaml::parser::Parser;

fn main() {
    println!("=== Parser Event Trace ===");
    
    let yaml = "- key: value";
    println!("Input: {:?}", yaml);
    println!("Events:");
    
    let mut parser = Parser::new(yaml.chars());
    
    loop {
        match parser.next() {
            Ok((event, marker)) => {
                println!("{:?} at {:?}", event, marker);
                if matches!(event, yyaml::events::Event::StreamEnd) {
                    break;
                }
            }
            Err(e) => {
                println!("❌ Error: {:?}", e);
                break;
            }
        }
    }
}