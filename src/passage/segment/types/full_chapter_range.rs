use serde::{de::Visitor, Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::passage::segment::{
    any_segment::AnySegment,
    individual_parse::{ParsableSegment, SegmentParseMethods},
    range_pair::RangePair,
    segment::SegmentFns,
};

use super::full_chapter::FullChapter;

/// - This is a chapter range reference
/// - Ex: `1-2` in `John 1-2`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord)]
pub struct FullChapterRange(RangePair<FullChapter>);

impl Display for FullChapterRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl Serialize for FullChapterRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct FullChapterRangeVisitor;

impl<'de> Visitor<'de> for FullChapterRangeVisitor {
    type Value = FullChapterRange;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("format '{}-{}'")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        Ok(FullChapterRange::new(
            seq.next_element()?
                .ok_or_else(|| serde::de::Error::custom("missing start chapter"))?,
            seq.next_element()?
                .ok_or_else(|| serde::de::Error::custom("missing end chapter"))?,
        ))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(|e| E::custom(e))
    }
}

impl<'de> Deserialize<'de> for FullChapterRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(FullChapterRangeVisitor)
    }
}

impl FromStr for FullChapterRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl SegmentFns for FullChapterRange {
    fn starting_chapter(&self) -> u8 {
        self.start.chapter
    }

    fn starting_verse(&self) -> u8 {
        1
    }

    fn ending_chapter(&self) -> u8 {
        self.end.chapter
    }

    fn ending_verse(&self) -> Option<u8> {
        None
    }
}

impl PartialOrd for FullChapterRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.start
                .chapter
                .cmp(&other.start.chapter)
                .then(self.end.chapter.cmp(&other.end.chapter)),
        )
    }
}

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
    pub fn new(start: u8, end: u8) -> Self {
        FullChapterRange(RangePair {
            start: FullChapter::new(start),
            end: FullChapter::new(end),
        })
    }
}

impl Into<AnySegment> for FullChapterRange {
    fn into(self) -> AnySegment {
        AnySegment::FullChapterRange(self)
    }
}

impl TryFrom<AnySegment> for FullChapterRange {
    type Error = String;

    fn try_from(value: AnySegment) -> Result<Self, Self::Error> {
        Ok(match value {
            AnySegment::ChapterVerse(chapter_verse) => {
                FullChapterRange::new(chapter_verse.chapter, chapter_verse.chapter)
            }
            AnySegment::ChapterVerseRange(chapter_verse_range) => {
                FullChapterRange::new(chapter_verse_range.chapter, chapter_verse_range.chapter)
            }
            AnySegment::ChapterRange(chapter_range) => {
                FullChapterRange::new(chapter_range.start.chapter, chapter_range.end.chapter)
            }
            AnySegment::FullChapter(full_chapter) => {
                FullChapterRange::new(full_chapter.chapter, full_chapter.chapter)
            }
            AnySegment::FullChapterRange(full_chapter_range) => full_chapter_range,
        })
    }
}

impl ParsableSegment for FullChapterRange {
    const EXPECTED_FORMAT: &'static str = "{}-{}";

    fn parse_strict(input: &str) -> Result<Self, String> {
        let chars = &mut input.chars().peekable();

        let start_chapter = FullChapterRange::take_number(chars)?;
        FullChapterRange::expect_char(chars, '-')?;
        let end_chapter = FullChapterRange::take_number(chars)?;
        FullChapterRange::expect_done(chars)?;

        Ok(FullChapterRange::new(start_chapter, end_chapter))
    }
}
