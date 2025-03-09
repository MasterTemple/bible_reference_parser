use crate::segment::PassageSegment;

pub struct VerseChapterVerseRange {
    start_chapter: usize,
    start_verse: usize,
    end_chapter: usize,
    end_verse: Option<usize>,
}

impl PassageSegment {
    fn verbosify(&self) -> VerseChapterVerseRange {
        VerseChapterVerseRange {
            start_chapter: self.get_starting_chapter(),
            start_verse: self.get_starting_verse(),
            end_chapter: self.get_ending_chapter(),
            end_verse: self.get_ending_verse(),
        }
    }

    /// This is only meant to be used while running tests for bi-directionality
    fn double_overlap(&self, other: PassageSegment) -> bool {
        let does_overlap = self.overlaps_with(other);
        let is_overlapped = other.overlaps_with(*self);
        assert_eq!(does_overlap, is_overlapped);
        does_overlap
    }

    pub fn overlaps_with(&self, other: PassageSegment) -> bool {
        let this = self.verbosify();
        let other = other.verbosify();
        // checking overlap by checking if there is space between their edges
        !(
            // other ends before this starts (which is also this starts before other ends)
            other.end_chapter < this.start_chapter
            || (other.end_chapter == this.start_chapter && other.end_verse.is_some_and(|end_verse| end_verse < this.start_verse))
            // other starts after this ends (which is also this ends after other starts)
            || other.start_chapter > this.end_chapter
                || (other.start_chapter == this.end_chapter && this.end_verse.is_some_and(|end_verse| other.start_verse > end_verse))
        )
    }
}

#[cfg(test)]
mod overlap_tests {
    use super::*;

    #[test]
    fn chapter_verse() {
        let this = PassageSegment::chapter_verse(3, 3);

        // ------------ //
        // ChapterVerse //
        // ------------ //

        // true
        assert!(this.double_overlap(PassageSegment::chapter_verse(3, 3)));

        // false
        assert!(!this.double_overlap(PassageSegment::chapter_verse(2, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse(3, 2)));

        assert!(!this.double_overlap(PassageSegment::chapter_verse(4, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse(3, 4)));

        // ----------------- //
        // ChapterVerseRange //
        // ----------------- //

        // true
        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 3, 3)));

        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 1, 3)));
        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 3, 4)));
        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 1, 4)));

        // false
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(1, 3, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(1, 1, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(1, 1, 4)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(1, 3, 4)));

        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(4, 3, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(4, 1, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(4, 1, 4)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(4, 3, 4)));

        // ------------ //
        // ChapterRange //
        // ------------ //

        // true
        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 3, 3)));

        assert!(this.double_overlap(PassageSegment::chapter_range(1, 1, 3, 3)));
        assert!(this.double_overlap(PassageSegment::chapter_range(1, 3, 3, 3)));

        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 4, 1)));
        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 4, 3)));
        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 4, 4)));

        // false
        assert!(!this.double_overlap(PassageSegment::chapter_range(3, 4, 4, 4)));
        assert!(!this.double_overlap(PassageSegment::chapter_range(1, 1, 3, 1)));

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter(3)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter(2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter(4)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter_range(3, 3)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 3)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(3, 4)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 4)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(1, 2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(4, 5)));
    }

    #[test]
    fn chapter_verse_range() {
        let this = PassageSegment::chapter_verse_range(3, 3, 7);

        // ----------------- //
        // ChapterVerseRange //
        // ----------------- //

        // true
        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 3, 7)));

        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 1, 3)));
        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 4, 6)));
        assert!(this.double_overlap(PassageSegment::chapter_verse_range(3, 7, 8)));

        // false
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 3, 7)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 1, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 4, 6)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 7, 8)));

        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 3, 7)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 1, 3)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 4, 6)));
        assert!(!this.double_overlap(PassageSegment::chapter_verse_range(2, 7, 8)));

        // ------------ //
        // ChapterRange //
        // ------------ //

        // true
        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 3, 7)));

        assert!(this.double_overlap(PassageSegment::chapter_range(1, 1, 3, 3)));
        assert!(this.double_overlap(PassageSegment::chapter_range(1, 1, 3, 7)));

        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 4, 4)));
        assert!(this.double_overlap(PassageSegment::chapter_range(3, 7, 4, 4)));

        assert!(this.double_overlap(PassageSegment::chapter_range(3, 4, 3, 6)));

        // false
        assert!(!this.double_overlap(PassageSegment::chapter_range(1, 1, 3, 2)));
        assert!(!this.double_overlap(PassageSegment::chapter_range(3, 8, 4, 4)));

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter(3)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter(2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter(4)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter_range(3, 3)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 3)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(3, 4)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 4)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(1, 2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(4, 5)));
      
    }

    #[test]
    fn chapter_range() {
        let this = PassageSegment::chapter_range(3, 3, 4, 4);

        // ------------ //
        // ChapterRange //
        // ------------ //

        // true
        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 4, 4)));

        assert!(this.double_overlap(PassageSegment::chapter_range(1, 1, 3, 3)));
        assert!(this.double_overlap(PassageSegment::chapter_range(1, 1, 4, 4)));

        assert!(this.double_overlap(PassageSegment::chapter_range(3, 3, 5, 5)));
        assert!(this.double_overlap(PassageSegment::chapter_range(4, 4, 5, 5)));

        assert!(this.double_overlap(PassageSegment::chapter_range(3, 4, 3, 6)));
        assert!(this.double_overlap(PassageSegment::chapter_range(4, 1, 4, 3)));

        // false
        assert!(!this.double_overlap(PassageSegment::chapter_range(1, 1, 3, 2)));
        assert!(!this.double_overlap(PassageSegment::chapter_range(4, 5, 5, 5)));

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter(3)));
        assert!(this.double_overlap(PassageSegment::full_chapter(4)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter(2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter(5)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter_range(3, 4)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 3)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(4, 5)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 5)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(1, 2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(5, 6)));
    }

    #[test]
    fn full_chapter() {
        let this = PassageSegment::full_chapter(3);

        // ----------- //
        // FullChapter //
        // ----------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter(3)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter(2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter(4)));

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 3)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(3, 4)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(1, 2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(4, 6)));
    }

    #[test]
    fn full_chapter_range() {
        let this = PassageSegment::full_chapter_range(3, 4);

        // ---------------- //
        // FullChapterRange //
        // ---------------- //

        // true
        assert!(this.double_overlap(PassageSegment::full_chapter_range(3, 4)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 3)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(4, 5)));
        assert!(this.double_overlap(PassageSegment::full_chapter_range(1, 5)));

        // false
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(1, 2)));
        assert!(!this.double_overlap(PassageSegment::full_chapter_range(5, 6)));
      
    }
}
