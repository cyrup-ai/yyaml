use yyaml::YamlLoader;

fn main() {
    // Test circular alias
    let yaml = r#"
aref: &aref *aref
"#;
    
    println!("Testing circular alias...");
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Parsed successfully: {:?}", docs);
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }

    // Test simple billion laughs
    let yaml2 = r#"
a: &a ~
b: &b [*a,*a,*a,*a,*a,*a,*a,*a,*a]
c: &c [*b,*b,*b,*b,*b,*b,*b,*b,*b]
"#;
    
    println!("\nTesting billion laughs...");
    match YamlLoader::load_from_str(yaml2) {
        Ok(docs) => {
            println!("Parsed successfully: {:?}", docs);
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
        }
    }
}