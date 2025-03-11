use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::{compare::SegmentCompare, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}};

/// Remember, these correspond to
/// ```text
///                `Ephesians 1:1-4,5-7,2:2-3:4,6`
///                          |     |   |       | |
///                ----------+     |   |       | |
/// ChapterRange:  `1:1-4`         |   |       | |
///                ----------------+   |       | |
/// ChapterRange:  `1:5-7`             |       | |
///                --------------------+       | |
/// BookRange:     `2:2-3:4`                   | |
///                ----------------------------+ |
/// ChapterVerse:  `3:6`                         |
///                ------------------------------+
/// ```
/// These should be grouped into a single reference
///
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
            self.get_starting_chapter().cmp(&other.get_starting_chapter())
            .then(self.get_starting_verse().cmp(&other.get_starting_verse()))
            .then(self.get_ending_chapter().cmp(&other.get_ending_chapter()))
            .then(self.get_ending_verse().cmp(&other.get_ending_verse()))
        )
    }
}

impl SegmentCompare for PassageSegment {
    fn get_starting_chapter(&self) -> u8 {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_starting_chapter(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_starting_chapter(),
            PassageSegment::ChapterRange(book_range) => book_range.get_starting_chapter(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.get_starting_chapter(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_starting_chapter(),
        }
    }

    fn get_starting_verse(&self) -> u8 {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_starting_verse(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_starting_verse(),
            PassageSegment::ChapterRange(book_range) => book_range.get_starting_verse(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.get_starting_verse(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_starting_verse(),
        }
    }

    fn get_ending_chapter(&self) -> u8 {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_ending_chapter(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_ending_chapter(),
            PassageSegment::ChapterRange(book_range) => book_range.get_ending_chapter(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.get_ending_chapter(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_ending_chapter(),
        }
    }

    fn get_ending_verse(&self) -> Option<u8> {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_ending_verse(),
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_ending_verse(),
            PassageSegment::ChapterRange(book_range) => book_range.get_ending_verse(),
            PassageSegment::FullChapter(full_chapter) => full_chapter.get_ending_verse(),
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_ending_verse(),
        }
    }

}

// Helpful methods for accessing data
// impl PassageSegment {
//     pub fn get_starting_chapter(&self) -> u8 {
//         match self {
//             PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_starting_chapter(),
//             PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_starting_chapter(),
//             PassageSegment::ChapterRange(book_range) => book_range.get_starting_chapter(),
//             PassageSegment::FullChapter(full_chapter) => full_chapter.get_starting_chapter(),
//             PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_starting_chapter(),
//         }
//     }
//
//     pub fn get_starting_verse(&self) -> u8 {
//         match self {
//             PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_starting_verse(),
//             PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_starting_verse(),
//             PassageSegment::ChapterRange(book_range) => book_range.get_starting_verse(),
//             PassageSegment::FullChapter(full_chapter) => full_chapter.get_starting_verse(),
//             PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_starting_verse(),
//         }
//     }
//
//     pub fn get_ending_chapter(&self) -> u8 {
//         match self {
//             PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_ending_chapter(),
//             PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_ending_chapter(),
//             PassageSegment::ChapterRange(book_range) => book_range.get_ending_chapter(),
//             PassageSegment::FullChapter(full_chapter) => full_chapter.get_ending_chapter(),
//             PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_ending_chapter(),
//         }
//     }
//
//     pub fn get_ending_verse(&self) -> Option<u8> {
//         match self {
//             PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.get_ending_verse(),
//             PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.get_ending_verse(),
//             PassageSegment::ChapterRange(book_range) => book_range.get_ending_verse(),
//             PassageSegment::FullChapter(full_chapter) => full_chapter.get_ending_verse(),
//             PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.get_ending_verse(),
//         }
//     }
// }

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
