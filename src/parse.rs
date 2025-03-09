use std::{fmt::Debug, ops::{Deref, DerefMut}};

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse, chapter_verse_range::ChapterVerseRange}, segment::PassageSegment, segments::PassageSegments};

static POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^ *\d+:\d+( *[,:;\-–] *\d+)*").unwrap());

static NON_SEGMENT_CHARACTERS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d,:;-]+").unwrap());

static TRAILING_NON_DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\D+$)").unwrap());

static SEGMENT_SPLITTERS: Lazy<Regex> = Lazy::new(|| Regex::new("(,|;)").unwrap());

pub(super) fn match_and_sanitize_segment_input(segment_input: &str) -> Option<String> {
    let segment_match = POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS
        .find_iter(segment_input)
        .next()?.as_str();

    // swap weird hyphens with normal dash
    let input = &segment_match.replace("–", "-");

    // input now only contains the following characters: [\d,:;-]
    let input = NON_SEGMENT_CHARACTERS.replace_all(&input, "").to_string();

    // removing trailing non-digits (leading shouldn't exist)
    let input = TRAILING_NON_DIGITS.replace_all(&input, "").to_string();

    Some(input)
}

/// full chapters are only parsed if there are no verses provided
pub(super) fn parse_full_chapters(segment_input: &str) -> Option<PassageSegments> {
    todo!()
}

/// - This function is meant to parse the `1:1-4,5-7,2:2-3:4,6` in `Ephesians 1:1-4,5-7,2:2-3:4,6`
/// - Don't pass it anything else please :)
/**
Passing `1` will result in
```no_run
[src/main.rs:27:5] parse_reference_segments("1") = [
    ChapterVerse(
        ChapterVerse {
            chapter: 1,
            verse: 1,
        },
    ),
]
```
Passing `1:` will result in
```no_run
[src/main.rs:28:5] parse_reference_segments("1:") = [
    ChapterVerse(
        ChapterVerse {
            chapter: 1,
            verse: 1,
        },
    ),
]
```
*/
pub(super) fn parse_reference_segments(input: &str) -> PassageSegments {
    // split at , or ; (because there is no uniform standard)
    // now I only have ranges (or a single verse)
    let ranges: Vec<&str> = SEGMENT_SPLITTERS.split(input).collect();
    // dbg!(&ranges);

    // ALWAYS UPDATE THE CHAPTER SO I CAN USE IT WHEN ONLY VERSES ARE PROVIDED
    let mut chapter = 1;
    let mut segments: Vec<PassageSegment> = Vec::new();
    for range in ranges {
        // if it is a range
        if let Some((left, right)) = range.split_once("-") {
            match (left.split_once(":"), right.split_once(":")) {
                // `ch1:v1 - ch2:v2`
                (Some((ch1, v1)), Some((ch2, v2))) => {
                    chapter = ch2.parse().unwrap();
                    segments.push(PassageSegment::chapter_range(
                            ch1.parse().unwrap(),
                            v1.parse().unwrap(),
                            chapter,
                            v2.parse().unwrap(),
                    ));
                }
                // `ch1:v1 - v2`
                (Some((ch1, v1)), None) => {
                    chapter = ch1.parse().unwrap();
                    segments.push(PassageSegment::chapter_verse_range(
                        chapter,
                        v1.parse().unwrap(),
                        right.parse().unwrap(),
                    ));
                }
                // `v1 - ch2:v2`
                (None, Some((ch2, v2))) => {
                    let start_chapter = chapter;
                    chapter = ch2.parse().unwrap();
                    segments.push(PassageSegment::chapter_range(
                            start_chapter,
                            left.parse().unwrap(),
                            chapter,
                            v2.parse().unwrap(),
                    ));
                }
                // `v1 - v2`
                (None, None) => {
                    segments.push(PassageSegment::chapter_verse_range(
                        chapter,
                        left.parse().unwrap(),
                        right.parse().unwrap(),
                    ));
                }
            };
        }
        // else it is not a range, either `ch:v` or `v`
        else {
            // handle `ch:v`
            if let Some((ch, v)) = range.split_once(":") {
                chapter = ch.parse().unwrap();
                segments.push(PassageSegment::ChapterVerse(ChapterVerse {
                    chapter,
                    verse: v.parse().unwrap(),
                }))
            }
            // handle `v`
            else {
                let v = range.parse().unwrap();
                segments.push(PassageSegment::ChapterVerse(ChapterVerse {
                    chapter,
                    verse: v,
                }))
            }
        }
    }
    PassageSegments(segments)
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    fn parse(input: &str) -> Vec<PassageSegment> {
        let parsed = PassageSegments::try_parse(input).unwrap();
        parsed.0
    }

    // ChapterVerse: 1:2
    #[test]
    fn chapter_verse() {
        assert_eq!(parse("1:2"), vec![
            PassageSegment::chapter_verse(1, 2)
        ]);
    }

    // ChapterVerseRange: 1:2-3
    #[test]
    fn chapter_verse_range() {
        assert_eq!(parse("1:2-3"), vec![
            PassageSegment::chapter_verse_range(1, 2, 3)
        ]);
    }

    // ChapterRange: 1:2-3:4
    #[test]
    fn chapter_range() {
        assert_eq!(parse("1:2-3:4"), vec![
            PassageSegment::chapter_range(1, 2, 3, 4)
        ]);
    }

    // FullChapter: 1
    #[test]
    fn full_chapter() {
        assert_eq!(parse("1"), vec![
            PassageSegment::full_chapter(1)
        ]);
    }

    // FullChapterRange: 1-2
    #[test]
    fn full_chapter_range() {
        assert_eq!(parse("1-2"), vec![
            PassageSegment::full_chapter_range(1, 2)
        ]);
    }

    // John 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8
    #[test]
    fn combined() {
        // 1
        assert_eq!(parse("1"), vec![
            PassageSegment::full_chapter(1),
        ]);

        // 1,2-4
        assert_eq!(parse("1,2-4"), vec![
            PassageSegment::full_chapter(1),
            PassageSegment::full_chapter_range(2, 4),
        ]);

        // 1,2-4,5:1-3
        assert_eq!(parse("1,2-4,5:1-3"), vec![
            PassageSegment::full_chapter(1),
            PassageSegment::full_chapter_range(2, 4),
            PassageSegment::chapter_verse_range(5, 1, 3),
        ]);

        // 1,2-4,5:1-3,5
        assert_eq!(parse("1,2-4,5:1-3,5"), vec![
            PassageSegment::full_chapter(1),
            PassageSegment::full_chapter_range(2, 4),
            PassageSegment::chapter_verse_range(5, 1, 3),
            PassageSegment::chapter_verse(5, 5),
        ]);

        // 1,2-4,5:1-3,5,7-9
        assert_eq!(parse("1,2-4,5:1-3,5,7-9"), vec![
            PassageSegment::full_chapter(1),
            PassageSegment::full_chapter_range(2, 4),
            PassageSegment::chapter_verse_range(5, 1, 3),
            PassageSegment::chapter_verse(5, 5),
            PassageSegment::chapter_verse_range(5, 7, 9),
        ]);

        // 1,2-4,5:1-3,5,7-9,12-6:6
        assert_eq!(parse("1,2-4,5:1-3,5,7-9,12-6:6"), vec![
            PassageSegment::full_chapter(1),
            PassageSegment::full_chapter_range(2, 4),
            PassageSegment::chapter_verse_range(5, 1, 3),
            PassageSegment::chapter_verse(5, 5),
            PassageSegment::chapter_verse_range(5, 7, 9),
            PassageSegment::chapter_range(5, 12, 6, 6),
        ]);

        // 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8
        assert_eq!(parse("1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8"), vec![
            PassageSegment::full_chapter(1),
            PassageSegment::full_chapter_range(2, 4),
            PassageSegment::chapter_verse_range(5, 1, 3),
            PassageSegment::chapter_verse(5, 5),
            PassageSegment::chapter_verse_range(5, 7, 9),
            PassageSegment::chapter_range(5, 12, 6, 6),
            PassageSegment::chapter_range(7, 7, 8, 8),
        ]);
    }
}
