//! A small library devoted to adding custom information, like indexing, to enums.
//!
//! Note that these traits are useless when they are not accompanied by
//! the `derive_custom_enum_traits` library, which automatically derives these traits.
//!
//! The utilities should be fairly straightforward ones, such as a custom trait
//! that indexes an enum's placement. I'm using this in a library so I can
//! hold information in an array, where each item in the array refers to
//! a different item in an enum.

/// Index items in an enum (and take an enum index and convert it to an enum variant)
pub trait EnumIndex {
    /// Convert an integer into an enum variant (where the first items are enumerated first, the second items second, etc.)
    fn from_index(idx: usize) -> Self;

    /// Take an enum variant and convert it into an index (where the first listed variant is enumerated first, the second second, etc.)
    fn to_index(&self) -> usize;
}