use std::collections::HashMap;

use log::debug;
use tui_logger::TuiWidgetState;

use crate::stick::CtlState;
use crate::ui::action::{UiAction, UiActions};
use crate::ui::key::UiKey;

pub struct App {
    actions: UiActions,
    pub tui: TuiWidgetState,
    pub previous: HashMap<u64, CtlState>,
    pub current: HashMap<u64, CtlState>,
    pub name_mapping: HashMap<u64, String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            actions: UiAction::iterator().cloned().collect::<Vec<_>>().into(),
            tui: TuiWidgetState::default(),
            current: HashMap::with_capacity(2),
            previous: HashMap::with_capacity(2),
            name_mapping: HashMap::with_capacity(2),
        }
    }

    pub async fn do_action(&mut self, key: UiKey) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("run action [{}]", action);
            match action {
                UiAction::Quit => AppReturn::Exit,
            }
        } else {
            debug!("no action associated with key [{}]", key);
            AppReturn::Continue
        }
    }

    fn ensure_name(&mut self, id: u64, name: String) {
        self.name_mapping
            .entry(id)
            .or_insert(name);
    }

    fn ensure_state(&mut self, id: u64) {
        self.previous.entry(id).or_insert_with(CtlState::default);
        self.current.entry(id).or_insert_with(CtlState::default);
    }

    pub fn ensure_ctl(&mut self, id: u64, name: String) {
        self.ensure_name(id, name);
        self.ensure_state(id);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}
