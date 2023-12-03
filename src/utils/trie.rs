use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    value: Option<char>,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn set_value(&mut self, value: Option<char>) {
        self.value = value;
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn from(data: Vec<(&str, char)>) -> Self {
        let mut trie = Trie::new();

        for d in data {
            trie.insert(d.0, d.1);
        }

        trie
    }

    pub fn insert(&mut self, word: &str, value: char) {
        if self.contains(word).is_some() {
            return;
        }

        let mut node = &mut self.root;
        let chars = word.chars();

        for c in chars {
            node = node.children.entry(c).or_default();
            node.set_value(Some('\0'));
        }

        node.set_value(Some(value));
        return;
    }

    pub fn contains(&self, word: &str) -> Option<char> {
        let mut node = &self.root;
        let chars = word.chars();

        for c in chars {
            match node.children.get(&c) {
                Some(n) => node = n,
                _ => return None,
            }
        }

        return node.value;
    }
}


