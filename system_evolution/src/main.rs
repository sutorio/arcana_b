use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use system_evolution::{
    input_indirection::ArcanaInputPlugin,
    playground::PlaygroundSetupPlugin,
};
use system_evolution::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Arcana B".into(),
                ..default()
            }),
            ..default()
        }))
        // .add_plugin(RonAssetPlugin::<Settings>::new(&["settings.ron"]))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            ..default()
        })
        .add_plugin(ArcanaInputPlugin)
        .add_plugin(PlaygroundSetupPlugin)
        .add_systems(
            Startup,
            (
                setup_actor,
                setup_obstacles,
                setup_camera,
                setup_light,
            ),
        )
        .add_systems(Update, (controlled_movement, camera_follow).chain())
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
