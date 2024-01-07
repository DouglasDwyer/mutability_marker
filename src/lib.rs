#![no_std]

//! This crate provides two common marker types, `Const` and `Mut`, for use in distinguishing
//! between mutable and immutable program state.

/// Marks an immutable type.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Const;

/// Marks a mutable type.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mut;

/// Marks whether a type is mutable or immutable.
pub trait Mutability: private::MutabilityInner {}

impl Mutability for Const {}
impl Mutability for Mut {}

/// Hides implementation details.
mod private {
    use super::*;

    /// Seals the mutability marker types.
    pub trait MutabilityInner: 'static + Copy + Clone + Send + Sync + core::fmt::Debug + Default + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash {}

    impl MutabilityInner for Const {}
    impl MutabilityInner for Mut {}
}