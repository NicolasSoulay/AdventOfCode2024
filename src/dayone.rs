use std::io::BufRead;

pub fn distances(list_path: &str) -> i32 {
    let (mut first_list, mut second_list) = dispatch_lists(list_path);
    first_list.sort_unstable();
    second_list.sort_unstable();
    
    let mut sum = 0;
    for i in 0..first_list.len() {
        sum +=(first_list[i] - second_list[i]).abs();
    }

    sum
}

pub fn similarities(list_path: &str) -> i32 {
    let (first_list, second_list) = dispatch_lists(list_path);
    
    let mut result = 0;
    (0..first_list.len()).for_each(|i| {
        let mut match_count = 0;
        (0..second_list.len()).for_each(|j| {
            if first_list[i] == second_list[j] {
                match_count += 1;
            }
        });
        result += first_list[i] * match_count;
    });

    result
}

fn dispatch_lists(list_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_list = vec![];
    let mut second_list = vec![];

    let file = std::fs::File::open(list_path).unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split("   ");
        let first = split.next().unwrap().parse::<i32>().unwrap();
        let second = split.next().unwrap().parse::<i32>().unwrap();
        first_list.push(first);
        second_list.push(second);
    }

    (first_list, second_list)
}
