use lazy_static::lazy_static;
use regex::Regex;

use crate::compiler::interfaces::{BaseNode, TemplateNode};

lazy_static! {
    pub static ref PATTERN: Regex = Regex::new(r"/^\s*svelte-ignore\s+([\s\S]+)\s*$/m").unwrap();
    pub static ref SPLIT: Regex = Regex::new(r"/[^\S]/").unwrap();
}

pub fn extract_svelte_ignore(text: &str) -> Vec<String> {
    PATTERN
        .find_iter(text)
        .map(|m| m.as_str())
        .nth(1)
        .map(|s| {
            SPLIT
                .split(s)
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![])
}

pub struct LeadingComment {
    pub value: String,
}

pub fn extract_svelte_ignore_from_comments(
    leading_comments: Option<&Vec<LeadingComment>>,
) -> Vec<String> {
    leading_comments
        .map(|comments| {
            comments
                .iter()
                .map(|comment| extract_svelte_ignore(&comment.value))
                .flatten()
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![])
}
pub fn extract_ignores_above_position(
    position: usize,
    template_node: Vec<TemplateNode>,
) -> Vec<String> {
    let previous_node_idx = template_node.iter().position(|child| {
        if let TemplateNode::BaseNode(node) = child {
            if node.end == position {
                return true;
            }
        }
        return false;
    });
}
