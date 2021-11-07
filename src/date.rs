use chrono::{self, Datelike, Local};

pub struct Date {
    day_name: String,
    day: String,
    month: String,
    year: String,
}

impl Date {
    // set the date to the current date and time
    pub fn new() -> Date {
        // get the current date and time
        let date = Local::now();
        Date {
            day_name: date.format("%a").to_string(),
            day: date.day().to_string(),
            month: date.month().to_string(),
            year: date.year().to_string(),
        }
    }

    pub fn _new_set(day_name: String, day: String, month: String, year: String) -> Date {
        Date {
            day_name,
            day,
            month,
            year,
        }
    }

    // get the date Year Month Day
    pub fn get_date_ymd(&self) -> String {
        let mut tmp: String = "".to_string();
        // set string with full date
        tmp.push_str(&self.year);
        tmp.push_str("-");
        tmp.push_str(&self.month);
        tmp.push_str("-");
        tmp.push_str(&self.day);
        tmp.push_str(" ");
        tmp.push_str(&self.day_name.as_str());
        return tmp;
    }

    // get the date Day Month Year
    pub fn get_date_dmy(&self) -> String {
        let mut tmp: String = "".to_string();
        // set string with full date
        tmp.push_str(&self.day);
        tmp.push_str("/");
        tmp.push_str(&self.month);
        tmp.push_str("/");
        tmp.push_str(&self.year);
        return tmp;
    }

    pub fn _print(&self) {
        println!("day name: {}", self.day_name);
        println!("day: {}", self.day);
        println!("month: {}", self.month);
        println!("year: {}", self.year);
        println!("Date DMY: {}", self.get_date_dmy());
        println!("Date YMD: {}", self.get_date_ymd());
    }

    /// Get a reference to the date's day name.
    pub fn day_name(&self) -> &str {
        self.day_name.as_ref()
    }
}
