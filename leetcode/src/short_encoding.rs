// 820. Short Encoding of Words
//
// https://leetcode.com/problems/short-encoding-of-words/
//
// A valid encoding of an array of words is any reference string s and array of indices indices such that:
//
// words.length == indices.length
// The reference string s ends with the '#' character.
// For each index indices[i], the substring of s starting from indices[i] and
//  up to (but not including) the next '#' character is equal to words[i].
// Given an array of words, return the length of the shortest reference
//  string s possible of any valid encoding of words.
//
// Example 1:
// Input: words = ["time", "me", "bell"]
// Output: 10
// Explanation: A valid encoding would be s = "time#bell#" and indices = [0, 2, 5].
// words[0] = "time", the substring of s starting from indices[0] = 0 to the next '#' is underlined in "time#bell#"
// words[1] = "me", the substring of s starting from indices[1] = 2 to the next '#' is underlined in "time#bell#"
// words[2] = "bell", the substring of s starting from indices[2] = 5 to the next '#' is underlined in "time#bell#"
//
// Example 2:
// Input: words = ["t"]
// Output: 2
// Explanation: A valid encoding would be s = "t#" and indices = [0].
//
// Constraints:
// 1 <= words.length <= 2000
// 1 <= words[i].length <= 7
// words[i] consists of only lowercase letters.

struct Solution;

trait ToIndex {
    fn to_index(self) -> usize;
}

impl ToIndex for char {
    fn to_index(self) -> usize {
        (self as u8 - 'a' as u8) as usize
    }
}

#[derive(Debug, Clone)]
struct TrieNode {
    ch: char,
    child: Vec<Option<Box<TrieNode>>>,
    leaf: bool,
}

impl TrieNode {
    fn new(ch: char) -> Self {
        let child = vec![None; 26];
        Self {
            ch,
            child,
            leaf: true,
        }
    }
}

#[derive(Debug, Clone)]
struct Trie {
    head: Box<TrieNode>,
    nodes: i32,
    leaves: i32,
}

impl Trie {
    fn new() -> Self {
        let mut head = Box::new(TrieNode::new('a'));
        head.leaf = false;
        Self {
            head,
            nodes: 0,
            leaves: 0,
        }
    }

    fn add_word(&mut self, word: &str) {
        let mut current_node = &mut self.head;
        for ch in word.chars().rev() {
            let index = ch.to_index();
            if current_node.child[index].is_none() {
                let child_node = Box::new(TrieNode::new(ch));
                current_node.child[index] = Some(child_node);
                self.nodes += 1;
                self.leaves += 1;

                if current_node.leaf {
                    current_node.leaf = false;
                    self.leaves -= 1;
                }
            }

            current_node = current_node.child[index].as_mut().unwrap();
        }
    }

    fn traverse(&self) -> i32 {
        let mut result = 0;
        let mut stack = vec![];
        stack.push((&self.head, 0));

        while !stack.is_empty() {
            let (curr, depth) = stack.pop().unwrap();

            if curr.leaf {
                result += depth;
            }

            for node in curr.child.iter().filter(|v| v.is_some()) {
                let child = node.as_ref().unwrap();
                stack.push((child, depth + 1));
            }
        }
        result
    }
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = Trie::new();

        for word in words {
            trie.add_word(&word);
        }
        trie.traverse() + trie.leaves
    }
}

#[test]
fn test_char_toindex() {
    let a = 'a';
    let b = 'b';
    let z = 'z';
    assert_eq!(a.to_index(), 0);
    assert_eq!(b.to_index(), 1);
    assert_eq!(z.to_index(), 25);
}

#[test]
fn test_trie_node() {
    let mut a = TrieNode::new('a');
    assert_eq!(a.ch, 'a');
    assert_eq!(a.leaf, false);
    assert_eq!(a.child.len(), 26);
    assert!(a.child[0].is_none());

    println!("TrieNode is {:?}", a);
    a.child[0] = Some(Box::new(TrieNode::new('a')));
    assert!(!a.child[0].is_none());
    println!("TrieNode is {:?}", a);
}

#[test]
fn test_trie() {
    let mut trie = Trie::new();
    trie.add_word("take");
    trie.add_word("ke");
    println!("Trie is {:?}", trie)
}

#[test]
fn test_minimum_encoding() {
    let words = vec!["take".to_owned(), "ke".to_owned()];
    assert_eq!(Solution::minimum_length_encoding(words), 5);
}
