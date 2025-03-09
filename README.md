# Bible Reference Parser

## Purpose

Parse all reference segments (everything but the book name) into a usable format

This will be used as a dependency for other projects of mine

## Features

### Segments

A segment is a unit or contiguous range of verses or chapters

**ChapterVerse**
- This is a single chapter/verse reference
- Ex: `1:2` in `John 1:2`

**ChapterVerseRange**
- This is a range of verse references within a single chapter
- Ex: `1:2-3` `John 1:2-3`

**ChapterRange**
- This is a range of verse references across a multiple chapters
- Ex: `John 1:2-3:4`

**FullChapter**
- This is a single chapter reference
- Ex: `1` in `John 1`

**FullChapterRange**
- This is a chapter range reference
- Ex: `1-2` in `John 1-2`

### Parsing

- Ignore white-space
- Handles various dash characters (`-`, `–`, `—`, `—`, or `⸺`) to delineate ranges
- Accept `,` or `;` to split segments

### Overlap

- Check if segments of any kind overlap with segments of any other kind
- Check if passages (sets of segments) overlap with other passages

### Example:

Actual passage reference:
`John 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8`

Reference segments:
`1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8`

<details>

<summary>Output</summary>

```ron
PassageSegments(
    [
        FullChapter(
            FullChapter {
                chapter: 1,
            },
        ),
        FullChapterRange(
            FullChapterRange(
                RangePair {
                    start: FullChapter {
                        chapter: 2,
                    },
                    end: FullChapter {
                        chapter: 4,
                    },
                },
            ),
        ),
        ChapterVerseRange(
            ChapterVerseRange {
                chapter: 5,
                verses: RangePair {
                    start: 1,
                    end: 3,
                },
            },
        ),
        ChapterVerse(
            ChapterVerse {
                chapter: 5,
                verse: 5,
            },
        ),
        ChapterVerseRange(
            ChapterVerseRange {
                chapter: 5,
                verses: RangePair {
                    start: 7,
                    end: 9,
                },
            },
        ),
        ChapterRange(
            ChapterRange(
                RangePair {
                    start: ChapterVerse {
                        chapter: 5,
                        verse: 12,
                    },
                    end: ChapterVerse {
                        chapter: 6,
                        verse: 6,
                    },
                },
            ),
        ),
        ChapterRange(
            ChapterRange(
                RangePair {
                    start: ChapterVerse {
                        chapter: 7,
                        verse: 7,
                    },
                    end: ChapterVerse {
                        chapter: 8,
                        verse: 8,
                    },
                },
            ),
        ),
    ],
),
```

</details>

## Installation

```toml
bible_reference_parser = { git = "https://github.com/MasterTemple/bible_reference_parser.git" }
```

## Usage

```rust
todo!()
```
