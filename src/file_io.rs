use std::fs;
use std::fs::File;
use std::io::Write;

use crate::date::Date;
use crate::task::Task;

pub struct FileIO {
    in_file: String,
    out_file: String,
    in_file_content: String,
    formated_tasks: String,
    out_file_content: String,
    tasks: Vec<Task>,
}

impl FileIO {
    // set the in and out file
    pub fn new(in_file: String, out_file: String) -> FileIO {
        let mut tmp = FileIO {
            in_file,
            out_file,
            in_file_content: "".to_string(),
            out_file_content: "".to_string(),
            tasks: Vec::new(),
            formated_tasks: "".to_string(),
        };

        // get the tasks to add to the file
        tmp.in_file_content
            .push_str(&fs::read_to_string(&tmp.in_file).expect("Error reading out file"));

        tmp.add_tasks();

        tmp.format_tasks();

        // get the contents out of the file
        tmp.out_file_content = fs::read_to_string(&tmp.out_file).expect("Error reading in file");
        return tmp;
    }

    fn check_if_there(&self, match_str: String) -> bool {
        // check if the tasks have are there already
        if self.out_file_content.contains(&match_str) == true {
            return true;
        }
        return false;
    }

    fn add_tasks(&mut self) {
        // split the tarsks on new line
        let mut in_splits: Vec<&str> = self.in_file_content.split('\n').collect();
        // revers the split to pot from the frount of the file
        in_splits.reverse();

        // add tarsks
        while in_splits.len() > 1 {
            let task = in_splits.pop().unwrap().to_string();
            let mut data: Vec<String> = Vec::new();
            // get all the lines under the task
            while in_splits
                .last()
                .unwrap()
                .to_string()
                .contains(&'*'.to_string())
                == false
                && in_splits.len() > 1
            {
                data.push(in_splits.pop().unwrap().to_string());
            }
            // build task

            self.tasks.push(Task::new(task, data));
        }
    }

    fn format_tasks(&mut self) {
        // add date to the top of tasks output
        let date = Date::new();
        self.formated_tasks.push_str("*** ");
        self.formated_tasks.push_str(&date.get_full_date());
        self.formated_tasks.push('\n');

        // loop through tasks and add them to a string
        for task in &self.tasks {
            self.formated_tasks.push_str(&task.task());
            self.formated_tasks.push('\n');
            for data in task.data() {
                self.formated_tasks.push_str(&data);
                self.formated_tasks.push('\n');
            }
        }
    }

    pub fn insert_at_point(&mut self, sub_str: String) {
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
                .insert_str(index.unwrap(), &self.formated_tasks);

            // write to file
            let mut out_fh =
                File::create(&self.out_file).expect("Error making file to save out put");
            write!(out_fh, "{}\n", self.out_file_content).expect("Error writing file");
        }
    }
}
