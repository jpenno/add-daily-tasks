use std::fs;
use std::fs::File;
use std::io::Write;

use crate::date::Date;

pub struct FileIO {
    _in_file: String,
    out_file: String,
    in_file_content: String,
    out_file_content: String,
}

impl FileIO {
    // set the in and out file
    pub fn new(in_file: String, out_file: String) -> FileIO {
        FileIO {
            in_file_content: fs::read_to_string(&in_file).expect("Error reading out file"),
            out_file_content: fs::read_to_string(&out_file).expect("Error reading out file"),
            _in_file: in_file,
            out_file,
        }
    }

    fn check_if_there(&self, match_str: String) -> bool {
        // check if the tasks have are there already
        if self.out_file_content.contains(&match_str) == true {
            return true;
        }
        return false;
    }

    pub fn insert_at_point(&mut self, sub_str: String, tasks: &str) {
        // date for the match string to see if tha tasks are there already
        let date = Date::new();
        // check if the tarsks are all ready there to not add them more than once
        if self.check_if_there(date.get_full_date().to_string()) == false {
            println!("SAVE TO FILE");
            // find where to add the tasks
            let mut index = self.out_file_content.find(&sub_str);

            // testing find the end of the line
            let index_offset =
                self.out_file_content[index.unwrap()..self.out_file_content.len()].find('\n');
            // move the index to the next line the 1 is to put it after the '\n'
            index = Some(index.unwrap() + index_offset.unwrap() + 1);

            // insert the tarsks in to the string
            self.out_file_content
                .insert_str(index.unwrap(), tasks);

            // write to file
            let mut out_fh =
                File::create(&self.out_file).expect("Error making file to save out put");
            write!(out_fh, "{}\n", self.out_file_content).expect("Error writing file");
        }
    }

    /// Get a reference to the file io's in file content.
    pub fn in_file_content(&self) -> String {
        self.in_file_content.clone()
    }
}