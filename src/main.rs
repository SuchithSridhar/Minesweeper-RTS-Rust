use bevy::prelude::*;


fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        width: 640.0,
        height: 360.0,
        title: "Minesweeper Showdown".to_string(),
        resizable: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .run();
}
