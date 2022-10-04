use bevy::prelude;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;

fn main() {
    prelude::App::new()
    .insert_resource(prelude::WindowDescriptor {
        width: WIDTH, 
        height: HEIGHT,
        title: "Minesweeper Showdown".to_string(),
        resizable: true,
        ..Default::default()
    })
    .add_plugins(prelude::DefaultPlugins)
    .run();
}
