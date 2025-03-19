use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use itertools::Itertools;
use numerals::roman::Roman;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use super::{
    segment::{any_segment::AnySegment, types::chapter_verse::ChapterVerse},
    segments::PassageSegments,
};

/// Basically, start with and end with a digit
/// and then collect digits joined by ranges `-–——⸺` or segments `,;` or chapters `:`
pub static POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS: Lazy<Regex> =
    // without roman numerals
    // Lazy::new(|| Regex::new(r"^ *\d+( *[\.,:;\-–——⸺] *\d+)*").unwrap());
    // with roman numerals `\d+` -> `([ivxlc]+|\d+)`
    Lazy::new(|| Regex::new(r"^ *([ivxlc]+|\d+)( *[\.,:;\-–——⸺] *([ivxlc]+|\d+))*").unwrap());

const ALL_DASHES: [char; 5] = ['-', '–', '—', '—', '⸺'];
const SEGMENT_SPLITTERS: [char; 2] = [',', ';'];

static NON_SEGMENT_CHARACTERS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d,:;-]+").unwrap());
static ROMAN_NUMERALS: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ivxlc]+").unwrap());

static TRAILING_NON_DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\D+$)").unwrap());

impl PassageSegments {
    pub fn parse(segment_input: &str) -> Result<Self, String> {
        let input = match_and_sanitize_segment_input(segment_input)
            .ok_or_else(|| String::from("Failed to parse segments"))?;
        let segments = parse_reference_segments(&input);
        Ok(segments)
    }
}

impl FromStr for PassageSegments {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

fn match_and_sanitize_segment_input(segment_input: &str) -> Option<String> {
    let segment_match = POST_BOOK_VALID_REFERENCE_SEGMENT_CHARACTERS
        .find_iter(segment_input)
        .next()?
        .as_str();

    // swap weird hyphens with normal dash
    let input = &segment_match.replace(ALL_DASHES, "-");

    // swap period with colon (to support 'Jn1.1')
    let input = &input.replace(".", ":");

    let input = ROMAN_NUMERALS
        .replace_all(&input, |m: &Captures| {
            let s = m.get(0).unwrap().as_str();
            let parsed = Roman::parse(s);
            parsed.unwrap().value().to_string()
        })
        .to_string();

    // input now only contains the following characters: [\d,:;-]
    let input = NON_SEGMENT_CHARACTERS.replace_all(&input, "").to_string();

    // removing trailing non-digits (leading shouldn't exist)
    let input = TRAILING_NON_DIGITS
        .replace_all(&input, "")
        .trim()
        .to_string();

    Some(input)
}

/// - This function is meant to parse the `1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8` in `John 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8`
/// - It expects input [`from match_and_sanitize_segment_input`]
fn parse_reference_segments(input: &str) -> PassageSegments {
    // split at , or ; (because there is no uniform standard)
    // now I only have ranges (or a single verse)
    let ranges: Vec<&str> = input.split(SEGMENT_SPLITTERS).collect();
    // dbg!(&ranges);

    // ALWAYS UPDATE THE CHAPTER SO I CAN USE IT WHEN ONLY VERSES ARE PROVIDED
    let mut chapter = 1;
    // once i see a verse, it is indeterminable when chapters are found, so I will no longer
    // consider them
    let mut check_for_full_chapters = true;
    let mut segments: Vec<AnySegment> = Vec::new();
    for range in ranges {
        // if it is a range
        if let Some((left, right)) = range.split_once("-") {
            if check_for_full_chapters {
                // try a chapter range
                if !left.contains(":") && !right.contains(":") {
                    let start = left.parse().unwrap();
                    let end = right.parse().unwrap();
                    segments.push(AnySegment::full_chapter_range(start, end));
                    chapter = end;
                    continue;
                }
            }
            check_for_full_chapters = false;

            match (left.split_once(":"), right.split_once(":")) {
                // `ch1:v1 - ch2:v2`
                (Some((ch1, v1)), Some((ch2, v2))) => {
                    chapter = ch2.parse().unwrap();
                    segments.push(AnySegment::chapter_range(
                        ch1.parse().unwrap(),
                        v1.parse().unwrap(),
                        chapter,
                        v2.parse().unwrap(),
                    ));
                }
                // `ch1:v1 - v2`
                (Some((ch1, v1)), None) => {
                    chapter = ch1.parse().unwrap();
                    segments.push(AnySegment::chapter_verse_range(
                        chapter,
                        v1.parse().unwrap(),
                        right.parse().unwrap(),
                    ));
                }
                // `v1 - ch2:v2`
                (None, Some((ch2, v2))) => {
                    let start_chapter = chapter;
                    chapter = ch2.parse().unwrap();
                    segments.push(AnySegment::chapter_range(
                        start_chapter,
                        left.parse().unwrap(),
                        chapter,
                        v2.parse().unwrap(),
                    ));
                }
                // `v1 - v2`
                (None, None) => {
                    segments.push(AnySegment::chapter_verse_range(
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
                segments.push(AnySegment::chapter_verse(chapter, v.parse().unwrap()));
            }
            // handle `ch` or `v`
            else {
                // handle `ch`
                if check_for_full_chapters {
                    chapter = range.parse().unwrap();
                    segments.push(AnySegment::full_chapter(chapter));
                    continue;
                }

                // handle `v`
                let v = range.parse().unwrap();
                segments.push(AnySegment::ChapterVerse(ChapterVerse { chapter, verse: v }))
            }
            check_for_full_chapters = false;
        }
    }
    PassageSegments(segments)
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    fn parse(input: &str) -> Vec<AnySegment> {
        let parsed = PassageSegments::parse(input).unwrap();
        parsed.0
    }

    #[test]
    fn roman() {
        assert_eq!(parse("i:2"), vec![AnySegment::chapter_verse(1, 2)]);
        assert_eq!(parse("iv:2"), vec![AnySegment::chapter_verse(4, 2)]);
        assert_eq!(parse("v:2"), vec![AnySegment::chapter_verse(5, 2)]);
    }

    // ChapterVerse: 1:2
    #[test]
    fn chapter_verse() {
        assert_eq!(parse("1:2"), vec![AnySegment::chapter_verse(1, 2)]);

        assert_eq!(parse("1.2"), vec![AnySegment::chapter_verse(1, 2)]);
    }

    // ChapterVerseRange: 1:2-3
    #[test]
    fn chapter_verse_range() {
        assert_eq!(
            parse("1:2-3"),
            vec![AnySegment::chapter_verse_range(1, 2, 3)]
        );
    }

    // ChapterRange: 1:2-3:4
    #[test]
    fn chapter_range() {
        assert_eq!(
            parse("1:2-3:4"),
            vec![AnySegment::chapter_range(1, 2, 3, 4)]
        );
    }

    // FullChapter: 1
    #[test]
    fn full_chapter() {
        assert_eq!(parse("1"), vec![AnySegment::full_chapter(1)]);
    }

    // FullChapterRange: 1-2
    #[test]
    fn full_chapter_range() {
        assert_eq!(parse("1-2"), vec![AnySegment::full_chapter_range(1, 2)]);
    }

    #[test]
    fn change_out_of_chapter() {
        assert_eq!(
            parse("1:1,3-4"),
            vec![
                AnySegment::chapter_verse(1, 1),
                AnySegment::chapter_verse_range(1, 3, 4),
            ]
        )
    }

    // John 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8
    #[test]
    fn combined() {
        // 1
        assert_eq!(parse("1"), vec![AnySegment::full_chapter(1),]);

        // 1,2-4
        assert_eq!(
            parse("1,2-4"),
            vec![
                AnySegment::full_chapter(1),
                AnySegment::full_chapter_range(2, 4),
            ]
        );

        // 1,2-4,5:1-3
        assert_eq!(
            parse("1,2-4,5:1-3"),
            vec![
                AnySegment::full_chapter(1),
                AnySegment::full_chapter_range(2, 4),
                AnySegment::chapter_verse_range(5, 1, 3),
            ]
        );

        // 1,2-4,5:1-3,5
        assert_eq!(
            parse("1,2-4,5:1-3,5"),
            vec![
                AnySegment::full_chapter(1),
                AnySegment::full_chapter_range(2, 4),
                AnySegment::chapter_verse_range(5, 1, 3),
                AnySegment::chapter_verse(5, 5),
            ]
        );

        // 1,2-4,5:1-3,5,7-9
        assert_eq!(
            parse("1,2-4,5:1-3,5,7-9"),
            vec![
                AnySegment::full_chapter(1),
                AnySegment::full_chapter_range(2, 4),
                AnySegment::chapter_verse_range(5, 1, 3),
                AnySegment::chapter_verse(5, 5),
                AnySegment::chapter_verse_range(5, 7, 9),
            ]
        );

        // 1,2-4,5:1-3,5,7-9,12-6:6
        assert_eq!(
            parse("1,2-4,5:1-3,5,7-9,12-6:6"),
            vec![
                AnySegment::full_chapter(1),
                AnySegment::full_chapter_range(2, 4),
                AnySegment::chapter_verse_range(5, 1, 3),
                AnySegment::chapter_verse(5, 5),
                AnySegment::chapter_verse_range(5, 7, 9),
                AnySegment::chapter_range(5, 12, 6, 6),
            ]
        );

        // 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8
        assert_eq!(
            parse("1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8"),
            vec![
                AnySegment::full_chapter(1),
                AnySegment::full_chapter_range(2, 4),
                AnySegment::chapter_verse_range(5, 1, 3),
                AnySegment::chapter_verse(5, 5),
                AnySegment::chapter_verse_range(5, 7, 9),
                AnySegment::chapter_range(5, 12, 6, 6),
                AnySegment::chapter_range(7, 7, 8, 8),
            ]
        );
    }
}
