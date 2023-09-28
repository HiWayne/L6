use std::collections::HashMap;

pub struct TrieNode {
    children_map: HashMap<char, TrieNode>,
    pub is_end_of_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children_map: HashMap::new(),
            is_end_of_word: false,
        }
    }
}

pub trait TrieMethods {
    fn insert(&mut self, string: &str);
    fn exist(&self, string: &str) -> bool;
    fn search_prefix(&self, string: &str) -> Option<&TrieNode>;
    fn reduce_find(&self, char: &char) -> Option<&TrieNode>;
    fn clone(&self) -> TrieNode;
}

impl TrieMethods for TrieNode {
    fn insert(&mut self, string: &str) {
        let mut current_node = self;
        for char in string.chars() {
            current_node = current_node
                .children_map
                .entry(char)
                .or_insert(TrieNode::new());
        }
        current_node.is_end_of_word = true;
    }

    fn exist(&self, string: &str) -> bool {
        let mut current_node = self;
        for char in string.chars() {
            if let Some(node) = current_node.children_map.get(&char) {
                current_node = node;
            } else {
                return false;
            }
        }
        true
    }

    fn search_prefix(&self, string: &str) -> Option<&TrieNode> {
        let mut current_node = self;
        for char in string.chars() {
            if let Some(node) = current_node.children_map.get(&char) {
                current_node = node;
            } else {
                return None;
            }
        }
        return Some(current_node);
    }

    fn reduce_find(&self, char: &char) -> Option<&TrieNode> {
        return self.children_map.get(&char);
    }
    fn clone(&self) -> Self {
        let mut new_children_map: HashMap<char, TrieNode> = HashMap::new();
        for (key, value) in self.children_map.iter() {
            new_children_map.insert(*key, value.clone());
        }
        return TrieNode {
            children_map: new_children_map,
            is_end_of_word: self.is_end_of_word,
        };
    }
}
