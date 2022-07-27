use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub},
};

use bevy::prelude::*;
use euclid::Angle;
use serde::{Deserialize, Serialize};

use crate::board::{step::BoardDirection, visualisation::TILE_SIZE};

#[derive(Default, Deref, DerefMut, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Vec2Board(Vec2);

impl Vec2Board {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn from_uvec2_middle(uvec2: &UVec2) -> Self {
        Self::new(uvec2.x as f32 + 0.5, uvec2.y as f32 + 0.5)
    }

    pub fn from_uvec2_tilesize_middle(uvec2: &UVec2, tile_size: f32) -> Self {
        Self::new(
            uvec2.x as f32 + tile_size * 0.5,
            uvec2.y as f32 + tile_size * 0.5,
        )
    }

    // Only for not diagonal vec2
    pub fn distance_from_zero(&self) -> f32 {
        self.x.abs() + self.y.abs()
    }

    //    pub fn distance(&self, other: Vec2Board) -> f32 {
    //((self.x - other.x).powi(2) - (self.y - other.y).powi(2))
    //.sqrt()
    //.abs()
    //}

    pub fn add_in_direction(&mut self, distance: f32, direction: BoardDirection) {
        match direction {
            BoardDirection::North => self.0.y += distance,
            BoardDirection::East => self.0.x += distance,
            BoardDirection::South => self.0.y -= distance,
            BoardDirection::West => self.0.x -= distance,
        };
    }

    pub fn degre_between_y(&self, other: Vec2Board) -> Angle<f32> {
        let b = self.distance(other.into());
        let c = (self.y - other.y).abs();
        Angle::degrees((b * c).acos())
    }

    pub fn to_scaled_vec3(self, z: f32) -> Vec3 {
        Vec3::new(self.x * TILE_SIZE, self.y * TILE_SIZE, z)
    }
}

impl Display for Vec2Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Vec2> for Vec2Board {
    fn from(vec2: Vec2) -> Self {
        Self(vec2)
    }
}

impl From<Vec2Board> for Vec2 {
    fn from(vec2: Vec2Board) -> Self {
        Self::new(vec2.x, vec2.y)
    }
}

impl From<UVec2> for Vec2Board {
    fn from(uvec2: UVec2) -> Self {
        Self(uvec2.as_vec2())
    }
}

impl Add for Vec2Board {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2Board {
    fn add_assign(&mut self, other: Vec2Board) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Mul for Vec2Board {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl Sub for Vec2Board {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}
