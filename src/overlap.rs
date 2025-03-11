// use crate::{compare::SegmentCompare, segment::PassageSegment};
//
// pub struct ExplicitChapterVerseRange {
//     start_chapter: u8,
//     start_verse: u8,
//     end_chapter: u8,
//     end_verse: Option<u8>,
// }
//
// impl PassageSegment {
//     /// **Note:** I am doing `<SegmentType>` -> `PassageSegment` -> `ExplicitChapterVerseRange` on each overlap comparison.
//     /// I should optimize this at somie point, cause I do not know how well the Rust compiler is
//     fn explicify(&self) -> ExplicitChapterVerseRange {
//         ExplicitChapterVerseRange {
//             start_chapter: self.get_starting_chapter(),
//             start_verse: self.get_starting_verse(),
//             end_chapter: self.get_ending_chapter(),
//             end_verse: self.get_ending_verse(),
//         }
//     }
//
//     fn segments_have_overlap(&self, other: &PassageSegment) -> bool {
//         let this = self.explicify();
//         let other = other.explicify();
//         // checking overlap by checking if there is space between their edges
//         !(
//             // other ends before this starts (which is also this starts before other ends)
//             other.end_chapter < this.start_chapter
//             || (other.end_chapter == this.start_chapter && other.end_verse.is_some_and(|end_verse| end_verse < this.start_verse))
//             // other starts after this ends (which is also this ends after other starts)
//             || other.start_chapter > this.end_chapter
//                 || (other.start_chapter == this.end_chapter && this.end_verse.is_some_and(|end_verse| other.start_verse > end_verse))
//         )
//     }
// }
//
// pub trait OverlapsWith: Into<PassageSegment> {
//     fn overlaps_segment(self, other: impl Into<PassageSegment>) -> bool {
//         let other_seg: PassageSegment = other.into();
//         let this_seg: PassageSegment  = self.into();
//         this_seg.segments_have_overlap(&other_seg)
//     }
// }
//
// impl<T: Into<PassageSegment>> OverlapsWith for T {}
//
// #[cfg(test)]
// mod overlap_tests {
//     use super::*;
//
//     fn double_overlap(this: PassageSegment, other: PassageSegment) -> bool {
//         let does_overlap = this.segments_have_overlap(&other);
//         let is_overlapped = other.segments_have_overlap(&this);
//         assert_eq!(does_overlap, is_overlapped);
//         does_overlap
//     }
//
//     #[test]
//     fn chapter_verse() {
//         let this = PassageSegment::chapter_verse(3, 3);
//
//         // ------------ //
//         // ChapterVerse //
//         // ------------ //
//
//         // true
//         // 3:3
//         assert!(double_overlap(this, PassageSegment::chapter_verse(3, 3)));
//
//         // false
//         // 2:3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse(2, 3)));
//         // 3:2
//         assert!(!double_overlap(this, PassageSegment::chapter_verse(3, 2)));
//
//         // 4:3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse(4, 3)));
//         // 3:4
//         assert!(!double_overlap(this, PassageSegment::chapter_verse(3, 4)));
//
//         // ----------------- //
//         // ChapterVerseRange //
//         // ----------------- //
//
//         // true
//         // 3:3-3
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 3, 3)));
//
//         // 3:1-3
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 1, 3)));
//         // 3:3-4
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 3, 4)));
//         // 3:1-4
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 1, 4)));
//
//         // false
//         // 1:3-3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 3, 3)));
//         // 1:1-3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 1, 3)));
//         // 1:1-4
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 1, 4)));
//         // 1:3-4
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(1, 3, 4)));
//
//         // 4:3-3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 3, 3)));
//         // 4:1-3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 1, 3)));
//         // 4:1-4
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 1, 4)));
//         // 4:3-4
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(4, 3, 4)));
//
//         // ------------ //
//         // ChapterRange //
//         // ------------ //
//
//         // true
//         // 3:3-3-3
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 3, 3)));
//
//         // 1:1-3-3
//         assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 3)));
//         // 1:3-3-3
//         assert!(double_overlap(this, PassageSegment::chapter_range(1, 3, 3, 3)));
//
//         // 3:3-4-1
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 1)));
//         // 3:3-4-3
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 3)));
//         // 3:3-4-4
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 4)));
//
//         // false
//         // 3:4-4-4
//         assert!(!double_overlap(this, PassageSegment::chapter_range(3, 4, 4, 4)));
//         // 1:1-3-1
//         assert!(!double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 1)));
//
//         // ----------- //
//         // FullChapter //
//         // ----------- //
//
//         // true
//         // 3
//         assert!(double_overlap(this, PassageSegment::full_chapter(3)));
//
//         // false
//         // 2
//         assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
//         // 4
//         assert!(!double_overlap(this, PassageSegment::full_chapter(4)));
//
//         // ---------------- //
//         // FullChapterRange //
//         // ---------------- //
//
//         // true
//         // 3-3
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 3)));
//         // 1-3
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
//         // 3-4
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
//         // 1-4
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 4)));
//
//         // false
//         // 1-2
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
//         // 4-5
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
//     }
//
//     #[test]
//     fn chapter_verse_range() {
//         let this = PassageSegment::chapter_verse_range(3, 3, 7);
//
//         // ----------------- //
//         // ChapterVerseRange //
//         // ----------------- //
//
//         // true
//         // 3:3-7
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 3, 7)));
//
//         // 3:1-3
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 1, 3)));
//         // 3:4-6
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 4, 6)));
//         // 3:7-8
//         assert!(double_overlap(this, PassageSegment::chapter_verse_range(3, 7, 8)));
//
//         // false
//         // 2:3-7
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 3, 7)));
//         // 2:1-3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 1, 3)));
//         // 2:4-6
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 4, 6)));
//         // 2:7-8
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 7, 8)));
//
//         // 2:3-7
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 3, 7)));
//         // 2:1-3
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 1, 3)));
//         // 2:4-6
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 4, 6)));
//         // 2:7-8
//         assert!(!double_overlap(this, PassageSegment::chapter_verse_range(2, 7, 8)));
//
//         // ------------ //
//         // ChapterRange //
//         // ------------ //
//
//         // true
//         // 3:3-3-7
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 3, 7)));
//
//         // 1:1-3-3
//         assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 3)));
//         // 1:1-3-7
//         assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 7)));
//
//         // 3:3-4-4
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 4)));
//         // 3:7-4-4
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 7, 4, 4)));
//
//         // 3:4-3-6
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 4, 3, 6)));
//
//         // false
//         // 1:1-3-2
//         assert!(!double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 2)));
//         // 3:8-4-4
//         assert!(!double_overlap(this, PassageSegment::chapter_range(3, 8, 4, 4)));
//
//         // ----------- //
//         // FullChapter //
//         // ----------- //
//
//         // true
//         // 3
//         assert!(double_overlap(this, PassageSegment::full_chapter(3)));
//
//         // false
//         // 2
//         assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
//         // 4
//         assert!(!double_overlap(this, PassageSegment::full_chapter(4)));
//
//         // ---------------- //
//         // FullChapterRange //
//         // ---------------- //
//
//         // true
//         // 3-3
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 3)));
//         // 1-3
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
//         // 3-4
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
//         // 1-4
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 4)));
//
//         // false
//         // 1-2
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
//         // 4-5
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
//       
//     }
//
//     #[test]
//     fn chapter_range() {
//         let this = PassageSegment::chapter_range(3, 3, 4, 4);
//
//         // ------------ //
//         // ChapterRange //
//         // ------------ //
//
//         // true
//         // 3:3-4-4
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 4, 4)));
//
//         // 1:1-3-3
//         assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 3)));
//         // 1:1-4-4
//         assert!(double_overlap(this, PassageSegment::chapter_range(1, 1, 4, 4)));
//
//         // 3:3-5-5
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 3, 5, 5)));
//         // 4:4-5-5
//         assert!(double_overlap(this, PassageSegment::chapter_range(4, 4, 5, 5)));
//
//         // 3:4-3-6
//         assert!(double_overlap(this, PassageSegment::chapter_range(3, 4, 3, 6)));
//         // 4:1-4-3
//         assert!(double_overlap(this, PassageSegment::chapter_range(4, 1, 4, 3)));
//
//         // false
//         // 1:1-3-2
//         assert!(!double_overlap(this, PassageSegment::chapter_range(1, 1, 3, 2)));
//         // 4:5-5-5
//         assert!(!double_overlap(this, PassageSegment::chapter_range(4, 5, 5, 5)));
//
//         // ----------- //
//         // FullChapter //
//         // ----------- //
//
//         // true
//         // 3
//         assert!(double_overlap(this, PassageSegment::full_chapter(3)));
//         // 4
//         assert!(double_overlap(this, PassageSegment::full_chapter(4)));
//
//         // false
//         // 2
//         assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
//         // 5
//         assert!(!double_overlap(this, PassageSegment::full_chapter(5)));
//
//         // ---------------- //
//         // FullChapterRange //
//         // ---------------- //
//
//         // true
//         // 3-4
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
//         // 1-3
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
//         // 4-5
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
//         // 1-5
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 5)));
//
//         // false
//         // 1-2
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
//         // 5-6
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(5, 6)));
//     }
//
//     #[test]
//     fn full_chapter() {
//         let this = PassageSegment::full_chapter(3);
//
//         // ----------- //
//         // FullChapter //
//         // ----------- //
//
//         // true
//         // 3
//         assert!(double_overlap(this, PassageSegment::full_chapter(3)));
//
//         // false
//         // 2
//         assert!(!double_overlap(this, PassageSegment::full_chapter(2)));
//         // 4
//         assert!(!double_overlap(this, PassageSegment::full_chapter(4)));
//
//         // ---------------- //
//         // FullChapterRange //
//         // ---------------- //
//
//         // true
//         // 1-3
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
//         // 3-4
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
//
//         // false
//         // 1-2
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
//         // 4-6
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(4, 6)));
//     }
//
//     #[test]
//     fn full_chapter_range() {
//         let this = PassageSegment::full_chapter_range(3, 4);
//
//         // ---------------- //
//         // FullChapterRange //
//         // ---------------- //
//
//         // true
//         // 3-4
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(3, 4)));
//         // 1-3
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 3)));
//         // 4-5
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(4, 5)));
//         // 1-5
//         assert!(double_overlap(this, PassageSegment::full_chapter_range(1, 5)));
//
//         // false
//         // 1-2
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(1, 2)));
//         // 5-6
//         assert!(!double_overlap(this, PassageSegment::full_chapter_range(5, 6)));
//       
//     }
// }
