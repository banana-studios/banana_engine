use bevy::prelude::*;

pub use bevy;
pub use bevy::app as bevy_app;
pub use bevy::asset as bevy_asset;
pub use bevy::core as bevy_core;
pub use bevy::diagnostic as bevy_diagnostic;
pub use bevy::ecs as bevy_ecs;
pub use bevy::input as bevy_input;
pub use bevy::log as bevy_log;
pub use bevy::math as bevy_math;
pub use bevy::reflect as bevy_reflect;
pub use bevy::scene as bevy_scene;
pub use bevy::tasks as bevy_tasks;
pub use bevy::transform as bevy_transform;
pub use bevy::utils as bevy_utils;
pub use bevy::window as bevy_window;

pub use banana_bevy_utils;
pub use iyes_loopless;
pub use iyes_progress;

pub use bevy_asset_loader;
pub use bevy_common_assets;
pub use bevy_tweening;

#[cfg(feature = "bevy_ecs_tilemap")]
pub use bevy_ecs_tilemap;

#[cfg(feature = "bevy_kira_audio")]
pub use bevy_kira_audio;

pub mod prelude {
    pub use crate::*;
    pub use bevy::prelude::*;

    pub use banana_bevy_utils::prelude::*;
    pub use iyes_loopless::prelude::*;
    pub use iyes_progress::prelude::*;

    pub use bevy_asset_loader::prelude::*;

    #[cfg(feature = "bevy_kira_audio")]
    pub use bevy_kira_audio::*;

    #[cfg(feature = "bevy_ecs_tilemap")]
    pub use bevy_ecs_tilemap::prelude::*;
}

/// Adds and configures all the stuff on top of Bevy
pub struct BananaExtrasPlugin;
impl Plugin for BananaExtrasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_tweening::TweeningPlugin);

        #[cfg(feature = "bevy_kira_audio")]
        app.add_plugin(bevy_kira_audio::AudioPlugin::default());

        #[cfg(feature = "bevy_ecs_tilemap")]
        app.add_plugin(bevy_ecs_tilemap::TilemapPlugin);
    }
}

/// Adds `bevy::DefaultPlugins` and `bananaengine::BananaExtras`
pub struct BananaEverythingPlugin;
impl Plugin for BananaEverythingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::DefaultPlugins).add_plugin(BananaExtrasPlugin);
    }
}
