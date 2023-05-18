use bevy::prelude::*;

pub enum SettingsEvent {
    RapierDebuggingToggled,
    RapierDebuggingLinesOnTopToggled,
}

/// Where should the camera be focussed? This group of events is used to transfer that information.
/// When a player takes control of a given actor, the camera should move to focus on that actor.
/// NOTE: although the switched/moved event variants carry the same information, they are separated
/// because the UI behaviour is not the same.
pub enum FocusEvent {
    /// The camera focus has been switched to a new target -- the camera should swoop to its
    /// new resting place.
    FocusSwitched { position: Vec3, y_rotation: f32 },
    /// The actor the camera is focussing on has moved -- the camera focus simply updates
    /// to keep it in view, retaining rotation etc.
    FocusMoved { position: Vec3, y_rotation: f32 },
}


pub struct EventBusPlugin;

impl Plugin for EventBusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SettingsEvent>()
            .add_event::<FocusEvent>();
    }
}