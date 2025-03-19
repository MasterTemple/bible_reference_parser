use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};

use crate::books::{book_data::BookInfo, book_segments::BookPassageSegments};

use super::segment::{any_segment::AnySegment, segment::SegmentFns};

#[derive(Clone, Debug, Serialize, Deserialize, IntoIterator, Deref, DerefMut)]
pub struct PassageSegments(pub Vec<AnySegment>);

impl PassageSegments {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_book<'a>(self, book_info: BookInfo<'a>) -> BookPassageSegments<'a> {
        BookPassageSegments {
            book: book_info,
            segments: self,
        }
    }

    // pub fn overlaps_segment(&self, other: impl Into<PassageSegment>) -> bool {
    pub fn overlaps_with(&self, other: &impl SegmentFns) -> bool {
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
    // use super::BookPassageSegments;

    // #[test]
    // fn test() {
    //     let segs = BookPassageSegments::parse(1, "1:1,3-4").unwrap();
    //     // segs.into_iter()
    //     for seg in segs {
    //         println!("{seg:#?}");
    //     }
    // }
}
