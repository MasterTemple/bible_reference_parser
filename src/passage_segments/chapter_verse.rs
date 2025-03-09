use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use crate::{impl_parsable_segment, segment::PassageSegment};

/// - This is a single chapter/verse reference
/// - Ex: `1:2` in `John 1:2`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub struct ChapterVerse {
    pub chapter: usize,
    pub verse: usize,
}

impl PartialOrd for ChapterVerse {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.chapter.cmp(&other.chapter)
            .then(self.verse.cmp(&other.verse)))
    }
}

impl ChapterVerse {
    pub fn new(chapter: usize, verse: usize) -> Self {
        ChapterVerse { chapter, verse }
    }
}

impl Into<PassageSegment> for ChapterVerse {
    fn into(self) -> PassageSegment {
        PassageSegment::ChapterVerse(self)
    }
}

impl Display for ChapterVerse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.chapter, self.verse)
    }
}

impl_parsable_segment!(ChapterVerse, "{}:{}");
