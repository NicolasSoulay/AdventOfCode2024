use std::io::BufRead;

pub fn dispatch(list_path: &str) -> (Vec<i32>, Vec<i32>) {
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
