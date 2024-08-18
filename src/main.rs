use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    }, layout::{self, Layout, Rect}, widgets::{Block, Paragraph}, Frame, Terminal
};



fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(start_page)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello World!").block(Block::bordered().title("Greeting")),
        frame.size(),
    );
}

fn start_page(frame: &mut Frame){
    use ratatui::prelude::*;

    let outer_layer = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10), 
            Constraint::Percentage(90),
        ])
        .split(frame.size()); 

    let sub_outer_layer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30), 
            Constraint::Percentage(70),
        ])
        .split(outer_layer[1]); 

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(sub_outer_layer[0]);

    frame.render_widget(
        Paragraph::new("Serchfield")
        .block(Block::bordered().title("Serch filed")),
       outer_layer[0]
    );    
    frame.render_widget(
        Paragraph::new("Liabary")
            .block(Block::bordered().title("Liabary")),
        layout[0]);

    frame.render_widget(
        Paragraph::new("playlist")
            .block(Block::bordered().title("playlist")),
        layout[1]);

    frame.render_widget(
        Paragraph::new("Welcome")
            .block(Block::bordered().title("Welcome")),
        sub_outer_layer[1]);
}