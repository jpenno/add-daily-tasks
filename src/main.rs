// use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

mod date;
use crate::date::Date;

// TODO make it so the tasks dont get added more then once
fn main() {
    // testing date
    let date = Date::new();
    date.print();

    // set file paths
    let in_file = "./res/tasks.org";
    let out_file = "./res/test.org";

    // get the tasks to add to the file
    let tasks_to_add = fs::read_to_string(&in_file).expect("Error reading in file");

    // get the contents out of the file
    let mut contents = fs::read_to_string(&out_file).expect("Error reading out file");

    // find where to add the tasks
    let index = contents.find("** Every Day");
    let mut off_set: usize = 0;

    //  find end of the line
    for i in 0..contents.len() {
        // get the character out of the string
        let b: u8 = contents.as_bytes()[index.unwrap() + i];
        let c: char = b as char;
        // chech if the charcter is a new line character
        if c == '\n' {
            // add one to go after the new line
            off_set = i + 1;
            break;
        }
    }

    // insert the tarsks in to the string
    contents.insert_str(index.unwrap() + off_set, &tasks_to_add);

    // write to file
    let mut out_fh = File::create(&out_file).expect("Error making file to save out put");
    write!(out_fh, "{}\n", contents).expect("Error writing file");
}
