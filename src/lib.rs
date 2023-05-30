use std::collections::HashMap;

#[derive(Debug)]
pub struct TNode {
    pub value: char,
    pub is_end: bool,
    pub children: HashMap<char, TNode>,
}

impl TNode {
    pub fn new(value: char, is_end: bool) -> Self {
        Self {
            value,
            is_end,
            children: Default::default(),
        }
    }

    pub fn get(&mut self, key: &char) -> Option<&mut TNode> {
        self.children.get_mut(key)
    }

    pub fn has(&self, ch: &char) -> bool {
        self.children.contains_key(&ch)
    }

    pub fn is_empty(&self) -> bool {
        !self.is_end && self.children.is_empty()
    }
}

pub struct Trie {
    pub root: TNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TNode::new('\0', false),
        }
    }

    pub fn insert_iter(&mut self, word: &str) {
        let mut node = &mut self.root;

        for (i, current) in word.chars().enumerate() {
            let next_node = node.children.entry(current).or_insert_with(|| {
                let is_end = i == word.len() - 1;
                TNode::new(current, is_end)
            });

            node = next_node;
        }
    }

    pub fn insert(&mut self, word: &str) {
        let node = &mut self.root;
        let word = word.chars();
        Trie::insert_rec(node, word);
    }

    fn insert_rec(node: &mut TNode, mut word: std::str::Chars<'_>) {
        if let Some(current_ch) = word.next() {
            let next_node = node
                .children
                .entry(current_ch)
                .or_insert_with(|| TNode::new(current_ch, false));

            Trie::insert_rec(next_node, word);
        } else {
            node.is_end = true;
        }
    }

    pub fn contains(&mut self, word: &str) -> bool {
        let mut node = &mut self.root;

        for (i, current) in word.chars().enumerate() {
            if let Some(next_node) = node.children.get_mut(&current) {
                if next_node.is_end && i == word.len() - 1 {
                    return true;
                }

                node = next_node;
            } else {
                return false;
            }
        }

        return false;
    }

    pub fn delete(&mut self, word: &str) {
        let node = &mut self.root;
        let word: Vec<_> = word.chars().collect();
        Self::delete_rec(node, &word, 0);
    }

    fn delete_rec(node: &mut TNode, word: &[char], depth: usize) -> bool {
        if depth > word.len() {
            return false;
        }

        if depth == word.len() {
            if node.is_end {
                node.is_end = false;
            }

            if node.is_empty() {
                return true; // delete key
            }

            return false;
        }

        let current = word[depth];

        if !node.has(&current) {
            return false;
        }

        let next = node.get(&current).unwrap();

        if Self::delete_rec(next, word, depth + 1) {
            node.children.remove(&current);
        }

        if node.is_empty() {
            return true; // delete key
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_test() {
        let mut trie = Trie::new();
        let words = ["coal", "cat", "cin", "catch", "cut", "cit", "cap"];
        for w in words.iter() {
            trie.insert(*w);
        }

        for w in words.iter() {
            assert!(trie.contains(*w), "should contain \"{}\"", &w);
        }

        assert!(!trie.contains("ca"), "shouldn't contain \"co\"");
        assert!(!trie.contains("ci"), "shouldn't contain \"worm\"");
        assert!(!trie.contains("co"), "shouldn't contain \"co\"");

        // println!("{:#?}", trie.root);

        trie.delete("cat");
        assert_eq!(trie.contains("cat"), false);
        assert_eq!(trie.contains("catch"), true);

        trie.delete("coal");
        assert_eq!(trie.contains("coal"), false);
        trie.delete("coal");

        // println!("{:#?}", trie.root);
    }

    #[test]
    fn insert_iter_test() {
        let mut trie = Trie::new();
        let words = ["coal", "cat", "cam", "calm", "cut", "camp"];
        for w in words.iter() {
            trie.insert_iter(*w);
        }

        for w in words.iter() {
            assert!(trie.contains(*w), "should contain \"{}\"", &w);
        }

        trie.delete("cat");
        assert_eq!(trie.contains("cat"), false);
        assert_eq!(trie.contains("calm"), true);

        trie.delete("camp");
        assert_eq!(trie.contains("camp"), false);
        assert_eq!(trie.contains("calm"), true);
    }
}
