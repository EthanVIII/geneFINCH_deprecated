mod state;
mod render;

//Uses
use std::{
    io,
    thread,
    sync::mpsc,
    time::{Duration, Instant}
};

use crossterm::{
    execute,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::Rect,
    Terminal,
    widgets::{Block, Borders},
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState, Paragraph}
};

// Tick Input
enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend :CrosstermBackend<io::Stdout>= CrosstermBackend::new(stdout);
    let mut terminal:Terminal<CrosstermBackend<io::Stdout>>  = Terminal::new(backend)?;


    let mut win_state: state::State = state::State{
        win_mode: state::WinMode::Selection,
        terminal,
        select_buffer: state
            ::SelectBuffer
            ::default_buffer("geneFINCH: Select an Option".to_string()),
        stats_buffer: "".to_string(),
        output_buffer: "".to_string(),
        vis_buffer: "".to_string(),
        current_key: KeyCode::Null
    };

    // Setup event polling thread.
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }
            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    loop {
        render::render_frame(&mut win_state);
        match rx.recv()? {
            Event::Input(event) => {
                        win_state.current_key = event.code;
                        if event.code == KeyCode::Char('q') {
                            break;
                        }
                    },
            Event::Tick => {
                win_state.current_key = KeyCode::Null;
            }
        }
    }
    // Clear Settings
    disable_raw_mode()?;
    execute!(
        win_state.terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    win_state.terminal.show_cursor()?;
    Ok(())
}
