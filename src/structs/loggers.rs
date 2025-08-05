use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

pub struct Logger<T> {
    entries: Vec<T>,
}

impl<T: Serialize> Logger<T> {
    pub fn new() -> Self {
        Logger {
            entries: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, entry: T) {
        self.entries.push(entry);
    }
    pub fn save_to_file(&self, file_path: &Path) -> Result<(), serde_json::Error> {
        let file = File::create(file_path).expect("Unable to create file");
        serde_json::to_writer_pretty(file, &self.entries)?;
        Ok(())
    }

    pub fn load_from_file(file_path: &Path) -> Result<Vec<T>, serde_json::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        let file = File::open(file_path).expect("Unable to open file");
        let entries = serde_json::from_reader(file)?;
        Ok(entries)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LogEvent {
    Init,
    Crossover,
    Mutation,
    Elite,
    Discard,
    Selection,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LogEntry {
    generation: u64,
    id: usize,
    parents: [usize; 2],
    event: LogEvent,
    info: String,
    fitness: f64,
    best_so_far: f64,
}

impl LogEntry {
    pub fn new(
        generation: u64,
        id: usize,
        parents: [usize; 2],
        event: LogEvent,
        info: String,
        fitness: f64,
        best_so_far: f64,
    ) -> Self {
        LogEntry {
            generation,
            id,
            parents,
            event,
            info,
            fitness,
            best_so_far,
        }
    }
}
