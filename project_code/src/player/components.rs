use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub animation_state: SpriteState,
    pub timer: Timer,
}

/// Velocity struct
#[derive(Component)]
pub struct Velocity {
    pub v: Vec2,
}

/// Velocity implementation
impl Velocity {
    pub fn new() -> Self {
        Self {
            //sets x and y velocity dsto 0
            v: Vec2::splat(0.),
        }
    }
}

/// Struct for the time between frames of animation
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    /// Initializes the animation timer
    pub fn new(timer:Timer) -> AnimationTimer {
        AnimationTimer(timer)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationFrameCount(usize);

impl AnimationFrameCount {
    /// Initializes the animation frame count
    pub fn new(size: usize) -> AnimationFrameCount {
        AnimationFrameCount(size)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SpriteState {
        Idle,
        LeftRun,
        RightRun,
}

impl SpriteState {
    /// Matches the player animation to the current player
    /// state
    pub fn animation_indices(&self) -> std::ops::Range<usize> {
        match self {
            SpriteState::Idle => 0..8,
            SpriteState::LeftRun => 8..16,
            SpriteState::RightRun => 16..24,
        }
    }

    /// Matches the speed of animation to the animation being
    /// played
    pub fn animation_speed(&self) -> f32 {
        match self {
            SpriteState::Idle => 0.1,
            SpriteState::LeftRun => 0.1,
            SpriteState::RightRun => 0.1,
        }
    }
}