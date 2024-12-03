#[derive(Debug)]
enum Order{
    Increasing,
    Decreasing,
}

pub fn safety_report(path: &str) -> i32 {
    let mut result = 0;
        
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let split = line.split(' ');
        let mut report = vec![];
        for value in split {
            report.push(value.parse::<i32>().unwrap());
        }
        let mut error = 0;

        if report[0] == report[1] {
            continue;
        }

        let order = if report[0] < report[1] {
            Order::Increasing
        } else {
            Order::Decreasing
        };
        
        for i in 1..report.len() {
            let preceding = report[i - 1];
            let current = report[i];
            
            match order {
                Order::Increasing => {
                    if current < preceding {
                        error += 1;
                        break
                    }
                },
                Order::Decreasing => {
                    if current > preceding {
                        error += 1;
                        break
                    }
                },
            }

            if (current - preceding).abs() > 3 || current == preceding {
                error += 1;
                break;
            }

        }
        if error >= 1 {
            continue;
        }

        result += 1;
    }

    result
}

//partialeq
#[derive(PartialEq, Debug)]
pub enum Status{
    Safe,
    Unsafe,
    Unknown
}

#[derive(Debug)]
pub struct ReportLine {
    pub levels: Vec<i32>,
    pub order: Order,
    pub damped_index: usize,
    pub damped_levels: Vec<i32>,
    pub damping: bool,
    pub status: Status,
}

impl ReportLine {
    pub fn new(levels: Vec<i32>) -> Self {
        Self {
            levels,
            order: Order::Increasing,
            damped_index: 0,
            damped_levels: vec![],
            damping: false,
            status: Status::Unknown,
        }
    }

    pub fn verify(&mut self) {
        
        let mut levels = &self.levels;
        if self.damping{
            levels = &self.damped_levels;
        }

        if levels[0] == levels[1] {
            return
        }
        self.order = if levels[0] < levels[1] {
            Order::Increasing
        } else {
            Order::Decreasing
        };

        for i in 1..levels.len() {
            let preceding = levels[i-1];
            let current = levels[i];
            match self.order {
                Order::Increasing => {
                    if current <= preceding {
                        return
                    }
                },
                Order::Decreasing => {
                    if current >= preceding {
                        return
                    }
                },
            }
            if (current - preceding).abs() > 3  {
                return;
            }
        }
        self.status = Status::Safe;
    }
}

pub fn safety_report_dampen(path: &str) -> i32 {
    let mut result = 0;
        
    for line in std::fs::read_to_string(path).unwrap().lines() {
        let split = line.split(' ');
        let mut levels = vec![];
        for value in split {
            levels.push(value.parse::<i32>().unwrap());
        }
        let mut report = ReportLine::new(levels);

        while report.status == Status::Unknown{
            report.verify();
            // println!("{:?}", report);
            match report.status {
                Status::Safe => {
                    result += 1;
                },
                Status::Unknown => {
                    if !report.damping {
                        report.damping = true;
                    }
                    if report.damped_index == report.levels.len() {
                        report.status = Status::Unsafe;
                        // println!("{:?}", report);
                        continue
                    }else{
                        report.damped_levels = report.levels.clone();
                        report.damped_levels.remove(report.damped_index);
                        report.damped_index += 1;
                    }
                },
                Status::Unsafe => {
                    // println!("{:?}", report);
                },
            }
        }
    }
    result
}
