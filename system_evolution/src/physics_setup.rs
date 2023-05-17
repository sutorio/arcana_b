use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Initialise the physics world: primarily Rapier + the main light.
/// TODO: need an ability to switch Rapier debug on and off via an in-game toggle.
/// TODO: the plugin should accept Rapier config settings & initialise a resource
///       that holds them and allows for in-game adjustment. Ditto for lighting.
pub struct PhysicsSetupPlugin {
    pub rapier_debugging_enabled: bool,
    pub rapier_debugging_lines_on_top: bool,
}

impl Plugin for PhysicsSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin {
                enabled: self.rapier_debugging_enabled,
                always_on_top: self.rapier_debugging_lines_on_top,
                ..default()
            })
            .add_systems(Startup, setup_light);
    }
}

#[derive(Component)]
pub struct MainLight;

pub fn setup_light(mut commands: Commands) {
    commands.spawn((
        MainLight,
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
    ));
}
