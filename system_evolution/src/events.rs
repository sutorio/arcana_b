use bevy::prelude::*;

pub enum SettingsEvent {
    RapierDebuggingToggled,
    RapierDebuggingLinesOnTopToggled,
}


pub struct EventBusPlugin;

impl Plugin for EventBusPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SettingsEvent>();
    }
}