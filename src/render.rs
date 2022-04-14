use crossterm::event::Event::Key;
use crossterm::event::KeyCode;
use tui::{
    layout::Rect
};
use tui::layout::Alignment;
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph};
use crate::{
    state
};

pub fn render_frame(win_state: &mut state::State) -> Result<(), Box<dyn std::error::Error>>  {
    if win_state.current_key == KeyCode::Up {
        win_state.select_buffer.dec();
    }
    if win_state.current_key == KeyCode::Down {
        win_state.select_buffer.inc();
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
            (width as f32 *0.7) as u16,
            0,
            ((width as f32 * 0.3) as u16),
            (height as f32 * 0.7) as u16
        ),
        Rect::new(
            0,
            (height as f32 * 0.7) as u16,
            ((width as f32 * 0.7) as u16),
            (height as f32 * 0.3) as u16
        ),
        Rect::new(
            (width as f32 *0.7) as u16,
            (height as f32 * 0.7) as u16,
            ((width as f32 * 0.3) as u16),
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
    Ok(())
}

