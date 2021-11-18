use std::{fs, str};

pub struct Config {
    tasks_path: String,
    agenda_path: String,
    match_str: String,
}

impl Config {
    pub fn new(config_file_path: &str) -> Self {
        let mut aganda_path: &[u8] = &[];
        let mut task_path: &[u8] = &[];
        let mut match_str: &[u8] = &[];
        let config_file = fs::read_to_string(config_file_path).expect("Error reading out file");

        let lines = config_file.lines();

        for line in lines {
            // get the task file path
            if line.to_string().contains("task:") {
                let id: Vec<_> = line.match_indices('"').collect();
                task_path = &line.as_bytes()[id[0].0 + 1..id[1].0];
            }
            // get the aganda file path
            else if line.to_string().contains("aganda:") {
                let id: Vec<_> = line.match_indices('"').collect();
                aganda_path = &line.as_bytes()[id[0].0 + 1..id[1].0];
            }
            // get the match string
            else if line.to_string().contains("match_str:") {
                let id: Vec<_> = line.match_indices('"').collect();
                match_str = &line.as_bytes()[id[0].0 + 1..id[1].0];
            }
        }
        Self {
            agenda_path: str::from_utf8(aganda_path).unwrap().to_string(),
            tasks_path: str::from_utf8(task_path).unwrap().to_string(),
            match_str: str::from_utf8(match_str).unwrap().to_string()
        }
    }

    /// Get a reference to the config's agenda path.
    pub fn agenda_path(&self) -> &str {
        self.agenda_path.as_ref()
    }

    /// Get a reference to the config's tasks path.
    pub fn tasks_path(&self) -> &str {
        self.tasks_path.as_ref()
    }

    /// Get a reference to the config's match str.
    pub fn match_str(&self) -> &str {
        self.match_str.as_ref()
    }
}
