use super::magnet::Magnet;
use bevy::prelude::{shape::Box, *};
use bevy_rapier3d::prelude::RigidBody;

const BAR_Y: f32 = 0.0;
const BAR_Z: f32 = 0.0;
const BAR_OFFSET: Vec3 = Vec3::new(0.0, BAR_Y, BAR_Z);

const CARRIER_Y: f32 = 0.0;
const CARRIER_OFFSET: Vec3 = Vec3::new(0.0, CARRIER_Y, 0.0);
pub struct FramePlugin;

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FrameColors>()
            .add_startup_system(create_frame)
            .add_startup_system(create_moving_bar)
            .add_startup_system(create_carrier)
            .add_system(move_bar_and_carrier);
    }
}

//The location of hte bar and carrier depend on the location of the magnet.
fn move_bar_and_carrier(
    magnet_query: Query<(&mut Transform, &mut Magnet, Without<Bar>, Without<Carrier>)>,
    mut bar_query: Query<(&mut Transform, With<Bar>, Without<Magnet>, Without<Carrier>)>,
    mut carrier_query: Query<(&mut Transform, With<Carrier>, Without<Magnet>, Without<Bar>)>,
) {
    let (mut bar_transform, _, _, _) = bar_query.get_single_mut().unwrap();
    let (mut carrier_transform, _, _, _) = carrier_query.get_single_mut().unwrap();
    let (magnet_transform, _, _, _) = magnet_query.get_single().unwrap();

    bar_transform.translation.z = magnet_transform.translation.z + 1.25;
    carrier_transform.translation.z = magnet_transform.translation.z + 1.25;
    carrier_transform.translation.x = magnet_transform.translation.x + 1.25;
}

#[derive(Resource)]
pub struct FrameColors {
    frame: Handle<StandardMaterial>,
    bar: Handle<StandardMaterial>,
    carrier: Handle<StandardMaterial>,
    pub magnet: Handle<StandardMaterial>,
}

impl FromWorld for FrameColors {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let frame = materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 1.0),
            metallic: 1.0,
            ..default()
        });
        let bar = materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 1.0),
            metallic: 1.0,
            ..default()
        });
        let carrier = materials.add(Color::rgb(0.5, 0.3, 0.0).into());
        let magnet = materials.add(StandardMaterial {
            base_color: Color::rgb(0.9, 0.9, 0.9),
            metallic: 1.0,
            ..default()
        });

        FrameColors {
            frame,
            bar,
            carrier,
            magnet,
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Bar {}
#[derive(Component, Copy, Clone, Debug)]
pub struct Carrier {}

//Creates the moving bar that carries the carrier, which is a box shaped, kinematic position based rigid body.
fn create_moving_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -2.5,
        max_x: 9.5,
        min_y: -1.25,
        max_y: -1.0,
        min_z: -1.5,
        max_z: -1.0,
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.bar.clone(),
            transform: Transform::from_translation(BAR_OFFSET),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Bar {});
}

//Creates the carrier that carries the magnet, which is a box shaped, kinematic position based rigid body.
fn create_carrier(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
) {
    let mesh = meshes.add(Mesh::from(shape::Box {
        min_x: -1.75,
        max_x: -0.75,
        min_y: -1.0,
        max_y: -0.45,
        min_z: -1.75,
        max_z: -0.75,
    }));
    commands
        .spawn(PbrBundle {
            mesh,
            material: colors.carrier.clone(),
            transform: Transform::from_translation(CARRIER_OFFSET),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Carrier {});
}

///The frame consists of 4 bars and 4 pillars on which the board rests, this function creates the frame.
fn create_frame(commands: Commands, meshes: ResMut<Assets<Mesh>>, colors: Res<FrameColors>) {
    let frame_shapes = vec![
        Box {
            min_x: -2.5,
            max_x: -2.0,
            min_y: -1.75,
            max_y: -1.25,
            min_z: -3.5,
            max_z: 10.5,
        },
        Box {
            min_x: 9.0,
            max_x: 9.5,
            min_y: -1.75,
            max_y: -1.25,
            min_z: -3.5,
            max_z: 10.5,
        },
        Box {
            min_x: -2.5,
            max_x: 9.5,
            min_y: -1.75,
            max_y: -1.25,
            min_z: -3.5,
            max_z: -3.0,
        },
        Box {
            min_x: -2.5,
            max_x: 9.5,
            min_y: -1.75,
            max_y: -1.25,
            min_z: 10.0,
            max_z: 10.5,
        },
        Box {
            min_x: -2.5,
            max_x: -2.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: 10.0,
            max_z: 10.5,
        },
        Box {
            min_x: -2.5,
            max_x: -2.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: -3.5,
            max_z: -3.0,
        },
        Box {
            min_x: 9.5,
            max_x: 9.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: -3.5,
            max_z: -3.0,
        },
        Box {
            min_x: 9.5,
            max_x: 9.0,
            min_y: -1.25,
            max_y: -0.25,
            min_z: 10.5,
            max_z: 10.0,
        },
    ];
    create_part_of_frame(commands, meshes, colors, frame_shapes);
}

/// Creates the subshapes out of which the whole frame exists.
fn create_part_of_frame(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<FrameColors>,
    shapes: Vec<Box>,
) {
    for shape in shapes {
        let mesh = meshes.add(Mesh::from(shape)).clone();
        commands
            .spawn(PbrBundle {
                mesh,
                material: colors.frame.clone(),
                ..default()
            })
            .insert(RigidBody::Fixed);
    }
}
