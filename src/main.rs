use dayone::{distances, similarities};
use daytwo::{safety_report, safety_report_dampen};
use daythree::{uncorrupt, uncorrupt_with_instructions};

mod dayone;
mod daytwo;
mod daythree;

fn main() {
    println!("Advent of Code 2024");

    println!("Day one distances result: {:?}", distances("src/inputs/01/list"));
    println!("Day one similarities result: {:?}", similarities("src/inputs/01/list"));

    println!("Day two safety report result: {:?}", safety_report("src/inputs/02/list"));
    println!("Day two safety report with dampening result: {:?}", safety_report_dampen("src/inputs/02/list"));

    println!("Day three result: {:?}", uncorrupt("src/inputs/03/input"));
    println!("Day three with instructions result: {:?}", uncorrupt_with_instructions("src/inputs/03/input"));
}
