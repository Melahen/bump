use bevy::app::{App, Plugin};
use bevy::prelude::{AppExtStates, States};

pub struct StatesPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splashscreen,
    Menu,
    Game
}


impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}
