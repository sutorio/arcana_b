pub mod actor_setup;
pub mod application_setup;
pub mod camera_setup;
pub mod events;
pub mod playground_setup;
pub mod physics_setup;

use bevy::prelude::*;

// TODO: pull config from a file:
use application_setup::*;
// TODO: eveything is going to use this, figure out some way to keep it sane:
use events::*;
// TODO:IMPORTANT: *need to be able to reset everything via an in-game button from here on in*:
// TODO: add a physics config resource + in-game controls:
use physics_setup::*;
// TODO: add in-game controls for the initial camera setup (orbit etc). Will move to focussing on current actor once that is implemented?:
use camera_setup::*;
// TODO: need *all* of this configured via an external resource, need to then be able to adjust this resource in-game:
use playground_setup::*;

use actor_setup::*;

// TODO: EVENTS and GLOBAL RESOURCES
fn main() {
    App::new()
        .add_plugin(ApplicationSetupPlugin { title: "Arcana B".into(), base_colour: Color::lcha(71.0, 28.0, 23.0, 1.0) })
        .add_plugin(EventBusPlugin)
        .add_plugin(PhysicsSetupPlugin { rapier_debugging_enabled: true, rapier_debugging_lines_on_top: true })
        .add_plugin(PlaygroundSetupPlugin)
        .add_plugin(OrbitCameraSetupPlugin { initial_position: Vec3::new(-2.0, 10.0, 10.0), initial_framing: Vec2::new(-0.5, -0.5) })
        .add_plugin(ActorSetupPlugin)
        .run();
}
