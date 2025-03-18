use std::{fmt::Display, ops::Bound};
use std::fmt::Debug;

use crate::books::book_data::BookInfo;
use crate::books::book_segment::BookSegment;
use crate::managers::content::PassageContent;

use super::any_segment::PassageSegment;
use super::types::chapter_range::ChapterRange;
use super::types::chapter_verse::ChapterVerse;
use super::types::chapter_verse_range::ChapterVerseRange;
use super::types::full_chapter::FullChapter;
use super::types::full_chapter_range::FullChapterRange;

// This is the trait with all the helpful operations; aka SegmentCompare
pub trait SegmentCompare: Copy + Sized + Debug +  Into<PassageSegment> + Display {
    fn starting_verse(&self) -> u8;

    fn starting_chapter(&self) -> u8;

    fn ending_verse(&self) -> Option<u8>;

    fn ending_chapter(&self) -> u8;

    /// - The verse range starts at 1 when not the starting chapter
    /// - The verse range is unbounded when not the ending chapter
    fn verse_range(&self, chapter: u8) -> (Bound<u8>, Bound<u8>) {
        let start_bound = if chapter == self.starting_chapter() {
            Bound::Included(self.starting_verse())
        } else {
            Bound::Included(1)
        };
        let end_bound = if chapter == self.ending_chapter() {
            match self.ending_verse() {
                Some(ending_verse) => Bound::Included(ending_verse),
                None => Bound::Unbounded
            }
        } else {
            Bound::Unbounded
        };
        (start_bound, end_bound)
    }

    fn chapter_range(&self) -> std::ops::RangeInclusive<u8> {
        self.starting_chapter()..=self.ending_chapter()
    }

    fn ends_before(&self, other: &impl SegmentCompare) -> bool {
        // it finishes in a chapter before the other one
        self.ending_chapter() < other.starting_chapter()
        // or it is in the same chapter and this ending verse < other starting verse
        || (
            self.ending_chapter() == other.starting_chapter()
            && self.ending_verse().is_some_and(|ending_verse| ending_verse < other.starting_verse())
        )
    }

    fn starts_after(&self, other: &impl SegmentCompare) -> bool {
        other.ends_before(self)
    }

    // If:
    // - This segment ends before the other segment starts
    // OR
    // - This segment starts after the other segment ends
    // Then:
    // - This segment does NOT overlap with the other segment
    fn overlaps_with(&self, other: &impl SegmentCompare) -> bool {
        !(self.ends_before(other) || self.starts_after(other))
    }

    /// determines what kind of passage segment this really is
    fn actual(&self) -> PassageSegment {
        let starting_chapter = self.starting_chapter();
        let starting_verse = self.starting_verse();
        let ending_chapter = self.ending_chapter();
        let same_chapter = starting_chapter == ending_chapter;

        if let Some(ending_verse) = self.ending_verse() {
            // it must be either a chapter verse or a chapter verse range
            if same_chapter {
                if starting_verse == ending_verse {
                    PassageSegment::ChapterVerse(ChapterVerse::new(starting_chapter, starting_verse))
                }
                else {
                    PassageSegment::ChapterVerseRange(ChapterVerseRange::new(starting_chapter, starting_verse, ending_verse))
                }

            }
            // it must be a chapter range
            else {
                PassageSegment::ChapterRange(ChapterRange::new(starting_chapter, starting_verse, ending_chapter, ending_verse))
            }
        }
        // it must be a full chapter or a full chapter range
        else {
            if same_chapter {
                PassageSegment::FullChapter(FullChapter::new(starting_chapter))
            } else {
                PassageSegment::FullChapterRange(FullChapterRange::new(starting_chapter, ending_chapter))
            }
        }
    }

    fn with_content<'a, Content>(&'_ self, content: &'a Content) -> PassageContent<'a, Self, Content> {
        PassageContent {
            segment: *self,
            content
        }
    }

    fn with_book<'a>(self, book: BookInfo<'a>) -> BookSegment<'a, Self>  {
        BookSegment {
            book,
            segment: self,
        }
    }
}
