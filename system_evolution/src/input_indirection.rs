use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// Detaches input mapping as a specific component, moving it to
/// resource/events that just spit values I want out. Builds on top of
/// Leafwing, but avoids having to add directly to a player.
///
/// I want to be able to switch to different character controllers,
/// I don't want to attach an input map to each one, just one will do.
///
/// TODO: add config
/// TODO: add different types on plugin -- this is a 3d platformer/3rd person kinda thing

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum RawInputAction {
    Move,
}

#[derive(Resource, Default, Debug)]
pub struct MovementInput {
    pub x_axis: f32,
    pub y_axis: f32,
    pub is_held: bool,
}

impl MovementInput {
    fn update_active_dpad_move(&mut self, x_data: f32, y_data: f32) {
        self.y_axis = y_data;
        self.x_axis = x_data;
        self.is_held = true;
    }

    fn update_inactive_dpad_move(&mut self) {
        self.y_axis = 0.0;
        self.x_axis = 0.0;
        self.is_held = false;
    }
}

pub struct ArcanaInputPlugin;

impl Plugin for ArcanaInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<RawInputAction>::default())
            .init_resource::<ActionState<RawInputAction>>()
            .init_resource::<MovementInput>()
            .insert_resource(InputMap::<RawInputAction>::new([(
                VirtualDPad {
                    up: KeyCode::Up.into(),
                    right: KeyCode::Right.into(),
                    down: KeyCode::Down.into(),
                    left: KeyCode::Left.into(),
                },
                RawInputAction::Move,
            )]))
            .add_systems(Update, update_movement_input);
    }
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
