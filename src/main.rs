use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Ball;

const FLOOR_Y: f32 = -200.;
const FLOOR_HEIGHT: f32 = 40.;
const FLOOR_WEIGHT: f32 = 400.;
const BALL_RADIUS: f32 = 50.;

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
        // Physics
        RigidBody::Dynamic,
        ExternalImpulse::default(),
        Collider::ball(BALL_RADIUS),
        LockedAxes::ROTATION_LOCKED,
    ));

    // Floor
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(FLOOR_WEIGHT, FLOOR_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::BLACK)),
            transform: Transform::from_translation(Vec3::new(0., FLOOR_Y, 1.)),
            ..default()
        },
        // Physics
        Collider::cuboid(FLOOR_WEIGHT / 2., FLOOR_HEIGHT / 2.),
        Restitution {
            coefficient: 1.15,
            ..Default::default()
        },
        Friction {
            coefficient: 0.,
            ..Default::default()
        },
    ));
}

fn handle_debug_commands(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Ball>>) {
    if keys.just_pressed(KeyCode::Key0) {
        info!("Reset ball position");
        for mut transform in query.iter_mut() {
            transform.translation.y = 0.;
        }
    }
}

fn move_ball_system(keys: Res<Input<KeyCode>>, mut query: Query<&mut ExternalImpulse, With<Ball>>) {
    for mut impulse in query.iter_mut() {
        if keys.pressed(KeyCode::D) {
            // Move ball to right
            impulse.impulse = Vec2::new(10000., 0.);
        }
        if keys.pressed(KeyCode::A) {
            // Move ball to left
            impulse.impulse = Vec2::new(-10000., 0.);
        }
        if keys.just_pressed(KeyCode::W) {
            // Move ball up
            impulse.impulse = Vec2::new(0., 1000000.);
        }
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, handle_debug_commands)
            .add_systems(Update, move_ball_system);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(RapierPhysicsPlugin::<()>::default());
    app.add_plugins((DefaultPlugins, GamePlugin));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin {
        mode: DebugRenderMode::all(),
        ..Default::default()
    });

    app.run();
}
