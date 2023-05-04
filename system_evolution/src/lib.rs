use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use input_indirection::MovementInput;

pub mod input_indirection;

#[derive(Component)]
pub struct MainCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, -1.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
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
            material: materials.add(Color::rgba_u8(216, 216, 216, 255).into()),
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

#[derive(Component)]
pub struct ControlledActor;

#[derive(Component)]
pub struct ActorMovement {
    speed: f32,
    max_speed: f32,
    rotation_speed: f32,
}

impl Default for ActorMovement {
    fn default() -> Self {
        Self {
            speed: 0.0,
            max_speed: 1000.0,
            rotation_speed: 5.0,
        }
    }
}

pub fn setup_controlled_actor(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        ActorMovement::default(),
        ControlledActor,
        Collider::cuboid(0.5, 0.5, 0.5),
        GravityScale::default(),
        KinematicCharacterController::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba_u8(89, 89, 89, 255).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        RigidBody::KinematicVelocityBased,
        Velocity::zero(),
    ));
}

// const MAX_SPEED: f32 = 10.0;

pub fn controlled_movement(
    mut query: Query<(&mut ActorMovement, &mut Transform, &mut Velocity), With<ControlledActor>>,
    movement_input: Res<MovementInput>,
    time: Res<Time>,
) {
    // THere will only ever be one controlled character (as things stand):
    let (mut actor_movement, mut transform, mut velocity) = query.single_mut();

    let speed = (actor_movement.speed
        + (if movement_input.is_held {
            100.0
        } else {
            -100.0
        }))
    .clamp(0.0, actor_movement.max_speed);

    velocity.linvel = (transform.forward() * movement_input.y_axis) * speed * time.delta_seconds();
    transform
        .rotate_y(movement_input.x_axis * actor_movement.rotation_speed * time.delta_seconds());
    actor_movement.speed = speed;
    // let forward_vector = transform.forward();
    // let forward_speed = movement_input.y_axis * 5.0 * time.delta_seconds();

    // controller.translation = match controller.translation {
    //     Some(v) => Some(v + (forward_vector * forward_speed)),
    //     None => Some(Vec3::ZERO + (forward_vector * forward_speed)),
    // };
    // transform.rotate_local_y(movement_input.x_axis * 0.0175);
}

pub fn camera_follow(
    mut actor_cam_set: ParamSet<(
        Query<(Option<&KinematicCharacterControllerOutput>, &Transform), With<ControlledActor>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
) {
    let mut translation = Vec3::ZERO;
    let mut target = Vec3::ZERO;

    for (controller_output, controller_transform) in actor_cam_set.p0().iter() {
        if let Some(output) = controller_output {
            translation = output.effective_translation;
        }
        target = controller_transform.translation;
    }

    for mut camera_transform in actor_cam_set.p1().iter_mut() {
        camera_transform.translation += translation;
        camera_transform.look_at(target, Vec3::Y);
    }
}
