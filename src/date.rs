use chrono::{self, Datelike, Local};
use std::collections::HashMap;

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
        tmp.push('-');
        tmp.push_str(&self.month);
        tmp.push('-');
        let mut day_tmp: String = String::new();
        // day_tmp.insert_str(0, "0");
        if self.day.len() < 2 {
            day_tmp.push('0');
        }
        day_tmp.push_str(&self.day);
        tmp.push_str(&day_tmp);
        tmp.push(' ');
        tmp.push_str(self.day_name.as_str());
        tmp
    }

    // get the date Day Month Year
    pub fn get_date_dmy(&self) -> String {
        let mut tmp: String = "".to_string();
        // set string with full date
        tmp.push_str(&self.day);
        tmp.push('/');
        tmp.push_str(&self.month);
        tmp.push('/');
        tmp.push_str(&self.year);
        tmp
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

    pub fn in_range(&self, firts_day: &str, second_day: &str) -> bool {
        let mut days = HashMap::new();
        days.insert(String::from("Mon"), 0);
        days.insert(String::from("Tue"), 1);
        days.insert(String::from("Wed"), 2);
        days.insert(String::from("Thu"), 3);
        days.insert(String::from("Fri"), 4);
        days.insert(String::from("Sat"), 5);
        days.insert(String::from("Sun"), 6);

        if days.get(&self.day_name) <= days.get(second_day)
            && days.get(&self.day_name) >= days.get(firts_day)
        {
            return true;
        }
        false
    }
}
