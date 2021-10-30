use std::fs;
use std::fs::File;
use std::io::Write;

pub struct FileIO {
    in_file: String,
    out_file: String,
    in_file_content: String,
    out_file_content: String,
}

impl FileIO {
    // set the in and out file
    pub fn new(in_file: String, out_file: String) -> FileIO {
        let mut tmp = FileIO {
            in_file,
            out_file,
            in_file_content: "".to_string(),
            out_file_content: "".to_string(),
        };
        // get the tasks to add to the file
        tmp.in_file_content = fs::read_to_string(&tmp.in_file).expect("Error reading in file");

        // get the contents out of the file
        tmp.out_file_content = fs::read_to_string(&tmp.out_file).expect("Error reading out file");

        return tmp;
    }

    pub fn insert_at_point(&mut self, sub_str: String) {
        // find where to add the tasks
        let mut index = self.out_file_content.find(&sub_str);

        // testing find the end of the line
        let index_offset =
            self.out_file_content[index.unwrap()..self.out_file_content.len()].find('\n');
        // move the index to the next line the 1 is to put it after the '\n'
        index = Some(index.unwrap() + index_offset.unwrap() + 1);

        // insert the tarsks in to the string
        self.out_file_content
            .insert_str(index.unwrap(), &self.in_file_content);

        // write to file
        let mut out_fh = File::create(&self.out_file).expect("Error making file to save out put");
        write!(out_fh, "{}\n", self.out_file_content).expect("Error writing file");
    }
}
