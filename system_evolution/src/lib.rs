use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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

#[derive(Component)]
pub struct ExampleObject;

#[no_mangle]
pub fn setup_example_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        ExampleObject,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(89, 89, 89).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));
}

#[no_mangle]
pub fn primitive_movement(
    mut query: Query<&mut Transform, With<ExampleObject>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut transform = query.single_mut();
    let forward_vector = transform.forward();

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::Up => {
                transform.translation -= forward_vector * 0.01;
            }
            KeyCode::Down => {
                transform.translation += forward_vector * 0.01;
            }
            KeyCode::Left => {
                transform.rotate_local_y(-0.0175);
            }
            KeyCode::Right => {
                transform.rotate_local_y(0.0175);
            }
            _ => return,
        }
    }

    // if keyboard_input.pressed(KeyCode::Left) {
    //     // 1 degree
    //     transform.rotate_local_y(-0.0175);
    // }
    // if keyboard_input.pressed(KeyCode::Right) {
    //     // 1 degree
    //     transform.rotate_local_y(0.0175);
    // }
    // TODO: reset
    //if keyboard_input.pressed(KeyCode::Return) {
    //}
}
