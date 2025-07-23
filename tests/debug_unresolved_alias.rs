use yyaml::{YamlLoader, Yaml};

#[test]
fn debug_unresolved_alias() {
    let yaml = r#"first:
  &alias
  1
second:
  *alias
third: 3"#;
    
    println!("=== PARSING YAML ===");
    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("✅ Parsed {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("\n📄 Document {}: {:#?}", i, doc);
                
                // Look for any remaining Alias values
                check_for_aliases(doc, 0);
            }
        }
        Err(e) => println!("❌ Parsing failed: {}", e),
    }
}

fn check_for_aliases(yaml: &Yaml, depth: usize) {
    let indent = "  ".repeat(depth);
    match yaml {
        Yaml::Alias(id) => {
            println!("{}🚨 FOUND UNRESOLVED ALIAS: {}", indent, id);
        }
        Yaml::Hash(map) => {
            for (key, value) in map.iter() {
                println!("{}📋 Hash entry:", indent);
                println!("{}  Key:", indent);
                check_for_aliases(key, depth + 2);
                println!("{}  Value:", indent);
                check_for_aliases(value, depth + 2);
            }
        }
        Yaml::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                println!("{}📦 Array[{}]:", indent, i);
                check_for_aliases(item, depth + 1);
            }
        }
        Yaml::BadValue => {
            println!("{}❌ BAD VALUE", indent);
        }
        other => {
            println!("{}✅ {}: {:#?}", indent, type_name(other), other);
        }
    }
}

fn type_name(yaml: &Yaml) -> &'static str {
    match yaml {
        Yaml::Real(_) => "Real",
        Yaml::Integer(_) => "Integer", 
        Yaml::String(_) => "String",
        Yaml::Boolean(_) => "Boolean",
        Yaml::Array(_) => "Array",
        Yaml::Hash(_) => "Hash",
        Yaml::Alias(_) => "Alias",
        Yaml::Null => "Null",
        Yaml::BadValue => "BadValue",
    }
}