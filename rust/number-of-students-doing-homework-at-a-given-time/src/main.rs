fn main() {
    let start_time = vec![1, 2, 3];
    let end_time = vec![3, 2, 7];
    let query_time = 4;
    println!("{}", busy_student(start_time, end_time, query_time));

    println!("{}", busy_student(vec![4], vec![4], 4));
}

pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    start_time
        .into_iter()
        .zip(end_time.into_iter())
        .map(|(start, end)| {
            if start <= query_time && query_time <= end {
                1
            } else {
                0
            }
        })
        .sum()
}
