use bevy::{
    asset::{AssetMetaCheck, embedded_asset},
    color::palettes::css,
    prelude::*,
};
use bevy_vello::{VelloPlugin, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    }))
    .add_plugins(VelloPlugin::default())
    .add_systems(Startup, load_svg);
    embedded_asset!(app, "assets/fountain.svg");
    app.run();
}

fn load_svg(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((Camera2d, VelloView));

    let one_third = Val::Percent(100.0 / 3.0);
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: one_third,
            top: one_third,
            width: one_third,
            height: one_third,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BorderColor(css::FUCHSIA.with_alpha(0.5).into()),
        VelloSvgHandle(asset_server.load("embedded://svg_ui/assets/fountain.svg")),
    ));
}
