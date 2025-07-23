extern crate yyaml;
use yyaml::scanner::Scanner;

fn main() {
    let yaml = "- provider: openai\n  models:\n    - name: test";
    println!("=== Token analysis ===");
    println!("Input YAML:");
    println!("{}", yaml);
    println!("\nTokens:");
    
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