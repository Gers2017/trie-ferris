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

    pub fn get_mut(&mut self, key: &char) -> Option<&mut TNode> {
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
        if word.is_empty() {
            return;
        }

        let mut node = &mut self.root;

        for current in word.chars() {
            let next_node = node
                .children
                .entry(current)
                .or_insert_with(|| TNode::new(current, false));

            node = next_node;
        }

        node.is_end = true;
    }

    pub fn insert(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }

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
        let mut node = &self.root;

        for current in word.chars() {
            if let Some(next_node) = node.children.get(&current) {
                node = next_node;
            } else {
                return false;
            }
        }

        node.is_end
    }

    pub fn delete(&mut self, word: &str) {
        let node = &mut self.root;
        let word: Vec<_> = word.chars().collect();
        Self::delete_rec(node, &word, 0);
    }

    pub fn delete_2(&mut self, word: &str) {
        if word.is_empty() {
            return;
        }

        let word = word.chars();
        Self::deleto_rec(&mut self.root, word);
    }

    fn delete_rec(node: &mut TNode, word: &[char], depth: usize) -> bool {
        if depth > word.len() {
            return false;
        }

        if depth == word.len() {
            if node.is_end {
                node.is_end = false;
            }

            return node.is_empty();
        }

        let current = word[depth];

        if let Some(next) = node.get_mut(&current) {
            if Self::delete_rec(next, word, depth + 1) {
                node.children.remove(&current);
            }

            return node.is_empty();
        }

        false
    }

    fn deleto_rec(node: &mut TNode, mut word: std::str::Chars<'_>) -> bool {
        let maybe_next = word
            .next()
            .and_then(|c| node.get_mut(&c).and_then(|next| Some((c, next))));

        if let Some((current_ch, next_node)) = maybe_next {
            if Self::deleto_rec(next_node, word) {
                // post traversal
                node.children.remove(&current_ch);
            }

            return node.is_empty();
        }

        if node.is_end {
            node.is_end = false;
        }

        node.is_empty()
    }

    pub fn clear(&mut self) {
        self.root.children.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORDS: [&'static str; 7] = ["coal", "cat", "cin", "catch", "cut", "cit", "camp"];

    #[test]
    fn integration_test() {
        let mut trie = Trie::new();
        assert_eq!(trie.root.is_end, false);

        trie.insert("");

        assert_eq!(trie.contains("\0"), false);
        assert_eq!(trie.root.is_end, false);

        for (i, w) in WORDS.iter().enumerate() {
            if i % 2 == 0 {
                trie.insert(*w);
            } else {
                trie.insert_iter(*w);
            }
        }

        for w in WORDS.iter() {
            assert!(trie.contains(*w), "should contain \"{}\"", &w);
        }

        assert_eq!(trie.contains("ca"), false, "shouldn't contain \"ca\"");
        assert_eq!(trie.contains("ci"), false, "shouldn't contain \"ci\"");
        assert_eq!(trie.contains("co"), false, "shouldn't contain \"co\"");

        // println!("{:#?}", trie.root);

        trie.delete("cat");
        assert_eq!(trie.contains("cat"), false);
        assert_eq!(trie.contains("catch"), true);

        trie.delete("coal");
        assert_eq!(trie.contains("coal"), false);
        assert_eq!(trie.contains("cut"), true);
        assert_eq!(trie.contains("catch"), true);

        trie.clear();

        for w in WORDS.iter() {
            assert_eq!(trie.contains(*w), false);
        }
        // println!("{:#?}", trie.root);
    }

    #[test]
    fn test_deleto() {
        let mut trie_me = Trie::new();

        trie_me.insert_iter("null");
        trie_me.insert_iter("none");
        trie_me.insert_iter("nope");
        trie_me.insert_iter("nine");

        trie_me.delete_2("null");
        trie_me.delete_2("none");
        assert_eq!(trie_me.contains("null"), false);
        assert_eq!(trie_me.contains("none"), false);
    }
}
