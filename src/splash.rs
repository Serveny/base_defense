use bevy::prelude::*;

use crate::utils::{despawn_all_of, GameState};

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
            // While in this state, run the `countdown` system
            .add_system_set(
                SystemSet::on_update(GameState::Splash)
                    .with_system(animation)
                    .with_system(timer),
            )
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(
                SystemSet::on_exit(GameState::Splash)
                    .with_system(despawn_all_of::<OnLoadingScreen>),
            );
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
            // This will center the logo
            margin: UiRect::all(Val::Auto),
            // This will set the logo to be 200px wide, and auto adjust its height
            size: Size::new(Val::Px(200.0), Val::Auto),
            ..default()
        },
        image: UiImage(asset_server.load("textures/bevy-icon.png")),
        ..default()
    };
    // Display the logo
    commands.spawn(image_bundle).insert(OnLoadingScreen);

    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1., TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn animation(mut query: Query<(&mut Transform, With<OnLoadingScreen>)>) {
    let (mut transform, _) = query.single_mut();
    let x = transform.scale.x;
    let y = transform.scale.x;
    let z = transform.scale.x;
    transform.scale = Vec3::from((x + 0.01, y + 0.01, z + 0.01));
    transform.rotate(Quat::from_rotation_z(0.01));
}

fn timer(mut state: ResMut<State<GameState>>, time: Res<Time>) {
    if time.elapsed_seconds() >= 2. {
        state.set(GameState::Menu).unwrap_or_default();
    }
}
