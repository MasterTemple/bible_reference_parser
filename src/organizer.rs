use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Bound;

use itertools::Itertools;

use crate::compare::SegmentCompare;

#[derive(Debug)]
pub struct BookOrganizer<Content: Debug> {
    chapter_verse: BTreeMap<u8, BTreeMap<u8, Content>>,
    // literally store it just like chapter_verse, but change
    // map[chapter] -> Map<StartIdx, Map<EndIdx, Content>>
    // map[chapter][start] -> Map<EndIdx, Content>
    // map[chapter][start][end] -> Content
    // so still return only 1 double iterator
    chapter_verse_range: BTreeMap<u8, BTreeMap<u8, BTreeMap<u8, Content>>>,
    chapter_range: BTreeMap<(u8, u8), Content>,
    full_chapter: BTreeMap<u8, Content>,
    full_chapter_range: BTreeMap<(u8, u8), Content>,
}

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

    pub fn get_chapter_verse_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (&'a u8, impl Iterator<Item = (&'a u8, &'a Content)>)> {
        self.chapter_verse.range(seg.chapter_range())
            .map(|(chapter, map)| {
                (chapter, map.range(seg.verse_range(*chapter)))
            })
    }


    // pub fn get_chapter_verse_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (&'a u8, impl Iterator<Item = ((&'a u8, &'a u8), &'a Content)>)> {
    //     self.chapter_verse_range.range(seg.chapter_range())
    //     .map(|(chapter, verse_range_map)| {
    //         let it2 = verse_range_map.range(seg.verse_range(*chapter))
    //         .map(|(start_verse, map)| {
    //             let it1 = map.range(seg.verse_range(*chapter)).flat_map(|(end, content)| {
    //                 ((start_verse, end), content)
    //             });
    //             it1
    //         });
    //         (chapter, it2)
    //     })
    // }

    // pub fn get_chapter_verse_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> Vec<(&'a u8, Vec<((&'a u8, &'a u8), &'a Content)>)> {
    //     let chapter_verse_range = self.chapter_verse_range.range(seg.chapter_range());
    //     let mut c = vec![];
    //     for (chapter, verse_range_map) in chapter_verse_range {
    //         let verse_range = verse_range_map.range(seg.verse_range(*chapter));
    //         let mut v = vec![];
    //         for(start_verse, map) in verse_range {
    //             for (end_verse, content) in map.range(seg.verse_range(*chapter)) {
    //                 v.push(((start_verse, end_verse), content));
    //             }
    //         }
    //         c.push((chapter, v));
    //     }
    //     c
    // }

    pub fn get_chapter_verse_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (&'a u8, Vec<((&'a u8, &'a u8), &'a Content)>)> {
        self.chapter_verse_range.range(seg.chapter_range()).map(|(chapter, verse_range_map)| {
            let verse_range = verse_range_map.range(seg.verse_range(*chapter));
            let mut v = vec![];
            for(start_verse, map) in verse_range {
                for (end_verse, content) in map.range(seg.verse_range(*chapter)) {
                    v.push(((start_verse, end_verse), content));
                }
            }
            (chapter, v)
        })
    }

    pub fn get_chapter_verse_range_content2<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (&'a u8, impl Iterator<Item = ((u8, &'a u8), &'a Content)>)> {
        self.chapter_verse_range.range(seg.chapter_range()).map(|(chapter, verse_range_map)| {
            let verse_range = verse_range_map.range(seg.verse_range(*chapter)).flat_map(|(ref start_verse, map)| {
                map.range(seg.verse_range(*chapter)).map(|(end_verse, content)| {
                    ((**start_verse, end_verse), content)
                })
            });
            (chapter, verse_range)
        })
    }

    pub fn get_chapter_verse_range_content3<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (u8, impl Iterator<Item = ((u8, u8), &'a Content)>)> {
        self.chapter_verse_range.range(seg.chapter_range()).map(|(chapter, verse_range_map)| {
            let verse_range = verse_range_map.range(seg.verse_range(*chapter)).flat_map(|(ref start_verse, map)| {
                map.range(seg.verse_range(*chapter)).map(|(end_verse, content)| {
                    ((**start_verse, *end_verse), content)
                })
            });
            (*chapter, verse_range)
        })
    }

    // pub fn get_chapter_verse_range_content<'a>(&'a self, seg: &'a impl SegmentCompare) -> impl Iterator<Item = (&'a u8, impl Iterator<Item = ((&'a u8, &'a u8), &'a Content)>)> {
    //     self.chapter_verse_range.range(seg.chapter_range())
    //     .map(|(chapter, verse_range_map)| {
    //         let it2 = verse_range_map.range(seg.verse_range(*chapter))
    //         .map(|(start_verse, map)| {
    //             let it1 = map.range(seg.verse_range(*chapter))/*.map(|(end, content)| {
    //                 ((start_verse, end), content)
    //             })*/;
    //             it1
    //         });
    //         (chapter, it2)
    //     })
    // }

    pub fn get_full_chapter_content(&self, seg: &impl SegmentCompare) -> impl Iterator<Item = (&u8, &Content)> {
        self.full_chapter.range(seg.chapter_range())
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
