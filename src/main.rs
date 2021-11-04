mod date;
mod file_io;
mod task;
mod task_manager;
mod test;

use date::Date;
use task_manager::TaskManager;

use crate::file_io::FileIO;

fn main() {
    let date = Date::new();
    // set file paths
    let in_file = "./res/tasks.org";
    let out_file = "./res/agenda.org";
    let match_str = "** Every Day".to_string();

    let mut fiel_io = FileIO::new(in_file.to_string(), out_file.to_string());
    let mut task_manager: TaskManager = TaskManager::new(fiel_io.in_file_content(), &date);

    task_manager.process();

    if task_manager.check_if_there(fiel_io.agenda_file_content().to_string()) == false {
        fiel_io.insert_at_point(match_str, task_manager.formated_tasks().as_str());
    }

    fiel_io.save_to_file();
}
