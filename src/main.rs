mod consts;
mod game_state;
mod player;
mod tree;

use game_state::AppStatePlugin;
use player::PlayerPlugin;
use tree::TreePlugin;

use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Same color as tilemap's BG
const COLOR_BACKGROUND: Color = Color::rgb(229.0 / 255.0, 247.0 / 255.0, 211.0 / 255.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::default())
        // basic systems
        .add_systems(Update, close_on_esc)
        .add_systems(Startup, setup_camera)
        // plugins
        .add_plugins(AppStatePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(TreePlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
