use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}};
use crate::segment::PassageSegment;

/// - This is a single chapter reference
/// - Ex: `1` in `John 1`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FullChapter {
    pub chapter: usize,
}
impl FullChapter {
    pub fn new(chapter: usize) -> Self {
        FullChapter { chapter }
    }
}
impl Into<PassageSegment> for FullChapter {
    fn into(self) -> PassageSegment {
        PassageSegment::FullChapter(self)
    }
}
impl Display for FullChapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chapter)
    }
}
