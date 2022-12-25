pub struct TrieNode<const N: usize>(Box<[Option<TrieNode<N>>; N]>, bool);
pub struct Trie<const N: usize>(TrieNode<N>);

impl<const N: usize> TrieNode<N> {
    fn new() -> Self {
        return TrieNode(Box::new(core::array::from_fn(|_| None)), false);
    }
}

impl<const N: usize> Trie<N> {
    pub fn new() -> Self {
        Trie(TrieNode::new())
    }
    pub fn add_word(&mut self, word: String) {
        let mut cur = &mut self.0;
        for c in word.bytes() {
            let index = (c - b'a') as usize;
            if cur.0[index].is_none() {
                cur.0[index] = Some(TrieNode::new());
            }
            cur = cur.0[index].as_mut().unwrap();
        }
        cur.1 = true;
    }

    pub fn get_all_word(&self) -> Vec<String> {
        let root = &self.0;
        let mut result = Vec::new();
        Self::dfs(root, "".to_string(), &mut result);
        result
    }

    pub fn search(&self, word: &str) -> bool {
        let mut res = false;
        let mut reach_tail = false;
        let mut cur = &self.0;
        for (i, c) in word.bytes().enumerate() {
            let index = (c - b'a') as usize;
            if let Some(next) = &cur.0[index] {
                cur = next;
            } else {
                break;
            }
            if i == word.len() - 1 {
                reach_tail = true;
            }
        }
        if reach_tail && cur.1 {
            res = true;
        }
        res
    }
    fn dfs(cur: &TrieNode<N>, s: String, result: &mut Vec<String>) {
        match cur.1 {
            true => {
                result.push(s.clone());
            }
            false => {}
        }
        cur.0.iter().enumerate().for_each(|(index, x)| {
            if let Some(child) = x {
                let mut new_str = s.clone();
                let c = char::from_u32((index as u8 + b'a') as u32).unwrap();
                new_str.push(c);
                Self::dfs(child, new_str, result)
            }
        })
    }
}
