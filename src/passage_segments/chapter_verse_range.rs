use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use crate::{impl_parsable_segment, segment::PassageSegment};
use super::range_pair::RangePair;

/// - This is a range of verse references within a single chapter
/// - Ex: `1:2-3` `John 1:2-3`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChapterVerseRange {
    pub chapter: usize,
    pub verses: RangePair<usize>,
}

impl ChapterVerseRange {
    pub fn new(chapter: usize, start_verse: usize, end_verse: usize) -> Self {
        ChapterVerseRange {
            chapter,
            verses: RangePair {
                start: start_verse,
                end: end_verse,
            },
        }
    }
}

impl Into<PassageSegment> for ChapterVerseRange {
    fn into(self) -> PassageSegment {
        PassageSegment::ChapterVerseRange(self)
    }
}

impl Display for ChapterVerseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.chapter, self.verses.start, self.verses.end)
    }
}

impl_parsable_segment!(ChapterVerseRange, "{}:{}-{}");
