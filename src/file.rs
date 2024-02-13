use std::{cell::RefCell, fs, rc::Rc};

use crate::config::Config;

pub trait ReadFile {
    fn all_lines(&self) -> Vec<String>;
}

pub struct LocalFile {
    lines: Vec<String>,
}

impl ReadFile for LocalFile {
    fn all_lines(&self) -> Vec<String> {
        self.lines.clone()
    }
}

impl LocalFile {
    pub fn new(path: String, config: Rc<RefCell<Config>>) -> Result<Self, String> {
        Ok(Self {
            lines: fs::read_to_string(path)
                .map_err(|e| e.to_string())?
                .replace('\t', " ".repeat(config.borrow().tab_size as usize).as_str())
                .replace('\r', "")
                .split('\n')
                .map(str::to_string)
                .collect(),
        })
    }
}
