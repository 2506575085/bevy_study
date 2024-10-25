use bevy::prelude::*;

use asset_loader::AssetLoaderPlugin;
// use spaceship::SpaceshipSystemPlugin;
use gravity_system::GravitySystemPlugin;

mod spaceship;
mod asset_loader;
mod gravity_system;
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 100.0,
            ..default()
        })
        .add_plugins(DefaultPlugins) // DefaultPlugins需要放在前面，否则会panic
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(GravitySystemPlugin)
        // .add_plugins(SpaceshipSystemPlugin)
        .run();
}

