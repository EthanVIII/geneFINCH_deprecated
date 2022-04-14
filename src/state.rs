use std::io;
use crossterm::{
    event::KeyCode
};
use tui::{
    backend::CrosstermBackend,
    Terminal
};

// pub enum Selection {
//     Selecting,
//     RunBasicSim,
//     RunCustomSim,
//     FileAccess,
//     GenomeWriter,
//     DictionaryAccess,
// }

pub enum WinMode {
    Selection,
    RunningSim,
    FileAccess,
}

pub struct SelectBuffer {
    pub(crate) header: String,
    pub(crate) items: Vec<String>,
    pub(crate) selection: usize,
}

impl SelectBuffer {
    pub(crate) fn dec(&mut self) {
        if self.selection == 0 {
            self.selection = self.items.len()-1;
        }
        else { self.selection -= 1 }
    }

    pub(crate) fn inc(&mut self) {
        if self.selection + 1== self.items.len() {
            self.selection = 0;
        }
        else { self.selection += 1; }
    }
    pub(crate) fn render_buffer(&self) -> String {
        let mut buffer: String = String::new();
        buffer.push_str("\n\n\n");
        buffer.push_str(&*self.header);
        buffer.push_str("\n\n");

        for (i, item) in self.items.iter().enumerate() {
            if i == self.selection {
                buffer.push_str(">>>");
            }
            else {
                buffer.push_str("   ");
            }
            buffer.push_str(item);
            buffer.push_str("\n");
        };
        buffer
    }

    pub(crate) fn default_buffer(title: String) -> SelectBuffer {
        SelectBuffer{
            header: title.parse().unwrap(),
            items: vec![
                "Run a Basic Simulation".to_string(),
                "Run a Custom Simulation".to_string(),
                "Write/Edit a Genome".to_string(),
                "View a Dictionary".to_string()
            ],
            selection: 0
        }
    }
}

pub struct State {
    pub(crate) win_mode: WinMode,
    pub(crate) terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub(crate) select_buffer: SelectBuffer,
    pub(crate) stats_buffer: String,
    pub(crate) output_buffer: String,
    pub(crate) vis_buffer: String,
    pub(crate) current_key: KeyCode
}