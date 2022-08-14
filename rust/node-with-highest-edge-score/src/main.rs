fn main() {
    println!("{}", edge_score(vec![1, 0, 0, 0, 0, 7, 7, 5]));
    println!("{}", edge_score(vec![2, 0, 0, 2]));
}

fn edge_score(edges: Vec<i32>) -> i32 {
    let n = edges.len();
    let mut count = vec![0; n];
    edges.iter().enumerate().for_each(|(from, &to)| {
        count[to as usize] += from;
    });
    // println!("{:?}", count);

    let mut max_score = count[0];
    let mut max_score_index: usize = 0;
    for i in 1..n {
        if count[i] > max_score {
            max_score = count[i];
            max_score_index = i;
        }
    }
    // println!("{} {}", max_score, max_score_index);
    max_score_index as i32
}
