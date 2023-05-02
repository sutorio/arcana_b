use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
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

#[derive(Component)]
pub struct Ground;

pub fn setup_base_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_size = 10.;
    let ground_height = 0.1;

    commands.spawn((
        Ground,
        Collider::cuboid(ground_size / 2.0, ground_height / 2.0, ground_size / 2.0),
        // TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                ground_size,
                ground_height,
                ground_size,
            ))),
            material: materials.add(Color::rgba_u8(216, 216, 216, 192).into()),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct Obstacle;

pub fn setup_obstacles(
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
            transform: Transform::from_xyz(2.0, 1.0, 2.0),
            ..default()
        },
    ));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputAction {
    Move,
}

// #[derive(Component)]
// pub struct Locomotion {
//     max_speed: f32,
//     acceleration: f32,
//     max_acceleration_force: f32,
// }

#[derive(Component)]
pub struct ControlledActor;

pub fn setup_controlled_actor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        ControlledActor,
        KinematicCharacterController::default(),
        RigidBody::KinematicPositionBased,
        // REQUIRES a collider else it does nowt
        Collider::cuboid(0.5, 0.5, 0.5),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba_u8(89, 89, 89, 192).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        InputManagerBundle::<InputAction> {
            input_map: InputMap::new([(
                VirtualDPad {
                    up: KeyCode::Up.into(),
                    right: KeyCode::Right.into(),
                    down: KeyCode::Down.into(),
                    left: KeyCode::Left.into(),
                },
                InputAction::Move,
            )])
            .build(),
            ..default()
        },
    ));
}

pub fn controlled_movement(
    mut query: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            &ActionState<InputAction>,
        ),
        With<ControlledActor>,
    >,
    time: Res<Time>,
) {
    let (mut transform, mut controller, action_state) = query.single_mut();
    let forward_vector = transform.forward();
    if let Some(dual_axis_data) = action_state.action_data(InputAction::Move).axis_pair {
        let forward_speed = dual_axis_data.y() * 5.0 * time.delta_seconds();

        controller.translation = match controller.translation {
            Some(v) => Some(v + (forward_vector * forward_speed)),
            None => Some(Vec3::ZERO + (forward_vector * forward_speed)),
        };
        transform.rotate_local_y(dual_axis_data.x() * 0.0175);
    }
}
