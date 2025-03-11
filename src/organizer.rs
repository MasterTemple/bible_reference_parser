use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Bound;

use itertools::Itertools;

use crate::compare::SegmentCompare;
use crate::passage_segments::chapter_range::ChapterRange;
use crate::passage_segments::chapter_verse::ChapterVerse;
use crate::passage_segments::chapter_verse_range::ChapterVerseRange;
use crate::passage_segments::full_chapter::FullChapter;
use crate::passage_segments::full_chapter_range::FullChapterRange;

#[derive(Debug)]
pub struct BookOrganizer<Content: Debug> {
    /// `map[chapter][verse] -> Content`
    chapter_verse: BTreeMap<u8, BTreeMap<u8, Content>>,
    /// `map[chapter][start_verse][end_verse] -> Content`
    chapter_verse_range: BTreeMap<u8, BTreeMap<u8, BTreeMap<u8, Content>>>,
    /// `map[start_chapter][start_verse][end_chapter][end_verse] -> Content`
    chapter_range: BTreeMap<u8, BTreeMap<u8, BTreeMap<u8, BTreeMap<u8, Content>>>>,
    /// `map[chapter] -> Content`
    full_chapter: BTreeMap<u8, Content>,
    /// `map[start_chapter][end_chapter] -> Content`
    full_chapter_range: BTreeMap<u8, BTreeMap<u8, Content>>,
}

/**
TODO:
- Figure out if it is more efficient to remove all references on u8
- They are much smaller, but they are already created
- Or maybe it is a reference that is just being incremented

Perhaps I should return `PassageSegment`s instead of nested u8 groups
impl Iterator<Item = (ChapterVerse, &Content)>
which could be serialized to be { "1:1": {...}}
*/
impl<Content: Debug> BookOrganizer<Content> {
    pub fn new() -> Self {
        Self {
            chapter_verse: BTreeMap::default(),
            chapter_verse_range: BTreeMap::default(),
            chapter_range: BTreeMap::default(),
            full_chapter: BTreeMap::default(),
            full_chapter_range: BTreeMap::default(),
        }
    }

    pub fn get_chapter_verse_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (ChapterVerse, &'a Content)> {
        self.chapter_verse.range(seg.chapter_range()).flat_map(|(&chapter, map)| {
            map.range(seg.verse_range(chapter))
                .map(move|(&verse, content)| (ChapterVerse::new(chapter, verse), content))
        })
    }

    pub fn get_chapter_verse_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (ChapterVerseRange, &'a Content)> {
        self.chapter_verse_range.range(seg.chapter_range()).flat_map(|(&chapter, verse_range_map)| {
            let verse_range = seg.verse_range(chapter);
            verse_range_map.range(verse_range).flat_map(move|(&start_verse, map)| {
                map.range(verse_range).map(move|(&end_verse, content)| {
                    (ChapterVerseRange::new(chapter, start_verse, end_verse), content)
                })
            })
        })
    }

    pub fn get_chapter_range_content<'a>(&'a self, seg: &impl SegmentCompare) -> impl Iterator<Item = (ChapterRange, &'a Content)> {
        // _ = self.chapter_range.range(seg.chapter_range()).flat_map(|(ref start_chapter, map)| {
        // });

        vec![].into_iter()
    }

    pub fn get_full_chapter_content(&self, seg: &impl SegmentCompare) -> impl Iterator<Item = (FullChapter, &Content)> {
        self.full_chapter.range(seg.chapter_range())
            .map(|(&chapter, content)| (FullChapter::new(chapter), content))
    }


    pub fn get_full_chapter_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (FullChapterRange, &'a Content)> {
        self.full_chapter_range.range(seg.chapter_range()).flat_map(|(&start_chapter, map)| {
            // I should make `seg.chapter_range()` be `start_chapter..=seg.`
            map.range(start_chapter..=seg.ending_chapter()).map(move |(end_chapter, content)| {
                (FullChapterRange::new(start_chapter, *end_chapter), content)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use itertools::Itertools;

    use crate::passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange};

    use super::BookOrganizer;

    #[test]
    fn chapter_verse() {
        let mut org = BookOrganizer::<()>::new();
        org.chapter_verse.insert(1, BTreeMap::from([
            (1, ()), (2, ()), (3, ())
        ]));
        org.chapter_verse.insert(2, BTreeMap::from([
            (1, ()), (2, ()), (3, ())
        ]));
        org.chapter_verse.insert(3, BTreeMap::from([
            (1, ()), (2, ()), (3, ())
        ]));

        // assert_eq!(org.get_chapter_verse_content(&ChapterVerse::new(1, 1)).map(|pair| pair.1.count()).sum::<usize>(), 1);
        // assert_eq!(org.get_chapter_verse_content(&ChapterVerse::new(4, 1)).map(|pair| pair.1.count()).sum::<usize>(), 0);
        //
        // assert_eq!(org.get_chapter_verse_content(&ChapterVerseRange::new(1, 1, 4)).map(|pair| pair.1.count()).sum::<usize>(), 3);
        // assert_eq!(org.get_chapter_verse_content(&ChapterVerseRange::new(4, 1, 2)).map(|pair| pair.1.count()).sum::<usize>(), 0);
        //
        // assert_eq!(org.get_chapter_verse_content(&ChapterRange::new(1, 1, 2, 1)).map(|pair| pair.1.count()).sum::<usize>(), 4);
        // assert_eq!(org.get_chapter_verse_content(&ChapterRange::new(4, 1, 5, 1)).map(|pair| pair.1.count()).sum::<usize>(), 0);
        //
        // assert_eq!(org.get_chapter_verse_content(&FullChapter::new(2)).map(|pair| pair.1.count()).sum::<usize>(), 3);
        // assert_eq!(org.get_chapter_verse_content(&FullChapter::new(4)).map(|pair| pair.1.count()).sum::<usize>(), 0);
        //
        // assert_eq!(org.get_chapter_verse_content(&FullChapterRange::new(1, 2)).map(|pair| pair.1.count()).sum::<usize>(), 6);
        // assert_eq!(org.get_chapter_verse_content(&FullChapterRange::new(4, 5)).map(|pair| pair.1.count()).sum::<usize>(), 0);
    }

    #[test]
    fn full_chapter() {
        let mut org = BookOrganizer::<()>::new();
        org.full_chapter.insert(1, ());
        org.full_chapter.insert(2, ());
        org.full_chapter.insert(3, ());

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
}
