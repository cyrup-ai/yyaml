use yyaml::events::{Event, TokenType};
use yyaml::parser::{Parser, loader::YamlLoader};
use yyaml::scanner::Scanner;

#[test]
fn trace_multiline_mapping() {
    let yaml = "hello: world\nint: 42";
    println!("Parsing YAML: {yaml:?}");

    // First, let's trace the scanner tokens
    println!("\n=== Scanner Token Trace ===");
    let mut scanner = Scanner::new(yaml.chars());

    loop {
        match scanner.peek_token() {
            Ok(token) => {
                println!("Token at {}:{} - {:?}", token.0.line, token.0.col, token.1);

                if matches!(token.1, TokenType::StreamEnd) {
                    break;
                }

                // Consume the token
                scanner.fetch_token();
            }
            Err(e) => {
                println!("Scanner error: {e:?}");
                break;
            }
        }
    }

    // Now let's trace the parser events
    println!("\n=== Parser Event Trace ===");
    let mut parser = Parser::new(yaml.chars());

    loop {
        match parser.next() {
            Ok(event) => {
                println!("Event: {event:?}");

                if matches!(event.0, Event::StreamEnd) {
                    break;
                }
            }
            Err(e) => {
                println!("Parser error: {e:?}");
                break;
            }
        }
    }

    // Finally, let's use YamlLoader which is the high-level API
    println!("\n=== YamlLoader Result ===");
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Success! Loaded {} document(s)", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {i}: {doc:?}");
            }
        }
        Err(e) => {
            println!("YamlLoader error: {e:?}");
        }
    }
}
