use bevy::prelude::*;

use crate::utils::{despawn_all_of, GameState};

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(
                Update,
                (animation, timer).run_if(in_state(GameState::Splash)),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(GameState::Splash), despawn_all_of::<OnLoadingScreen>);
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnLoadingScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Component)]
struct LogoImage;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_bundle = ImageBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            width: Val::Px(200.),
            height: Val::Px(200.),
            ..default()
        },
        image: UiImage::new(asset_server.load("textures/bevy-icon.png")),
        ..default()
    };
    // Display the logo
    commands.spawn(image_bundle).insert(OnLoadingScreen);

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1., TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn animation(mut query: Query<&mut Transform, With<OnLoadingScreen>>) {
    let mut transform = query.single_mut();
    let x = transform.scale.x;
    let y = transform.scale.x;
    let z = transform.scale.x;
    transform.scale = Vec3::from((x + 0.01, y + 0.01, z + 0.01));
    transform.rotate(Quat::from_rotation_z(0.01));
}

fn timer(mut state: ResMut<NextState<GameState>>, time: Res<Time>) {
    if time.elapsed_seconds() >= 2. {
        state.set(GameState::Menu);
    }
}
