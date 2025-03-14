use std::{fmt::Debug, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::{book_segment::BookSegment, compare::SegmentCompare, segment::PassageSegment};

/// todo: create method to iter segments as book segments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookPassageSegments {
    book: u8,
    segments: PassageSegments,
}

// impl IntoIterator for &BookPassageSegments {
//     type Item;
//
//     type IntoIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

// impl Iterator for BookPassageSegments {
//     type Item = BookSegment<PassageSegment>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter()
//     }
// }


impl BookPassageSegments {
    pub fn iter(&self) -> impl Iterator<Item = BookSegment<PassageSegment>> + '_ {
        let book = self.book;
        self.segments.iter().map(move |seg| seg.with_book(book))
    }

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

    pub fn with_book(self, book: u8) -> BookPassageSegments {
        BookPassageSegments {
            book,
            segments: self
        }
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

#[cfg(test)]
mod tests {
    use super::BookPassageSegments;

    #[test]
    fn test() {
        let segs = BookPassageSegments::parse(1, "1:1,3-4").unwrap();
        for seg in segs.iter() {
            println!("{seg:#?}");
        }
    }
}
