use yyaml::YamlLoader;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
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
            println!("Loaded documents: {:#?}", docs);
        }
        Err(e) => {
            eprintln!("Error: {e:?}");
        }
    }
}