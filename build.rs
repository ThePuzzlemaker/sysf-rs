fn main() {
    lalrpop::Configuration::new()
        .process_file("src/grammar.lalrpop")
        .unwrap();
}
