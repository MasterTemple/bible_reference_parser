use std::collections::BTreeMap;
use std::fmt::Debug;

use itertools::Itertools;

use crate::compare::SegmentCompare;
use crate::passage_segments::chapter_range::ChapterRange;
use crate::passage_segments::chapter_verse::ChapterVerse;
use crate::passage_segments::chapter_verse_range::ChapterVerseRange;
use crate::passage_segments::full_chapter::FullChapter;
use crate::passage_segments::full_chapter_range::{self, FullChapterRange};
use crate::segment::PassageSegment;

/// It requires default not because the data type must impl Default, but it's container should
#[derive(Debug, Default)]
pub struct BookOrganizer<Container: Debug + Default> {
    /// `map[chapter][verse] -> Container`
    chapter_verse: BTreeMap<u8, BTreeMap<u8, Container>>,
    /// `map[chapter][start_verse][end_verse] -> Container`
    chapter_verse_range: BTreeMap<u8, BTreeMap<(u8, u8), Container>>,
    /// `map[start_chapter][start_verse][end_chapter][end_verse] -> Container`
    chapter_range: BTreeMap<u8, BTreeMap<u8, BTreeMap<u8, BTreeMap<u8, Container>>>>,
    /// `map[chapter] -> Container`
    full_chapter: BTreeMap<u8, Container>,
    /// `map[start_chapter][end_chapter] -> Container`
    // full_chapter_range: BTreeMap<u8, BTreeMap<u8, Container>>,
    full_chapter_range: BTreeMap<(u8, u8), Container>,
}

/**
TODO:
- Figure out if it is more efficient to remove all references on u8
- They are much smaller, but they are already created
- Or maybe it is a reference that is just being incremented

Perhaps I should return `PassageSegment`s instead of nested u8 groups
impl Iterator<Item = (ChapterVerse, &Container)>
which could be serialized to be { "1:1": {...}}
*/
impl<Container: Debug + Default> BookOrganizer<Container> {
    pub fn new() -> Self {
        Self {
            chapter_verse: BTreeMap::default(),
            chapter_verse_range: BTreeMap::default(),
            chapter_range: BTreeMap::default(),
            full_chapter: BTreeMap::default(),
            full_chapter_range: BTreeMap::default(),
        }
    }

    /// returns the requested object to modify (creating necessary defaults)
    pub fn modify(&mut self, seg: impl SegmentCompare) -> &mut Container {
        // convert into best passage segment
        let seg = seg.actual();
        match seg {
            PassageSegment::ChapterVerse(seg) => {
                self.chapter_verse.entry(seg.chapter).or_default()
                    .entry(seg.verse).or_default()
            },
            PassageSegment::ChapterVerseRange(seg) => {
                self.chapter_verse_range.entry(seg.chapter).or_default()
                    .entry((seg.verses.start, seg.verses.end)).or_default()
            },
            PassageSegment::ChapterRange(seg) => {
                self.chapter_range.entry(seg.start.chapter).or_default()
                    .entry(seg.start.verse).or_default()
                    .entry(seg.end.chapter).or_default()
                    .entry(seg.end.verse).or_default()
            },
            PassageSegment::FullChapter(seg) => {
                self.full_chapter.entry(seg.chapter).or_default()
            },
            PassageSegment::FullChapterRange(seg) => {
                self.full_chapter_range.entry((seg.start.chapter, seg.end.chapter)).or_default()
            },
        }
    }
}


impl<Container: Debug + Default> BookOrganizer<Container> {
    pub fn get_chapter_verse_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (ChapterVerse, &'a Container)> {
        self.chapter_verse.range(seg.chapter_range()).flat_map(|(&chapter, map)| {
            map.range(seg.verse_range(chapter))
                .map(move|(&verse, container)| (ChapterVerse::new(chapter, verse), container))
        })
    }

    // pub fn get_chapter_verse_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (ChapterVerseRange, &'a Container)> {
    //     self.chapter_verse_range.range(seg.chapter_range()).flat_map(|(&chapter, verse_range_map)| {
    //         let verse_range = seg.verse_range(chapter);
    //         verse_range_map.range(verse_range).flat_map(move|(&start_verse, map)| {
    //             // Are you sure `verse_range` should not be `start_verse..=seg.ending_verse()`?
    //             // But start verse will always be 1 except for the first time, which is covered by
    //             // the `verse_range` method
    //             map.range(verse_range).map(move|(&end_verse, container)| {
    //                 (ChapterVerseRange::new(chapter, start_verse, end_verse), container)
    //             })
    //         })
    //     })
    // }

    pub fn get_chapter_verse_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = (ChapterVerseRange, &'a Container)> {
        self.chapter_verse_range.range(key.chapter_range()).flat_map(move |(&chapter, verse_range_map)| {
            // I just do `iter` because I need to start from the beginning of a range because I dont know when it ends
            verse_range_map.iter().filter_map(move|(&(start_verse, end_verse), container)| {
                let seg = ChapterVerseRange::new(chapter, start_verse, end_verse);
                seg.overlaps_with(key).then(|| (seg, container))
            })
            // early terminate when the key ends before the start of this segment
            .take_while(|(seg, _)| !key.ends_before(seg))
        })
    }


    pub fn get_chapter_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (ChapterRange, &'a Container)> {
         self.chapter_range.range(seg.chapter_range()).flat_map(move|(&start_chapter, map1)| {
            map1.range(seg.verse_range(start_chapter)).flat_map(move|(&start_verse, ending_chapter_map)| {
                ending_chapter_map.range(seg.chapter_range()).flat_map(move|(&end_chapter, ending_verse_map)| {
                    ending_verse_map.range(seg.verse_range(end_chapter)).map(move|(&end_verse, container)| {
                        (ChapterRange::new(start_chapter, start_verse, end_chapter, end_verse), container)
                    })
                })
            })
        })
    }

    pub fn get_full_chapter_content(&self, seg: &impl SegmentCompare) -> impl Iterator<Item = (FullChapter, &Container)> {
        self.full_chapter.range(seg.chapter_range())
            .map(|(&chapter, container)| (FullChapter::new(chapter), container))
    }

    pub fn get_full_chapter_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = (FullChapterRange, &'a Container)> {
        // I just do `iter` because I need to start from the beginning of a range because I dont know when it ends
        self.full_chapter_range.iter().filter_map(move |(&(start_chapter, end_chapter), container)| {
            let seg = FullChapterRange::new(start_chapter, end_chapter);
            seg.overlaps_with(key).then(|| (seg, container))
        })
        // early terminate when the key ends before the start of this segment
        .take_while(|(seg, _)| !key.ends_before(seg))
    }

}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{compare::SegmentCompare, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}};

    use super::BookOrganizer;

    #[test]
    fn chapter_verse() {
        let mut org = BookOrganizer::<()>::new();
        for ch in 1..=3 {
            for v in 1..=3 {
                org.modify(ChapterVerse::new(ch, v));
            }
        }

        assert_eq!(org.get_chapter_verse_content(&ChapterVerse::new(1, 1)).count(), 1);
        assert_eq!(org.get_chapter_verse_content(&ChapterVerse::new(4, 1)).count(), 0);

        assert_eq!(org.get_chapter_verse_content(&ChapterVerseRange::new(1, 1, 4)).count(), 3);
        assert_eq!(org.get_chapter_verse_content(&ChapterVerseRange::new(4, 1, 2)).count(), 0);

        assert_eq!(org.get_chapter_verse_content(&ChapterRange::new(1, 1, 2, 1)).count(), 4);
        assert_eq!(org.get_chapter_verse_content(&ChapterRange::new(4, 1, 5, 1)).count(), 0);

        assert_eq!(org.get_chapter_verse_content(&FullChapter::new(2)).count(), 3);
        assert_eq!(org.get_chapter_verse_content(&FullChapter::new(4)).count(), 0);

        assert_eq!(org.get_chapter_verse_content(&FullChapterRange::new(1, 2)).count(), 6);
        assert_eq!(org.get_chapter_verse_content(&FullChapterRange::new(4, 5)).count(), 0);
    }

    #[test]
    fn chapter_verse_range() {
        let mut org = BookOrganizer::<()>::new();
        org.modify(ChapterVerseRange::new(2, 1, 2));
        org.modify(ChapterVerseRange::new(2, 3, 4));
        org.modify(ChapterVerseRange::new(2, 2, 7));

        assert_eq!(org.get_chapter_verse_range_content(&ChapterVerse::new(2, 1)).count(), 1);
        assert_eq!(org.get_chapter_verse_range_content(&ChapterVerse::new(2, 2)).count(), 2);
        assert_eq!(org.get_chapter_verse_range_content(&ChapterVerse::new(4, 1)).count(), 0);

        assert_eq!(org.get_chapter_verse_range_content(&ChapterVerseRange::new(2, 1, 4)).count(), 3);
        assert_eq!(org.get_chapter_verse_range_content(&ChapterVerseRange::new(4, 1, 2)).count(), 0);

        assert_eq!(org.get_chapter_verse_range_content(&ChapterRange::new(1, 1, 2, 2)).count(), 2);
        assert_eq!(org.get_chapter_verse_range_content(&ChapterRange::new(4, 1, 5, 1)).count(), 0);

        assert_eq!(org.get_chapter_verse_range_content(&FullChapter::new(2)).count(), 3);
        assert_eq!(org.get_chapter_verse_range_content(&FullChapter::new(4)).count(), 0);

        assert_eq!(org.get_chapter_verse_range_content(&FullChapterRange::new(1, 2)).count(), 3);
        assert_eq!(org.get_chapter_verse_range_content(&FullChapterRange::new(4, 5)).count(), 0);
    }

    #[test]
    fn full_chapter() {
        let mut org = BookOrganizer::<()>::new();
        for ch in 1..=3 {
            org.modify(FullChapter::new(ch));
        }

        assert_eq!(org.get_full_chapter_content(&ChapterVerse::new(1, 1)).count(), 1);
        assert_eq!(org.get_full_chapter_content(&ChapterVerse::new(4, 1)).count(), 0);

        assert_eq!(org.get_full_chapter_content(&ChapterVerseRange::new(1, 1, 2)).count(), 1);
        assert_eq!(org.get_full_chapter_content(&ChapterVerseRange::new(4, 1, 2)).count(), 0);

        assert_eq!(org.get_full_chapter_content(&ChapterRange::new(1, 1, 2, 1)).count(), 2);
        assert_eq!(org.get_full_chapter_content(&ChapterRange::new(4, 1, 5, 1)).count(), 0);

        assert_eq!(org.get_full_chapter_content(&FullChapter::new(2)).count(), 1);
        assert_eq!(org.get_full_chapter_content(&FullChapter::new(4)).count(), 0);

        assert_eq!(org.get_full_chapter_content(&FullChapterRange::new(1, 2)).count(), 2);
        assert_eq!(org.get_full_chapter_content(&FullChapterRange::new(4, 5)).count(), 0);
    }

    #[test]
    fn full_chapter_range() {
        let mut org = BookOrganizer::<()>::new();
        // 1-3, 2-4, 3-5
        for start in 1..=3 {
            org.modify(FullChapterRange::new(start, start + 2));
        }

        assert_eq!(org.get_full_chapter_range_content(&ChapterVerse::new(1, 1)).count(), 1);
        assert_eq!(org.get_full_chapter_range_content(&ChapterVerse::new(6, 1)).count(), 0);

        assert_eq!(org.get_full_chapter_range_content(&ChapterVerseRange::new(1, 1, 2)).count(), 1);
        assert_eq!(org.get_full_chapter_range_content(&ChapterVerseRange::new(6, 1, 2)).count(), 0);

        assert_eq!(org.get_full_chapter_range_content(&ChapterRange::new(1, 1, 2, 1)).count(), 2);
        assert_eq!(org.get_full_chapter_range_content(&ChapterRange::new(6, 1, 7, 1)).count(), 0);

        assert_eq!(org.get_full_chapter_range_content(&FullChapter::new(2)).count(), 2);
        assert_eq!(org.get_full_chapter_range_content(&FullChapter::new(6)).count(), 0);

        // assert_eq!(org.get_full_chapter_range_content(&FullChapterRange::new(1, 2)).count(), 2);
        // assert_eq!(org.get_full_chapter_range_content(&FullChapterRange::new(6, 7)).count(), 0);
    }
}
