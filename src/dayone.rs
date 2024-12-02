use crate::list::dispatch;

pub fn dayone(list_path: &str) -> i32 {
    let (mut first_list, mut second_list) = dispatch(list_path);
    first_list.sort_unstable();
    second_list.sort_unstable();
    
    let mut sum = 0;
    for i in 0..first_list.len() {
        sum +=(first_list[i] - second_list[i]).abs();
    }

    sum
}


