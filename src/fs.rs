extern crate alloc;

use crate::api::*;
use alloc::{borrow::ToOwned, string::String, vec::Vec};

#[derive(Default)]
pub struct RustNode {
    data: Vec<u8>,
}

pub enum Node {
    Directory(Vec<(String, Node)>),
    Vnode(RustNode),
}

impl RustNode {
    pub fn read(&self) -> &[u8] {
        self.data.as_slice()
    }
    pub fn write(&mut self, io: &UIO) {
        self.data.extend_from_slice(io.as_slice())
    }
}

impl Node {
    pub fn lookup(&self, path: &str) -> Option<&Node> {
        let (file, suffix) = path.split_once('/').unwrap_or(("", path));
        if file.is_empty() && !suffix.is_empty() {
            return Some(self);
        }
        match &self {
            Node::Vnode(_) => None,
            Node::Directory(vec) => vec
                .iter()
                .find(|(name, _)| name == file)
                .and_then(|(_, node)| node.lookup(suffix)),
        }
    }
    pub fn create(&mut self, path: &str) -> Option<&Node> {
        let (file, suffix) = path.split_once('/').unwrap_or(("", path));
        match self {
            Node::Vnode(_) => None,
            Node::Directory(vec) => {
                if file.is_empty() && !suffix.is_empty() {
                    vec.push((suffix.to_owned(), Node::Vnode(RustNode::default())));
                    vec.last().map(|(_, node)| node)
                } else {
                    vec.iter_mut()
                        .find(|(name, _)| name == file)
                        .and_then(|(_, node)| node.create(suffix))
                }
            }
        }
    }
    pub fn remove(&mut self, to_remove: &Node) -> Option<Node> {
        match self {
            Node::Vnode(_) => None,
            Node::Directory(vec) => {
                let pos = vec.iter().position(|(_, node)| {
                    core::ptr::eq(to_remove as *const Node, node as *const Node)
                });
                if let Some(pos) = pos {
                    Some(vec.swap_remove(pos).1)
                } else {
                    for (_, node) in vec {
                        if let Some(node) = node.remove(to_remove) {
                            return Some(node);
                        }
                    }
                    None
                }
            }
        }
    }
}
