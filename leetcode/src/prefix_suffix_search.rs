// 745. Prefix and Suffix Search
// https://leetcode.com/problems/prefix-and-suffix-search/

// Design a special dictionary with some words that searchs the words in it by a prefix and a suffix.

// Implement the WordFilter class:

// WordFilter(string[] words) Initializes the object with the words in the dictionary.
// f(string prefix, string suffix) Returns the index of the word in the dictionary, which has the prefix prefix and the suffix suffix.
// If there is more than one valid index, return the largest of them. If there is no such word in the dictionary, return -1.

// Example 1:
// Input
// ["WordFilter", "f"]
// [[["apple"]], ["a", "e"]]
// Output
// [null, 0]

// Explanation
// WordFilter wordFilter = new WordFilter(["apple"]);
// wordFilter.f("a", "e"); // return 0, because the word at index 0 has prefix = "a" and suffix = 'e".

// Constraints:

// 1 <= words.length <= 15000
// 1 <= words[i].length <= 10
// 1 <= prefix.length, suffix.length <= 10
// words[i], prefix and suffix consist of lower-case English letters only.
// At most 15000 calls will be made to the function f.

use std::collections::{HashMap, HashSet};

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

#[derive(Debug)]
struct WordFilter {
    words: Vec<String>,
    all_index: Vec<usize>,
    prefix: Vec<HashMap<char, HashSet<usize>>>,
    suffix: Vec<HashMap<char, HashSet<usize>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut prefix: Vec<HashMap<char, HashSet<usize>>> = vec![HashMap::new(); 10];
        let mut suffix: Vec<HashMap<char, HashSet<usize>>> = vec![HashMap::new(); 10];
        let mut all_index: Vec<usize> = Vec::with_capacity(words.len());

        (0..words.len()).for_each(|x| all_index.push(x));

        for (word_index, word) in words.iter().enumerate() {
            for (index, ch) in word.chars().enumerate() {
                prefix[index]
                    .entry(ch)
                    .and_modify(|s| {
                        s.insert(word_index);
                    })
                    .or_insert(set![word_index]);
            }

            for (index, ch) in word.chars().rev().enumerate() {
                suffix[index]
                    .entry(ch)
                    .and_modify(|s| {
                        s.insert(word_index);
                    })
                    .or_insert(set![word_index]);
            }
        }

        Self {
            words,
            all_index,
            prefix,
            suffix,
        }
    }

    fn f(&self, prefix: String, suffix: String) -> i32 {
        let mut probable_words: Vec<usize> = self.all_index.clone();

        for (index, chr) in prefix.chars().enumerate() {
            match self.prefix[index].get(&chr) {
                None => return -1,
                Some(s) => probable_words.retain(|index| s.contains(index)),
            }
        }

        for (index, chr) in suffix.chars().rev().enumerate() {
            match self.suffix[index].get(&chr) {
                None => return -1,
                Some(s) => probable_words.retain(|index| s.contains(index)),
            }
        }
        // println!("Probable words are {:?}", probable_words);

        *probable_words.iter().max().unwrap() as i32
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */

#[test]
fn test_word_filter() {
    let word_list = vec![
        "coffee".to_owned(),
        "mari".to_owned(),
        "maali".to_owned(),
        "one".to_owned(),
    ];
    let wf = WordFilter::new(word_list);
    println!("WordFilter is {:?}", wf.words);
    assert_eq!(wf.f("m".to_owned(), "i".to_owned()), 2);
}

#[test]
fn test_word_filter1() {
    // [[["cabaabaaaa","ccbcababac","bacaabccba","bcbbcbacaa","abcaccbcaa","accabaccaa","cabcbbbcca","ababccabcb","caccbbcbab","bccbacbcba"]],
    // ["bccbacbcba","a"],["ab","abcaccbcaa"],["a","aa"],["cabaaba","abaaaa"],["cacc","accbbcbab"],["ccbcab","bac"],["bac","cba"],["ac","accabaccaa"],["bcbb","aa"],["ccbca","cbcababac"]]
    // [null,9,4,5,0,8,1,2,5,3,1]
    let word_list = vec![
        "cabaabaaaa".to_owned(),
        "ccbcababac".to_owned(),
        "bacaabccba".to_owned(),
        "bcbbcbacaa".to_owned(),
        "abcaccbcaa".to_owned(),
        "accabaccaa".to_owned(),
        "cabcbbbcca".to_owned(),
        "ababccabcb".to_owned(),
        "caccbbcbab".to_owned(),
        "bccbacbcba".to_owned(),
    ];
    let wf = WordFilter::new(word_list);

    assert_eq!(wf.f("ccbca".to_owned(), "cbcababac".to_owned()), 1);
    assert_eq!(wf.f("bcbb".to_owned(), "aa".to_owned()), 3);
    assert_eq!(wf.f("ac".to_owned(), "accabaccaa".to_owned()), 5);
    assert_eq!(wf.f("a".to_owned(), "aa".to_owned()), 5);
    assert_eq!(wf.f("bccbacbcba".to_owned(), "a".to_owned()), 9);
    assert_eq!(wf.f("ab".to_owned(), "abcaccbcaa".to_owned()), 4);
    assert_eq!(wf.f("cabaaba".to_owned(), "abaaaa".to_owned()), 0);
}
