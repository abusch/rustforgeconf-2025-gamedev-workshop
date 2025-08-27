use bevy::{log, prelude::*};

use crate::{GameState, bird_selector::BirdType};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), setup);
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            StateScoped(GameState::Playing),
        ))
        .with_children(|children| {
            children
                .spawn(button("bird 1"))
                .observe(|trigger: Trigger<Pointer<Click>>| on_click(trigger, BirdType::Regular));
            children
                .spawn(button("bird 2"))
                .observe(|trigger: Trigger<Pointer<Click>>| on_click(trigger, BirdType::Custom));
        });
}

fn button(name: &str) -> impl Bundle {
    (
        Button,
        Node::default(),
        Text::new(name),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextShadow::default(),
    )
}

fn on_click(_: Trigger<Pointer<Click>>, bird_type: BirdType) {
    log::info!("Click {bird_type:?}");
}
