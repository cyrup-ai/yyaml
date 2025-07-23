use yyaml::YamlLoader;

fn main() {
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

fn check_for_aliases(yaml: &yyaml::Yaml, depth: usize) {
    let indent = "  ".repeat(depth);
    match yaml {
        yyaml::Yaml::Alias(id) => {
            println!("{}🚨 FOUND UNRESOLVED ALIAS: {}", indent, id);
        }
        yyaml::Yaml::Hash(map) => {
            for (key, value) in map {
                println!("{}📋 Hash entry:", indent);
                println!("{}  Key:", indent);
                check_for_aliases(key, depth + 2);
                println!("{}  Value:", indent);
                check_for_aliases(value, depth + 2);
            }
        }
        yyaml::Yaml::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                println!("{}📦 Array[{}]:", indent, i);
                check_for_aliases(item, depth + 1);
            }
        }
        yyaml::Yaml::BadValue => {
            println!("{}❌ BAD VALUE", indent);
        }
        other => {
            println!("{}✅ {}: {:#?}", indent, type_name(other), other);
        }
    }
}

fn type_name(yaml: &yyaml::Yaml) -> &'static str {
    match yaml {
        yyaml::Yaml::Real(_) => "Real",
        yyaml::Yaml::Integer(_) => "Integer", 
        yyaml::Yaml::String(_) => "String",
        yyaml::Yaml::Boolean(_) => "Boolean",
        yyaml::Yaml::Array(_) => "Array",
        yyaml::Yaml::Hash(_) => "Hash",
        yyaml::Yaml::Alias(_) => "Alias",
        yyaml::Yaml::Null => "Null",
        yyaml::Yaml::BadValue => "BadValue",
    }
}