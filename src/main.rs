mod date;
mod file_io;
mod task;
mod task_manager;

use task_manager::TaskManager;

use crate::file_io::FileIO;

fn main() {
    // set file paths
    let in_file = "./res/tasks.org";
    let out_file = "./res/test.org";
    let match_str = "** Every Day".to_string();

    let mut fiel_io = FileIO::new(in_file.to_string(), out_file.to_string());
    let mut task_manager: TaskManager = TaskManager::new(fiel_io.in_file_content());
    task_manager.format_tasks();
    fiel_io.insert_at_point(match_str, task_manager.formated_tasks());
}
