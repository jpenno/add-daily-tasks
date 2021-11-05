#[cfg(test)]
mod file_io {
    use crate::{date::Date, file_io::FileIO, task_manager::TaskManager};
    use std::fs;
    struct Data {
        tasks_file: String,
        agenda_file: String,
        match_str: String,
    }

    // test if the tasks are added to the agenda file
    #[test]
    fn add_tasks_to_file() {
        let file_path = "./test_files/add_tasks_to_file".to_string();
        sim_save(
            Data {
                tasks_file: "./test_files/shared/tasks.org".to_string(),
                agenda_file: file_path.to_string() + "/agenda_in.org",
                match_str: "** Every Day".to_string(),
            },
            &Date::_new_set(
                "wed".to_string(),
                "3".to_string(),
                "11".to_string(),
                "2021".to_string(),
            ),
        );
    }
    // test if the tasks are added to the agenda file
    // more than once
    #[test]
    fn check_if_tasks_are_there() {
        let file_path = "./test_files/check_if_tasks_are_there".to_string();
        sim_save(
            Data {
                tasks_file: "./test_files/shared/tasks.org".to_string(),
                agenda_file: file_path.to_string() + "/agenda_in.org",
                match_str: "** Every Day".to_string(),
            },
            &Date::_new_set(
                "wed".to_string(),
                "3".to_string(),
                "11".to_string(),
                "2021".to_string(),
            ),
        );
    }

    fn sim_save(data: Data, date: &Date) {
        let epcted = fs::read_to_string( "./test_files/shared/agenda_out.org")
            .expect("Error reading out file");

        let mut fiel_io = FileIO::new(data.tasks_file.to_string(), data.agenda_file.to_string());
        let task_manager: TaskManager = TaskManager::new(fiel_io.in_file_content(), &date);

        let mut save_to_file = fiel_io.agenda_file_content();

        if task_manager.check_if_there(fiel_io.agenda_file_content().to_string()) == false {
            save_to_file =
                fiel_io.insert_at_point(data.match_str, task_manager.formated_tasks().as_str());
        }
        assert!(epcted == save_to_file);
    }
}
