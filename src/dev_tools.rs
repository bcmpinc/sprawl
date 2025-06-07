//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{
        picking_debug::{DebugPickingMode, DebugPickingPlugin},
        states::log_transitions
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
    ui::UiDebugOptions,
};

use crate::screens::Screen;

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;
const PICKING_DEBUG_KEY: KeyCode = KeyCode::F3;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DebugPickingPlugin);

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    #[cfg(feature = "dev_native")] {
        use bevy_inspector_egui::{
            bevy_egui::{EguiPlugin, EguiGlobalSettings},
            quick::WorldInspectorPlugin
        };

        app.insert_resource(EguiGlobalSettings{
            enable_focused_non_window_context_updates: true,
            input_system_settings: default(),
            enable_absorb_bevy_input_system: true,
        });

        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
        ));
    }

    app.add_systems(
        PreUpdate,
        (|mut mode: ResMut<DebugPickingMode>| {
            *mode = match *mode {
                DebugPickingMode::Disabled => DebugPickingMode::Normal,
                DebugPickingMode::Normal => DebugPickingMode::Noisy,
                DebugPickingMode::Noisy => DebugPickingMode::Disabled,
            }
        }).distributive_run_if(input_just_pressed(
            PICKING_DEBUG_KEY,
        )),
    );

}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
