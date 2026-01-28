use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crate::core::finance_app::FinanceApp;
use crate::persistence::transaction_repository::{TransactionRepository};
use crate::persistence::budget_repository::BudgetRepository;

use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders,Cell,Row, Paragraph, Table, TableState};
use ratatui::Terminal;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};




fn move_arrow(table_state: &mut ratatui::widgets::TableState, len: usize, direction:bool){
    if len==0 {
        table_state.select(None);
        return;
    }
    let selected_row = table_state.selected();
    let next_row;
    
    if !direction{
        next_row = match selected_row{
            Some(row) => 
                if row == len-1 {
                    row
                }else{
                    row+1
                },
            None => 0
        }
    }else{
        next_row = match selected_row{
            Some(row) => 
                if row == 0 {
                    row
                }else{
                    row-1
                },
            None => 0
        }
    }
    table_state.select(Some(next_row));

}


pub fn run_tui<TR, BR>(app: &mut FinanceApp<TR, BR>) -> anyhow::Result<()> 
where
    TR: TransactionRepository,
    BR: BudgetRepository,
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    run_app(&mut terminal,app)?;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}


fn run_app<TR,BR>(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut FinanceApp<TR, BR>) -> anyhow::Result<()> 
where
    TR: TransactionRepository,
    BR: BudgetRepository,
{
    let mut exit = false;
    let mut draw = true;
    let mut fetch_data = true;
    let mut transactions_db: Vec<Vec<String>> = Vec::new();
    let mut table_state = TableState::default();

    while !exit {
        if fetch_data{
            transactions_db = app.select()?;
            fetch_data = false;
            draw = true;

            if transactions_db.is_empty() {
                table_state.select(None);
                    } else {
                        table_state.select(Some(0));
                    }
        }

        if draw {
            terminal.draw(|frame| {
                let area = frame.area();
                let vertical_area = Layout::default().direction(Direction::Vertical)
                    .constraints([Constraint::Min(5), Constraint::Length(3), Constraint::Length(1),]).split(area);

                let main_area = Layout::default().direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(65), Constraint::Percentage(35)]).split(vertical_area[0]);

                let header = Row::new(vec![Cell::from("Date"), Cell::from(format!("{:<60}", "Kind")), Cell::from(format!("{:<40}", "Amount")),])
                    .style(Style::default().add_modifier(Modifier::BOLD));
                
                let rows = transactions_db.iter().map(|cols| {
                    let date = cols.get(0).map(String::as_str).unwrap_or("");
                    let kind = cols.get(2).map(String::as_str).unwrap_or("");
                    let amount_str = cols.get(1).map(String::as_str).unwrap_or("");
                    let amount:f64 = amount_str.parse().unwrap();

                    Row::new(vec![Cell::from(date),Cell::from(format!("{:<60}", kind)), Cell::from(format!("{:<40.2}", amount))])
                });

                let transactions_table = Table::new(rows,[Constraint::Length(80),Constraint::Length(60),Constraint::Length(40)]).header(header).block(Block::default()
                    .borders(Borders::ALL).title("Transactions")).highlight_style(Style::default().add_modifier(Modifier::REVERSED));
                frame.render_stateful_widget(transactions_table, main_area[0], &mut table_state);

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
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    if key.code == KeyCode::Char('x') {
                        exit = true;
                    }
                    else if key.code == KeyCode::Up{
                        move_arrow(&mut table_state, transactions_db.len(), true);
                        draw = true;   
                    }else if key.code == KeyCode::Down{
                        move_arrow(&mut table_state, transactions_db.len(), false);
                        draw = true;  
                        
                    }else{
                        draw = false;
                    }
                }
                Event::Resize(_, _) => {
                    terminal.autoresize()?;
                    terminal.clear()?;
                    draw = true;
                }
                _ => {}
            }
        }
    }

    Ok(())
}