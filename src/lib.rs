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

    /// Allows for choosing between one of two types based upon whether this
    /// is [`Const`] (in which case `I` is chosen) or [`Mut`] (in which case `M` is chosen).
    type Select<I, M>;

    /// Allows for choosing between one of two values based upon whether this
    /// is [`Const`] (in which case `immut` is chosen) or [`Mut`] (in which case `mut` is chosen).
    fn select<I, M>(immut: I, mutable: M) -> Self::Select<I, M>;
}

impl Mutability for Const {
    type Ref<'a, T: 'a + ?Sized> = &'a T;
    type Select<I, M> = I;

    fn select<I, M>(immut: I, _: M) -> Self::Select<I, M> {
        immut
    }
}

impl Mutability for Mut {
    type Ref<'a, T: 'a + ?Sized> = &'a mut T;
    type Select<I, M> = M;

    fn select<I, M>(_: I, mutable: M) -> Self::Select<I, M> {
        mutable
    }
}

/// Hides implementation details.
mod private {
    use super::*;

    /// Seals the mutability marker types.
    pub trait MutabilityInner: 'static + Copy + Clone + Send + Sync + core::fmt::Debug + Default + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash {}

    impl MutabilityInner for Const {}
    impl MutabilityInner for Mut {}
}