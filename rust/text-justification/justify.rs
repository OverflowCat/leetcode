pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max_width = max_width as usize;
    let mut res: Vec<String> = Vec::new();
    let mut current_length = 0usize;
    let mut linequeue: Vec<&String> = Vec::new();
    for word in words.iter() {
        println!(
            "word is {}, current_length is {}, linequeue has {}",
            word,
            current_length,
            linequeue.len()
        );
        if current_length + word.len() > max_width {
            let lengths = linequeue.iter().map(|x| x.len());
            let sum = lengths.sum::<usize>();
            let spaces_count = linequeue.len() - 1;
            let min_length = sum + spaces_count;
            if min_length == max_width {
                let mut s: String = String::new();
                for (i, word) in linequeue.iter().enumerate() {
                    s.push_str(word);
                    if i != linequeue.len() - 1 {
                        s.push(' ');
                    }
                }
                res.push(s);
            } else {
                if linequeue.len() == 1 {
                    res.push(linequeue[0].clone() + &" ".repeat(max_width - linequeue[0].len()));
                } else {
                    let diff = max_width - sum;
                    let spaces_between_each_word = diff / spaces_count;
                    let extra_spaces = diff % spaces_count;
                    let mut i = 0;
                    let mut s = String::from("");
                    let len = linequeue.len();
                    for word in linequeue.iter() {
                        s.push_str(word);
                        if i == len - 1 {
                            break;
                        }
                        for _ in 0..spaces_between_each_word {
                            s.push_str(" ");
                        }
                        if i < extra_spaces {
                            s.push_str(" ");
                            i += 1;
                        }
                    }
                    res.push(s);
                }
            }
            current_length = 0;
            linequeue.clear();
            linequeue.push(word);
        } else {
            current_length += word.len() + 1;
            linequeue.push(&word);
        }
    }
    if linequeue.len() != 0 {
        let mut s: String = String::new();
        for (i, word) in linequeue.iter().enumerate() {
            s.push_str(word);
            if i != linequeue.len() - 1 {
                s.push(' ');
            }
        }
        if s.len() < max_width {
            s.push_str(&" ".repeat(max_width - s.len()));
        }
        res.push(s);
    }
    res
}

fn main() {
    let words = vec![
        "This",
        "is",
        "an",
        "example",
        "of",
        "text",
        "justification.",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let max_width = 16;
    let res = full_justify(words, max_width);
    println!("{:?}", res);
}
