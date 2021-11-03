use std::fs;
use std::fs::File;
use std::io::Write;

use crate::date::Date;

pub struct FileIO {
    _task_path: String,
    agenda_path: String,
    in_file_content: String,
    agenda_file_content: String,
}

impl FileIO {
    // set the in and out file
    pub fn new(task_path: String, agenda_path: String) -> FileIO {
        FileIO {
            in_file_content: fs::read_to_string(&task_path).expect("Error reading out file"),
            agenda_file_content: fs::read_to_string(&agenda_path).expect("Error reading out file"),
            _task_path: task_path,
            agenda_path,
        }
    }

    fn check_if_there(&self, match_str: String) -> bool {
        // check if the tasks have are there already
        if self.agenda_file_content.contains(&match_str) == true {
            return true;
        }
        return false;
    }

    pub fn insert_at_point(&mut self, sub_str: String, tasks: &str, date: &Date) -> &str {
        // date for the match string to see if tha tasks are there already
        // let date = Date::new();
        // check if the tarsks are all ready there to not add them more than once
        if self.check_if_there(date.get_full_date().to_string()) == false {
            // find where to add the tasks
            let mut index = self.agenda_file_content.find(&sub_str);

            // find the end of the line
            let index_offset =
                self.agenda_file_content[index.unwrap()..self.agenda_file_content.len()].find('\n');
            // move the index to the next line the 1 is to put it after the '\n'
            index = Some(index.unwrap() + index_offset.unwrap() + 1);

            // insert the tarsks in to the string
            self.agenda_file_content.insert_str(index.unwrap(), tasks);
        }
        return self.agenda_file_content.as_ref();
    }

    pub fn save_to_file(&self) {
        // write to file
        let mut agenda_file =
            File::create(&self.agenda_path).expect("Error making file to save out put");
        write!(agenda_file, "{}", self.agenda_file_content).expect("Error writing file");
    }

    /// Get a reference to the file io's in file content.
    pub fn in_file_content(&self) -> String {
        self.in_file_content.clone()
    }
}
