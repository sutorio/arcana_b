//! WIP: The core camera.
//! 
//! This is moving towards an emulation of a Godot camera attached to a spring arm.
//! So the focus of the camera "arm" entity should be placed at the centre of the target
//! (*ie* the currently controlled character). It's just a SpacialBundle. This provides
//! the Transform component. Then attached as a child is the camera entity. It being a
//! child means that the transform applied to it will be relative (move it up Y, 
//! move it back X, look at the focus). Following Godot, what the camera also requires
//! is a collider: this can then be used to adjust the "arm". Also need to look at some 
//! spring physics.
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
            .add_systems(Update, orbit_camera_arm_movement);
    }
}

/// The resource that defines where the `OrbitCamera` is centred: it is held in a resource
/// to allow it to be updated depending on the current focus.
#[derive(Resource)]
pub struct OrbitCameraFocus {
    pub target_position: Vec3,
    pub current_rotation: f32,
}

// FIXME: This is a placeholder.
impl Default for OrbitCameraFocus {
    fn default() -> Self {
        info!("START: current rotation: {:?}", Quat::IDENTITY);
        Self {
            target_position: Vec3::new(0.0, 1.0, 0.0),
            current_rotation: 0.0,
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

/// The orbit camera itself.
#[derive(Component)]
pub struct OrbitCamera;

/// The "arm" upon which the camera sits. The arm rotates, the camera looks at the focus of the arm.
#[derive(Component)]
pub struct OrbitCameraArm;

/// Initial setup for the orbit camera.
fn spawn_orbit_camera(
    mut commands: Commands,
    focus: Res<OrbitCameraFocus>,
) {
    let arm = commands.spawn((
        OrbitCameraArm,
        InputManagerBundle::<OrbitCameraAction> {
            input_map: build_orbit_camera_control_mapping(),
            ..default()
        },
        SpatialBundle {
            transform: Transform {
                translation: focus.target_position,
                ..default()
            },
            ..default()
        },
    )).id();

    let camera = commands.spawn((
        Camera3dBundle {
            transform: Transform {
                 translation: Vec3::new(-10.0, 5.0, 0.0),
                 ..default()
            }.looking_at(focus.target_position, Vec3::Y),
            ..default()
        },
        Collider::ball(1.0),
        OrbitCamera,
    )).id();

    commands.entity(arm).push_children(&[camera]);
}

/// TODO: need to move camera in or out (using y-axis from axis pair)
/// TODO: when the target moves, the rotaion needs to *smoothly* move back to its original position (HOW THE FUCK DO I DO THIS?)
fn orbit_camera_arm_movement(
    mut query: Query<(&ActionState<OrbitCameraAction>, &mut Transform), With<OrbitCameraArm>>,
    mut focus: ResMut<OrbitCameraFocus>,
    time: Res<Time>,
) {
    const ROTATION_SEQ: EulerRot = EulerRot::XYZ;

    let (action_state, mut transform) = query.single_mut();

    if action_state.pressed(OrbitCameraAction::Move) {
        if let Some(axis_data) = action_state.clamped_axis_pair(OrbitCameraAction::Move) {
            focus.current_rotation += axis_data.x() * time.delta_seconds();
            transform.rotation = Quat::from_euler(ROTATION_SEQ, 0.0, focus.current_rotation, 0.0);
        };
    }
}