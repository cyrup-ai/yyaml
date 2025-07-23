extern crate yyaml;
use yyaml::scanner::Scanner;

fn main() {
    let yaml = "- key: value";
    println!("=== Detailed Token Analysis ===");
    println!("Input: {:?}", yaml);
    println!("Length: {} chars", yaml.len());
    for (i, c) in yaml.chars().enumerate() {
        println!("  [{}]: {:?}", i, c);
    }
    
    println!("\nTokens with positions:");
    let mut scanner = Scanner::new(yaml.chars());
    
    loop {
        match scanner.peek_token() {
            Ok(token) => {
                println!("{:?}", token);
                if matches!(token.1, yyaml::events::TokenType::StreamEnd) {
                    break;
                }
                let _ = scanner.fetch_token();
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}