use std::ops::Bound;
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}, segment::{BookSegment, PassageSegment}};

pub trait SegmentCompare: Copy + Sized + Debug +  Into<PassageSegment> {
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

    fn with_book(&self, book: u8) -> BookSegment<Self> {
        BookSegment {
            book,
            segment: *self,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PassageContent<'a, Segment: SegmentCompare, Content> {
    pub segment: Segment,
    pub content: &'a Content
}

impl<'a, Segment: SegmentCompare, Content> PassageContent<'a, Segment, Content> {
    pub fn generalize(self) -> PassageContent<'a, PassageSegment, Content> {
        PassageContent {
            segment: self.segment.into(),
            content: self.content,
        }
    }
    pub fn with_book(self, book: u8) -> BookPassageContent<'a, Segment, Content> {
        BookPassageContent {
            book,
            segment: self.segment,
            content: self.content
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BookPassageContent<'a, Segment: SegmentCompare, Content> {
    pub book: u8,
    pub segment: Segment,
    pub content: &'a Content
}


// impl<T: SegmentCompare> PartialOrd for T {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(
//             self.get_starting_chapter().cmp(&other.get_starting_chapter())
//             .then(self.get_starting_verse().cmp(&other.get_starting_verse()))
//             .then(self.get_ending_chapter().cmp(&other.get_ending_chapter()))
//             .then(self.get_ending_verse().cmp(&other.get_ending_verse()))
//         )
//     }
// }

#[cfg(test)]
mod tests {
    use crate::segment::PassageSegment;

    use super::*;

    fn double_overlap(this: impl SegmentCompare, other: impl SegmentCompare) -> bool {
        let does_overlap = this.overlaps_with(&other);
        let is_overlapped = other.overlaps_with(&this);
        assert_eq!(does_overlap, is_overlapped);
        does_overlap
    }

    // ----------- //
    // Overlapping //
    // ----------- //

    #[test]
    fn chapter_verse() {
        let this = PassageSegment::chapter_verse(3, 3);

        // ------------ //
        // ChapterVerse //
        // ------------ //

        // true
        // 3:3
        assert!(double_overlap(this, PassageSegment::chapter_verse(3, 3)));

        // false
        // 2:3
        assert!(!double_overlap(this, PassageSegment::chapter_verse(2, 3)));
        // 3:2
        assert!(!double_overlap(this, PassageSegment::chapter_verse(3, 2)));

        // 4:3
        assert!(!double_overlap(this, PassageSegment::chapter_verse(4, 3)));
        // 3:4
        assert!(!double_overlap(this, PassageSegment::chapter_verse(3, 4)));

        // ----------------- //
        // ChapterVerseRange //
        // ----------------- //

        // true
        // 3:3-3
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 3, 3)));

        // 3:1-3
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 1, 3)));
        // 3:3-4
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 3, 4)));
        // 3:1-4
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 1, 4)));

        // false
        // 1:3-3
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 3, 3)));
        // 1:1-3
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 1, 3)));
        // 1:1-4
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 1, 4)));
        // 1:3-4
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 3, 4)));

        // 4:3-3
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 3, 3)));
        // 4:1-3
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 1, 3)));
        // 4:1-4
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 1, 4)));
        // 4:3-4
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 3, 4)));

        // ------------ //
        // ChapterRange //
        // ------------ //

        // true
        // 3:3-3-3
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 3, 3)));

        // 1:1-3-3
        assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 3)));
        // 1:3-3-3
        assert!(double_overlap(this, PassageSegment::chapter_range(1, 3, 3, 3)));

        // 3:3-4-1
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 1)));
        // 3:3-4-3
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 3)));
        // 3:3-4-4
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 4)));

        // false
        // 3:4-4-4
        assert!(!double_overlap(this, PassageSegment::chapter_range(3, 4, 4, 4)));
        // 1:1-3-1
        assert!(!double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 1)));

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        // 3
        assert!(double_overlap(this, PassageSegment::full_chapter(3)));

        // false
        // 2
        assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
        // 4
        assert!(!double_overlap(this, PassageSegment::full_chapter(4)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        // 3-3
        assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 3)));
        // 1-3
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
        // 3-4
        assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
        // 1-4
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 4)));

        // false
        // 1-2
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
        // 4-5
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
    }

    #[test]
    fn chapter_verse_range() {
        let this = PassageSegment::chapter_verse_range(3, 3, 7);

        // ----------------- //
        // ChapterVerseRange //
        // ----------------- //

        // true
        // 3:3-7
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 3, 7)));

        // 3:1-3
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 1, 3)));
        // 3:4-6
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 4, 6)));
        // 3:7-8
        assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 7, 8)));

        // false
        // 2:3-7
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 3, 7)));
        // 2:1-3
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 1, 3)));
        // 2:4-6
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 4, 6)));
        // 2:7-8
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 7, 8)));

        // 2:3-7
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 3, 7)));
        // 2:1-3
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 1, 3)));
        // 2:4-6
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 4, 6)));
        // 2:7-8
        assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 7, 8)));

        // ------------ //
        // ChapterRange //
        // ------------ //

        // true
        // 3:3-3-7
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 3, 7)));

        // 1:1-3-3
        assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 3)));
        // 1:1-3-7
        assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 7)));

        // 3:3-4-4
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 4)));
        // 3:7-4-4
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 7, 4, 4)));

        // 3:4-3-6
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 4, 3, 6)));

        // false
        // 1:1-3-2
        assert!(!double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 2)));
        // 3:8-4-4
        assert!(!double_overlap(this, PassageSegment::chapter_range(3, 8, 4, 4)));

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        // 3
        assert!(double_overlap(this, PassageSegment::full_chapter(3)));

        // false
        // 2
        assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
        // 4
        assert!(!double_overlap(this, PassageSegment::full_chapter(4)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        // 3-3
        assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 3)));
        // 1-3
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
        // 3-4
        assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
        // 1-4
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 4)));

        // false
        // 1-2
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
        // 4-5
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
      
    }

    #[test]
    fn chapter_range() {
        let this = PassageSegment::chapter_range(3, 3, 4, 4);

        // ------------ //
        // ChapterRange //
        // ------------ //

        // true
        // 3:3-4-4
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 4)));

        // 1:1-3-3
        assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 3)));
        // 1:1-4-4
        assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 4, 4)));

        // 3:3-5-5
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 5, 5)));
        // 4:4-5-5
        assert!(double_overlap(this, PassageSegment::chapter_range(4, 4, 5, 5)));

        // 3:4-3-6
        assert!(double_overlap(this, PassageSegment::chapter_range(3, 4, 3, 6)));
        // 4:1-4-3
        assert!(double_overlap(this, PassageSegment::chapter_range(4, 1, 4, 3)));

        // false
        // 1:1-3-2
        assert!(!double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 2)));
        // 4:5-5-5
        assert!(!double_overlap(this, PassageSegment::chapter_range(4, 5, 5, 5)));

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        // 3
        assert!(double_overlap(this, PassageSegment::full_chapter(3)));
        // 4
        assert!(double_overlap(this, PassageSegment::full_chapter(4)));

        // false
        // 2
        assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
        // 5
        assert!(!double_overlap(this, PassageSegment::full_chapter(5)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        // 3-4
        assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
        // 1-3
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
        // 4-5
        assert!(double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
        // 1-5
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 5)));

        // false
        // 1-2
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
        // 5-6
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(5, 6)));
    }

    #[test]
    fn full_chapter() {
        let this = PassageSegment::full_chapter(3);

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        // 3
        assert!(double_overlap(this, PassageSegment::full_chapter(3)));

        // false
        // 2
        assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
        // 4
        assert!(!double_overlap(this, PassageSegment::full_chapter(4)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        // 1-3
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
        // 3-4
        assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));

        // false
        // 1-2
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
        // 4-6
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(4, 6)));
    }

    #[test]
    fn full_chapter_range() {
        let this = PassageSegment::full_chapter_range(3, 4);

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        // 3-4
        assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
        // 1-3
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
        // 4-5
        assert!(double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
        // 1-5
        assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 5)));

        // false
        // 1-2
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
        // 5-6
        assert!(!double_overlap(this, PassageSegment::full_chapter_range(5, 6)));
      
    }
}
