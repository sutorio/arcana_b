use bevy::prelude::*;

/// The setup data for the application itself. This includes window data and relevant configuration
/// for the Bevy plugins included within the game. Basically, anything that has to happen *before*
/// anything related to the game is loaded - *eg* the window title cannot be set after the window
/// itself exists.
pub struct ApplicationSetupPlugin {
    pub title: String,
    pub base_colour: Color,
}

impl Plugin for ApplicationSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(self.base_colour))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window { 
                    title: self.title.clone(),
                    ..default()
                }),
                ..default()
            }))
            .add_systems(Update, bevy::window::close_on_esc);
    }
}
