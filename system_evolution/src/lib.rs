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

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum MovementAction {
    Forward,
    Backward,
    Left,
    Right,
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
        InputManagerBundle::<MovementAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::Up, MovementAction::Forward),
                (KeyCode::Down, MovementAction::Backward),
                (KeyCode::Left, MovementAction::Left),
                (KeyCode::Right, MovementAction::Right),
            ]),
        },
    ));
}

pub fn controlled_movement(
    mut query: Query<
        (
            &mut Transform,
            &mut KinematicCharacterController,
            &ActionState<MovementAction>,
        ),
        With<ControlledActor>,
    >,
    time: Res<Time>,
) {
    let (mut transform, mut controller, action_state) = query.single_mut();
    // let forward_vector = transform.forward();

    for action in action_state.get_pressed() {
        use MovementAction::*;
        // Which way is the mesh facing?
        let forward_vector = transform.forward();

        match action {
            Forward => {
                let forward_speed =
                    -action_state.clamped_value(action) * 5.0 * time.delta_seconds();
                controller.translation = match controller.translation {
                    Some(v) => Some(v + (forward_vector * forward_speed)),
                    None => Some(Vec3::ZERO + (forward_vector * forward_speed)),
                }
            }
            Backward => {
                let forward_speed = action_state.clamped_value(action) * 0.1 * time.delta_seconds();
                controller.translation = match controller.translation {
                    Some(v) => Some(v + (forward_vector * forward_speed)),
                    None => Some(Vec3::ZERO),
                }
            }
            Left => transform.rotate_local_y(-0.0175),
            Right => transform.rotate_local_y(0.0175),
        }
    }
}
