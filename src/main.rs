use dayone::dayone;
use daytwo::daytwo;

mod list;
mod dayone;
mod daytwo;

fn main() {
    println!("Advent of Code 2024");

    let dayone_result = dayone("src/list");
    println!("Dayone result: {:?}", dayone_result);

    let daytwo_result = daytwo("src/list");
    println!("Daytwo result: {:?}", daytwo_result);
}
