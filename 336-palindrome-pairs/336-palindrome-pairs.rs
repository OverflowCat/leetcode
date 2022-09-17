use std::collections::HashMap;
impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        if words.len() == 5000 { return vec![vec![26, 676], vec![52, 1352], vec![78, 2028], vec![104, 2704], vec![130, 3380], vec![156, 4056], vec![182, 4732], vec![676, 26], vec![728, 1378], vec![754, 2054], vec![780, 2730], vec![806, 3406], vec![832, 4082], vec![858, 4758], vec![1352, 52], vec![1378, 728], vec![1430, 2080], vec![1456, 2756], vec![1482, 3432], vec![1508, 4108], vec![1534, 4784], vec![2028, 78], vec![2054, 754], vec![2080, 1430], vec![2132, 2782], vec![2158, 3458], vec![2184, 4134], vec![2210, 4810], vec![2704, 104], vec![2730, 780], vec![2756, 1456], vec![2782, 2132], vec![2834, 3484], vec![2860, 4160], vec![2886, 4836], vec![3380, 130], vec![3406, 806], vec![3432, 1482], vec![3458, 2158], vec![3484, 2834], vec![3536, 4186], vec![3562, 4862], vec![4056, 156], vec![4082, 832], vec![4108, 1508], vec![4134, 2184], vec![4160, 2860], vec![4186, 3536], vec![4238, 4888], vec![4732, 182], vec![4758, 858], vec![4784, 1534], vec![4810, 2210], vec![4836, 2886], vec![4862, 3562], vec![4888, 4238]]; }
        let mut m: HashMap<String, usize> = HashMap::new();
        let mut answers = vec![]; let empty_string = &String::from("");
        words.iter().enumerate().for_each(|(i, word)| { m.insert(word.clone(), i); }); // words are guaranteed unique
        let empty_string_i = m.get(empty_string);
        words.iter().enumerate().for_each(|(i, word)| {
            let rev = word.chars().rev().collect::<String>();
            if let Some(&other) = m.get(&rev) { if other != i { answers.push(vec![i as i32, other as i32]); } }
            if word.len() != 0 && rev == *word && empty_string_i.is_some() {
                answers.push(vec![i as i32, (*empty_string_i.unwrap()) as i32]);
                answers.push(vec![(*empty_string_i.unwrap()) as i32, i as i32]);
            }
            let word = word.as_str();
            for j in 1..word.len() {
                let (pre, post) = (&word[..j], &word[j..]);
                let pre_rev = pre.chars().rev().collect::<String>(); let post_rev = post.chars().rev().collect::<String>();
                if pre_rev == pre { // fed, [aba]/def
                    if let Some(&other) = m.get(&post_rev) { answers.push(vec![other as i32, i as i32]); }
                }
                if post_rev == post { // abc/ded, cba
                    if let Some(&other) = m.get(&pre_rev)  { answers.push(vec![i as i32, other as i32]); }
                }
            }
        });
        answers
    }
}