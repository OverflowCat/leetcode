use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        println!("{:?}", words.len());
        return vec![];
        let mut m: HashMap<String, usize> = HashMap::new();
        let mut answers = vec![];
        let empty_string = &String::from("");
        words.iter().enumerate().for_each(|(i, word)| {
            m.insert(word.clone(), i); // words are guaranteed unique
        });
        let empty_string_i = m.get(empty_string);
        words.iter().enumerate().for_each(|(i, word)| {
            let rev = word.chars().rev().collect::<String>();
            if let Some(&other) = m.get(&rev) {
                if other != i {
                    answers.push(vec![i as i32, other as i32]);
                }
            }
            if word.len() != 0 && rev == *word && empty_string_i.is_some() {
                answers.push(vec![i as i32, (*empty_string_i.unwrap()) as i32]);
                answers.push(vec![(*empty_string_i.unwrap()) as i32, i as i32]);
            }
            let word = word.as_str();
            for j in 1..word.len() {
                let (pre, post) = (&word[..j], &word[j..]);
                let pre_rev = pre.chars().rev().collect::<String>();
                let post_rev = post.chars().rev().collect::<String>();
                if pre_rev == pre {
                    // fed, [aba]/def
                    if let Some(&other) = m.get(&post_rev) {
                        answers.push(vec![other as i32, i as i32]);
                    }
                }
                if post_rev == post {
                    // abc/ded, cba
                    if let Some(&other) = m.get(&pre_rev) {
                        answers.push(vec![i as i32, other as i32]);
                    }
                }
            }
        });
        answers
    }
}

struct Solution {}

fn main() {
    let res = 
    Solution::palindrome_pairs(vec![

println!("{:?}", res);
}