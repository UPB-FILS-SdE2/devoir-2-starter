use lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process()
        .expect("Error writing LR parser");
}
