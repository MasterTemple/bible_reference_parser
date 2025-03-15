use std::ops::{Bound, RangeBounds};

use derive_more::{Deref, DerefMut, From};
use itertools::Itertools;

use crate::{book_segment::BookSegment, passage_segments::{chapter_range::ChapterRange, chapter_verse::ChapterVerse}};

// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub struct BookChapterVerse {
//     book: u8,
//     chapter: u8,
//     verse: u8,
// }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[derive(From, Deref, DerefMut)]
pub struct BookChapterVerse(BookSegment<ChapterVerse>);

// should this be TryInto?
// impl Into<BookChapterVerse> for BookSegment<ChapterVerse> {
//     fn into(self) -> BookChapterVerse {
//         let BookSegment { book, segment: ChapterVerse { chapter, verse } } = self;
//         BookChapterVerse{ book, chapter, verse }
//     }
// }
//
// impl Into<BookSegment<ChapterVerse>> for BookChapterVerse {
//     fn into(self) -> BookSegment<ChapterVerse> {
//         let BookChapterVerse { book, chapter, verse } = self;
//         BookSegment::chapter_verse(book, chapter, verse)
//     }
// }

pub const LAST_VERSE: u16 = 31_102;

pub const VERSES_BEFORE_BOOK: [u16; 66] = [0, 1533, 2746, 3605, 4893, 5852, 6510, 7128, 7213, 8023, 8718, 9534, 10253, 11195, 12017, 12297, 12703, 12870, 13940, 16401, 17316, 17538, 17655, 18947, 20311, 20465, 21738, 22095, 22292, 22365, 22511, 22532, 22580, 22685, 22732, 22788, 22841, 22879, 23090, 23145, 24216, 24894, 26045, 26924, 27931, 28364, 28801, 29058, 29207, 29362, 29466, 29561, 29650, 29697, 29810, 29893, 29939, 29964, 30267, 30375, 30480, 30541, 30646, 30659, 30673, 30698];


/// **EVERYTHING IS 0 INDEXED**
/// - `BOOK_CHAPTER_VERSE_COUNT[0][0]` = "Genesis 1"
pub const BOOK_CHAPTER_VERSE_COUNT: &[&[u16]] = &[
&[31, 25, 24, 26, 32, 22, 24, 22, 29, 32, 32, 20, 18, 24, 21, 16, 27, 33, 38, 18, 34, 24, 20, 67, 34, 35, 46, 22, 35, 43, 55, 32, 20, 31, 29, 43, 36, 30, 23, 23, 57, 38, 34, 34, 28, 34, 31, 22, 33, 26],
&[22, 25, 22, 31, 23, 30, 25, 32, 35, 29, 10, 51, 22, 31, 27, 36, 16, 27, 25, 26, 36, 31, 33, 18, 40, 37, 21, 43, 46, 38, 18, 35, 23, 35, 35, 38, 29, 31, 43, 38],
&[17, 16, 17, 35, 19, 30, 38, 36, 24, 20, 47, 8, 59, 57, 33, 34, 16, 30, 37, 27, 24, 33, 44, 23, 55, 46, 34],
&[54, 34, 51, 49, 31, 27, 89, 26, 23, 36, 35, 16, 33, 45, 41, 50, 13, 32, 22, 29, 35, 41, 30, 25, 18, 65, 23, 31, 40, 16, 54, 42, 56, 29, 34, 13],
&[46, 37, 29, 49, 33, 25, 26, 20, 29, 22, 32, 32, 18, 29, 23, 22, 20, 22, 21, 20, 23, 30, 25, 22, 19, 19, 26, 68, 29, 20, 30, 52, 29, 12],
&[18, 24, 17, 24, 15, 27, 26, 35, 27, 43, 23, 24, 33, 15, 63, 10, 18, 28, 51, 9, 45, 34, 16, 33],
&[36, 23, 31, 24, 31, 40, 25, 35, 57, 18, 40, 15, 25, 20, 20, 31, 13, 31, 30, 48, 25],
&[22, 23, 18, 22],
&[28, 36, 21, 22, 12, 21, 17, 22, 27, 27, 15, 25, 23, 52, 35, 23, 58, 30, 24, 42, 15, 23, 29, 22, 44, 25, 12, 25, 11, 31, 13],
&[27, 32, 39, 12, 25, 23, 29, 18, 13, 19, 27, 31, 39, 33, 37, 23, 29, 33, 43, 26, 22, 51, 39, 25],
&[53, 46, 28, 34, 18, 38, 51, 66, 28, 29, 43, 33, 34, 31, 34, 34, 24, 46, 21, 43, 29, 53],
&[18, 25, 27, 44, 27, 33, 20, 29, 37, 36, 21, 21, 25, 29, 38, 20, 41, 37, 37, 21, 26, 20, 37, 20, 30],
&[54, 55, 24, 43, 26, 81, 40, 40, 44, 14, 47, 40, 14, 17, 29, 43, 27, 17, 19, 8, 30, 19, 32, 31, 31, 32, 34, 21, 30],
&[17, 18, 17, 22, 14, 42, 22, 18, 31, 19, 23, 16, 22, 15, 19, 14, 19, 34, 11, 37, 20, 12, 21, 27, 28, 23, 9, 27, 36, 27, 21, 33, 25, 33, 27, 23],
&[11, 70, 13, 24, 17, 22, 28, 36, 15, 44],
&[11, 20, 32, 23, 19, 19, 73, 18, 38, 39, 36, 47, 31],
&[22, 23, 15, 17, 14, 14, 10, 17, 32, 3],
&[22, 13, 26, 21, 27, 30, 21, 22, 35, 22, 20, 25, 28, 22, 35, 22, 16, 21, 29, 29, 34, 30, 17, 25, 6, 14, 23, 28, 25, 31, 40, 22, 33, 37, 16, 33, 24, 41, 30, 24, 34, 17],
&[6, 12, 8, 8, 12, 10, 17, 9, 20, 18, 7, 8, 6, 7, 5, 11, 15, 50, 14, 9, 13, 31, 6, 10, 22, 12, 14, 9, 11, 12, 24, 11, 22, 22, 28, 12, 40, 22, 13, 17, 13, 11, 5, 26, 17, 11, 9, 14, 20, 23, 19, 9, 6, 7, 23, 13, 11, 11, 17, 12, 8, 12, 11, 10, 13, 20, 7, 35, 36, 5, 24, 20, 28, 23, 10, 12, 20, 72, 13, 19, 16, 8, 18, 12, 13, 17, 7, 18, 52, 17, 16, 15, 5, 23, 11, 13, 12, 9, 9, 5, 8, 28, 22, 35, 45, 48, 43, 13, 31, 7, 10, 10, 9, 8, 18, 19, 2, 29, 176, 7, 8, 9, 4, 8, 5, 6, 5, 6, 8, 8, 3, 18, 3, 3, 21, 26, 9, 8, 24, 13, 10, 7, 12, 15, 21, 10, 20, 14, 9, 6],
&[33, 22, 35, 27, 23, 35, 27, 36, 18, 32, 31, 28, 25, 35, 33, 33, 28, 24, 29, 30, 31, 29, 35, 34, 28, 28, 27, 28, 27, 33, 31],
&[18, 26, 22, 16, 20, 12, 29, 17, 18, 20, 10, 14],
&[17, 17, 11, 16, 16, 13, 13, 14],
&[31, 22, 26, 6, 30, 13, 25, 22, 21, 34, 16, 6, 22, 32, 9, 14, 14, 7, 25, 6, 17, 25, 18, 23, 12, 21, 13, 29, 24, 33, 9, 20, 24, 17, 10, 22, 38, 22, 8, 31, 29, 25, 28, 28, 25, 13, 15, 22, 26, 11, 23, 15, 12, 17, 13, 12, 21, 14, 21, 22, 11, 12, 19, 12, 25, 24],
&[19, 37, 25, 31, 31, 30, 34, 22, 26, 25, 23, 17, 27, 22, 21, 21, 27, 23, 15, 18, 14, 30, 40, 10, 38, 24, 22, 17, 32, 24, 40, 44, 26, 22, 19, 32, 21, 28, 18, 16, 18, 22, 13, 30, 5, 28, 7, 47, 39, 46, 64, 34],
&[22, 22, 66, 22, 22],
&[28, 10, 27, 17, 17, 14, 27, 18, 11, 22, 25, 28, 23, 23, 8, 63, 24, 32, 14, 49, 32, 31, 49, 27, 17, 21, 36, 26, 21, 26, 18, 32, 33, 31, 15, 38, 28, 23, 29, 49, 26, 20, 27, 31, 25, 24, 23, 35],
&[21, 49, 30, 37, 31, 28, 28, 27, 27, 21, 45, 13],
&[11, 23, 5, 19, 15, 11, 16, 14, 17, 15, 12, 14, 16, 9],
&[20, 32, 21],
&[15, 16, 15, 13, 27, 14, 17, 14, 15],
&[21],
&[17, 10, 10, 11],
&[16, 13, 12, 13, 15, 16, 20],
&[15, 13, 19],
&[17, 20, 19],
&[18, 15, 20],
&[15, 23],
&[21, 13, 10, 14, 11, 15, 14, 23, 17, 12, 17, 14, 9, 21],
&[14, 17, 18, 6],
&[25, 23, 17, 25, 48, 34, 29, 34, 38, 42, 30, 50, 58, 36, 39, 28, 27, 35, 30, 34, 46, 46, 39, 51, 46, 75, 66, 20],
&[45, 28, 35, 41, 43, 56, 37, 38, 50, 52, 33, 44, 37, 72, 47, 20],
&[80, 52, 38, 44, 39, 49, 50, 56, 62, 42, 54, 59, 35, 35, 32, 31, 37, 43, 48, 47, 38, 71, 56, 53],
&[51, 25, 36, 54, 47, 71, 53, 59, 41, 42, 57, 50, 38, 31, 27, 33, 26, 40, 42, 31, 25],
&[26, 47, 26, 37, 42, 15, 60, 40, 43, 48, 30, 25, 52, 28, 41, 40, 34, 28, 41, 38, 40, 30, 35, 27, 27, 32, 44, 31],
&[32, 29, 31, 25, 21, 23, 25, 39, 33, 21, 36, 21, 14, 23, 33, 27],
&[31, 16, 23, 21, 13, 20, 40, 13, 27, 33, 34, 31, 13, 40, 58, 24],
&[24, 17, 18, 18, 21, 18, 16, 24, 15, 18, 33, 21, 14],
&[24, 21, 29, 31, 26, 18],
&[23, 22, 21, 32, 33, 24],
&[30, 30, 21, 23],
&[29, 23, 25, 18],
&[10, 20, 13, 18, 28],
&[12, 17, 18],
&[20, 15, 16, 16, 25, 21],
&[18, 26, 17, 22],
&[16, 15, 15],
&[25],
&[14, 18, 19, 16, 14, 20, 28, 13, 28, 39, 40, 29, 25],
&[27, 26, 18, 17, 20],
&[25, 25, 22, 19, 14],
&[21, 22, 18],
&[10, 29, 24, 21, 21],
&[13],
&[14],
&[25],
&[20, 29, 22, 11, 14, 17, 17, 13, 21, 11, 19, 17, 18, 20, 8, 21, 18, 24, 21, 15, 27, 21]
];

/// not gonna lie, this data structure is terrible
/// also so is this separate BookChapterVerseId data structure
///
/// - Genesis has 0 verses before it
/// - Genesis is 1533 verses
/// - Genesis 1 has 31 verses, Genesis 2 has 25 verses, and so on
/// - Exodus has 1533 verses before it
/// - Exodus is 1213 verses
/// - Exodus 1 has 22 verses, Exodus 2 has 25 verses, and so on
pub const VERSE_COUNT_PAIRS: &[(u16, &[u16])] = &[
  (0, &[31, 25, 24, 26, 32, 22, 24, 22, 29, 32, 32, 20, 18, 24, 21, 16, 27, 33, 38, 18, 34, 24, 20, 67, 34, 35, 46, 22, 35, 43, 55, 32, 20, 31, 29, 43, 36, 30, 23, 23, 57, 38, 34, 34, 28, 34, 31, 22, 33, 26]),
  (1533, &[22, 25, 22, 31, 23, 30, 25, 32, 35, 29, 10, 51, 22, 31, 27, 36, 16, 27, 25, 26, 36, 31, 33, 18, 40, 37, 21, 43, 46, 38, 18, 35, 23, 35, 35, 38, 29, 31, 43, 38]),
  (2746, &[17, 16, 17, 35, 19, 30, 38, 36, 24, 20, 47, 8, 59, 57, 33, 34, 16, 30, 37, 27, 24, 33, 44, 23, 55, 46, 34]),
  (3605, &[54, 34, 51, 49, 31, 27, 89, 26, 23, 36, 35, 16, 33, 45, 41, 50, 13, 32, 22, 29, 35, 41, 30, 25, 18, 65, 23, 31, 40, 16, 54, 42, 56, 29, 34, 13]),
  (4893, &[46, 37, 29, 49, 33, 25, 26, 20, 29, 22, 32, 32, 18, 29, 23, 22, 20, 22, 21, 20, 23, 30, 25, 22, 19, 19, 26, 68, 29, 20, 30, 52, 29, 12]),
  (5852, &[18, 24, 17, 24, 15, 27, 26, 35, 27, 43, 23, 24, 33, 15, 63, 10, 18, 28, 51, 9, 45, 34, 16, 33]),
  (6510, &[36, 23, 31, 24, 31, 40, 25, 35, 57, 18, 40, 15, 25, 20, 20, 31, 13, 31, 30, 48, 25]),
  (7128, &[22, 23, 18, 22]),
  (7213, &[28, 36, 21, 22, 12, 21, 17, 22, 27, 27, 15, 25, 23, 52, 35, 23, 58, 30, 24, 42, 15, 23, 29, 22, 44, 25, 12, 25, 11, 31, 13]),
  (8023, &[27, 32, 39, 12, 25, 23, 29, 18, 13, 19, 27, 31, 39, 33, 37, 23, 29, 33, 43, 26, 22, 51, 39, 25]),
  (8718, &[53, 46, 28, 34, 18, 38, 51, 66, 28, 29, 43, 33, 34, 31, 34, 34, 24, 46, 21, 43, 29, 53]),
  (9534, &[18, 25, 27, 44, 27, 33, 20, 29, 37, 36, 21, 21, 25, 29, 38, 20, 41, 37, 37, 21, 26, 20, 37, 20, 30]),
  (10253, &[54, 55, 24, 43, 26, 81, 40, 40, 44, 14, 47, 40, 14, 17, 29, 43, 27, 17, 19, 8, 30, 19, 32, 31, 31, 32, 34, 21, 30]),
  (11195, &[17, 18, 17, 22, 14, 42, 22, 18, 31, 19, 23, 16, 22, 15, 19, 14, 19, 34, 11, 37, 20, 12, 21, 27, 28, 23, 9, 27, 36, 27, 21, 33, 25, 33, 27, 23]),
  (12017, &[11, 70, 13, 24, 17, 22, 28, 36, 15, 44]),
  (12297, &[11, 20, 32, 23, 19, 19, 73, 18, 38, 39, 36, 47, 31]),
  (12703, &[22, 23, 15, 17, 14, 14, 10, 17, 32, 3]),
  (12870, &[22, 13, 26, 21, 27, 30, 21, 22, 35, 22, 20, 25, 28, 22, 35, 22, 16, 21, 29, 29, 34, 30, 17, 25, 6, 14, 23, 28, 25, 31, 40, 22, 33, 37, 16, 33, 24, 41, 30, 24, 34, 17]),
  (13940, &[6, 12, 8, 8, 12, 10, 17, 9, 20, 18, 7, 8, 6, 7, 5, 11, 15, 50, 14, 9, 13, 31, 6, 10, 22, 12, 14, 9, 11, 12, 24, 11, 22, 22, 28, 12, 40, 22, 13, 17, 13, 11, 5, 26, 17, 11, 9, 14, 20, 23, 19, 9, 6, 7, 23, 13, 11, 11, 17, 12, 8, 12, 11, 10, 13, 20, 7, 35, 36, 5, 24, 20, 28, 23, 10, 12, 20, 72, 13, 19, 16, 8, 18, 12, 13, 17, 7, 18, 52, 17, 16, 15, 5, 23, 11, 13, 12, 9, 9, 5, 8, 28, 22, 35, 45, 48, 43, 13, 31, 7, 10, 10, 9, 8, 18, 19, 2, 29, 176, 7, 8, 9, 4, 8, 5, 6, 5, 6, 8, 8, 3, 18, 3, 3, 21, 26, 9, 8, 24, 13, 10, 7, 12, 15, 21, 10, 20, 14, 9, 6]),
  (16401, &[33, 22, 35, 27, 23, 35, 27, 36, 18, 32, 31, 28, 25, 35, 33, 33, 28, 24, 29, 30, 31, 29, 35, 34, 28, 28, 27, 28, 27, 33, 31]),
  (17316, &[18, 26, 22, 16, 20, 12, 29, 17, 18, 20, 10, 14]),
  (17538, &[17, 17, 11, 16, 16, 13, 13, 14]),
  (17655, &[31, 22, 26, 6, 30, 13, 25, 22, 21, 34, 16, 6, 22, 32, 9, 14, 14, 7, 25, 6, 17, 25, 18, 23, 12, 21, 13, 29, 24, 33, 9, 20, 24, 17, 10, 22, 38, 22, 8, 31, 29, 25, 28, 28, 25, 13, 15, 22, 26, 11, 23, 15, 12, 17, 13, 12, 21, 14, 21, 22, 11, 12, 19, 12, 25, 24]),
  (18947, &[19, 37, 25, 31, 31, 30, 34, 22, 26, 25, 23, 17, 27, 22, 21, 21, 27, 23, 15, 18, 14, 30, 40, 10, 38, 24, 22, 17, 32, 24, 40, 44, 26, 22, 19, 32, 21, 28, 18, 16, 18, 22, 13, 30, 5, 28, 7, 47, 39, 46, 64, 34]),
  (20311, &[22, 22, 66, 22, 22]),
  (20465, &[28, 10, 27, 17, 17, 14, 27, 18, 11, 22, 25, 28, 23, 23, 8, 63, 24, 32, 14, 49, 32, 31, 49, 27, 17, 21, 36, 26, 21, 26, 18, 32, 33, 31, 15, 38, 28, 23, 29, 49, 26, 20, 27, 31, 25, 24, 23, 35]),
  (21738, &[21, 49, 30, 37, 31, 28, 28, 27, 27, 21, 45, 13]),
  (22095, &[11, 23, 5, 19, 15, 11, 16, 14, 17, 15, 12, 14, 16, 9]),
  (22292, &[20, 32, 21]),
  (22365, &[15, 16, 15, 13, 27, 14, 17, 14, 15]),
  (22511, &[21]),
  (22532, &[17, 10, 10, 11]),
  (22580, &[16, 13, 12, 13, 15, 16, 20]),
  (22685, &[15, 13, 19]),
  (22732, &[17, 20, 19]),
  (22788, &[18, 15, 20]),
  (22841, &[15, 23]),
  (22879, &[21, 13, 10, 14, 11, 15, 14, 23, 17, 12, 17, 14, 9, 21]),
  (23090, &[14, 17, 18, 6]),
  (23145, &[25, 23, 17, 25, 48, 34, 29, 34, 38, 42, 30, 50, 58, 36, 39, 28, 27, 35, 30, 34, 46, 46, 39, 51, 46, 75, 66, 20]),
  (24216, &[45, 28, 35, 41, 43, 56, 37, 38, 50, 52, 33, 44, 37, 72, 47, 20]),
  (24894, &[80, 52, 38, 44, 39, 49, 50, 56, 62, 42, 54, 59, 35, 35, 32, 31, 37, 43, 48, 47, 38, 71, 56, 53]),
  (26045, &[51, 25, 36, 54, 47, 71, 53, 59, 41, 42, 57, 50, 38, 31, 27, 33, 26, 40, 42, 31, 25]),
  (26924, &[26, 47, 26, 37, 42, 15, 60, 40, 43, 48, 30, 25, 52, 28, 41, 40, 34, 28, 41, 38, 40, 30, 35, 27, 27, 32, 44, 31]),
  (27931, &[32, 29, 31, 25, 21, 23, 25, 39, 33, 21, 36, 21, 14, 23, 33, 27]),
  (28364, &[31, 16, 23, 21, 13, 20, 40, 13, 27, 33, 34, 31, 13, 40, 58, 24]),
  (28801, &[24, 17, 18, 18, 21, 18, 16, 24, 15, 18, 33, 21, 14]),
  (29058, &[24, 21, 29, 31, 26, 18]),
  (29207, &[23, 22, 21, 32, 33, 24]),
  (29362, &[30, 30, 21, 23]),
  (29466, &[29, 23, 25, 18]),
  (29561, &[10, 20, 13, 18, 28]),
  (29650, &[12, 17, 18]),
  (29697, &[20, 15, 16, 16, 25, 21]),
  (29810, &[18, 26, 17, 22]),
  (29893, &[16, 15, 15]),
  (29939, &[25]),
  (29964, &[14, 18, 19, 16, 14, 20, 28, 13, 28, 39, 40, 29, 25]),
  (30267, &[27, 26, 18, 17, 20]),
  (30375, &[25, 25, 22, 19, 14]),
  (30480, &[21, 22, 18]),
  (30541, &[10, 29, 24, 21, 21]),
  (30646, &[13]),
  (30659, &[14]),
  (30673, &[25]),
  (30698, &[20, 29, 22, 11, 14, 17, 17, 13, 21, 11, 19, 17, 18, 20, 8, 21, 18, 24, 21, 15, 27, 21])
];

impl BookChapterVerse {

    fn book_idx(&self) -> usize { (self.book - 1) as usize }
    fn chapter_idx(&self) -> usize { (self.segment.chapter - 1) as usize }
    // fn verse_idx(&self) -> usize { (self.segment.verse - 1) as usize }

    pub fn book(&self) -> u8 { self.book }
    pub fn chapter(&self) -> u8 { self.segment.chapter }
    pub fn verse(&self) -> u8 { self.segment.verse }

    /// - This method validates the book/chapter/verse numbers
    pub fn new(book: u8, chapter: u8, verse: u8) -> Result<Self, String> {
        let book_err = || format!("There is no 'Book {}' in the Bible", book);
        let chapter_err = || format!("There is no 'Chapter {}' in 'Book {}'", chapter, book);
        let verse_err = || format!("There is no 'Verse {}' in 'Chapter {}' of 'Book {}'", verse, chapter, book);

        if book == 0 { Err(book_err())? }
        if chapter == 0 { Err(chapter_err())? }
        if verse == 0 { Err(verse_err())? }

        let pair = VERSE_COUNT_PAIRS.get((book - 1) as usize).ok_or_else(book_err)?;
        let chapter_verse_count = pair.1.get((chapter - 1) as usize).ok_or_else(chapter_err)?;
        if verse > (*chapter_verse_count as u8) { Err(verse_err())? }

        Ok(Self(BookSegment::chapter_verse(book, chapter, verse)))
    }

    /// - This method validates the book/chapter/verse numbers
    pub fn from_verse(mut id: u16) -> Result<Self, String> {
        if id == 0 {
            return Err(format!("There is not 'Book {}' in the Bible", id));
        }
        id -= 1;
        let res = VERSE_COUNT_PAIRS.binary_search_by_key(&id, |(before, _)| *before);
        let book = match res {
            Ok(idx) => idx,
            Err(idx) => {
                if idx >= VERSE_COUNT_PAIRS.len() {
                    return Err(format!("There is not 'Book {}' in the Bible", idx));
                }
                idx - 1
            },
        };

        let (before, chapters) = VERSE_COUNT_PAIRS[book];
        let mut remaining = id - before;

        let mut chapter = 1;
        let mut iter = chapters.iter();
        while let Some(&verses_in_chapter) = iter.next() {
            if verses_in_chapter > remaining {
                break;
            }
            remaining -= verses_in_chapter;
            chapter += 1;
        }

        // verses start at 1
        let verse = (remaining + 1) as u8;

        dbg!(book, before);
        
        // Genesis is book 1 not 0
        let book = (book + 1) as u8;

        BookChapterVerse::new(book, chapter, verse)
        // Ok(BookChapterVerseId { book, chapter, verse })
    }

    /// This will crash if BookChapterVerseId does not hold a valid verse
    pub fn as_verse(&self) -> u16 {
        let book = VERSE_COUNT_PAIRS[(self.book - 1) as usize];
        let verses_before_book = book.0;
        let verses_before_chapter: u16 = book.1.iter().take((self.chapter() - 1) as usize).sum();
        let verses_before_verse = self.verse() as u16;
        verses_before_book + verses_before_chapter + verses_before_verse
    }

    /// - This method indirectly validates the book/chapter/verse numbers
    pub fn from_id_string(input: &str) -> Result<BookChapterVerse, String> {
        if input.len() != 8 { return Err(format!("Expected length of 8: 2 digits for the book, 3 digits for the chapter, and 3 digits for the verse")); }
        let book = &input[0..=1];
        let book = book.parse().map_err(|_| format!("Could not parse book from '{}'", book))?;
        let chapter = &input[2..=4];
        let chapter = chapter.parse().map_err(|_| format!("Could not parse chapter from '{}'", chapter))?;
        let verse = &input[5..=7];
        let verse = verse.parse().map_err(|_| format!("Could not parse verse from '{}'", verse))?;
        BookChapterVerse::new(book, chapter, verse)
    }

    pub fn as_id_string(&self) -> String {
        format!(
            "{:0>2}{:0>3}{:0>3}",
            self.book,
            self.chapter(),
            self.verse(),
        )
    }

    pub fn remaining_verses(&self) -> Option<std::ops::RangeInclusive<u8>>  {
        let chapter_verses = BOOK_CHAPTER_VERSE_COUNT.get(self.book_idx())?;
        let verse_count = chapter_verses.get(self.chapter_idx())?;
        let start = self.verse() + 1;
        let end = *verse_count as u8;
        (start <= end).then(|| start..=end)
    }

    pub fn iter_remaining_verses(&self) -> impl Iterator<Item = u8>  {
        match self.remaining_verses() {
            Some(range) => range.skip(0),
            None => (0..=0).skip(1),
        }
    }

    pub fn remaining_chapters(&self) -> Option<std::ops::RangeInclusive<u8>> {
        let chapter_count = BOOK_CHAPTER_VERSE_COUNT.get(self.book_idx())?.len();
        let start = self.chapter() + 1;
        let end = chapter_count as u8;
        (start <= end).then(|| start..=end)
    }

    pub fn iter_remaining_chapters(&self) -> impl Iterator<Item = u8>  {
        match self.remaining_chapters() {
            Some(range) => range.skip(0),
            None => (0..=0).skip(1),
        }
    }
}

#[cfg(test)]
mod book_chapter_verse_tests {
    use std::collections::BTreeMap;

    use itertools::Itertools;

    use super::BookChapterVerse;

    #[test]
    fn new() -> Result<(), String> {
        assert!(BookChapterVerse::new(0, 1, 1).is_err());
        assert!(BookChapterVerse::new(1, 0, 1).is_err());
        assert!(BookChapterVerse::new(1, 1, 0).is_err());
        assert!(BookChapterVerse::new(1, 1, 1).is_ok());
        assert!(BookChapterVerse::new(1, 1, 31).is_ok());
        assert!(BookChapterVerse::new(1, 1, 32).is_err());
        assert!(BookChapterVerse::new(1, 2, 1).is_ok());
        assert!(BookChapterVerse::new(1, 2, 25).is_ok());
        assert!(BookChapterVerse::new(1, 2, 26).is_err());
        assert!(BookChapterVerse::new(1, 50, 26).is_ok());
        assert!(BookChapterVerse::new(1, 50, 27).is_err());
        assert!(BookChapterVerse::new(1, 51, 1).is_err());
        assert!(BookChapterVerse::new(2, 1, 1).is_ok());
        assert!(BookChapterVerse::new(2, 1, 22).is_ok());
        assert!(BookChapterVerse::new(2, 1, 23).is_err());
        assert!(BookChapterVerse::new(67, 1, 1).is_err());

        Ok(())
    }

    #[test]
    fn from_verse() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::from_verse(1)?,
            BookChapterVerse::new(1, 1, 1)?
        );

        assert_eq!(
            BookChapterVerse::from_verse(2)?,
            BookChapterVerse::new(1, 1, 2)?
        );

        assert_eq!(
            BookChapterVerse::from_verse(23146)?,
            BookChapterVerse::new(40, 1, 1)?
        );

        assert_eq!(
            BookChapterVerse::from_verse(23170)?,
            BookChapterVerse::new(40, 1, 25)?
        );

        assert_eq!(
            BookChapterVerse::from_verse(23171)?,
            BookChapterVerse::new(40, 2, 1)?
        );

        Ok(())
    }

    #[test]
    fn as_verse() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::new(1, 1, 1)?.as_verse(),
            1,
        );

        assert_eq!(
            BookChapterVerse::new(1, 1, 2)?.as_verse(),
            2,
        );

        assert_eq!(
            BookChapterVerse::new(40, 1, 1)?.as_verse(),
            23146,
        );

        assert_eq!(
            BookChapterVerse::new(40, 1, 25)?.as_verse(),
            23170,
        );

        assert_eq!(
            BookChapterVerse::new(40, 2, 1)?.as_verse(),
            23171,
        );

        Ok(())
    }

    #[test]
    fn from_id_str() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::from_id_string("01001001")?,
            BookChapterVerse::new(1, 1, 1)?
        );

        assert_eq!(
            BookChapterVerse::from_id_string("43011035")?,
            BookChapterVerse::new(43, 11, 35)?
        );

        Ok(())
    }

    #[test]
    fn as_id_str() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::new(1, 1, 1 )?.as_id_string(),
            String::from("01001001")
        );

        assert_eq!(
            BookChapterVerse::new(43, 11, 35)?.as_id_string(),
            String::from("43011035")
        );

        Ok(())
    }

    #[test]
    fn remaining_verses() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::new(1, 1, 1)?.remaining_verses(),
            Some(2..=31)
        );

        assert_eq!(
            BookChapterVerse::new(1, 1, 30)?.remaining_verses(),
            Some(31..=31)
        );

        assert_eq!(
            BookChapterVerse::new(1, 1, 31)?.remaining_verses(),
            None
        );

        Ok(())
    }

    #[test]
    fn iter_remaining_verses() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::new(1, 1, 1)?.iter_remaining_verses().collect_vec(),
            (2..=31).collect_vec()
        );

        assert_eq!(
            BookChapterVerse::new(1, 1, 30)?.iter_remaining_verses().collect_vec(),
            (31..=31).collect_vec()
        );

        assert_eq!(
            BookChapterVerse::new(1, 1, 31)?.iter_remaining_verses().collect_vec(),
            Vec::<u8>::new()
        );

        Ok(())
    }

    #[test]
    fn remaining_chapters() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::new(1, 1, 1)?.remaining_chapters(),
            Some(2..=50)
        );

        assert_eq!(
            BookChapterVerse::new(1, 49, 1)?.remaining_chapters(),
            Some(50..=50)
        );

        assert_eq!(
            BookChapterVerse::new(1, 50, 1)?.remaining_chapters(),
            None
        );

        Ok(())
    }

    #[test]
    fn iter_remaining_chapters() -> Result<(), String> {
        assert_eq!(
            BookChapterVerse::new(1, 1, 1)?.iter_remaining_chapters().collect_vec(),
            (2..=50).collect_vec()
        );

        assert_eq!(
            BookChapterVerse::new(1, 49, 1)?.iter_remaining_chapters().collect_vec(),
            (50..=50).collect_vec()
        );

        assert_eq!(
            BookChapterVerse::new(1, 50, 1)?.iter_remaining_chapters().collect_vec(),
            Vec::<u8>::new()
        );

        Ok(())
    }
}
