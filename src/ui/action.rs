use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::slice::Iter;

use crate::ui::key::UiKey;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UiAction {
    Quit,
}

impl UiAction {
    pub fn iterator() -> Iter<'static, Self> {
        static ACTIONS: [UiAction; 1] = [UiAction::Quit];
        ACTIONS.iter()
    }

    pub fn keys(&self) -> &[UiKey] {
        match self {
            UiAction::Quit => &[UiKey::Ctrl('c'), UiKey::Char('q')],
        }
    }
}

impl Display for UiAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            UiAction::Quit => "Action[Quit]",
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct UiActions(Vec<UiAction>);

impl UiActions {
    pub fn find(&self, key: UiKey) -> Option<&UiAction> {
        UiAction::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    pub fn actions(&self) -> &[UiAction] {
        self.0.as_slice()
    }
}

impl From<Vec<UiAction>> for UiActions {
    fn from(actions: Vec<UiAction>) -> Self {
        let mut map: HashMap<UiKey, Vec<UiAction>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                map.entry(*key).or_default().push(*action);
            }
        }

        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1)
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(UiAction::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();

        if !errors.is_empty() {
            panic!("{}", errors.join("; "));
        }

        Self(actions)
    }
}
