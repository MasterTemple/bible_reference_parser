use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}};

use crate::segment::PassageSegment;
/// - This is a single chapter/verse reference
/// - Ex: `1:2` in `John 1:2`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChapterVerse {
    pub chapter: usize,
    pub verse: usize,
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
