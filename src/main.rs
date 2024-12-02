use dayone::distances;
use dayone::similarities;

mod dayone;
mod daytwo;

fn main() {
    println!("Advent of Code 2024");

    println!("Day one distances result: {:?}", distances("src/list"));

    println!("Day one similarities result: {:?}", similarities("src/list"));
}
