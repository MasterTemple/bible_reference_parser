use std::{collections::BTreeMap, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::{overlap::OverlapsWith, passage_segments::chapter_verse_range::ChapterVerseRange, segment::PassageSegment};

// /**
// This is references to all the related media for a book
// */
// #[derive(Default)]
// pub struct RelatedMediaBook<Data> {
//     // chapter:verse (Map<chapter, Map<verse, Vec<ref>>>)
//     chapter_verse: BTreeMap<usize, BTreeMap<usize, Vec<RelatedMediaRef>>>,
//     // chapter:start_verse-end_verse (Map<chapter, Map<(start_verse, end_verse), ref>>)
//     chapter_verse_range: BTreeMap<usize, OverlapMap<RangePair, Vec<RelatedMediaRef>>>,
//     // start_chapter:start_verse-end_chapter:end_verse
//     chapter_range: OverlapMap<ChapterRangePair, Vec<RelatedMediaRef>>,
// }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChapterVerseRangeMap<T>(BTreeMap<ChapterVerseRange, T>);
impl<T> ChapterVerseRangeMap<T> {
    pub fn new() -> Self {
        Self(BTreeMap::default())
    }
}

impl<T> Deref for ChapterVerseRangeMap<T> {
    type Target = BTreeMap<ChapterVerseRange, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ChapterVerseRangeMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn find_overlapping_segments<'a, K, V>(
    map: &'a BTreeMap<K, V>,
    key: &'a K,
) -> Vec<(&'a K, &'a V)>
where
    K: Ord + OverlapsWith + Into<PassageSegment> + Copy,
{
    let mut result = Vec::new();

    // search left
    let mut range = map.range(..key);
    while let Some((prev_k, prev_v)) = range.next_back() {
        if key.overlaps_segment(*prev_k) {
            result.push((prev_k, prev_v));
        } else {
            break;
        }
    }

    result = result.into_iter().rev().collect();

    // search right (inclusive)
    let mut range = map.range(key..);
    while let Some((next_k, next_v)) = range.next() {
        if key.overlaps_segment(*next_k) {
            result.push((next_k, next_v));
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod map_tests {
    use itertools::Itertools;

    use crate::{overlap::OverlapsWith, parse::ParsableSegment};

    use super::*;

    #[test]
    fn test1() -> Result<(), String> {
        let mut map: ChapterVerseRangeMap<i32> = ChapterVerseRangeMap(BTreeMap::new());

        map.insert(ChapterVerseRange::parse("1:1-2")?, 1);
        map.insert(ChapterVerseRange::parse("1:4-5")?, 1);
        map.insert(ChapterVerseRange::parse("1:6-7")?, 1);
        map.insert(ChapterVerseRange::parse("2:1-2")?, 1);

        let key = ChapterVerseRange::parse("2:1-2")?;
        dbg!(find_overlapping_segments(&map, &key));

        let key = ChapterVerseRange::parse("1:2-3")?;
        dbg!(find_overlapping_segments(&map, &key));

        let key = ChapterVerseRange::parse("1:2-4")?;
        dbg!(find_overlapping_segments(&map, &key));

        // dbg!(map.lower_bound(std::ops::Bound::Included(&key)));

        // dbg!(&map);

        Ok(())
    }

}
