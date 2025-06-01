use bevy::prelude::*;

mod map;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        map::plugin,
    ));
}
