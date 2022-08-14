fn main() {
    println!("{}", smallest_number("IIIDIDDD".to_string()));
}

fn smallest_number(pattern: String) -> String {
    let length = pattern.len();
    let minimum: u32 = 10u32.pow(length as u32);
    let maximum = 10u32.pow(length as u32 + 1) - 1;
    let ops = pattern
        .chars()
        .map(|c| match c {
            'I' => true,
            _ => false,
        })
        .collect::<Vec<bool>>();
    for i in minimum..=maximum {
        if is_match(i, &ops) {
            return i.to_string();
        }
    }
    return "".to_string();
}

// fn is_match(number: u32, ops: &Vec<bool>) -> bool {}
