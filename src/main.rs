use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

#[cfg(not(feature = "reload"))]
use system_evolution::*;
#[cfg(feature = "reload")]
use system_evolution_hot::*;

#[allow(unused_imports)]
#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "system_evolution")]
mod system_evolution_hot {
    use bevy::prelude::*;
    use bevy_rapier3d::prelude::*;
    use leafwing_input_manager::prelude::*;
    use system_evolution::*;
    hot_functions_from_file!("system_evolution/src/lib.rs");
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(223, 156, 156)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy development playground with hot reloading".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(InputManagerPlugin::<MovementAction>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                setup_ground,
                setup_controlled_actor,
                setup_camera,
                setup_light,
            ),
        )
        .add_systems(Update, controlled_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
