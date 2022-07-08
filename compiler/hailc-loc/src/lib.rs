//! Debugging locations in Hail source code.

use std::marker::PhantomData;
use std::fmt::Debug;
use std::ops::Range;

/// If an object is indexable, it can be referenced to by an [`Idx`].
/// 
/// [`Idx`]s can be displayed to the terminal, which is why this trait must be implemented if an object is indexable.  It allows displaying the
/// name of this type as a string.
pub trait Indexable {
    /// Returns the name of this indexable object.
    fn name() -> String;
}

/// An index for SSA-style data structures.
#[derive(Clone, Copy, PartialEq)]
pub struct Idx<T: Indexable> {
    /// The raw number index.
    idx: u32,

    /// The phantom data, of course.
    phantom: PhantomData<T>,
}

impl<T: Indexable> Idx<T> {
    /// Constructs an [`Idx`] from a [`u32`].
    pub fn from_u32(idx: u32) -> Self {
        Self { idx, phantom: PhantomData }
    }

    /// Constructs an [`Idx`] from a [`usize`].
    /// 
    /// If the `idx` is greater than the maximum value of a [`u32`], then it will be clamped down the maximum value of a [`u32`] (lossy).
    pub fn from_usize(idx: usize) -> Self {
        let u32_idx = if idx > u32::MAX as usize {
            u32::MAX
        } else {
            idx.try_into().unwrap()
        };

        Self::from_u32(u32_idx)
    }

    /// Converts `idx` into an [`Idx`] value, if possible.
    pub fn convert<Num: TryInto<u32>>(idx: Num) -> Result<Self, Num::Error> {
        Ok(Self { idx: idx.try_into()?, phantom: PhantomData })
    }

    /// Converts this [`Idx`] into a [`u32`], returning the underlying index value.
    pub fn as_u32(&self) -> u32 {
        self.idx
    }

    /// Converts this [`Idx`] into a [`usize`], returning the underlying index value.  Slightly less efficient than [`Idx::as_u32`].
    pub fn as_usize(&self) -> usize {
        self.idx as usize
    }
}

impl<T: Indexable> Into<u32> for Idx<T> {
    fn into(self) -> u32 {
        self.idx
    }
}

impl<T: Indexable> Into<usize> for Idx<T> {
    fn into(self) -> usize {
        self.idx as usize
    }
}

impl<T: Indexable> ToString for Idx<T> {
    fn to_string(&self) -> String {
        format!("Idx::<{}>({})", T::name(), self.idx)
    }
}

impl<T: Indexable> Debug for Idx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

/// A type that references the name of a file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Source;

impl Indexable for Source {
    fn name() -> String {
        "Source".to_string()
    }
}

/// A location in the source file.
/// 
/// [`Loc`]s are usually locations of tokens, AST items or other trees that are used in Hailc.  They are used in diagnostics if something goes
/// wrong.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Loc {
    /// The starting index of the location.
    start: u32,

    /// The ending index of the location.
    end: u32,

    /// The name of the file.
    source: Idx<Source>,
}

impl Loc {
    /// Creates a location from a range of [`u32`]s.
    pub fn from_u32_range(range: Range<u32>, source: Idx<Source>) -> Self {
        Self { start: range.start, end: range.end, source }
    }

    /// Creates a location from a range of [`usize`]s.  Less efficient than [`Loc::from_u32_range`].
    pub fn from_usize_range(range: Range<usize>, source: Idx<Source>) -> Self {
        Self { start: range.start as u32, end: range.end as u32, source }
    }

    /// Returns the starting index of this location.
    pub fn start(&self) -> u32 {
        self.start
    }

    /// Returns the ending index of this location.
    pub fn end(&self) -> u32 {
        self.end
    }
    
    /// Returns the source file of this location.
    pub fn source(&self) -> Idx<Source> {
        self.source
    }
}