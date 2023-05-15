use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use input_indirection::MovementInput;

pub mod input_indirection;
pub mod playground;

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
pub struct Actor;

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

pub fn setup_actor(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        ActorMovement::default(),
        Actor,
        Collider::cuboid(0.5, 0.5, 0.5),
        Damping {
            linear_damping: 1.5,
            angular_damping: 0.5,
        },
        // GravityScale::default(),
        KinematicCharacterController::default(),
        RigidBody::KinematicVelocityBased,
        Velocity::zero(),
    ));
}

// const MAX_SPEED: f32 = 10.0;

pub fn controlled_movement(
    mut query: Query<(&mut ActorMovement, &mut Transform, &mut Velocity), With<Actor>>,
    input: Res<MovementInput>,
    time: Res<Time>,
) {
    // THere will only ever be one controlled character (as things stand):
    let (mover, mut transform, mut vel) = query.single_mut();

    vel.linvel = (transform.forward() * input.y_axis) * mover.max_speed * time.delta_seconds();
    transform.rotate_y(input.x_axis * mover.rotation_speed * time.delta_seconds());
}

pub fn camera_follow(
    mut actor_cam_set: ParamSet<(
        Query<(Option<&KinematicCharacterControllerOutput>, &Transform), With<Actor>>,
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
