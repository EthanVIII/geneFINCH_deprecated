use crossterm::event::KeyCode;
use tui::{
    layout::Rect
};
use tui::layout::Alignment;
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph};
use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crate::sim;

pub struct State {
    pub terminal: Terminal<CrosstermBackend<io::Stdout>>,
    pub win_mode: WinMode,
    pub select_buffer: SelectBuffer,
    pub stats_buffer: String,
    pub output_buffer: String,
    pub vis_buffer: String,
    pub current_key: KeyCode
}

// STATE
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum WinMode {
    Selecting,
    RunBasicSim,
    RunCustomSim,
    FileAccess,
    GenomeWriter,
    DictionaryAccess,
}

pub struct SelectBuffer {
    pub header: String,
    pub items: Vec<String>,
    pub selection: usize,
    pub win_mode: Vec<WinMode>
}

impl SelectBuffer {
    fn dec(&mut self) {
        if self.selection == 0 {
            self.selection = self.items.len()-1;
        }
        else { self.selection -= 1 }
    }

    fn inc(&mut self) {
        if self.selection + 1== self.items.len() {
            self.selection = 0;
        }
        else { self.selection += 1; }
    }
    fn render_buffer(&self) -> String {
        let mut buffer: String = String::new();
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

    pub fn default_buffer(title: String) -> SelectBuffer {
        SelectBuffer{
            header: title.parse().unwrap(),
            items: vec![
                "Run a Basic Simulation".to_string(),
                "Run a Custom Simulation".to_string(),
                "Write/Edit a Genome".to_string(),
                "View a Dictionary".to_string()
            ],
            win_mode: vec![
                WinMode::RunBasicSim,
                WinMode::RunCustomSim,
                WinMode::GenomeWriter,
                WinMode::DictionaryAccess
            ],
            selection: 0
        }
    }
}

// RENDERING
pub fn render_frame(win_state: &mut State, sim_state: &mut sim::Sim) -> Result<(), Box<dyn std::error::Error>>  {
    // Input Handling goes here.
    if win_state.current_key == KeyCode::Esc {
        win_state.win_mode = WinMode::Selecting;
    }
    match win_state.win_mode {

        WinMode::Selecting => {
            if win_state.current_key == KeyCode::Up {
                win_state.select_buffer.dec();
            }
            if win_state.current_key == KeyCode::Down {
                win_state.select_buffer.inc();
            }
            if win_state.current_key == KeyCode::Enter {
                match win_state.select_buffer.items [
                        win_state.select_buffer.selection] {

                    _ => {}
                }
            }
            if win_state.current_key == KeyCode::Enter {
                win_state.win_mode =
                    win_state
                        .select_buffer
                        .win_mode[
                        win_state
                            .select_buffer
                            .selection
                        ]
            }
        },
        WinMode::RunBasicSim => {
            // Simulation
            sim::run_sim(50,sim_state);
            win_state.output_buffer = format!(
                "Sim Age: {} \nSim Pop: {}",
                sim_state.cycle,
                sim_state.count_pop());
        },
        WinMode::RunCustomSim => {
            win_state.output_buffer = "Currently Run Custom Sim".parse().unwrap();
        },
        WinMode::FileAccess => {
            win_state.output_buffer = "Currently File Access".parse().unwrap();
        },
        WinMode::GenomeWriter => {
            win_state.output_buffer = "Currently Genome Writer".parse().unwrap();
        },
        WinMode::DictionaryAccess => {
            win_state.output_buffer = "Currently Dictionary Access".parse().unwrap();
        }
    }



    let height: u16 = win_state.terminal.size().unwrap().height;
    let width:  u16 = win_state.terminal.size().unwrap().width;
    let chunks: [Rect;4] = [
        Rect::new(
            0,
            0,
            (width as f32 * 0.7) as u16,
            (height as f32 * 0.7) as u16
        ),
        Rect::new(
            (width as f32 * 0.7) as u16,
            0,
            (width as f32 * 0.3) as u16,
            (height as f32 * 0.7) as u16
        ),
        Rect::new(
            0,
            (height as f32 * 0.7) as u16,
            (width as f32 * 0.7) as u16,
            (height as f32 * 0.3) as u16
        ),
        Rect::new(
            (width as f32 *0.7) as u16,
            (height as f32 * 0.7) as u16,
            (width as f32 * 0.3) as u16,
            (height as f32 * 0.3) as u16
        )
    ];
    win_state.terminal.draw(|f| {
        let visual: Paragraph = Paragraph::new(win_state.vis_buffer.clone())
            .block(Block::default().title("Visualisation").borders(Borders::ALL))
            .style(Style::default());
        let output: Paragraph = Paragraph::new(win_state.output_buffer.clone())
            .block(Block::default().title("Output").borders(Borders::ALL))
            .style(Style::default());
        let stats: Paragraph = Paragraph::new(win_state.stats_buffer.clone())
            .block(Block::default().title("Statistics").borders(Borders::ALL))
            .style(Style::default());
        let select: Paragraph = Paragraph::new(win_state.select_buffer.render_buffer())
            .block(Block::default().title("Selection Buffer").borders(Borders::ALL))
            .style(Style::default())
            .alignment(Alignment::Center);

        f.render_widget(visual,chunks[0]);
        f.render_widget(output,chunks[1]);
        f.render_widget(stats, chunks[2]);
        f.render_widget(select,chunks[3]);

    })?;
    win_state.terminal.set_cursor(1, 1)?;
    Ok(())
}

