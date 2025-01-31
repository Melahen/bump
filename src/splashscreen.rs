use bevy::prelude::*;
use crate::splashscreen::SplashscreenState::*;
use bevy::asset::LoadState;

pub struct SplashScreenPlugin;

#[derive(Resource, Default)]
struct LogoAssetsLoading(Vec<Sprite>);

#[derive(Resource, Default)]
struct FadeInDone(f32);

#[derive(Component)]
struct Logo;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SplashscreenState {
    #[default]
    LoadCall,
    Loading,
    AlphaInc,
    AlphaDec,
    Unload,
    CheckDespawned,
    SplashscreenEnd
}

fn load_call(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LogoAssetsLoading>,
    mut app_state: ResMut<NextState<SplashscreenState>>)
{
    let logo_sprite = Sprite {
        image: asset_server.load("images/alors_la.png"),
        color: Color::from(Srgba::new(1., 1., 1., 0.)),
        custom_size: Some(Vec2::new(500., 500.)),
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
                loading.0.pop().unwrap(),
                Logo
            ));
            app_state.set(AlphaInc);
        }
        Some(_) => {}
        None => {}
    }
}

fn increase_alpha(
    mut query: Query<(&mut Sprite, &Logo)>,
    time: Res<Time>,
    mut fade_in_done: ResMut<FadeInDone>,
    mut app_state: ResMut<NextState<SplashscreenState>>
) {
    let (mut sprite, _) = query.single_mut();
    let fade_duration = 2.1;

    let t = (time.elapsed_secs() / fade_duration).clamp(0.0, 1.0);

    let smoothed = 2_f32.powf(4. * ((t - 0.7) / 0.3 - 1.));
    sprite.color.set_alpha(smoothed);

    if t >= 1.0 {
        app_state.set(AlphaDec);
        fade_in_done.0 = time.elapsed_secs();
    }
}

fn decrease_alpha(
    mut query: Query<(&mut Sprite, &Logo)>,
    time: Res<Time>,
    fade_in_done: Res<FadeInDone>,
    mut app_state: ResMut<NextState<SplashscreenState>>
) {
    let (mut sprite, _) = query.single_mut();
    let fade_duration = 1.1;

    let t = (time.elapsed_secs() - fade_in_done.0) / fade_duration;
    let smoothed = 0.9f32.powf(5. * (t - 0.85) / 0.15 - 1.0);

    sprite.color.set_alpha(smoothed);

    if sprite.color.alpha() < 0.0001 { app_state.set(Unload); }
}

fn unload(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &Logo)>,
    mut app_state: ResMut<NextState<SplashscreenState>>)
{
    let (to_despawn_logo_entity, _, _) = query.single_mut();
    commands.entity(to_despawn_logo_entity).despawn();
    app_state.set(CheckDespawned);
}

fn check_despawned(
    query: Query<(&Sprite, &Logo)>,
    mut app_state: ResMut<NextState<SplashscreenState>>
) {
    if query.is_empty() { app_state.set(SplashscreenEnd); }
}


impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LogoAssetsLoading>()
            .init_resource::<FadeInDone>()
            .init_state::<SplashscreenState>()
            .add_systems(OnEnter(LoadCall),  load_call)
            .add_systems(Update, loading.run_if(in_state(Loading)))
            .add_systems(Update, increase_alpha.run_if(in_state(AlphaInc)))
            .add_systems(Update, decrease_alpha.run_if(in_state(AlphaDec)))
            .add_systems(OnEnter(Unload),  unload)
            .add_systems(Update, check_despawned.run_if(in_state(CheckDespawned)))
        ;
    }
}