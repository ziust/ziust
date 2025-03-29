use tree_sitter::{Parser, Tree};
use tree_sitter_ziust::LANGUAGE;

fn main() {
    let mut parser = Parser::new();
    parser.set_language(&LANGUAGE.into()).expect("Error loading Ziust grammar");

    let source_code = r#"
        let x = 42;
        print(x);
    "#;

    let tree: Tree = parser.parse(source_code, None).unwrap();
    println!("{:#?}", tree.root_node().to_sexp());
}
