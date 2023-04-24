use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[cfg(not(feature = "reload"))]
use amassment_dev_systems::*;
#[cfg(feature = "reload")]
use amassment_dev_systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod amassment_dev_systems_hot {
    use bevy::prelude::*;
    hot_functions_from_file!("../amassment_dev_systems/src/lib.rs");
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy development playground with hot reloading".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_systems(
            Startup,
            (
                setup_ground,
                setup_example_object,
                setup_camera,
                setup_light,
            ),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
