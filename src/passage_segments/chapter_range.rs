use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}};

use crate::segment::PassageSegment;

use super::{chapter_verse::ChapterVerse, range_pair::RangePair};

/// - This is a range of verse references across a multiple chapters
/// - Ex: `1:2-3:4` in `John 1:2-3:4`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChapterRange(RangePair<ChapterVerse>);
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
    pub fn new(start_chapter: usize, start_verse: usize, end_chapter: usize, end_verse: usize) -> Self {
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
