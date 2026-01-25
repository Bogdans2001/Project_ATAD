use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use ratatui::layout::{Constraint, Direction, Layout};

use crate::domain::transaction;

pub fn run_tui() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let app = run_app(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    app
}


fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
    let mut exit = false;
    let mut draw = true;

    while !exit {
        if draw {
            terminal.draw(|frame| {
                let area = frame.area();
                let vertical_area = Layout::default().direction(Direction::Vertical)
                    .constraints([Constraint::Min(5), Constraint::Length(3), Constraint::Length(1),]).split(area);

                let main_area = Layout::default().direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(65), Constraint::Percentage(35)]).split(vertical_area[0]);

                let transactions_area = Paragraph::new("Transactions area")
                    .block(Block::default().borders(Borders::ALL).title("Transactions"));
                frame.render_widget(transactions_area, main_area[0]);

                let transaction_details = Paragraph::new("Transaction details")
                    .block(Block::default().borders(Borders::ALL).title("Details"));
                frame.render_widget(transaction_details, main_area[1]);

                let cmd_area = Paragraph::new("Input command area")
                    .block(Block::default().borders(Borders::ALL).title("Command"));
                frame.render_widget(cmd_area, vertical_area[1]);

                let status = Paragraph::new("Status area, press x to exit");
                frame.render_widget(status, vertical_area[2]);
            })?;
            draw = false;
        }

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('x') {
                        exit = true;
                    }
                    else {
                        draw = true;
                    }
                }
                Event::Resize(_, _) => {
                    terminal.clear()?;
                    draw = true;
                }
                _ => {}
    }
        }
    }

    Ok(())
}