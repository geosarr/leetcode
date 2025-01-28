//! Contains Rust solutions for Leetcode problems.

use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Display, Error},
    hash::Hash,
};

pub mod structure;

/// Contains all easy problems, based on leetcode difficulty levels nomenclature.
pub mod easy;

pub trait Number
where
    Self: std::ops::Add<Self, Output = Self> + PartialOrd + Sized + Copy,
{
    fn max() -> Self;
    fn zero() -> Self;
}
macro_rules! impl_number {
    ($typ:ty) => {
        impl Number for $typ {
            fn max() -> Self {
                <$typ>::MAX
            }
            fn zero() -> Self {
                0 as Self
            }
        }
    };
}
impl_number!(usize);
impl_number!(i32);
impl_number!(u32);
