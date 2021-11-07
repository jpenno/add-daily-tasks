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
            if data.len() > MATCHES.scheduled.len() {
                let mut insert_date: bool = true;
                // check if the tsk is scheduled
                for (i, match_char) in MATCHES.scheduled.chars().enumerate() {
                    if match_char != data.as_bytes()[i] as char {
                        insert_date = false;
                        break;
                    }
                }
                if insert_date {
                    let mut add_date: bool = true;
                    // check if it is the right day to add the task
                    for (i, match_char) in self.date.day_name().chars().enumerate() {
                        if match_char != data.as_bytes()[MATCHES.scheduled_idx + i] as char {
                            add_date = false;
                            break;
                        }
                    }
                    if add_date {
                        data.replace_range(MATCHES.scheduled_idx..MATCHES.scheduled_idx + 4, "");
                        let mut date_str = self.date.get_date_ymd().to_owned();
                        date_str.push_str(" ");
                        data.insert_str(MATCHES.scheduled_idx, &date_str);
                        println!("SCHEDULED: {}", data);
                    } else {
                        self.add = false;
                    }
                }
            }
        }
    }

    /// Get a reference to the task's add.
    pub fn add(&self) -> bool {
        self.add
    }
}
