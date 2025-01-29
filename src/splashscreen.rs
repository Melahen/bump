use bevy::prelude::*;
use crate::splashscreen::SplashscreenState::Loading;
use bevy::asset::LoadState;

pub struct SplashScreenPlugin;

#[derive(Resource, Default)]
struct LogoAssetsLoading(Vec<Sprite>);

#[derive(Component)]
struct Logo;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SplashscreenState {
    #[default]
    LoadCall,
    Loading,
    AlphaInc,
    AlphaDec,
    Unload
}

fn load_call(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LogoAssetsLoading>,
    mut app_state: ResMut<NextState<SplashscreenState>>)
{
    let logo_sprite = Sprite {
        image: asset_server.load("images/bmw_logo.png"),
        color: Color::from(Srgba::new(0., 0., 0., 0.)),
        custom_size: Some(Vec2::new(100., 100.)),
        ..default()
    };

    loading.0.push(logo_sprite);
    app_state.set(Loading);
}

fn loading(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut loading: ResMut<LogoAssetsLoading>,
    mut app_state: ResMut<NextState<SplashscreenState>>
) {

    match server.get_load_state(loading.0[0].image.id()) {
        Some(LoadState::Loaded) => {
            commands.spawn((
                Transform::from_xyz(0., 0., -1.),
                loading.0.pop().unwrap(),
                Logo
            ));
            app_state.set(SplashscreenState::AlphaInc);
            println!("Loaded logo");
        }
        Some(_) => {}
        None => {}
    }
}

fn increase_alpha(mut app_state: ResMut<NextState<SplashscreenState>>) {
    app_state.set(SplashscreenState::AlphaDec);
}

fn decrease_alpha(mut app_state: ResMut<NextState<SplashscreenState>>) {
    app_state.set(SplashscreenState::Unload);
}

fn unload(mut app_state: ResMut<NextState<SplashscreenState>>) {

}


impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LogoAssetsLoading>()
            .init_state::<SplashscreenState>()
            .add_systems(OnEnter(SplashscreenState::LoadCall),  load_call)
            .add_systems(Update, loading.run_if(in_state(Loading)))
        ;
    }
}