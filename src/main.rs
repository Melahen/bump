mod camera;
mod splashscreen;
mod states;
mod menu;

use bevy::prelude::*;
use camera::CameraPlugin;
use crate::splashscreen::SplashScreenPlugin;
use crate::states::StatesPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(StatesPlugin)
        .add_plugins(SplashScreenPlugin)

        .run();
}