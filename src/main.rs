use bevy::prelude::*;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel}
};
use kayak_ui::bevy::BevyKayakUIPlugin;

mod button;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(button::button_system)
        .add_plugin(BevyKayakUIPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("icon.png"),
        ..Default::default()
    });

    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

   
}

