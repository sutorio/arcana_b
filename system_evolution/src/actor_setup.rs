use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use super::camera_setup::OrbitCameraParams;

pub struct ActorSetupPlugin;

impl Plugin for ActorSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<ActorAction>::default())
            .init_resource::<ActionState<ActorAction>>()
            .insert_resource(actor_control_mapping())
            .add_systems(Startup, spawn_actor)
            .add_systems(Update, move_actor);
    }
}

/// Input actions related to the orbit camera (TODO: link Leafwing input manager docs).
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum ActorAction {
    Move,
}

/// Input mapping for the orbit camera controls (TODO: link Leafwing input manager).
fn actor_control_mapping() -> InputMap<ActorAction> {
    let mut input_map = InputMap::default();

    input_map.insert(
        VirtualDPad {
            up: KeyCode::W.into(),
            right: KeyCode::D.into(),
            down: KeyCode::S.into(),
            left: KeyCode::A.into(),
        },
        ActorAction::Move,
    );

    input_map.insert(
        DualAxis::left_stick(),
        ActorAction::Move
    );

    input_map
}

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct ControlledActor;

fn spawn_actor(
    mut commands: Commands,
    mut camera_params: ResMut<OrbitCameraParams>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let spawn_location = Vec3::new(0.0, 0.5, 0.0);

    commands
        .spawn((
            Actor,
            ControlledActor,
            Collider::cuboid(0.5, 0.5, 0.5),
            Damping {
                angular_damping: 1.0,
                linear_damping: 1.0,
            },
            KinematicCharacterController::default(),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb_u8(89, 89, 89).into()),
                transform: Transform::from_translation(spawn_location),
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            Velocity::zero(),
        )
    );

    camera_params.tracking_position = spawn_location;
}


fn move_actor(
    mut camera_params: ResMut<OrbitCameraParams>,
    movement_input_action: Res<ActionState<ActorAction>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<ControlledActor>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    if movement_input_action.pressed(ActorAction::Move) {
        if let Some(axis_data) = movement_input_action.clamped_axis_pair(ActorAction::Move) {
            velocity.linvel = transform.forward() * axis_data.y() * 500.0 * time.delta_seconds();
            transform.rotate_y(axis_data.x() * time.delta_seconds());
            camera_params.tracking_position = transform.translation * time.delta_seconds();
        };
    }
}
