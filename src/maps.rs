use std::{collections::BTreeMap, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::{overlap::OverlapsWith, segment::PassageSegment};

pub trait OverlapKey: Ord + OverlapsWith + Into<PassageSegment> + Copy {}
impl<K: Ord + OverlapsWith + Into<PassageSegment> + Copy> OverlapKey for K {}

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

    pub fn get_overlapping(&self, key: &K) -> Vec<(&K, &V)> {
        // so I convert the key only once
        let seg: PassageSegment = (*key).into();

        let mut result = Vec::new();

        // search left
        let mut range = self.range(..key);
        while let Some((prev_k, prev_v)) = range.next_back() {
            if seg.overlaps_segment(*prev_k) {
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
            if seg.overlaps_segment(*next_k) {
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
        let mut map: OverlapMap<ChapterVerseRange, i32> = OverlapMap::new();

        map.insert(ChapterVerseRange::parse("1:1-2")?, 1);
        map.insert(ChapterVerseRange::parse("1:4-5")?, 1);
        map.insert(ChapterVerseRange::parse("1:6-7")?, 1);
        map.insert(ChapterVerseRange::parse("2:1-2")?, 1);

        let key = ChapterVerseRange::parse("2:1-2")?;
        dbg!(map.get_overlapping(&key));

        let key = ChapterVerseRange::parse("1:2-3")?;
        dbg!(map.get_overlapping(&key));

        let key = ChapterVerseRange::parse("1:2-4")?;
        dbg!(map.get_overlapping(&key));

        Ok(())
    }

}
