use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::{compare::SegmentCompare, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}};

/// Remember, these correspond to
/// ```text
///                   `John 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8`
///                        | |   |     | |   |      |       |
/// -----------------------+ |   |     | |   |      |       |
/// Full Chapter:        `1` |   |     | |   |      |       |
/// -------------------------+   |     | |   |      |       |
/// Full Chapter Range:  `2-4`   |     | |   |      |       |
/// -----------------------------+     | |   |      |       |
/// Chapter Range:       `5:1-3`       | |   |      |       |
/// -----------------------------------+ |   |      |       |
/// Chapter Verse:       `5:5            |   |      |       |
/// -------------------------------------+   |      |       |
/// Chapter Verse Range: `5:7-9`             |      |       |
/// -----------------------------------------+      |       |
/// Chapter Range:       `5:12-6:6`                 |       |
/// ------------------------------------------------+       |
/// Chapter Range:       `7:7-8:8`                          |
/// --------------------------------------------------------+
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PassageSegment {
    /// - This is a single chapter/verse reference
    /// - Ex: `1:2` in `John 1:2`
    ChapterVerse(ChapterVerse),
    /// - This is a range of verse references within a single chapter
    /// - Ex: `1:2-3` `John 1:2-3`
    ChapterVerseRange(ChapterVerseRange),
    /// - This is a range of verse references across a multiple chapters
    /// - Ex: `John 1:2-3:4`
    ChapterRange(ChapterRange),
    /// - This is a single chapter reference
    /// - Ex: `1` in `John 1`
    FullChapter(FullChapter),
    /// - This is a chapter range reference
    /// - Ex: `1-2` in `John 1-2`
    FullChapterRange(FullChapterRange),
}

impl PartialOrd for PassageSegment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.starting_chapter().cmp(&other.starting_chapter())
            .then(self.starting_verse().cmp(&other.starting_verse()))
            .then(self.ending_chapter().cmp(&other.ending_chapter()))
            .then(self.ending_verse().cmp(&other.ending_verse()))
        )
    }
}

impl SegmentCompare for PassageSegment {
    fn starting_chapter(&self) -> u8 {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.starting_chapter(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.starting_chapter(),
            PassageSegment::ChapterRange(book_range) => book_range.starting_chapter(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.starting_chapter(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.starting_chapter(),
        }
    }

    fn starting_verse(&self) -> u8 {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.starting_verse(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.starting_verse(),
            PassageSegment::ChapterRange(book_range) => book_range.starting_verse(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.starting_verse(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.starting_verse(),
        }
    }

    fn ending_chapter(&self) -> u8 {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.ending_chapter(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.ending_chapter(),
            PassageSegment::ChapterRange(book_range) => book_range.ending_chapter(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.ending_chapter(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.ending_chapter(),
        }
    }

    fn ending_verse(&self) -> Option<u8> {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.ending_verse(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.ending_verse(),
            PassageSegment::ChapterRange(book_range) => book_range.ending_verse(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.ending_verse(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.ending_verse(),
        }
    }

}

// Easy constructors
impl PassageSegment {
    pub fn chapter_verse(chapter: u8, verse: u8) -> Self {
        Self::ChapterVerse(ChapterVerse::new(chapter, verse))
    }

    pub fn chapter_verse_range(chapter: u8, start_verse: u8, end_verse: u8) -> Self {
        Self::ChapterVerseRange(ChapterVerseRange::new(chapter, start_verse, end_verse))
    }

    pub fn chapter_range(start_chapter: u8, start_verse: u8, end_chapter: u8, end_verse: u8) -> Self {
        Self::ChapterRange(ChapterRange::new(start_chapter, start_verse, end_chapter, end_verse))
    }

    pub fn full_chapter(chapter: u8) -> Self {
        Self::FullChapter(FullChapter::new(chapter))
    }

    pub fn full_chapter_range(start: u8, end: u8) -> Self {
        Self::FullChapterRange(FullChapterRange::new(start, end))
    }
}

// Formatting
impl Display for PassageSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.to_string(),
            PassageSegment::ChapterVerseRange(chapter_verse_range) => chapter_verse_range.to_string(),
            PassageSegment::ChapterRange(chapter_range) => chapter_range.to_string(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.to_string(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.to_string(),
        })
    }
}
