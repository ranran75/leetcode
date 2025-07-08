use leetcode::trie::Trie;

fn main() {
    let mut trie: Trie<32> = Trie::new();
    trie.add_word("abc".to_string());
    trie.add_word("abd".to_string());
    trie.add_word("mgc".to_string());
    let test_res1 = trie.get_all_word();
    println!("{:?}", test_res1);
    let test_res2 = trie.search("mgc");
    println!("{:?}", test_res2);
    let test_res3 = trie.search("mgcd");
    println!("{:?}", test_res3);
}
