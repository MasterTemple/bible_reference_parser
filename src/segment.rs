use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}};

use crate::passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange, range_pair::RangePair};

// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct RangePair<T>
//     // where T: Copy + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Serialize + DeserializeOwned
// {
//     pub start: T,
//     pub end: T,
// }
//
// impl<T> RangePair<T>
//     where T: Copy + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + Serialize + DeserializeOwned
// {
//     pub fn new(start: T, end: T) -> Self {
//         Self { start, end }
//     }
//
//     pub fn from_point(point: T) -> Self {
//         Self {
//             start: point,
//             end: point,
//         }
//     }
// }


// /// - This is a single chapter reference
// /// - Ex: `1` in `John 1`
// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct FullChapter {
//     pub chapter: usize,
// }
// impl FullChapter {
//     pub fn new(chapter: usize) -> Self {
//         FullChapter { chapter }
//     }
// }
// impl Into<PassageSegment> for FullChapter {
//     fn into(self) -> PassageSegment {
//         PassageSegment::FullChapter(self)
//     }
// }
// impl Display for FullChapter {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.chapter)
//     }
// }

// /// - This is a chapter range reference
// /// - Ex: `1-2` in `John 1-2`
// // #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub type FullChapterRange = RangePair<FullChapter>;
// // pub struct FullChapterRange {
// //     pub start: FullChapter,
// //     pub end: FullChapter,
// // }

// /// - This is a single chapter/verse reference
// /// - Ex: `1:2` in `John 1:2`
// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct ChapterVerse {
//     pub chapter: usize,
//     pub verse: usize,
// }
// impl ChapterVerse {
//     pub fn new(chapter: usize, verse: usize) -> Self {
//         ChapterVerse { chapter, verse }
//     }
// }
// impl Into<PassageSegment> for ChapterVerse {
//     fn into(self) -> PassageSegment {
//         PassageSegment::ChapterVerse(self)
//     }
// }
// impl Display for ChapterVerse {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}:{}", self.chapter, self.verse)
//     }
// }

// /// - This is a range of verse references within a single chapter
// /// - Ex: `1:2-3` `John 1:2-3`
// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct ChapterVerseRange {
//     pub chapter: usize,
//     pub verses: RangePair<usize>,
// }
// impl ChapterVerseRange {
//     pub fn new(chapter: usize, start_verse: usize, end_verse: usize) -> Self {
//         ChapterVerseRange {
//             chapter,
//             verses: RangePair {
//                 start: start_verse,
//                 end: end_verse,
//             },
//         }
//     }
// }
// impl Into<PassageSegment> for ChapterVerseRange {
//     fn into(self) -> PassageSegment {
//         PassageSegment::ChapterVerseRange(self)
//     }
// }
// impl Display for ChapterVerseRange {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}:{}-{}", self.chapter, self.verses.start, self.verses.end)
//     }
// }

// /// - This is a range of verse references across a multiple chapters
// /// - Ex: `1:2-3:4` in `John 1:2-3:4`
// // pub type ChapterRange = RangePair<ChapterVerse>;
// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct ChapterRange(RangePair<ChapterVerse>);
// impl Deref for ChapterRange {
//     type Target = RangePair<ChapterVerse>;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// impl DerefMut for ChapterRange {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
// impl ChapterRange {
//     pub fn new(start_chapter: usize, start_verse: usize, end_chapter: usize, end_verse: usize) -> Self {
//         ChapterRange(RangePair{
//             start: ChapterVerse::new(
//                 start_chapter,
//                 start_verse,
//             ),
//             end: ChapterVerse::new(
//                 end_chapter,
//                 end_verse,
//             ),
//         })
//     }
// }
// impl Into<PassageSegment> for ChapterRange {
//     fn into(self) -> PassageSegment {
//         PassageSegment::ChapterRange(self)
//     }
// }
// impl Display for ChapterRange {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}-{}:{}-{}", self.start.chapter, self.start.verse, self.end.chapter, self.end.verse)
//     }
// }


// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
// pub struct ChapterRange {
//     pub start: ChapterVerse,
//     pub end: ChapterVerse,
// }

/// Remember, these correspond to
/// ```
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

// Helpful methods for accessing data
impl PassageSegment {
    pub fn get_starting_verse(&self) -> usize {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.verse,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.verses.start,
            PassageSegment::ChapterRange(book_range) => book_range.start.verse,
            PassageSegment::FullChapter(_) | PassageSegment::FullChapterRange(_) => 1,
        }
    }

    pub fn get_starting_chapter(&self) -> usize {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.chapter,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.chapter,
            PassageSegment::ChapterRange(book_range) => book_range.start.chapter,
            PassageSegment::FullChapter(full_chapter) => full_chapter.chapter,
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.start.chapter,
        }
    }

    pub fn get_ending_verse(&self) -> Option<usize> {
        Some(match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.verse,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.verses.end,
            PassageSegment::ChapterRange(book_range) => book_range.end.verse,
            PassageSegment::FullChapter(_) | PassageSegment::FullChapterRange(_) => None?,
        })
    }

    pub fn get_ending_chapter(&self) -> usize {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => chapter_verse.chapter,
            PassageSegment::ChapterVerseRange(chapter_range) => chapter_range.chapter,
            PassageSegment::ChapterRange(book_range) => book_range.end.chapter,
            PassageSegment::FullChapter(full_chapter) => full_chapter.chapter,
            PassageSegment::FullChapterRange(full_chapter_range) => full_chapter_range.end.chapter,
        }
    }
}

// Easy constructors
impl PassageSegment {
    pub fn chapter_verse(chapter: usize, verse: usize) -> Self {
        Self::ChapterVerse(ChapterVerse { chapter, verse })
    }

    pub fn chapter_verse_range(chapter: usize, start_verse: usize, end_verse: usize) -> Self {
        Self::ChapterVerseRange(ChapterVerseRange::new(chapter, start_verse, end_verse))
    }

    pub fn chapter_range(
        start_chapter: usize,
        start_verse: usize,
        end_chapter: usize,
        end_verse: usize,
    ) -> Self {
        Self::ChapterRange(ChapterRange::new(start_chapter, start_verse, end_chapter, end_verse))
    }

    pub fn full_chapter(chapter: usize) -> Self {
        Self::FullChapter(FullChapter::new(chapter))
    }

    pub fn full_chapter_range(start: usize, end: usize) -> Self {
        Self::FullChapterRange(FullChapterRange::new(start, end))
    }
}

// Formatting
impl PassageSegment {
    pub fn label(&self) -> String {
        match self {
            PassageSegment::ChapterVerse(chapter_verse) => {
                format!("{}:{}", chapter_verse.chapter, chapter_verse.verse)
            }
            PassageSegment::ChapterVerseRange(chapter_range) => {
                format!(
                    "{}:{}-{}",
                    chapter_range.chapter, chapter_range.verses.start, chapter_range.verses.end
                )
            }
            PassageSegment::ChapterRange(book_range) => {
                format!(
                    "{}:{}-{}:{}",
                    book_range.start.chapter,
                    book_range.start.verse,
                    book_range.end.chapter,
                    book_range.end.verse
                )
            }
            PassageSegment::FullChapter(full_chapter) => {
                format!("{}", full_chapter.chapter)
            }
            PassageSegment::FullChapterRange(full_chapter_range) => {
                format!("{}-{}", full_chapter_range.start.chapter, full_chapter_range.end.chapter)
            }
        }
    }
}
