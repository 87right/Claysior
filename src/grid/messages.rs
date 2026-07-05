//! # Path: src/messages.rs

use bevy::prelude::*;

#[derive(Message)]
pub struct RightClicked (pub Entity);

#[derive(Message)]
pub struct LeftClicked (pub Entity);

#[derive(Message)]
pub struct NetworkUpdated (pub Entity);

#[derive(Message)]
pub struct Placed (pub Entity);

#[derive(Message)]
pub struct Broken (pub Entity);
