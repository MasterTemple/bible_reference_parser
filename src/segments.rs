use std::{fmt::Debug, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::{compare::SegmentCompare, segment::PassageSegment};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookPassageSegments {
    book: u8,
    segments: PassageSegments,
}

impl BookPassageSegments {
    pub fn parse(book: u8, segment_input: &str) -> Result<Self, String> {
        Ok(Self {
            book,
            segments: PassageSegments::parse(segment_input)?,
        })
    }

    pub fn overlaps_with(&self, other: &BookPassageSegments) -> bool {
        if self.book != other.book { return false; }
        self.segments.contains_overlap(&other.segments)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassageSegments(pub Vec<PassageSegment>);

impl Deref for PassageSegments {
    type Target = Vec<PassageSegment>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PassageSegments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PassageSegments {
    pub fn new() -> Self {
        Self(vec![])
    }

    // pub fn overlaps_segment(&self, other: impl Into<PassageSegment>) -> bool {
    pub fn overlaps_with(&self, other: &impl SegmentCompare) -> bool {
        self.iter().any(|this| this.overlaps_with(other))
    }

    /// - This can be better optimized, but that is not a priority right now
    /// - I just need some way to order the segments and do it in linear time
    pub fn contains_overlap(&self, other: &PassageSegments) -> bool {
        self.iter().any(|this| other.overlaps_with(this))
    }
}
