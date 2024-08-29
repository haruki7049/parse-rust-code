fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use tree_sitter::{InputEdit, Language, Parser, Point};

    #[test]
    fn tree_sitter() {
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_rust::language()).expect("Error loading Rust grammar");

        let source_code = "fn test() {}";
        let mut tree = parser.parse(source_code, None).unwrap();
        let root_node = tree.root_node();

        assert_eq!(root_node.kind(), "source_file");
        assert_eq!(root_node.start_position().column, 0);
        assert_eq!(root_node.end_position().column, 12);
    }
}
