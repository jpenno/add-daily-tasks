use chrono;
use chrono::Local;

pub struct Date {
    day_name: String,
    day: String,
    month: String,
    year: String,
    full_date: String,
}

impl Date {
    // set the date to the current date and time
    pub fn new() -> Date {
        // get the current date and time
        let date = Local::now();
        // set date
        let mut tmp = Date {
            day_name: date.format("%a").to_string(),
            day: date.format("%-d").to_string(),
            month: date.format("%m").to_string(),
            year: date.format("%y").to_string(),
            full_date: "".to_string(),
        };
        // set string with full date
        tmp.full_date.push_str(tmp.day.as_str());
        tmp.full_date.push_str("/");
        tmp.full_date.push_str(tmp.month.as_str());
        tmp.full_date.push_str("/");
        tmp.full_date.push_str(tmp.year.as_str());
        return tmp;
    }

    pub fn print(&self) {
        println!("day name: {}", self.day_name);
        println!("day: {}", self.day);
        println!("month: {}", self.month);
        println!("year: {}", self.year);
        println!("full date: {}", self.full_date);
    }
}
