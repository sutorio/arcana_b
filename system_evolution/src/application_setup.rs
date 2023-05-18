use bevy::prelude::*;

/// The setup data for the application itself. This includes window data and relevant configuration
/// for the Bevy plugins included within the game. Basically, anything that has to happen *before*
/// anything related to the game is loaded - *eg* the window title cannot be set after the window
/// itself exists.
/// TODO: load in initial config from a config file, don't hardcode anything here.
pub struct ApplicationSetupPlugin {
    pub title: String,
    pub base_colour: Color,
}

impl Plugin for ApplicationSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(self.base_colour))
            .add_plugins(DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window { 
                        title: self.title.clone(),
                        ..default()
                    }),
                    ..default()
                })
                .set(
                    AssetPlugin {
                        watch_for_changes: true,
                        ..Default::default()
                    },
                )
            )
            .add_systems(PreStartup, load_ui_assets)
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

#[derive(Resource)]
struct UIFontBold(Handle<Font>);

fn load_ui_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ui_font_bold = asset_server.load("fonts/Satoshi-Black.otf");

    commands
        .insert_resource(UIFontBold(ui_font_bold));

}
