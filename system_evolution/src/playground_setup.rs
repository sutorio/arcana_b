//! Playground
//!
//! `playround` is literally the playground environment in which the game
//! exists. At root, it's a squat primitive shape with a collider attached,
//! used as an arena to move things around.
//!
//! The plugin exists to set the window and this up.
//! It is also a test of the RON file asset loader. I want to specify configs
//! in an external file, this is the testbed for that.
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


/// Initialises the stage upon which the actors perform, and tosses some obstacles onto it.
pub struct PlaygroundSetupPlugin;

impl Plugin for PlaygroundSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_play_ground, spawn_obstacles).chain());
    }
}


#[derive(Component)]
pub struct PlayGround;

// TODO: move these to settings
const GROUND_SIZE: f32 = 50.0;
const GROUND_HEIGHT: f32 = 0.1;

fn spawn_play_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PlayGround,
        Collider::cuboid(GROUND_SIZE / 2.0, GROUND_HEIGHT / 2.0, GROUND_SIZE / 2.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(GROUND_SIZE, GROUND_HEIGHT, GROUND_SIZE))),
            material: materials.add(Color::rgba_u8(216, 216, 216, 255).into()),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct Obstacle;

pub fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Obstacle,
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba_u8(89, 89, 89, 255).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
    ));
}
