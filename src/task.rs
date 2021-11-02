

pub struct Task {
    task: String,
    data: Vec<String>,
}

impl Task {
    pub fn new(task: String, data: Vec<String>) -> Self { Self { task, data } }
    
    pub fn _print(&self) {
        println!("Task: {}", self.task);
        println!("Data: {:?}\n", self.data);
    }

    /// Get a reference to the task's task.
    pub fn task(&self) -> &str {
        self.task.as_ref()
    }

    /// Get a reference to the task's data.
    pub fn data(&self) -> &[String] {
        self.data.as_ref()
    }
}