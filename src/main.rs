fn main() {
    let mut trie = Trie::new();

    let words = ["rust", "rustacean", "rusty", "ruby", "run", "react", "cat"];
    for word in &words {
        trie.insert(word);
    }

    println!("search('rust'): {}", trie.search("rust"));     // true
    println!("search('rus'):  {}", trie.search("rus"));      // false (단어 없음)
    println!("starts_with('rus'): {}", trie.starts_with("rus")); // true

    println!("\n'ru'로 시작하는 단어:");
    for word in trie.autocomplete("ru") {
        println!("  {}", word);
    }
    // run
    // rust
    // rustacean
    // rusty
    // ruby

    println!("\n'rust'로 시작하는 단어:");
    for word in trie.autocomplete("rust") {
        println!("  {}", word);
    }
    // rust
    // rustacean
    // rusty
}