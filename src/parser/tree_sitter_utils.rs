// src/parser/tree_sitter_utils.rs
use tree_sitter::Node;

/// Gets the source code string for a given node.
pub fn get_node_source<'a>(node: &Node, source: &'a [u8]) -> &'a str {
    std::str::from_utf8(&source[node.byte_range()]).unwrap_or("")
}
