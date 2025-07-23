#[test]
fn debug_alias_loader() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init()
        .ok();

    let yaml = r#"
first:
  &alias
  1
second:
  *alias
third: 3
"#;

    match yyaml::YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("Loaded documents: {:#?}", docs);
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {i}: {doc:#?}");
            }
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
}