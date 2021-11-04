use std::fs;
use std::fs::File;
use std::io::Write;

// use crate::date::Date;

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

    pub fn insert_at_point(&mut self, sub_str: String, tasks: &str) -> &str {
        // find where to add the tasks
        let mut index = self.agenda_file_content.find(&sub_str);

        // find the end of the line
        let index_offset =
            self.agenda_file_content[index.unwrap()..self.agenda_file_content.len()].find('\n');
        // move the index to the next line the 1 is to put it after the '\n'
        index = Some(index.unwrap() + index_offset.unwrap() + 1);

        // insert the tarsks in to the string
        self.agenda_file_content.insert_str(index.unwrap(), tasks);
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

    /// Get a reference to the file io's agenda file content.
    pub fn agenda_file_content(&self) -> &str {
        self.agenda_file_content.as_ref()
    }
}
