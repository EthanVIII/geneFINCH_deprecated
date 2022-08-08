mod render;
mod finch;
mod sim;
mod test;

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

use tui;

// Tick Input
enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup Term for input.
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend : tui::backend::CrosstermBackend<io::Stdout> =
        tui::backend::CrosstermBackend::new(stdout);
    let terminal: tui::Terminal<tui::backend::CrosstermBackend<io::Stdout>>  =
        tui::Terminal::new(backend)?;


    let mut win_state: render::State = render::State{
        win_mode: render::WinMode::Selecting,
        terminal,
        select_buffer: render
            ::SelectBuffer
            ::default_buffer("geneFINCH: Select an Option".to_string()),
        stats_buffer: "".to_string(),
        output_buffer: "".to_string(),
        vis_buffer: "".to_string(),
        current_key: KeyCode::Null
    };
    let mut sim_state: sim::Sim = sim::Sim{
        cycle: 0,
        space: Vec::new(),
        new_finches: Vec::new(),
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
        // Display
        match render::render_frame(&mut win_state, &mut sim_state) {
            Ok(_) => (),
            Err(_) => {
                disable_raw_mode()?;
                execute!(
                    win_state.terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                win_state.terminal.show_cursor()?;
                println!("Error: Could not render Frame");
                return Ok(())
            },
        }
        match rx.recv()? {
            // Global Exit
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
