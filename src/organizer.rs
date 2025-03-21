use std::collections::BTreeMap;
use std::fmt::Debug;

use itertools::Itertools;

use crate::compare::{PassageContent, SegmentCompare};
use crate::passage_segments::chapter_range::ChapterRange;
use crate::passage_segments::chapter_verse::ChapterVerse;
use crate::passage_segments::chapter_verse_range::ChapterVerseRange;
use crate::passage_segments::full_chapter::FullChapter;
use crate::passage_segments::full_chapter_range::FullChapterRange;
use crate::segment::PassageSegment;

#[derive(Clone, Debug, Default)]
pub struct GroupedContent<'a, Container: Debug + Default> {
    pub chapter_verse: Vec<PassageContent<'a, ChapterVerse, Container>>,
    pub chapter_verse_range: Vec<PassageContent<'a, ChapterVerseRange, Container>>,
    pub chapter_range: Vec<PassageContent<'a, ChapterRange, Container>>,
    pub full_chapter: Vec<PassageContent<'a, FullChapter, Container>>,
    pub full_chapter_range: Vec<PassageContent<'a, FullChapterRange, Container>>,
}

/// It requires default not because the data type must impl Default, but it's container should
#[derive(Debug, Default)]
pub struct PassageOrganizer<Container: Debug + Default> {
    /// `map[chapter][verse] -> Container`
    chapter_verse: BTreeMap<u8, BTreeMap<u8, Container>>,
    /// `map[chapter][start_verse][end_verse] -> Container`
    chapter_verse_range: BTreeMap<u8, BTreeMap<(u8, u8), Container>>,
    /// `map[start_chapter][start_verse][end_chapter][end_verse] -> Container`
    chapter_range: BTreeMap<(u8, u8), BTreeMap<(u8, u8), Container>>,
    /// `map[chapter] -> Container`
    full_chapter: BTreeMap<u8, Container>,
    /// `map[start_chapter][end_chapter] -> Container`
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
impl<Container: Debug + Default> PassageOrganizer<Container> {
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
                self.chapter_range.entry((seg.start.chapter, seg.end.chapter)).or_default()
                    .entry((seg.start.verse, seg.end.verse)).or_default()
            },
            PassageSegment::FullChapter(seg) => {
                self.full_chapter.entry(seg.chapter).or_default()
            },
            PassageSegment::FullChapterRange(seg) => {
                self.full_chapter_range.entry((seg.start.chapter, seg.end.chapter)).or_default()
            },
        }
    }

    pub fn iter_all_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = PassageContent<'a, PassageSegment, Container>> {
        self.iter_chapter_verse_content(key).map(|psg| psg.generalize())
            .chain(self.iter_chapter_verse_range_content(key).map(|psg| psg.generalize()))
            .chain(self.iter_chapter_range_content(key).map(|psg| psg.generalize()))
            .chain(self.iter_full_chapter_content(key).map(|psg| psg.generalize()))
            .chain(self.iter_full_chapter_range_content(key).map(|psg| psg.generalize()))
    }

    pub fn get_all_content<'a>(&'a self, key: &'a impl SegmentCompare) -> Vec<PassageContent<'a, PassageSegment, Container>> {
        self.iter_all_content(key)
            .collect_vec()
    }

    pub fn get_all_content_grouped<'a>(&'a self, key: &'a impl SegmentCompare) -> GroupedContent<'a, Container> {
        GroupedContent {
            chapter_verse: self.get_chapter_verse_content(key),
            chapter_verse_range: self.get_chapter_verse_range_content(key),
            chapter_range: self.get_chapter_range_content(key),
            full_chapter: self.get_full_chapter_content(key),
            full_chapter_range: self.get_full_chapter_range_content(key),
        }
    }
}


impl<Container: Debug + Default> PassageOrganizer<Container> {
    pub fn get_chapter_verse_content<'a>(&'a self, key: &'a impl SegmentCompare) -> Vec<PassageContent<'a, ChapterVerse, Container>> {
        self.iter_chapter_verse_content(key).collect_vec()
    }

    pub fn iter_chapter_verse_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = PassageContent<'a, ChapterVerse, Container>> {
        self.chapter_verse.range(key.chapter_range()).flat_map(|(&chapter, map)| {
            map.range(key.verse_range(chapter))
                .map(move|(&verse, container)| (ChapterVerse::new(chapter, verse).with_content(container)))
        })
    }

    pub fn get_chapter_verse_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> Vec<PassageContent<'a, ChapterVerseRange, Container>> {
        self.iter_chapter_verse_range_content(key).collect_vec()
    }

    pub fn iter_chapter_verse_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = PassageContent<'a, ChapterVerseRange, Container>> {
        self.chapter_verse_range.range(key.chapter_range()).flat_map(move |(&chapter, verse_range_map)| {
            // I just do `iter` because I need to start from the beginning of a range because I dont know when it ends
            verse_range_map.iter().filter_map(move|(&(start_verse, end_verse), container)| {
                let seg = ChapterVerseRange::new(chapter, start_verse, end_verse);
                seg.overlaps_with(key).then(|| seg.with_content(container))
            })
            // early terminate when the key ends before the start of this segment
            .take_while(|psg| !key.ends_before(&psg.segment))
        })
    }

    pub fn get_chapter_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> Vec<PassageContent<'a, ChapterRange, Container>> {
        self.iter_chapter_range_content(key).collect_vec()
    }

    pub fn iter_chapter_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = PassageContent<'a, ChapterRange, Container>> {
         self.chapter_range.iter().flat_map(move|(&(start_chapter, end_chapter), verse_range_map)| {
            // I just do `iter` because I need to start from the beginning of a range because I dont know when it ends
            verse_range_map.iter().filter_map(move|(&(start_verse, end_verse), container)| {
                let seg = ChapterRange::new(start_chapter, start_verse, end_chapter, end_verse);
                seg.overlaps_with(key).then(|| seg.with_content(container))
            })
            // early terminate when the key ends before the start of this segment
            .take_while(|psg| !key.ends_before(&psg.segment))
        })
        // early terminate when the key ends before the start of this segment
        .take_while(|psg| !key.ends_before(&psg.segment))
    }

    pub fn get_full_chapter_content<'a>(&'a self, key: &impl SegmentCompare) -> Vec<PassageContent<'a, FullChapter, Container>> {
        self.iter_full_chapter_content(key).collect_vec()
    }

    pub fn iter_full_chapter_content<'a>(&'a self, key: &impl SegmentCompare) -> impl Iterator<Item = PassageContent<'a, FullChapter, Container>> {
        self.full_chapter.range(key.chapter_range())
            .map(|(&chapter, container)| (FullChapter::new(chapter).with_content(container)))
    }

    pub fn get_full_chapter_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> Vec<PassageContent<'a, FullChapterRange, Container>> {
        self.iter_full_chapter_range_content(key).collect_vec()
    }

    pub fn iter_full_chapter_range_content<'a>(&'a self, key: &'a impl SegmentCompare) -> impl Iterator<Item = PassageContent<'a, FullChapterRange, Container>> {
        // I just do `iter` because I need to start from the beginning of a range because I dont know when it ends
        self.full_chapter_range.iter().filter_map(move |(&(start_chapter, end_chapter), container)| {
            let seg = FullChapterRange::new(start_chapter, end_chapter);
            seg.overlaps_with(key).then(|| seg.with_content(container))
        })
        // early terminate when the key ends before the start of this segment
        .take_while(|psg| !key.ends_before(&psg.segment))
    }

}

#[cfg(test)]
mod tests {

    use crate::passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange};

    use super::PassageOrganizer;

    #[test]
    fn chapter_verse() {
        let mut org = PassageOrganizer::<()>::new();
        for ch in 1..=3 {
            for v in 1..=3 {
                org.modify(ChapterVerse::new(ch, v));
            }
        }

        assert_eq!(org.iter_chapter_verse_content(&ChapterVerse::new(1, 1)).count(), 1);
        assert_eq!(org.iter_chapter_verse_content(&ChapterVerse::new(4, 1)).count(), 0);

        assert_eq!(org.iter_chapter_verse_content(&ChapterVerseRange::new(1, 1, 4)).count(), 3);
        assert_eq!(org.iter_chapter_verse_content(&ChapterVerseRange::new(4, 1, 2)).count(), 0);

        assert_eq!(org.iter_chapter_verse_content(&ChapterRange::new(1, 1, 2, 1)).count(), 4);
        assert_eq!(org.iter_chapter_verse_content(&ChapterRange::new(4, 1, 5, 1)).count(), 0);

        assert_eq!(org.iter_chapter_verse_content(&FullChapter::new(2)).count(), 3);
        assert_eq!(org.iter_chapter_verse_content(&FullChapter::new(4)).count(), 0);

        assert_eq!(org.iter_chapter_verse_content(&FullChapterRange::new(1, 2)).count(), 6);
        assert_eq!(org.iter_chapter_verse_content(&FullChapterRange::new(4, 5)).count(), 0);
    }

    #[test]
    fn chapter_verse_range() {
        let mut org = PassageOrganizer::<()>::new();
        org.modify(ChapterVerseRange::new(2, 1, 2));
        org.modify(ChapterVerseRange::new(2, 3, 4));
        org.modify(ChapterVerseRange::new(2, 2, 7));

        assert_eq!(org.iter_chapter_verse_range_content(&ChapterVerse::new(2, 1)).count(), 1);
        assert_eq!(org.iter_chapter_verse_range_content(&ChapterVerse::new(2, 2)).count(), 2);
        assert_eq!(org.iter_chapter_verse_range_content(&ChapterVerse::new(4, 1)).count(), 0);

        assert_eq!(org.iter_chapter_verse_range_content(&ChapterVerseRange::new(2, 1, 4)).count(), 3);
        assert_eq!(org.iter_chapter_verse_range_content(&ChapterVerseRange::new(4, 1, 2)).count(), 0);

        assert_eq!(org.iter_chapter_verse_range_content(&ChapterRange::new(1, 1, 2, 2)).count(), 2);
        assert_eq!(org.iter_chapter_verse_range_content(&ChapterRange::new(4, 1, 5, 1)).count(), 0);

        assert_eq!(org.iter_chapter_verse_range_content(&FullChapter::new(2)).count(), 3);
        assert_eq!(org.iter_chapter_verse_range_content(&FullChapter::new(4)).count(), 0);

        assert_eq!(org.iter_chapter_verse_range_content(&FullChapterRange::new(1, 2)).count(), 3);
        assert_eq!(org.iter_chapter_verse_range_content(&FullChapterRange::new(4, 5)).count(), 0);
    }

    #[test]
    fn chapter_range() {
        let mut org = PassageOrganizer::<()>::new();
        org.modify(ChapterRange::new(2, 1, 3, 3));
        org.modify(ChapterRange::new(2, 4, 3, 7));
        org.modify(ChapterRange::new(1, 1, 4, 3));

        assert_eq!(org.iter_chapter_range_content(&ChapterVerse::new(2, 1)).count(), 2);
        assert_eq!(org.iter_chapter_range_content(&ChapterVerse::new(2, 4)).count(), 3);
        assert_eq!(org.iter_chapter_range_content(&ChapterVerse::new(4, 1)).count(), 1);
        assert_eq!(org.iter_chapter_range_content(&ChapterVerse::new(5, 1)).count(), 0);

        assert_eq!(org.iter_chapter_range_content(&ChapterVerseRange::new(2, 1, 4)).count(), 3);
        assert_eq!(org.iter_chapter_range_content(&ChapterVerseRange::new(2, 5, 8)).count(), 3);
        assert_eq!(org.iter_chapter_range_content(&ChapterVerseRange::new(4, 1, 2)).count(), 1);
        assert_eq!(org.iter_chapter_range_content(&ChapterVerseRange::new(5, 1, 2)).count(), 0);

        assert_eq!(org.iter_chapter_range_content(&ChapterRange::new(1, 1, 2, 2)).count(), 2);
        assert_eq!(org.iter_chapter_range_content(&ChapterRange::new(4, 1, 5, 1)).count(), 1);
        assert_eq!(org.iter_chapter_range_content(&ChapterRange::new(4, 4, 5, 1)).count(), 0);

        assert_eq!(org.iter_chapter_range_content(&FullChapter::new(2)).count(), 3);
        assert_eq!(org.iter_chapter_range_content(&FullChapter::new(4)).count(), 1);
        assert_eq!(org.iter_chapter_range_content(&FullChapter::new(5)).count(), 0);

        assert_eq!(org.iter_chapter_range_content(&FullChapterRange::new(1, 2)).count(), 3);
        assert_eq!(org.iter_chapter_range_content(&FullChapterRange::new(4, 5)).count(), 1);
    }

    #[test]
    fn full_chapter() {
        let mut org = PassageOrganizer::<()>::new();
        for ch in 1..=3 {
            org.modify(FullChapter::new(ch));
        }

        assert_eq!(org.iter_full_chapter_content(&ChapterVerse::new(1, 1)).count(), 1);
        assert_eq!(org.iter_full_chapter_content(&ChapterVerse::new(4, 1)).count(), 0);

        assert_eq!(org.iter_full_chapter_content(&ChapterVerseRange::new(1, 1, 2)).count(), 1);
        assert_eq!(org.iter_full_chapter_content(&ChapterVerseRange::new(4, 1, 2)).count(), 0);

        assert_eq!(org.iter_full_chapter_content(&ChapterRange::new(1, 1, 2, 1)).count(), 2);
        assert_eq!(org.iter_full_chapter_content(&ChapterRange::new(4, 1, 5, 1)).count(), 0);

        assert_eq!(org.iter_full_chapter_content(&FullChapter::new(2)).count(), 1);
        assert_eq!(org.iter_full_chapter_content(&FullChapter::new(4)).count(), 0);

        assert_eq!(org.iter_full_chapter_content(&FullChapterRange::new(1, 2)).count(), 2);
        assert_eq!(org.iter_full_chapter_content(&FullChapterRange::new(4, 5)).count(), 0);
    }

    #[test]
    fn full_chapter_range() {
        let mut org = PassageOrganizer::<()>::new();
        // 1-3, 2-4, 3-5
        for start in 1..=3 {
            org.modify(FullChapterRange::new(start, start + 2));
        }

        assert_eq!(org.iter_full_chapter_range_content(&ChapterVerse::new(1, 1)).count(), 1);
        assert_eq!(org.iter_full_chapter_range_content(&ChapterVerse::new(6, 1)).count(), 0);

        assert_eq!(org.iter_full_chapter_range_content(&ChapterVerseRange::new(1, 1, 2)).count(), 1);
        assert_eq!(org.iter_full_chapter_range_content(&ChapterVerseRange::new(6, 1, 2)).count(), 0);

        assert_eq!(org.iter_full_chapter_range_content(&ChapterRange::new(1, 1, 2, 1)).count(), 2);
        assert_eq!(org.iter_full_chapter_range_content(&ChapterRange::new(6, 1, 7, 1)).count(), 0);

        assert_eq!(org.iter_full_chapter_range_content(&FullChapter::new(2)).count(), 2);
        assert_eq!(org.iter_full_chapter_range_content(&FullChapter::new(6)).count(), 0);

        assert_eq!(org.iter_full_chapter_range_content(&FullChapterRange::new(1, 2)).count(), 2);
        assert_eq!(org.iter_full_chapter_range_content(&FullChapterRange::new(6, 7)).count(), 0);
    }
}
