use crate::task::Task;
use crate::date::Date;

pub struct TaskManager {
    tasks: Vec<Task>,
    formated_tasks: String,
}

impl TaskManager {
    pub fn new(task_file: String) -> Self {
        // split the tarsks on new line
        let mut task_file_lines: Vec<&str> = task_file.split('\n').collect();
        // revers the split to pot from the frount of the file
        task_file_lines.reverse();

        // create tasks list
        let mut tasks: Vec<Task> = Vec::new();

        // add tarsks
        while task_file_lines.len() > 1 {
            let task = task_file_lines.pop().unwrap().to_string();
            let mut data: Vec<String> = Vec::new();
            // get all the lines under the task
            while task_file_lines
                .last()
                .unwrap()
                .to_string()
                .contains(&'*'.to_string())
                == false
                && task_file_lines.len() > 1
            {
                data.push(task_file_lines.pop().unwrap().to_string());
            }
            // build task

            tasks.push(Task::new(task, data));
        }

        // tmp retur of nothing for testing
        return TaskManager {
            tasks,
            formated_tasks: "test".to_string(),
        };
    }

    pub fn format_tasks(&mut self) {
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

    /// Get a reference to the task manager's formated tasks.
    pub fn formated_tasks(&self) -> &str {
        self.formated_tasks.as_ref()
    }
}