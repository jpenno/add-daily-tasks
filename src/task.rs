// use std::default::default;
use std::str;

use crate::date::Date;
pub struct Task<'a> {
    task: String,
    data: Vec<String>,
    date: &'a Date,
    add: bool,
}

struct Matches<'a> {
    scheduled: &'a str,
    scheduled_idx: usize,
}
const MATCHES: Matches = Matches {
    scheduled: "SCHEDULED: <",
    scheduled_idx: "SCHEDULED: <".len(),
};

impl<'a> Task<'a> {
    pub fn new(task: String, data: Vec<String>, date: &'a Date) -> Self {
        Self {
            task,
            data,
            date,
            add: true,
        }
    }

    pub fn _print(&self) {
        println!("Task: {}", self.task);
        println!("Data: {:?}\n", self.data);
    }

    /// Get a reference to the task's task.
    pub fn task(&self) -> &str {
        self.task.as_ref()
    }

    /// Get a reference to the task's data.
    pub fn data(&self) -> &[String] {
        self.data.as_ref()
    }

    pub fn process(&mut self) {
        // check all data items
        for data in &mut self.data {
            let mut insert_date: bool = false;
            if data.len() > MATCHES.scheduled.len() {
                // check if the task is scheduled
                if &data.as_bytes()[0..MATCHES.scheduled_idx] == MATCHES.scheduled.as_bytes() {
                    insert_date = true;
                }
            }
            if insert_date {
                let mut add_date: bool = false;
                match data.as_bytes()[MATCHES.scheduled_idx + 3] as char {
                    // handle if there is a day range eg: Mon-Fri
                    '-' => {
                        // get the first day in range
                        let first_day =
                            &data.as_bytes()[MATCHES.scheduled_idx..MATCHES.scheduled_idx + 3];
                        // get teh second day in the range
                        let second_day = &data.as_bytes()
                            [MATCHES.scheduled_idx + 4..MATCHES.scheduled_idx + 4 + 3];
                        // check if the day is in range
                        if self.date.in_range(
                            &str::from_utf8(first_day).unwrap(),
                            &str::from_utf8(second_day).unwrap(),
                        ) {
                            data.replace_range(
                                MATCHES.scheduled_idx..MATCHES.scheduled_idx + 4 + 4,
                                "",
                            );
                            add_date = true;
                        }
                    }
                    // handle day list eg: Mon, Wed, Fri
                    ',' => {
                        let mut keep_looking = true;
                        let off_set = 5;
                        let mut start = MATCHES.scheduled_idx;
                        let mut end = MATCHES.scheduled_idx + 3;
                        while keep_looking {
                            let day_in_list = &data.as_bytes()[start..end];
                            // check if there are more days in the list
                            if data.as_bytes()[end] as char != ',' {
                                keep_looking = false;
                            } else {
                                start += off_set;
                                end += off_set;
                                if end >= data.len() {
                                    keep_looking = false;
                                }
                            }

                            // check if the day matches
                            if day_in_list == self.date.day_name().as_bytes() {
                                keep_looking = false;
                                add_date = true;

                                let tmp = str::from_utf8(data.as_bytes()).unwrap();
                                let n = tmp.to_string().find('|').unwrap();

                                data.replace_range(MATCHES.scheduled_idx..n + 2, "");
                            }
                        }
                    }
                    // handle if there is just one day eg: Mon
                    ' ' => {
                        let day =
                            &data.as_bytes()[MATCHES.scheduled_idx..MATCHES.scheduled_idx + 3];
                        if day == self.date.day_name().as_bytes() {
                            data.replace_range(
                                MATCHES.scheduled_idx..MATCHES.scheduled_idx + 4,
                                "",
                            );
                            add_date = true;
                        }
                    }
                    _ => {
                        add_date = false;
                        println!("Not handled");
                    }
                }
                if add_date {
                    // data.replace_range(MATCHES.scheduled_idx..MATCHES.scheduled_idx + 4, "");
                    let mut date_str = self.date.get_date_ymd().to_owned();
                    date_str.push_str(" ");
                    data.insert_str(MATCHES.scheduled_idx, &date_str);
                } else {
                    self.add = false;
                }
            }
        }
    }

    /// Get a reference to the task's add.
    pub fn add(&self) -> bool {
        self.add
    }
}
