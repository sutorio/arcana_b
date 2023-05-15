use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::fn_plugin::FnPluginExt;
use leafwing_input_manager::prelude::*;
use spew::prelude::*;

// Each different type of actor entity has a plugin
// TODO: Each different type of actor entity has a module
// The plugin answers these questions:
//
// - what components do different types of entities have?
// - what is the effect of the input on each different entity?
//
// ## Common characteristics (REVIEW: subject to change)
//
// Actors use a 3D platformer-ish style movement.
// No strafe (yet)
// Mesh-driven: forward/back do so along mesh's forward/back direction, left/right rotate mesh.
//
// ## Entity types
//
// *No physics* - just uses transforms to teleport around. No rigidbody, no character controller. Basically a ghost.
// *Kinematic velocity-based* - has a rigidbody & a character controller, velocity adjustments determine movement.

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub(crate) enum ActorAction {
    Move,
}

pub(crate) fn actor_input_plugin(app: &mut App) {
    let mut input_map = InputMap::default();

    input_map.insert(
        VirtualDPad {
            up: KeyCode::Up.into(),
            right: KeyCode::Right.into(),
            down: KeyCode::Down.into(),
            left: KeyCode::Left.into(),
        },
        ActorAction::Move,
    );
    input_map.insert(
        DualAxis::left_stick(),
        ActorAction::Move
    );

    app
        .add_plugin(InputManagerPlugin::<ActorAction>::default())
        .insert_resource(input_map);
}

fn update_movement_input(
    raw_input: Res<ActionState<RawInputAction>>,
    mut movement_input: ResMut<MovementInput>,
) {
    match raw_input.action_data(RawInputAction::Move).axis_pair {
        Some(dual_axis_data) => {
            movement_input.update_active_dpad_move(dual_axis_data.x(), dual_axis_data.y());
        }
        None => {
            movement_input.update_inactive_dpad_move();
        }
    }
}

/// A player can only control one actor at a time, this component should be switched between the available actors.
// pub struct CurrentlyControlledActor;

/// Actors that will be spawned into the playground -- see https://github.com/janhohenheim/spew/tree/main#usage
#[derive(Debug, Eq, PartialEq)]
pub enum Actors {
    NoPhysics,
    KinematicVelocityBased,
}


pub fn actor_spawn_plugin(app: &mut App) {
    app
        .add_plugin(SpewPlugin::<Actors, Transform>::default())
        .add_spawners(
            (Actors::NoPhysics, spawn_no_physics_actor),
            (Actors::KinematicVelocityBased, spawn_kinematic_velocity_based_actor),
        )
        .add_systems(Startup, spawn_actors);
        .add_systems(Update, no_physics_actor_movememt);
}

#[derive(Component)]
pub struct NoPhysicsActor;

fn spawn_no_physics_actor(In(transform): In<Transform>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("No physics"),
            NoPhysicsActor,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb_u8(89, 89, 89).into()),
                transform,
                ..default()
            },
        ));
}

fn no_physics_actor_movement(
    mut query: Query<(&ActionState<ActorAction>, &mut Transform), With<NoPysicsActor>>,
    input: Res<InputMap>,
    time: Res<Time>,
) {
    let (action_state, mut transform) = query.single_mut();
    let forward_vector = transform.forward();

    if action_state.pressed(ActorAction::Move) {
        if let Some(dual_axis_data) = match action_state.clamped_axis_pair(ActorAction::Move) {
            // transform.translation += forward_vector * dual_axis_data.x() * dual_axis_data.length();
            // transform.rotate_local_y(dual_axis_data_y());
            debug!("x-axis data: {:?}", dual_axis_data.x());
            debug!("y-axis data: {:?}", dual_axis_data.y());
            debug!("dual-axis data length: {:?}", dual_axis_data.length());
        }
    }
}

#[derive(Component)]
pub struct KinematicVelocityBasedActor;

fn spawn_kinematic_velocity_based_actor(In(transform): In<Transform>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("Partial physics (kinematic, velocity)"),
            Collider::cuboid(0.5, 0.5, 0.5),
            Damping {
                linear_damping: 1.5,
                angular_damping: 0.5,
            },
            KinematicCharacterController::default(),
            KinematicVelocityBasedActor,
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb_u8(89, 89, 89).into()),
                transform,
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            Velocity::zero(),
        ));
    
}

fn spawn_actors(mut spawn_events: EventWriter<SpawnEvent<Actors, Transform>>) {
    spawn_events.send(SpawnEvent::with_data(
        Actors::NoPhysics,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    spawn_events.send(SpawnEvent::with_data(
        Actors::KinematicVelocityBased,
        Transform::from_xyz(2.0, 0.0, 0.0),
    ));
}




