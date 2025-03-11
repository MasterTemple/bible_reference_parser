use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Bound;

use crate::compare::SegmentCompare;

#[derive(Debug)]
pub struct BookOrganizer<Content: Debug> {
    // chapter:verse (Map<chapter, Map<verse, Vec<ref>>>)
    chapter_verse: BTreeMap<u8, BTreeMap<u8, Content>>,
    // chapter:start_verse-end_verse (Map<chapter, Map<(start_verse, end_verse), ref>>)
    // chapter_verse_range: BTreeMap<u8, OverlapMap<RangePair, Content>>,
    // start_chapter:start_verse-end_chapter:end_verse
    // chapter_range: BTreeMap<, Content>,
    full_chapter: BTreeMap<u8, Content>,
    full_chapter_range: BTreeMap<(u8, u8), Content>,
}

impl<Content: Debug> BookOrganizer<Content> {
    pub fn new() -> Self {
        Self {
            chapter_verse: BTreeMap::default(),
            full_chapter: BTreeMap::default(),
            full_chapter_range: BTreeMap::default(),
        }
    }

    pub fn get_chapter_verse_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (&'a u8, impl Iterator<Item = (&'a u8, &'a Content)>)> {
        self.chapter_verse.range(seg.starting_chapter()..=seg.ending_chapter())
            .map(|(chapter, map)| {
                let end_bound = if *chapter == seg.ending_chapter() {
                    match seg.ending_verse() {
                        Some(ending_verse) => Bound::Included(ending_verse),
                        None => Bound::Unbounded
                    }
                } else {
                    Bound::Unbounded
                };
                let range = (Bound::Included(seg.starting_verse()), end_bound);
                (chapter, map.range(range))
            })
    }

    pub fn get_full_chapter_content(&self, seg: &impl SegmentCompare) -> impl Iterator<Item = (&u8, &Content)> {
        self.full_chapter.range(seg.starting_chapter()..=seg.ending_chapter())
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

        assert_eq!(org.get_chapter_verse_content(&ChapterVerse::new(1, 1)).map(|pair| pair.1.count()).sum::<usize>(), 1);
        assert_eq!(org.get_chapter_verse_content(&ChapterVerse::new(4, 1)).map(|pair| pair.1.count()).sum::<usize>(), 0);

        assert_eq!(org.get_chapter_verse_content(&ChapterVerseRange::new(1, 1, 4)).map(|pair| pair.1.count()).sum::<usize>(), 3);
        assert_eq!(org.get_chapter_verse_content(&ChapterVerseRange::new(4, 1, 2)).map(|pair| pair.1.count()).sum::<usize>(), 0);

        assert_eq!(org.get_chapter_verse_content(&ChapterRange::new(1, 1, 2, 1)).map(|pair| pair.1.count()).sum::<usize>(), 4);
        assert_eq!(org.get_chapter_verse_content(&ChapterRange::new(4, 1, 5, 1)).map(|pair| pair.1.count()).sum::<usize>(), 0);

        assert_eq!(org.get_chapter_verse_content(&FullChapter::new(2)).map(|pair| pair.1.count()).sum::<usize>(), 3);
        assert_eq!(org.get_chapter_verse_content(&FullChapter::new(4)).map(|pair| pair.1.count()).sum::<usize>(), 0);

        assert_eq!(org.get_chapter_verse_content(&FullChapterRange::new(1, 2)).map(|pair| pair.1.count()).sum::<usize>(), 6);
        assert_eq!(org.get_chapter_verse_content(&FullChapterRange::new(4, 5)).map(|pair| pair.1.count()).sum::<usize>(), 0);
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
