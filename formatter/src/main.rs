fn main() {
    let source_code = r#"
        let x = 42;
        print(x);
    "#;

    ziust_ast::ast(source_code);
}
