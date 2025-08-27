use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BirdType {
    #[default]
    Regular,
    Custom,
}

#[derive(Resource, Default)]
pub struct BirdSelection(BirdType);

pub fn plugin(app: &mut App) {
    app.init_resource::<BirdSelection>();
}
