use dayone::{distances, similarities};
use daytwo::{safety_report, safety_report_dampen};

mod dayone;
mod daytwo;

fn main() {
    println!("Advent of Code 2024");

    println!("Day one distances result: {:?}", distances("src/inputs/01/list"));
    println!("Day one similarities result: {:?}", similarities("src/inputs/01/list"));

    println!("Day two safety report result: {:?}", safety_report("src/inputs/02/list"));
    println!("Day two safety report with dampening result: {:?}", safety_report_dampen("src/inputs/02/list"));
}
