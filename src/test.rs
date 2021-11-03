#[cfg(test)]
mod file_io {
    use crate::{date::Date, file_io::FileIO, task_manager::TaskManager};
    use std::fs;

    #[test]
    fn add_tasks_to_file() {
        let file_path = "./test_files/add_tasks_to_file".to_string();
        let in_file = file_path.to_string() + "/tasks.org";
        let out_file = file_path.to_string() + "/agenda_in.org";
        let match_str = "** Every Day".to_string();
        let date = Date::new_set(
            "wed".to_string(),
            "3".to_string(),
            "11".to_string(),
            "21".to_string(),
        );

        let mut fiel_io = FileIO::new(in_file.to_string(), out_file.to_string());
        let task_manager: TaskManager = TaskManager::new(fiel_io.in_file_content());
        let save_to_file = fiel_io.insert_at_point(
            match_str,
            task_manager.formated_tasks(&date).as_str(),
            &date,
        );

        let epcted = fs::read_to_string(file_path.to_string() + "/agenda_out.org")
            .expect("Error reading out file");

        assert!(epcted == save_to_file);
    }
}
