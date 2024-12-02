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
    pub damping: Option<usize>,
    pub levels: Vec<i32>,
    pub order: Order,
    pub status: Status,
}

impl ReportLine {
    pub fn new(levels: Vec<i32>) -> Self {
        let mut status = Status::Unknown;
        let mut damping = None;

        if levels[0] == levels[1] {
            damping = Some(0);
            if levels[0] == levels[2] {
                status = Status::Unsafe;
            }
        }
        let order = if levels[0] < levels[1] {
            Order::Increasing
        } else {
            Order::Decreasing
        };

        Self {
            damping,
            levels,
            order,
            status,
        }
    }

    pub fn verify(&mut self) {
        match self.damping {
            None => {
                for i in 1..self.levels.len() {
                    let preceding = self.levels[i-1];
                    let current = self.levels[i];
                    match self.order {
                        Order::Increasing => {
                            if current <= preceding {
                                self.damping = Some(i);
                                break
                            }
                        },
                        Order::Decreasing => {
                            if current >= preceding {
                                self.damping = Some(i);
                                break
                            }
                        },
                    }

                    if (current - preceding).abs() > 3  {
                        self.damping = Some(i);
                        break;
                    }
                }
                if self.damping.is_none() {
                    self.status = Status::Safe;
                }
            }
            Some(index) => {
                self.levels.remove(index);

                for i in 1..self.levels.len() {
                    let preceding = self.levels[i-1];
                    let current = self.levels[i];
                    match self.order {
                        Order::Increasing => {
                            if current <= preceding {
                                self.status = Status::Unsafe;
                                return;
                            }
                        },
                        Order::Decreasing => {
                            if current >= preceding {
                                self.status = Status::Unsafe;
                                return;
                            }
                        },
                    }

                    if (current - preceding).abs() > 3  {
                        self.status = Status::Unsafe;
                        return;
                    }
                }
                self.status = Status::Safe;
            }
        }
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
            println!("{:?}", report);
            match report.status {
                Status::Safe => {
                    result += 1;
                },
                Status::Unsafe => {},
                Status::Unknown => { report.verify()},
            }
        }
    }

    result
}
