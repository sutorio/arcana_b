use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

/// Initialises the orbit camera and its associated functionality
pub struct OrbitCameraSetupPlugin;

impl Plugin for OrbitCameraSetupPlugin {
    fn build(&self, app: &mut App) {
        
        app
            .init_resource::<OrbitCameraFocus>()
            .add_plugin(InputManagerPlugin::<OrbitCameraAction>::default())
            .add_systems(Startup, spawn_orbit_camera)
            .add_systems(Update, orbit_camera_movement);
    }
}

/// The resource that defines where the `OrbitCamera` is centred: it is held in a resource
/// to allow it to be updated depending on the current focus.
#[derive(Resource)]
pub struct OrbitCameraFocus {
    pub position: Vec3,
    pub rotation: f32,
}

// FIXME: This is a placeholder.
impl Default for OrbitCameraFocus {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 1.0, 0.0),
            rotation: 0.0,
        }
    }
}

/// Input actions related to the orbit camera (TODO: link Leafwing input manager docs).
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum OrbitCameraAction {
    Move,
}

/// Input mapping for the orbit camera controls (TODO: link Leafwing input manager).
fn build_orbit_camera_control_mapping() -> InputMap<OrbitCameraAction> {
    let mut input_map = InputMap::default();

    input_map.insert(
        VirtualDPad {
            up: KeyCode::Up.into(),
            right: KeyCode::Right.into(),
            down: KeyCode::Down.into(),
            left: KeyCode::Left.into(),
        },
        OrbitCameraAction::Move,
    );

    input_map.insert(
        DualAxis::left_stick(),
        OrbitCameraAction::Move
    );

    input_map
}

/// The orbit camera is attached to an "arm" (in reality just a point in space that can be rotated).
/// This massively simplifies the control of the camera -- all it needs to do is point to the focus of the arm. 
#[derive(Component)]
pub struct OrbitCameraArm;

/// The orbit camera itself.
#[derive(Component)]
pub struct OrbitCamera;

/// Initial setup for the orbit camera.
fn spawn_orbit_camera(
    mut commands: Commands,
    focus: Res<OrbitCameraFocus>,
) {
    let camera_arm = commands.spawn((
        OrbitCameraArm,
        SpatialBundle {
            transform: Transform { 
                translation: focus.position, 
                rotation: Quat::from_axis_angle(Vec3::Y, focus.rotation),
                ..default()
            },
            ..default()
        },
        InputManagerBundle::<OrbitCameraAction> {
            input_map: build_orbit_camera_control_mapping(),
            ..default()
        },
    )).id();

    let camera = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-10.0, 5.0, 0.0).looking_at(focus.position, Vec3::Y),
            ..default()
        },
        Collider::ball(1.0),
        OrbitCamera,
    )).id();

    commands.entity(camera_arm).push_children(&[camera]);
}


fn orbit_camera_movement(
    mut arm_query: Query<(&ActionState<OrbitCameraAction>, &mut Transform), With<OrbitCameraArm>>,
    mut focus: ResMut<OrbitCameraFocus>,
    time: Res<Time>,
) {
    let (action_state, mut arm_transform) = arm_query.single_mut();

    if action_state.pressed(OrbitCameraAction::Move) {
        if let Some(axis_data) = action_state.clamped_axis_pair(OrbitCameraAction::Move) {
            info!("Move action for camera: axis data x = {:?}", axis_data.x());
            focus.rotation += axis_data.x() * 0.01 * time.delta_seconds();
            arm_transform.rotate_y(focus.rotation);
            // NOTE: just set this again? otherwise it'll not update properly:
            // camera_transform.look_at(focus.position, Vec3::Y);
        };
    }
}