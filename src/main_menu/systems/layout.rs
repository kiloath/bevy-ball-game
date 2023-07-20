use bevy::prelude::*;

pub fn spawn_main_menu(mut commands: Commands) {
    build_main_menu(&mut commands);
}

pub fn build_main_menu(commands: &mut Commands) {
    commands.spawn(NodeBundle {
        background_color: Color::RED.into(),
        ..default()
    });
}
