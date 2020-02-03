use std::io::{self, Write};

use crate::app::{App, InputMode};

use termion::cursor::Goto;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, SelectableList, Text, Widget};
use tui::Terminal;
use unicode_width::UnicodeWidthStr;

pub fn draw<B: Write + Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(f.size());
        let help_message = match app.input_mode {
            InputMode::Normal => "Press : to enter on Command mode, i to start editing.",
            InputMode::Insert => "Press Esc to stop editing, Enter to record the task",
            InputMode::Command => "Insert q to exit",
        };

        Paragraph::new([Text::raw(help_message)].iter()).render(&mut f, chunks[0]);
        Paragraph::new([Text::raw(&app.input)].iter())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input"))
            .render(&mut f, chunks[1]);

        SelectableList::default()
            .block(Block::default().borders(Borders::ALL).title("Tasks"))
            .items(&app.tasks().items)
            .select(Some(app.tasks().selected))
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            .highlight_symbol(">")
            .render(&mut f, chunks[2]);
    })?;

    // Put the cursor back inside the input box
    write!(
        terminal.backend_mut(),
        "{}",
        Goto(4 + app.input.width() as u16, 5)
    )?;
    // stdout is buffered, flush it to see the effect immediately when hitting backspace
    io::stdout().flush().ok();
    Ok(())
}
