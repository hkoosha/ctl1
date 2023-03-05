use std::str::FromStr;
use std::sync::{Arc, Mutex};

use ctl1::app::AppState;
use ctl1::start_ui;
use ctl1::stick::read_ctls;

fn init_log() {
    let level_filter = if let Ok(level) = std::env::var("CTL1_LOG") {
        log::LevelFilter::from_str(&level).unwrap_or(log::LevelFilter::Info)
    } else {
        log::LevelFilter::Info
    };

    tui_logger::init_logger(level_filter).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Info);
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> eyre::Result<()> {
    init_log();

    let (mut ctl_rx, _) = read_ctls();

    let app_state = Arc::new(Mutex::new(AppState::default()));
    let ui_app_state = app_state.clone();

    tokio::spawn(async move {
        while let Some(ctl_event) = ctl_rx.recv().await {
            let mut app = app_state.lock().unwrap();
            app.ensure_ctl(ctl_event.ctl_id, ctl_event.ctl_name);
            let previous = app.current.remove(&ctl_event.ctl_id).unwrap();
            let current = previous.updated(ctl_event.triggering_event);
            app.previous.insert(ctl_event.ctl_id, previous);
            app.current.insert(ctl_event.ctl_id, current);
        }
    });

    start_ui(ui_app_state).await?;

    println!();
    Ok(())
}
