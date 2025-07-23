use yyaml::YamlLoader;

#[test]
fn debug_event_sequence() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
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

    match YamlLoader::load_from_str(yaml) {
        Ok(docs) => {
            println!("SUCCESS: Loaded {} documents", docs.len());
            for (i, doc) in docs.iter().enumerate() {
                println!("Document {i}: {doc:#?}");
            }
        }
        Err(e) => {
            println!("ERROR: {e:?}");
        }
    }
}