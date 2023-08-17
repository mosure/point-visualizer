use bevy::{
    prelude::*,
    app::AppExit,
    diagnostic::{
        DiagnosticsStore,
        FrameTimeDiagnosticsPlugin,
    },
    render::{
        RenderPlugin,
        settings::{
            WgpuFeatures,
            WgpuSettings,
        },
    },
};

pub mod utils;


pub struct PointVisualizerApp {
    esc_close: bool,
    show_fps: bool,
    width: f32,
    height: f32,
    name: String,
}

impl Default for PointVisualizerApp {
    fn default() -> PointVisualizerApp {
        PointVisualizerApp {
            esc_close: true,
            show_fps: true,
            width: 1920.0,
            height: 1080.0,
            name: "point visualizer".to_string(),
        }
    }
}

impl Plugin for PointVisualizerApp {
    fn build(&self, app: &mut App) {
        let mut wgpu_features = WgpuFeatures::default();
        wgpu_features.set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

        app.insert_resource(ClearColor(Color::rgb_u8(112 / 2, 48 / 2, 48 / 2)));
        app.add_plugins(
            DefaultPlugins
            .set(RenderPlugin {
                wgpu_settings: WgpuSettings {
                    features: wgpu_features,
                    ..Default::default()
                }
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    mode: bevy::window::WindowMode::Windowed,
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    resolution: (self.width, self.height).into(),
                    title: self.name.clone(),
                    ..default()
                }),
                ..default()
            })
        );

        if self.esc_close {
            app.add_systems(Update, esc_close);
        }

        if self.show_fps {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
            app.add_systems(Startup, fps_display_setup);
            app.add_systems(Update, fps_update_system);
        }
    }
}


pub fn esc_close(
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}


fn fps_display_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "fps: ",
                TextStyle {
                    font: asset_server.load("fonts/Caveat-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/Caveat-Medium.ttf"),
                font_size: 60.0,
                color: Color::GOLD,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        FpsText,
    ));
}

#[derive(Component)]
struct FpsText;

fn fps_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
