use crate::date::Date;
use crate::task::Task;

pub struct TaskManager<'a> {
    tasks: Vec<Task<'a>>,
    date: &'a Date,
}

// impl TaskManager {
impl<'a> TaskManager<'a> {
    pub fn new(task_file: String, date: &'a Date) -> Self {
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

            tasks.push(Task::new(task, data, date));
        }

        // tmp retur of nothing for testing
        return TaskManager { tasks, date };
    }

    pub fn formated_tasks(&self) -> String {
        let mut formated_tasks: String = String::new();
        formated_tasks.push_str("*** ");
        formated_tasks.push_str(self.date.get_full_date());
        formated_tasks.push('\n');

        // loop through tasks and add them to a string
        for task in &self.tasks {
            formated_tasks.push_str(&task.task());
            formated_tasks.push('\n');
            for data in task.data() {
                formated_tasks.push_str(&data);
                formated_tasks.push('\n');
            }
        }
        return formated_tasks;
    }

    pub fn process(&mut self) {
        for task in &mut self.tasks {
            task.process();
        }
    }

    pub fn check_if_there(&self, agenda_file_content: String) -> bool {
        // check if the tasks have are there already
        if agenda_file_content.contains(self.date.get_full_date()) == true {
            return true;
        }
        return false;
    }
}
