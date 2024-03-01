use std::ops::Deref;
use std::sync::MutexGuard;
use crate::{STATE, str};
use crate::options::{Game, Options};

#[derive(Clone)]
pub struct State {
    pub options: Options,
    pub selected_game: String,
    pub require_admin: bool
}

impl State {
    /// Returns the global state instance.
    pub fn instance() -> MutexGuard<'static, State> {
        STATE.deref().lock().unwrap()
    }

    /// Fetch options without needing to drop the state.
    pub fn options() -> Options {
        let state = State::instance();
        state.options.clone()
    }

    /// Fetch the game configuration options for what is set.
    pub fn game() -> Game {
        let state = State::instance();
        state.options.game_from_name(&state.selected_game)
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            options: Options::default(),
            selected_game: str!("genshin"),
            require_admin: false
        }
    }
}
