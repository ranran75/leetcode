use leetcode::trie::Trie;

fn main() {
    let mut trie: Trie<32> = Trie::new();
    trie.add_word("abc".to_string());
    trie.add_word("abd".to_string());
    trie.add_word("mgc".to_string());
    let res = trie.get_all_word();
    println!("{:?}", res);
    let bool = trie.search("mgc");
    println!("{:?}", bool);
    let bool2 = trie.search("mgcd");
    println!("{:?}", bool2);
}
