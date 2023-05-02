use bevy::prelude::*;
use bevy_mod_picking::{Hover, PickableBundle, Selection};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SquareColors>()
            .add_startup_system(create_board)
            .add_system(color_squares);
    }
}

#[derive(Resource)]
struct SquareColors {
    white: Handle<StandardMaterial>,
    black: Handle<StandardMaterial>,
    white_hovered: Handle<StandardMaterial>,
    black_hovered: Handle<StandardMaterial>,
    white_selected: Handle<StandardMaterial>,
    black_selected: Handle<StandardMaterial>,
}

impl FromWorld for SquareColors {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let white = materials.add(Color::rgb(0.7, 0.6, 0.6).into());
        let black = materials.add(Color::rgb(0.3, 0.2, 0.2).into());
        let white_hovered = materials.add(Color::rgb(0.8, 0.7, 0.7).into());
        let black_hovered = materials.add(Color::rgb(0.4, 0.3, 0.3).into());
        let white_selected = materials.add(Color::rgb(0.8, 0.7, 1.0).into());
        let black_selected = materials.add(Color::rgb(0.4, 0.3, 0.6).into());

        SquareColors {
            white,
            black,
            white_hovered,
            black_hovered,
            white_selected,
            black_selected,
        }
    }
}

#[derive(Component)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<SquareColors>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 1.,
        ..default()
    }));

    // Spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    // Change material according to position to get alternating pattern
                    material: if (i + j + 1) % 2 == 0 {
                        colors.white.clone()
                    } else {
                        colors.black.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                    ..Default::default()
                })
                .insert(PickableBundle::default())
                .insert(Square { x: i, y: j });
        }
    }
}

fn color_squares(
    mut query: Query<(&Square, &mut Handle<StandardMaterial>, &Hover, &Selection)>,
    colors: Res<SquareColors>,
) {
    for (square, mut handle, hover, selection) in query.iter_mut() {
        if selection.selected() {
            *handle = if square.is_white() {
                colors.white_selected.clone()
            } else {
                colors.black_selected.clone()
            }
        } else if hover.hovered() {
            *handle = if square.is_white() {
                colors.white_hovered.clone()
            } else {
                colors.black_hovered.clone()
            }
        }
    }
}
