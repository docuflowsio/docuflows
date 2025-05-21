use tree_sitter::{Parser, Node};
use walkdir::WalkDir;
use std::fs;

pub fn extract_function_names_from_dir(dir: &str) -> Vec<String> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).expect("Error loading Rust grammar");

    let mut functions = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(source) = fs::read_to_string(entry.path()) {
            if let Some(tree) = parser.parse(&source, None) {
                let root_node = tree.root_node();
                visit_node(&source, root_node, &mut functions);
            }
        }
    }

    functions
}

fn visit_node(source: &str, node: Node, functions: &mut Vec<String>) {
    if node.kind() == "function_item" {
        if let Some(identifier) = node.child_by_field_name("name") {
            if let Ok(name) = identifier.utf8_text(source.as_bytes()) {
                functions.push(name.to_string());
            }
        }
    }

    for i in 0..node.child_count() {
        visit_node(source, node.child(i).unwrap(), functions);
    }
}