use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid}, core_pipeline::tonemapping::Tonemapping,
    render,
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_panorbit_camera::{
    PanOrbitCamera,
    PanOrbitCameraPlugin,
};
use bevy_vector_shapes::prelude::*;
use serde::Deserialize;

use point_visualizer::{
    PointVisualizerApp,
    utils::setup_hooks,
};


#[derive(Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-7234-7423-7421-4b8b380a2c46"]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-6233-6345-7534-4b8b380a2c46"]
pub struct Location {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-7234-1753-5413-4b8b380a2c46"]
pub struct Point {
    pub color: Color,
    pub highlight: bool, // TODO: implement highlight animation
    pub location: Location,
    pub size: f32,
}

#[derive(Deserialize, TypeUuid, TypePath, Debug)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
struct Points {
    points: Vec<Point>,
}

#[derive(Resource)]
struct PointsHandle(Handle<Points>);


fn point_visualizer_app() {
    App::new()
        .add_plugins((
            PointVisualizerApp::default(),
            JsonAssetPlugin::<Points>::new(&[
                "points.json",
            ]),
            PanOrbitCameraPlugin,
            ShapePlugin {
                base_config: ShapeConfig {
                    alignment: Alignment::Billboard,
                    ..ShapeConfig::default_3d()
                },
                ..default()
            }
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_points)
        .insert_resource(Msaa::Off)
        .run();
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(
        PointsHandle(asset_server.load("tsne_p100_i1000.points.json"))
    );

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            tonemapping: Tonemapping::None,
            ..default()
        },
        PanOrbitCamera {
            button_pan: MouseButton::Middle,
            ..default()
        },
    ));
}

fn draw_points(
    point_groups: ResMut<Assets<Points>>,
    mut shapes: ShapePainter
) {
    for point_group in point_groups.iter() {
        for point in point_group.1.points.iter() {
            shapes.set_translation(Vec3::new(point.location.x, point.location.y, point.location.z));
            shapes.color = render::color::Color::rgba(point.color.r, point.color.g, point.color.b, point.color.a);
            shapes.circle(point.size);
        }
    }
}


pub fn main() {
    setup_hooks();
    point_visualizer_app();
}
