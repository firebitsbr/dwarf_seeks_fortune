use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    /// An array of values that 'time_scale' can have.
    /// Debug controls will allow switching between these values,
    /// to slow time down or speed it up.
    pub time_scale_presets: Vec<f32>,
    /// How fast the clock is ticking. A value of 1.0 means time is
    /// behaving normally, higher values mean time is sped up and
    /// 0.0 means time is frozen.
    pub time_scale: f32,
    /// The max speed of the player in meters per second.
    pub player_speed: f32,
    /// Number of seconds to leave between frames when rewinding time.
    pub seconds_per_rewind_frame: f32,
    /// Enable this when debugging, to save time when rapidly iterating.
    /// It saves you from having to navigate the menu every time you start the game.
    /// If true, the game will open in the editor state.
    /// If false, it will open on the main menu.
    pub skip_straight_to_editor: bool,
    /// Whether or not to display debug frames indicating the player's discrete position.
    pub display_debug_frames: bool,
}

impl DebugConfig {
    /// Increase the time scale. Everything in the world will move more quickly.
    /// Return a tuple containing the old scale and the new scale.
    /// If the time is already operating at the fastest speed, the time scale will not change.
    pub fn increase_speed(&mut self) -> (f32, f32) {
        let old_time_scale = self.time_scale;
        let new_time_scale = self
            .time_scale_presets
            .iter()
            .find(|&&scale| scale > self.time_scale);
        if let Some(new_time_scale) = new_time_scale {
            self.time_scale = *new_time_scale;
            (old_time_scale, self.time_scale)
        } else {
            (self.time_scale, self.time_scale)
        }
    }

    /// Decrease the time scale. Everything in the world will move more slowly.
    /// Return a tuple containing the old scale and the new scale.
    /// If the time is already operating at the slowest speed, the time scale will not change.
    pub fn decrease_speed(&mut self) -> (f32, f32) {
        let old_time_scale = self.time_scale;
        let new_time_scale = self
            .time_scale_presets
            .iter()
            .rev()
            .find(|&&scale| scale < self.time_scale);
        if let Some(new_time_scale) = new_time_scale {
            self.time_scale = *new_time_scale;
            (old_time_scale, self.time_scale)
        } else {
            (self.time_scale, self.time_scale)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct MovementConfig {
    /// The max speed of the player in meters per second.
    pub player_speed: f32,
    /// How many seconds can pass between starting your jump and starting to move sideways for it to
    /// still register. If you start moving sideways later than that, it will not work and the
    /// character will simply jump straight up into the air instead.
    pub jump_allowance: f32,
}