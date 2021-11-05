use crate::date::Date;
pub struct Task<'a> {
    task: String,
    data: Vec<String>,
    date: &'a Date,
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
        Self { task, data, date }
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
                for (i, match_char) in MATCHES.scheduled.chars().enumerate() {
                    if match_char != data.as_bytes()[i] as char {
                        insert_date = false;
                        continue;
                    }
                }
                if insert_date {
                    println!("TASK: {}", &self.task);
                    println!("SCHEDULED: {}", data);
                    let mut date_str = self.date.get_date_ymd().to_owned();
                    date_str.push_str(" ");
                    data.insert_str(MATCHES.scheduled_idx,&date_str);
                    println!("SCHEDULED: {}", data);
                }
            }
        }
    }
}
