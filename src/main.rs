mod rgol;
mod cell;


use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal, Layout, Direction, Constraint},
    widgets::{Paragraph, Borders, Block, Padding},
    style::Style
};
use rgol::Rgol;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let n_rows: usize = 3;
    let n_cols: usize = 4;
    let mut gol = Rgol::new(n_rows,n_cols); 
    gol.update(0,0);

    
    let mut cols_in_row = Vec::new();
    let mut rows_in_grid = Vec::new();

    for _ in 0..n_rows {
        rows_in_grid.push(Constraint::Percentage(100 / n_rows as u16))
    }
   
    for _ in 0..n_cols {
        cols_in_row.push(Constraint::Percentage(100 / n_cols as u16))
    }
    let mut rows = vec![];
    let mut grid= vec![];


    loop {
        rows = vec![];
        grid = vec![];
        terminal.draw(|frame| {
            for _ in 0..n_rows {
                rows.push(Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(rows_in_grid.clone())
                    .split(frame.size()))
            }
            for (i, r) in rows.clone().into_iter().enumerate() {
                    grid.push(Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(cols_in_row.clone())
                        .split(r[i])
                    )
            }
            
            for (i, g) in grid.clone().into_iter().enumerate() {
                for j in 0..g.len() {
                    if gol.game[i %n_rows][j % n_cols] {
                        frame.render_widget(
                            Paragraph::new(format!("{} {}", i % n_rows, i % n_cols))
                                .black()
                                .on_green()
                                .block(Block::new()
                                    .borders(Borders::ALL)
                                    .padding(Padding::zero())
                                    .style(Style::new().red().on_black().bold().italic())),
                            g[j]
                        );
                    }else {
                        frame.render_widget(
                            Paragraph::new(format!("i:{} / j:{} g.len: {} grid len: {}", i, j, g.len(), grid.len()))
                                .green()
                                .on_black()
                                .block(Block::new()
                                    .borders(Borders::ALL)
                                    .padding(Padding::zero())
                                    .style(Style::new().red().on_black().bold().italic())),
                            g[j]
                        );
                    }
                }
            }
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                {
                    break;
                }
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('g')
                {
                    gol.update(1, 1);
                }
            }
        }
    
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
