# Bible Reference Parser

## Purpose

Parse all reference segments (everything but the book name) into a usable format

This will be used as a dependency for other projects of mine

## Features

### Parsing

- Ignore white-space
- Handles various dash characters (`-`, `–`, `—`, `—`, or `⸺`) to delineate ranges
- Accept `,` or `;` to split segments

### Overlap

- Check if segments of any kind overlap with segments of any other kind
- Check if passages (sets of segments) overlap with other passages or individual segments

## Installation

```toml
bible_reference_parser = { git = "https://github.com/MasterTemple/bible_reference_parser.git" }
```

## Usage

**Note:** books are included in comments for readability sake

### Data Structures

### Segments

A segment is a unit or contiguous range of verses or chapters

```text
                  `John 1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8`
                       | |   |     | |   |      |       |
-----------------------+ |   |     | |   |      |       |
Full Chapter:        `1` |   |     | |   |      |       |
-------------------------+   |     | |   |      |       |
Full Chapter Range:  `2-4`   |     | |   |      |       |
-----------------------------+     | |   |      |       |
Chapter Range:       `5:1-3`       | |   |      |       |
-----------------------------------+ |   |      |       |
Chapter Verse:       `5:5            |   |      |       |
-------------------------------------+   |      |       |
Chapter Verse Range: `5:7-9`             |      |       |
-----------------------------------------+      |       |
Chapter Range:       `5:12-6:6`                 |       |
------------------------------------------------+       |
Chapter Range:       `7:7-8:8`                          |
--------------------------------------------------------+
```

```rust
pub enum PassageSegment {
    /// - This is a single chapter/verse reference
    /// - Ex: `1:2` in `John 1:2`
    ChapterVerse(ChapterVerse),
    /// - This is a range of verse references within a single chapter
    /// - Ex: `1:2-3` `John 1:2-3`
    ChapterVerseRange(ChapterVerseRange),
    /// - This is a range of verse references across a multiple chapters
    /// - Ex: `John 1:2-3:4`
    ChapterRange(ChapterRange),
    /// - This is a single chapter reference
    /// - Ex: `1` in `John 1`
    FullChapter(FullChapter),
    /// - This is a chapter range reference
    /// - Ex: `1-2` in `John 1-2`
    FullChapterRange(FullChapterRange),
}
```

### Creating/Parsing

```rust
// Create specific passage reference segment type
ChapterVerseRange::new(1, 2, 3);
ChapterVerseRange::parse("1:2-3")?;
let _: ChapterVerseRange = "1:2-3".parse()?;

// Create as generic passage segment
PassageSegment::ChapterVerseRange(ChapterVerseRange::new(1, 2, 3));
PassageSegment::chapter_verse_range(1, 2, 3);
let _: PassageSegment = ChapterVerseRange::new(1, 2, 3).into();
PassageSegment::parse("1:2-3")?;
let _: PassageSegment = "1:2-3".parse()?;

// Coerce segment types into other compatible types
// coerce to a wider scope: John 1:2 -> John 1
FullChapter::try_from(PassageSegment::chapter_verse(1, 2))?; // FullChapter
// coerce to actual type of a segment: John 1:2-2 -> John 1:2
ChapterVerseRange::parse("1:2-2")?.actual(); // ChapterVerse

// Parse multiple contextually-joined segments
PassageSegments::parse("1,2-4,5:1-3,5,7-9,12-6:6,7:7-8:8")?;
PassageSegments::parse("   1, 2-4; 5:1–3,5,7—9,12—6:6,  7:7⸺  8:8, ")?;
```

### Check Overlap

```rust
let psg = |p: &str| PassageSegment::parse(p).unwrap(); // shorthand for single passage

// check individual segment overlap
psg("1:1").overlaps_with(&psg("1:2")); // false
psg("1:1-3").overlaps_with(&psg("1:2")); // true
psg("1:1-3").overlaps_with(&psg("1:3-4")); // true

let first = PassageSegments::parse("1:1-3, 4; 2:1-3")?;
let second = PassageSegments::parse("2:2-8; 3:1-4:1")?;
let third = PassageSegments::parse("2:9-12")?;

// check if segment list has any overlap with a single segment
first.overlaps_with(&psg("1:1")); // true
second.overlaps_with(&psg("1:1")); // false

// check for any overlapping segments between segment list
first.contains_overlap(&second); // true
first.contains_overlap(&third); // false
second.contains_overlap(&third); // false
```

### Passage Organizer

The `PassageOrganizer` is an efficient data structure that stores any data in a container, **accessible via any passage overlap**

> [!IMPORTANT]
> This has a massive use case for storing content related to a Bible verse,
> because **from any selected Scripture reference, you can access all stored overlapping content**

```rust
let psg = |p: &str| PassageSegment::parse(p).unwrap();
// the best practice would be to have one [`PassageOrganizer`] per book
// but books are beyond the scope of this module
let mut data = PassageOrganizer::<MyContainer>::new();

// modify returns a mutable reference to whatever your container is
// if the container is not initialized, it initializes it 
*data.modify(psg("1:1")) = vec![MyData::Note("Here is a note on 1:1".into())];
data.modify(psg("1:2")).push(MyData::Note("Here is a note on 1:2".into()));
data.modify(psg("1")).push(MyData::Note("Some thoughts on chapter 1".into()));
data.modify(psg("1:2-3")).push(MyData::Tag("#some-tag".into()));
```

#### Get Content with a Specific Proximity

```rust
// get content of a specific proximity
println!("{:#?}", data.get_chapter_verse_content(&psg("1:2")));
```
Prints
```ron
[
    (
        ChapterVerse {
            chapter: 1,
            verse: 2,
        },
        [
            Note(
                "Here is a note on 1:2",
            ),
        ],
    ),
]
```

#### Get All Content (Ordered by Proximity)

```rust
// get all content
println!("{:#?}", data.get_all_content(&psg("1:1-3")).collect_vec());
```
Prints
```ron
[
    (
        ChapterVerse(
            ChapterVerse {
                chapter: 1,
                verse: 1,
            },
        ),
        [
            Note(
                "Here is a note on 1:1",
            ),
        ],
    ),
    (
        ChapterVerse(
            ChapterVerse {
                chapter: 1,
                verse: 2,
            },
        ),
        [
            Note(
                "Here is a note on 1:2",
            ),
        ],
    ),
    (
        ChapterVerseRange(
            ChapterVerseRange {
                chapter: 1,
                verses: RangePair {
                    start: 2,
                    end: 3,
                },
            },
        ),
        [
            Tag(
                "#some-tag",
            ),
        ],
    ),
    (
        FullChapter(
            FullChapter {
                chapter: 1,
            },
        ),
        [
            Note(
                "Some thoughts on chapter 1",
            ),
        ],
    ),
]
```

#### Get All Content (Grouped by Proximity)

```rust
// get all content grouped by proximity
println!("{:#?}", data.get_all_content_grouped(&psg("1:1-3")));
```
Prints
```ron
GroupedContent {
    chapter_verse: [
        (
            ChapterVerse {
                chapter: 1,
                verse: 1,
            },
            [
                Note(
                    "Here is a note on 1:1",
                ),
            ],
        ),
        (
            ChapterVerse {
                chapter: 1,
                verse: 2,
            },
            [
                Note(
                    "Here is a note on 1:2",
                ),
            ],
        ),
    ],
    chapter_verse_range: [
        (
            ChapterVerseRange {
                chapter: 1,
                verses: RangePair {
                    start: 2,
                    end: 3,
                },
            },
            [
                Tag(
                    "#some-tag",
                ),
            ],
        ),
    ],
    chapter_range: [],
    full_chapter: [
        (
            FullChapter {
                chapter: 1,
            },
            [
                Note(
                    "Some thoughts on chapter 1",
                ),
            ],
        ),
    ],
    full_chapter_range: [],
}
```

