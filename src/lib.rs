#![no_std]

//! This crate provides two common marker types, `Const` and `Mut`, for use in distinguishing
//! between mutable and immutable program state.

use core::ops::*;

/// Marks an immutable type.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Const;

/// Marks a mutable type.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mut;

/// Marks whether a type is mutable or immutable.
pub trait Mutability: private::MutabilityInner {
    /// The reference type associated with this mutability.
    type Ref<'a, T: 'a + ?Sized>: Deref<Target = T>;
}

impl Mutability for Const {
    type Ref<'a, T: 'a + ?Sized> = &'a T;
}

impl Mutability for Mut {
    type Ref<'a, T: 'a + ?Sized> = &'a mut T;
}

/// Hides implementation details.
mod private {
    use super::*;

    /// Seals the mutability marker types.
    pub trait MutabilityInner: 'static + Copy + Clone + Send + Sync + core::fmt::Debug + Default + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash {}

    impl MutabilityInner for Const {}
    impl MutabilityInner for Mut {}
}