use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use system_evolution::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(223, 156, 156)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy development playground".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(InputManagerPlugin::<MovementAction>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            always_on_top: true,
            ..default()
        })
        .add_systems(
            Startup,
            (
                setup_base_environment,
                setup_controlled_actor,
                setup_obstacles,
                setup_camera,
                setup_light,
            ),
        )
        .add_systems(Update, controlled_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
