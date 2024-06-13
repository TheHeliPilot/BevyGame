use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use iyes_perf_ui::prelude::*;
use rand::Rng;

const PLAYER_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(TilemapPlugin)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .init_gizmo_group::<MyGizmos>()
        .add_systems(Startup, startup)
        .add_systems(Update, move_player)
        .add_systems(Update, move_camera)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyGizmos;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("Dirt-Tile-Set-2.png");

    let map_size = TilemapSize { x: 256, y: 256 };
    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);
    let mut random = rand::thread_rng();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(random.gen_range(0..14)),
                    ..default()
                })
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {
        x: 31.999,
        y: 31.999,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        spacing: TilemapSpacing { x: 1.0, y: 1.0 },
        ..default()
    });

    commands.spawn((Camera2dBundle::default(), PlayerCamera));

    let texture = asset_server.load("F_Body_blueEyes.png");
    let layout =
        TextureAtlasLayout::from_grid(Vec2::new(31.0, 37.0), 1, 2, Some(Vec2::new(0.0, 6.0)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Player,
    ));

    commands.spawn(PerfUiCompleteBundle::default());
}

fn move_player(
    mut query_player: Query<(&mut Transform, &mut TextureAtlas), With<Player>>,
    mut gizmos: Gizmos<MyGizmos>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut player = query_player.single_mut();
    let mut direction = Vec3::new(0.0, 0.0, 0.0);

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.y > 0. {
        player.1.index = 1;
    } else {
        player.1.index = 0;
    }

    player.0.translation += direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();

    gizmos.circle_2d(player.0.translation.xy(), 2., Color::RED);
}

fn move_camera(
    mut query_camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    query_player: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query_camera.single_mut();
    let player_position = query_player.single();

    transform.translation = transform
        .translation
        .lerp(player_position.translation, 0.93 * time.delta_seconds());
}
