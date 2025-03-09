use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}};
use crate::segment::PassageSegment;

use super::{full_chapter::FullChapter, range_pair::RangePair};

/// - This is a chapter range reference
/// - Ex: `1-2` in `John 1-2`
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FullChapterRange(RangePair<FullChapter>);
impl Deref for FullChapterRange {
    type Target = RangePair<FullChapter>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for FullChapterRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl FullChapterRange {
    pub fn new(start: usize, end: usize) -> Self {
        FullChapterRange(RangePair{
            start: FullChapter::new(
                start,
            ),
            end: FullChapter::new(
                end,
            ),
        })
    }
}
impl Into<PassageSegment> for FullChapterRange {
    fn into(self) -> PassageSegment {
        PassageSegment::FullChapterRange(self)
    }
}
impl Display for FullChapterRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}
