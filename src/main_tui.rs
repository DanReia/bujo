mod ui;
use ui::draw;
use ui::Event;
mod app;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    // Setup backend and event loops
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

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

    // Create default app state
    let mut app = App::new("My title");
    loop {
        // Draw UI
        terminal.draw(|f| draw(f, &mut app))?;

        // Listen for events
        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Right => app.on_right(),
                KeyCode::Left => app.on_left(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            } // _ => {}
        }

        if app.should_quit {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                // DisableMouseCapture //thread 'main' panicked at 'Original console mode not set'
            )?;
            break;
        }
    }
    Ok(())
}
