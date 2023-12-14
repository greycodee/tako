use std::result::Result;
use std::{error::Error, io};

use crossterm::event::KeyCode;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    core::start_mulicast_server();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut termianl = Terminal::new(backend)?;

    // TODO somethine
    let res = run_app(&mut termianl);

    disable_raw_mode()?;
    execute!(
        termianl.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    termianl.show_cursor()?;
    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(termianl: &mut Terminal<B>) -> io::Result<()> {
    loop {
        termianl.draw(ui)?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui(frame: &mut Frame) {
    frame.render_widget(Paragraph::new("hello"), frame.size());
}
