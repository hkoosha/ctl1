extern crate core;

use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::{AppReturn, AppState, AppTui};
use crate::ui::draw::draw;
use crate::ui::events::{UiEvents, UiInputEvent};

pub mod app;
pub mod ui;
pub mod stick;

pub async fn start_ui(ui_state: Arc<Mutex<AppState>>) -> eyre::Result<()> {
    let mut app_tui = AppTui::default();

    let stdout = stdout();

    crossterm::terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(40);
    let mut events = UiEvents::new(tick_rate);

    loop {
        terminal.draw(|rect| draw(rect, &app_tui, &ui_state))?;

        let result = match events.next().await {
            UiInputEvent::Input(key) => app_tui.do_action(key).await,
            UiInputEvent::Tick => AppReturn::Continue,
        };

        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
