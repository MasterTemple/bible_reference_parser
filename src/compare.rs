pub trait SegmentCompare: Sized {
    fn starting_verse(&self) -> u8;

    fn starting_chapter(&self) -> u8;

    fn ending_verse(&self) -> Option<u8>;

    fn ending_chapter(&self) -> u8;

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

    fn overlaps_with(&self, other: &impl SegmentCompare) -> bool {
        !(self.ends_before(other) || self.starts_after(other))
    }
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
    use crate::{segment::PassageSegment};

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
