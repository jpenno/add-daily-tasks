// use std::env;

// mod date;
// use crate::date::Date;

mod file_io;
use crate::file_io::FileIO;

// TODO make it so the tasks dont get added more then once
fn main() {
    // set file paths
    let in_file = "./res/tasks.org";
    let out_file = "./res/test.org";
    let match_str = "** Every Day".to_string();

    let mut fiel_io = FileIO::new(in_file.to_string(), out_file.to_string());
    fiel_io.insert_at_point(match_str);
}
