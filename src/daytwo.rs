use crate::list::dispatch;

pub fn daytwo(list_path: &str) -> i32 {
    let (first_list, second_list) = dispatch(list_path);
    
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
