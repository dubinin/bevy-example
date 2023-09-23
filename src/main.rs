use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct Ball;

#[derive(Resource)]
struct GameTimer(Timer);

const FLOOR_Y: f32 = -200.;
const FLOOR_HEIGHT: f32 = 40.0;
const BALL_SPEED: f32 = 45.;
const BALL_RADIUS: f32 = 50.;
const BALL_STOP_HEIGHT: f32 = FLOOR_Y + FLOOR_HEIGHT / 2.0 + BALL_RADIUS;

fn move_ball(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Ball>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        if transform.translation.y < BALL_STOP_HEIGHT {
            commands.entity(entity).remove::<Ball>();
            info!("Ball at floor!");
        } else {
            transform.translation.y -= BALL_SPEED * time.delta_seconds();
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Ball
    commands.spawn((
        Ball,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
    ));

    // Floor
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Box::new(400., 40., 0.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLACK)),
        transform: Transform::from_translation(Vec3::new(0., FLOOR_Y, 1.)),
        ..default()
    });
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(Update, move_ball);
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
