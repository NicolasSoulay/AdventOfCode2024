use regex::Regex;

pub fn uncorrupt(input_path: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut operations = vec![];

    // use regex to find all operations
    for line in std::fs::read_to_string(input_path).unwrap().lines() {
        let line = line.to_string();
        for cap in re.captures_iter(&line) {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            operations.push((a, b));
        }
    }

    (0..operations.len()).for_each(|i| result += mul(operations[i].0, operations[i].1));

    result
} 

fn mul(a: i32, b: i32) -> i32 {
    a * b
}
