use bevy::prelude::*;
use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_systems(Startup, start)
        .add_systems(Update, update)
        .run();
}

fn start(
    mut commands: Commands
){
    commands.spawn(Camera2dBundle::default());

    commands.spawn(PerfUiCompleteBundle::default());
}

fn update(
    mut commands: Commands
){
    
}