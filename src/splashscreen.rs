use bevy::prelude::*;
pub struct SplashScreenPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Spawn,
    AlphaInc,
    AlphaDec,
    Despawn
}

fn spawn_splashscreen(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set(GameState::AlphaInc);
}

fn despawn_splashscreen(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set()
}

fn increase_alpha(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set(GameState::AlphaDec);
}

fn decrease_alpha(mut app_state: ResMut<NextState<GameState>>) {
    app_state.set(GameState::Despawn);
}


impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Spawn), spawn_splashscreen)
            .add_system(Update, increase_alpha.run_if(in_state(GameState::AlphaInc)))
            .add_system(Update, increase_alpha.run_if(in_state(GameState::AlphaDec)))
            .add_systems(OnEnter(GameState::Spawn), despawn_splashscreen)
    }
}