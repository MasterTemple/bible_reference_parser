use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use crate::{impl_parsable_segment, segment::PassageSegment};

/// - This is a single chapter reference
/// - Ex: `1` in `John 1`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FullChapter {
    pub chapter: u8,
}

impl FullChapter {
    pub fn new(chapter: u8) -> Self {
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

impl_parsable_segment!(FullChapter, "{}");
