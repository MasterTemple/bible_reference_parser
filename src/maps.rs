use std::{collections::BTreeMap, ops::{Deref, DerefMut}};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{compare::SegmentCompare, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange, full_chapter::FullChapter, full_chapter_range::FullChapterRange}, segment::PassageSegment};

pub type ChapterRangeMap<V> = OverlapMap<ChapterRange, V>;
pub type ChapterVerseRangeMap<V> = OverlapMap<ChapterVerseRange, V>;
pub type ChapterVerseMap<V> = OverlapMap<ChapterVerse, V>;
pub type FullChapterRangeMap<V> = OverlapMap<FullChapterRange, V>;
pub type FullChapterMap<V> = OverlapMap<FullChapter, V>;
// pub type PassageSegmentMap<V> = OverlapMap<PassageSegment, V>;

// pub enum OverlappingPassageSegments<V> {
//     ChapterRange(Vec<(ChapterRange, V)>),
//     ChapterVerseRange(Vec<(ChapterVerseRange, V)>),
//     ChapterVerse(Vec<(ChapterVerse, V)>),
//     FullChapterRange(Vec<(FullChapterRange, V)>),
//     FullChapter(Vec<(FullChapter, V)>),
// }

pub struct OverlappingPassageSegments<'a, V> {
    pub chapter_range: Vec<(&'a ChapterRange, &'a V)>,
    pub chapter_verse_range: Vec<(&'a ChapterVerseRange, &'a V)>,
    pub chapter_verse: Vec<(&'a ChapterVerse, &'a V)>,
    pub full_chapter_range: Vec<(&'a FullChapterRange, &'a V)>,
    pub full_chapter: Vec<(&'a FullChapter, &'a V)>,
}

pub struct BookPassageMap<V> {
    chapter_range_map: ChapterRangeMap<V>,
    chapter_verse_range_map: ChapterVerseRangeMap<V>,
    chapter_verse_map: ChapterVerseMap<V>,
    full_chapter_range_map: FullChapterRangeMap<V>,
    full_chapter_map: FullChapterMap<V>,
}

impl<V> BookPassageMap<V> {
    /// - But what about updates when I don't want to completely write over the data?
    /// - How can I make an interface that would allow for V to be mutated?
    pub fn insert(&mut self, key: PassageSegment, value: V) -> Option<V> {
        match key {
            PassageSegment::ChapterVerse(key) => self.chapter_verse_map.insert(key, value),
            PassageSegment::ChapterVerseRange(key) => self.chapter_verse_range_map.insert(key, value),
            PassageSegment::ChapterRange(key) => self.chapter_range_map.insert(key, value),
            PassageSegment::FullChapter(key) => self.full_chapter_map.insert(key, value),
            PassageSegment::FullChapterRange(key) => self.full_chapter_range_map.insert(key, value),
        }
    }

    pub fn update(&mut self, key: PassageSegment) -> Option<&mut V> {
        match key {
            PassageSegment::ChapterVerse(key) => self.chapter_verse_map.get_mut(&key),
            PassageSegment::ChapterVerseRange(key) => self.chapter_verse_range_map.get_mut(&key),
            PassageSegment::ChapterRange(key) => self.chapter_range_map.get_mut(&key),
            PassageSegment::FullChapter(key) => self.full_chapter_map.get_mut(&key),
            PassageSegment::FullChapterRange(key) => self.full_chapter_range_map.get_mut(&key),
        }
    }
}

// I'm pretty sure these should take the general key
impl<V> BookPassageMap<V> {
    pub fn get_chapter_range_overlap(&self, key: &impl OverlapKey) -> Vec<(&ChapterRange, &V)> {
        self.chapter_range_map.get_overlapping(key)
    }

    pub fn get_chapter_verse_range_overlap(&self, key: &impl OverlapKey) -> Vec<(&ChapterVerseRange, &V)> {
        self.chapter_verse_range_map.get_overlapping(key)
    }

    pub fn get_chapter_verse_overlap(&self, key: &impl OverlapKey) -> Vec<(&ChapterVerse, &V)> {
        self.chapter_verse_map.get_overlapping(key)
    }

    pub fn get_full_chapter_range_overlap(&self, key: &impl OverlapKey) -> Vec<(&FullChapterRange, &V)> {
        self.full_chapter_range_map.get_overlapping(key)
    }

    pub fn get_full_chapter_overlap(&self, key: &impl OverlapKey) -> Vec<(&FullChapter, &V)> {
        self.full_chapter_map.get_overlapping(key)
    }

    pub fn get_all_overlap(&self, key: &impl OverlapKey) -> OverlappingPassageSegments<'_, V> {
        OverlappingPassageSegments {
            chapter_range: self.get_chapter_range_overlap(key),
            chapter_verse_range: self.get_chapter_verse_range_overlap(key),
            chapter_verse: self.get_chapter_verse_overlap(key),
            full_chapter_range: self.get_full_chapter_range_overlap(key),
            full_chapter: self.get_full_chapter_overlap(key),
        }
    }
}

pub trait OverlapKey: Ord + SegmentCompare + Into<PassageSegment> + Copy {}
impl<K: Ord + SegmentCompare + Into<PassageSegment> + Copy> OverlapKey for K {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OverlapMap<K: OverlapKey, V>(BTreeMap<K, V>);

impl<K: OverlapKey, V> Deref for OverlapMap<K, V>
{
    type Target = BTreeMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: OverlapKey, V> DerefMut for OverlapMap<K, V>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: OverlapKey, V> OverlapMap<K, V> {
    pub fn new() -> Self {
        Self(BTreeMap::default())
    }

    pub fn get_overlapping(&self, key: &impl OverlapKey) -> Vec<(&K, &V)> {
        // so I convert the key only once
        let seg: PassageSegment = (*key).into();
        self.iter().filter(|(key, _)| {
            seg.overlaps_with(*key)
        }).collect_vec()
    }

    /**
    Okay, I need to rethink this and consider ordering which determines how I search through and what heuristics can be done to early terminate


    Are you sure about removing the search left `break`?
    I should be able to stop when ending is less than (if
    entries are sorted by ending)

    Also the mechanics of how this works is probably different on PassageSegment
    than all other heterogeneous variants

    I think that my heuristics for ordering probably only work on heterogeneous
    variants, so I probably want to just have different maps anyway
    */
    pub fn get_overlapping_broken(&self, key: &K) -> Vec<(&K, &V)> {
        // so I convert the key only once
        let seg: PassageSegment = (*key).into();

        let mut result = Vec::new();

        // search left
        let mut range = self.range(..key);
        while let Some((prev_k, prev_v)) = range.next_back() {
            if seg.overlaps_with(prev_k) {
                result.push((prev_k, prev_v));
            } else {
                break;
            }
        }

        // since first elements are inserted backward
        result = result.into_iter().rev().collect();

        // search right (inclusive)
        let mut range = self.range(key..);
        while let Some((next_k, next_v)) = range.next() {
            if seg.overlaps_with(next_k) {
                result.push((next_k, next_v));
            } else {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod map_tests {

    use crate::{parse::ParsableSegment, passage_segments::chapter_verse_range::ChapterVerseRange};

    use super::*;

    #[test]
    fn test1() -> Result<(), String> {
        let mut map: OverlapMap<ChapterVerseRange, ()> = OverlapMap::new();

        map.insert(ChapterVerseRange::parse_strict("1:1-2")?, ());
        map.insert(ChapterVerseRange::parse_strict("1:4-5")?, ());
        map.insert(ChapterVerseRange::parse_strict("1:6-7")?, ());
        map.insert(ChapterVerseRange::parse_strict("2:1-2")?, ());

        let key = ChapterVerseRange::parse_strict("2:1-2")?;
        dbg!(map.get_overlapping(&key));

        let key = ChapterVerseRange::parse_strict("1:2-3")?;
        dbg!(map.get_overlapping(&key));

        let key = ChapterVerseRange::parse_strict("1:2-4")?;
        dbg!(map.get_overlapping(&key));

        Ok(())
    }

}
