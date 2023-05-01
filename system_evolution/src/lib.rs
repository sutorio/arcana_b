use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
pub fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let width = 10.;
    let height = 0.1;

    commands.spawn((
        Ground,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(width))),
            material: materials.add(Color::rgb_u8(216, 216, 216).into()),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(width / 2., height, width / 2.),
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
pub struct ControlledActor;



#[no_mangle]
pub fn setup_controlled_actor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        ControlledActor,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(89, 89, 89).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
        }
    ));
}

#[no_mangle]
pub fn controlled_movement(
    mut query: Query<(&mut Transform, &ActionState<MovementAction>), With<ControlledActor>>
) {
    
    let (mut transform, action_state) = query.single_mut();
    let forward_vector = transform.forward();

    for action in action_state.get_pressed() {
        use MovementAction::*;

        match action {
            Forward =>  transform.translation -= forward_vector * 0.01,
            Backward =>  transform.translation += forward_vector * 0.01,
            Left => transform.rotate_local_y(-0.0175),
            Right => transform.rotate_local_y(0.0175),
        }
    }
}

