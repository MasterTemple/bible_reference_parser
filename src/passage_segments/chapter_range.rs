use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::{Deref, DerefMut}};
use crate::{impl_parsable_segment, segment::PassageSegment};
use super::{chapter_verse::ChapterVerse, range_pair::RangePair};

/// - This is a range of verse references across a multiple chapters
/// - Ex: `1:2-3:4` in `John 1:2-3:4`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub struct ChapterRange(RangePair<ChapterVerse>);

impl PartialOrd for ChapterRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.start.chapter.cmp(&other.start.chapter)
            .then(self.start.verse.cmp(&other.start.verse))
            .then(self.end.chapter.cmp(&other.end.chapter))
            .then(self.end.verse.cmp(&other.end.verse))
        )
    }
}

impl Deref for ChapterRange {
    type Target = RangePair<ChapterVerse>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChapterRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ChapterRange {
    pub fn new(start_chapter: u8, start_verse: u8, end_chapter: u8, end_verse: u8) -> Self {
        ChapterRange(RangePair{
            start: ChapterVerse::new(
                start_chapter,
                start_verse,
            ),
            end: ChapterVerse::new(
                end_chapter,
                end_verse,
            ),
        })
    }
}

impl Into<PassageSegment> for ChapterRange {
    fn into(self) -> PassageSegment {
        PassageSegment::ChapterRange(self)
    }
}

impl Display for ChapterRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}:{}-{}", self.start.chapter, self.start.verse, self.end.chapter, self.end.verse)
    }
}

impl_parsable_segment!(ChapterRange, "{}-{}:{}-{}");
