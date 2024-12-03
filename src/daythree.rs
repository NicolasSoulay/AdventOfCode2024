use regex::Regex;

pub fn uncorrupt(input_path: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut operations = vec![];

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

pub fn uncorrupt_with_instructions(input_path: &str) -> i32 {
    let mut result = 0;

    let mut enabled_texts = vec![];
    let re_begin = Regex::new(r"^(.*?)don't\(\)").unwrap();
    let re_enabled = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap();
    let re_end = Regex::new(r"do\(\)(.*)").unwrap();

    let mut text_vec = vec![];
    for line in std::fs::read_to_string(input_path).unwrap().lines() {
        text_vec.push(line.to_string());
    }

    let mut compacted_text = text_vec.join("");

    for cap in re_begin.captures_iter(&compacted_text) {
        enabled_texts.push(cap[1].to_string());
    }
    compacted_text = compacted_text.replace(&enabled_texts[0], "");
    for cap in re_enabled.captures_iter(&compacted_text) {
        enabled_texts.push(cap[1].to_string());
    }
    for cap in re_end.captures_iter(&compacted_text) {
        let end_text = cap[1].to_string();
        if !end_text.contains("don't()"){
            enabled_texts.push(cap[1].to_string());
        }
    }


    let re_operations = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut operations = vec![];
    for cap in re_operations.captures_iter(&enabled_texts.join("")) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        operations.push((a, b));
    }

    (0..operations.len()).for_each(|i| result += mul(operations[i].0, operations[i].1));

    result
}


fn mul(a: i32, b: i32) -> i32 {
    a * b
}
