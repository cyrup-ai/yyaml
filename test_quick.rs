fn main() {
    let test_cases = ["0xF0", "+0xF0", "-0xF0", ".inf"];
    for case in &test_cases {
        let result = yyaml::yaml::Yaml::parse_str(case);
        println\!("{}: {:?}", case, result);
    }
}
EOF < /dev/null