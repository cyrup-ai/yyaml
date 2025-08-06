use yyaml; fn main() { let result = yyaml::parse_str::<Vec<i32>>("\u{feff}- 0\n"); println!("Debug: {:?}", result); }
