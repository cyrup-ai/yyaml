use yyaml::*;

fn main() {
    let s = "hello: world
int: 42
bool: true
nulltest: ~";
    
    println\!("Input YAML:");
    println\!("{}", s);
    println\!("\nParsing...");
    
    let result = YamlLoader::load_from_str(s);
    match result {
        Ok(docs) => {
            println\!("Number of documents: {}", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println\!("Document {}: {:?}", i, doc);
            }
        }
        Err(e) => {
            println\!("Error: {:?}", e);
        }
    }
}
EOF < /dev/null