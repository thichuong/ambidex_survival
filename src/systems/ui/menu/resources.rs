use crate::components::player::HandType;
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveDescriptionSide(pub HandType);

impl Default for ActiveDescriptionSide {
    fn default() -> Self {
        Self(HandType::Left)
    }
}
