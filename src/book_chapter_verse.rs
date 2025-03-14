use crate::{book_segment::BookSegment, passage_segments::chapter_verse::ChapterVerse};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BookChapterVerseId {
    book: u8,
    chapter: u8,
    verse: u8,
}

impl Into<BookSegment<ChapterVerse>> for BookChapterVerseId {
    fn into(self) -> BookSegment<ChapterVerse> {
        let BookChapterVerseId { book, chapter, verse } = self;
        BookSegment::chapter_verse(book, chapter, verse)
    }
}

pub const LAST_VERSE: u16 = 31_102;

/// - Genesis has 0 verses before it
/// - Genesis is 1533 verses
/// - Genesis 1 has 31 verses, Genesis 2 has 25 verses, and so on
/// - Exodus has 1533 verses before it
/// - Exodus is 1213 verses
/// - Exodus 1 has 22 verses, Exodus 2 has 25 verses, and so on
pub const VERSE_COUNT_PAIRS: &[(u16, u16, &[u16])] = &[
  (0, 1533, &[31, 25, 24, 26, 32, 22, 24, 22, 29, 32, 32, 20, 18, 24, 21, 16, 27, 33, 38, 18, 34, 24, 20, 67, 34, 35, 46, 22, 35, 43, 55, 32, 20, 31, 29, 43, 36, 30, 23, 23, 57, 38, 34, 34, 28, 34, 31, 22, 33, 26]),
  (1533, 1213, &[22, 25, 22, 31, 23, 30, 25, 32, 35, 29, 10, 51, 22, 31, 27, 36, 16, 27, 25, 26, 36, 31, 33, 18, 40, 37, 21, 43, 46, 38, 18, 35, 23, 35, 35, 38, 29, 31, 43, 38]),
  (2746, 859, &[17, 16, 17, 35, 19, 30, 38, 36, 24, 20, 47, 8, 59, 57, 33, 34, 16, 30, 37, 27, 24, 33, 44, 23, 55, 46, 34]),
  (3605, 1288, &[54, 34, 51, 49, 31, 27, 89, 26, 23, 36, 35, 16, 33, 45, 41, 50, 13, 32, 22, 29, 35, 41, 30, 25, 18, 65, 23, 31, 40, 16, 54, 42, 56, 29, 34, 13]),
  (4893, 959, &[46, 37, 29, 49, 33, 25, 26, 20, 29, 22, 32, 32, 18, 29, 23, 22, 20, 22, 21, 20, 23, 30, 25, 22, 19, 19, 26, 68, 29, 20, 30, 52, 29, 12]),
  (5852, 658, &[18, 24, 17, 24, 15, 27, 26, 35, 27, 43, 23, 24, 33, 15, 63, 10, 18, 28, 51, 9, 45, 34, 16, 33]),
  (6510, 618, &[36, 23, 31, 24, 31, 40, 25, 35, 57, 18, 40, 15, 25, 20, 20, 31, 13, 31, 30, 48, 25]),
  (7128, 85, &[22, 23, 18, 22]),
  (7213, 810, &[28, 36, 21, 22, 12, 21, 17, 22, 27, 27, 15, 25, 23, 52, 35, 23, 58, 30, 24, 42, 15, 23, 29, 22, 44, 25, 12, 25, 11, 31, 13]),
  (8023, 695, &[27, 32, 39, 12, 25, 23, 29, 18, 13, 19, 27, 31, 39, 33, 37, 23, 29, 33, 43, 26, 22, 51, 39, 25]),
  (8718, 816, &[53, 46, 28, 34, 18, 38, 51, 66, 28, 29, 43, 33, 34, 31, 34, 34, 24, 46, 21, 43, 29, 53]),
  (9534, 719, &[18, 25, 27, 44, 27, 33, 20, 29, 37, 36, 21, 21, 25, 29, 38, 20, 41, 37, 37, 21, 26, 20, 37, 20, 30]),
  (10253, 942, &[54, 55, 24, 43, 26, 81, 40, 40, 44, 14, 47, 40, 14, 17, 29, 43, 27, 17, 19, 8, 30, 19, 32, 31, 31, 32, 34, 21, 30]),
  (11195, 822, &[17, 18, 17, 22, 14, 42, 22, 18, 31, 19, 23, 16, 22, 15, 19, 14, 19, 34, 11, 37, 20, 12, 21, 27, 28, 23, 9, 27, 36, 27, 21, 33, 25, 33, 27, 23]),
  (12017, 280, &[11, 70, 13, 24, 17, 22, 28, 36, 15, 44]),
  (12297, 406, &[11, 20, 32, 23, 19, 19, 73, 18, 38, 39, 36, 47, 31]),
  (12703, 167, &[22, 23, 15, 17, 14, 14, 10, 17, 32, 3]),
  (12870, 1070, &[22, 13, 26, 21, 27, 30, 21, 22, 35, 22, 20, 25, 28, 22, 35, 22, 16, 21, 29, 29, 34, 30, 17, 25, 6, 14, 23, 28, 25, 31, 40, 22, 33, 37, 16, 33, 24, 41, 30, 24, 34, 17]),
  (13940, 2461, &[6, 12, 8, 8, 12, 10, 17, 9, 20, 18, 7, 8, 6, 7, 5, 11, 15, 50, 14, 9, 13, 31, 6, 10, 22, 12, 14, 9, 11, 12, 24, 11, 22, 22, 28, 12, 40, 22, 13, 17, 13, 11, 5, 26, 17, 11, 9, 14, 20, 23, 19, 9, 6, 7, 23, 13, 11, 11, 17, 12, 8, 12, 11, 10, 13, 20, 7, 35, 36, 5, 24, 20, 28, 23, 10, 12, 20, 72, 13, 19, 16, 8, 18, 12, 13, 17, 7, 18, 52, 17, 16, 15, 5, 23, 11, 13, 12, 9, 9, 5, 8, 28, 22, 35, 45, 48, 43, 13, 31, 7, 10, 10, 9, 8, 18, 19, 2, 29, 176, 7, 8, 9, 4, 8, 5, 6, 5, 6, 8, 8, 3, 18, 3, 3, 21, 26, 9, 8, 24, 13, 10, 7, 12, 15, 21, 10, 20, 14, 9, 6]),
  (16401, 915, &[33, 22, 35, 27, 23, 35, 27, 36, 18, 32, 31, 28, 25, 35, 33, 33, 28, 24, 29, 30, 31, 29, 35, 34, 28, 28, 27, 28, 27, 33, 31]),
  (17316, 222, &[18, 26, 22, 16, 20, 12, 29, 17, 18, 20, 10, 14]),
  (17538, 117, &[17, 17, 11, 16, 16, 13, 13, 14]),
  (17655, 1292, &[31, 22, 26, 6, 30, 13, 25, 22, 21, 34, 16, 6, 22, 32, 9, 14, 14, 7, 25, 6, 17, 25, 18, 23, 12, 21, 13, 29, 24, 33, 9, 20, 24, 17, 10, 22, 38, 22, 8, 31, 29, 25, 28, 28, 25, 13, 15, 22, 26, 11, 23, 15, 12, 17, 13, 12, 21, 14, 21, 22, 11, 12, 19, 12, 25, 24]),
  (18947, 1364, &[19, 37, 25, 31, 31, 30, 34, 22, 26, 25, 23, 17, 27, 22, 21, 21, 27, 23, 15, 18, 14, 30, 40, 10, 38, 24, 22, 17, 32, 24, 40, 44, 26, 22, 19, 32, 21, 28, 18, 16, 18, 22, 13, 30, 5, 28, 7, 47, 39, 46, 64, 34]),
  (20311, 154, &[22, 22, 66, 22, 22]),
  (20465, 1273, &[28, 10, 27, 17, 17, 14, 27, 18, 11, 22, 25, 28, 23, 23, 8, 63, 24, 32, 14, 49, 32, 31, 49, 27, 17, 21, 36, 26, 21, 26, 18, 32, 33, 31, 15, 38, 28, 23, 29, 49, 26, 20, 27, 31, 25, 24, 23, 35]),
  (21738, 357, &[21, 49, 30, 37, 31, 28, 28, 27, 27, 21, 45, 13]),
  (22095, 197, &[11, 23, 5, 19, 15, 11, 16, 14, 17, 15, 12, 14, 16, 9]),
  (22292, 73, &[20, 32, 21]),
  (22365, 146, &[15, 16, 15, 13, 27, 14, 17, 14, 15]),
  (22511, 21, &[21]),
  (22532, 48, &[17, 10, 10, 11]),
  (22580, 105, &[16, 13, 12, 13, 15, 16, 20]),
  (22685, 47, &[15, 13, 19]),
  (22732, 56, &[17, 20, 19]),
  (22788, 53, &[18, 15, 20]),
  (22841, 38, &[15, 23]),
  (22879, 211, &[21, 13, 10, 14, 11, 15, 14, 23, 17, 12, 17, 14, 9, 21]),
  (23090, 55, &[14, 17, 18, 6]),
  (23145, 1071, &[25, 23, 17, 25, 48, 34, 29, 34, 38, 42, 30, 50, 58, 36, 39, 28, 27, 35, 30, 34, 46, 46, 39, 51, 46, 75, 66, 20]),
  (24216, 678, &[45, 28, 35, 41, 43, 56, 37, 38, 50, 52, 33, 44, 37, 72, 47, 20]),
  (24894, 1151, &[80, 52, 38, 44, 39, 49, 50, 56, 62, 42, 54, 59, 35, 35, 32, 31, 37, 43, 48, 47, 38, 71, 56, 53]),
  (26045, 879, &[51, 25, 36, 54, 47, 71, 53, 59, 41, 42, 57, 50, 38, 31, 27, 33, 26, 40, 42, 31, 25]),
  (26924, 1007, &[26, 47, 26, 37, 42, 15, 60, 40, 43, 48, 30, 25, 52, 28, 41, 40, 34, 28, 41, 38, 40, 30, 35, 27, 27, 32, 44, 31]),
  (27931, 433, &[32, 29, 31, 25, 21, 23, 25, 39, 33, 21, 36, 21, 14, 23, 33, 27]),
  (28364, 437, &[31, 16, 23, 21, 13, 20, 40, 13, 27, 33, 34, 31, 13, 40, 58, 24]),
  (28801, 257, &[24, 17, 18, 18, 21, 18, 16, 24, 15, 18, 33, 21, 14]),
  (29058, 149, &[24, 21, 29, 31, 26, 18]),
  (29207, 155, &[23, 22, 21, 32, 33, 24]),
  (29362, 104, &[30, 30, 21, 23]),
  (29466, 95, &[29, 23, 25, 18]),
  (29561, 89, &[10, 20, 13, 18, 28]),
  (29650, 47, &[12, 17, 18]),
  (29697, 113, &[20, 15, 16, 16, 25, 21]),
  (29810, 83, &[18, 26, 17, 22]),
  (29893, 46, &[16, 15, 15]),
  (29939, 25, &[25]),
  (29964, 303, &[14, 18, 19, 16, 14, 20, 28, 13, 28, 39, 40, 29, 25]),
  (30267, 108, &[27, 26, 18, 17, 20]),
  (30375, 105, &[25, 25, 22, 19, 14]),
  (30480, 61, &[21, 22, 18]),
  (30541, 105, &[10, 29, 24, 21, 21]),
  (30646, 13, &[13]),
  (30659, 14, &[14]),
  (30673, 25, &[25]),
  (30698, 404, &[20, 29, 22, 11, 14, 17, 17, 13, 21, 11, 19, 17, 18, 20, 8, 21, 18, 24, 21, 15, 27, 21])
];

impl BookChapterVerseId {
    pub fn book(&self) -> u8 { self.book }
    pub fn chapter(&self) -> u8 { self.chapter }
    pub fn verse(&self) -> u8 { self.verse }

    /// - This method validates the book/chapter/verse numbers
    pub fn new(book: u8, chapter: u8, verse: u8) -> Result<Self, String> {
        let book_err = || format!("There is no 'Book {}' in the Bible", book);
        let chapter_err = || format!("There is no 'Chapter {}' in 'Book {}'", chapter, book);
        let verse_err = || format!("There is no 'Verse {}' in 'Chapter {}' of 'Book {}'", verse, chapter, book);

        if book == 0 { Err(book_err())? }
        if chapter == 0 { Err(chapter_err())? }
        if verse == 0 { Err(verse_err())? }

        let pair = VERSE_COUNT_PAIRS.get((book - 1) as usize).ok_or_else(book_err)?;
        let chapter_verse_count = pair.2.get((chapter - 1) as usize).ok_or_else(chapter_err)?;
        if verse > (*chapter_verse_count as u8) { Err(verse_err())? }

        Ok(Self {
            book,
            chapter,
            verse,
        })
    }

    /// - This method validates the book/chapter/verse numbers
    pub fn from_verse(mut id: u16) -> Result<Self, String> {
        if id == 0 {
            return Err(format!("There is not 'Book {}' in the Bible", id));
        }
        id -= 1;
        let res = VERSE_COUNT_PAIRS.binary_search_by_key(&id, |(before, _, _)| *before);
        let book = match res {
            Ok(idx) => idx,
            Err(idx) => {
                if idx >= VERSE_COUNT_PAIRS.len() {
                    return Err(format!("There is not 'Book {}' in the Bible", idx));
                }
                idx - 1
            },
        };

        let (before, total, chapters) = VERSE_COUNT_PAIRS[book];
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

        dbg!(book, before, total);
        
        // Genesis is book 1 not 0
        let book = (book + 1) as u8;

        BookChapterVerseId::new(book, chapter, verse)
        // Ok(BookChapterVerseId { book, chapter, verse })
    }

    /// This will crash if BookChapterVerseId does not hold a valid verse
    pub fn as_verse(&self) -> u16 {
        let book = VERSE_COUNT_PAIRS[(self.book - 1) as usize];
        let verses_before_book = book.0;
        let verses_before_chapter: u16 = book.2.iter().take((self.chapter - 1) as usize).sum();
        let verses_before_verse = self.verse as u16;
        verses_before_book + verses_before_chapter + verses_before_verse
    }

    /// - This method indirectly validates the book/chapter/verse numbers
    pub fn from_id_str(input: &str) -> Result<BookChapterVerseId, String> {
        if input.len() != 8 { return Err(format!("Expected length of 8: 2 digits for the book, 3 digits for the chapter, and 3 digits for the verse")); }
        let book = &input[0..=1];
        let book = book.parse().map_err(|_| format!("Could not parse book from '{}'", book))?;
        let chapter = &input[2..=4];
        let chapter = chapter.parse().map_err(|_| format!("Could not parse chapter from '{}'", chapter))?;
        let verse = &input[5..=7];
        let verse = verse.parse().map_err(|_| format!("Could not parse verse from '{}'", verse))?;
        BookChapterVerseId::new(book, chapter, verse)
    }

    pub fn as_id_string(&self) -> String {
        format!(
            "{:0>2}{:0>3}{:0>3}",
            self.book,
            self.chapter,
            self.verse,
        )
    }
}

#[cfg(test)]
mod book_chapter_verse_tests {
    use super::BookChapterVerseId;

    #[test]
    fn new() -> Result<(), String> {
        assert!(BookChapterVerseId::new(0, 1, 1).is_err());
        assert!(BookChapterVerseId::new(1, 0, 1).is_err());
        assert!(BookChapterVerseId::new(1, 1, 0).is_err());
        assert!(BookChapterVerseId::new(1, 1, 1).is_ok());
        assert!(BookChapterVerseId::new(1, 1, 31).is_ok());
        assert!(BookChapterVerseId::new(1, 1, 32).is_err());
        assert!(BookChapterVerseId::new(1, 2, 1).is_ok());
        assert!(BookChapterVerseId::new(1, 2, 25).is_ok());
        assert!(BookChapterVerseId::new(1, 2, 26).is_err());
        assert!(BookChapterVerseId::new(1, 50, 26).is_ok());
        assert!(BookChapterVerseId::new(1, 50, 27).is_err());
        assert!(BookChapterVerseId::new(1, 51, 1).is_err());
        assert!(BookChapterVerseId::new(2, 1, 1).is_ok());
        assert!(BookChapterVerseId::new(2, 1, 22).is_ok());
        assert!(BookChapterVerseId::new(2, 1, 23).is_err());
        assert!(BookChapterVerseId::new(67, 1, 1).is_err());

        Ok(())
    }

    #[test]
    fn from_verse() -> Result<(), String> {
        assert_eq!(
            BookChapterVerseId::from_verse(1)?,
            BookChapterVerseId::new(1, 1, 1)?
        );

        assert_eq!(
            BookChapterVerseId::from_verse(2)?,
            BookChapterVerseId::new(1, 1, 2)?
        );

        assert_eq!(
            BookChapterVerseId::from_verse(23146)?,
            BookChapterVerseId::new(40, 1, 1)?
        );

        assert_eq!(
            BookChapterVerseId::from_verse(23170)?,
            BookChapterVerseId::new(40, 1, 25)?
        );

        assert_eq!(
            BookChapterVerseId::from_verse(23171)?,
            BookChapterVerseId::new(40, 2, 1)?
        );

        Ok(())
    }

    #[test]
    fn as_verse() -> Result<(), String> {
        assert_eq!(
            BookChapterVerseId::new(1, 1, 1)?.as_verse(),
            1,
        );

        assert_eq!(
            BookChapterVerseId::new(1, 1, 2)?.as_verse(),
            2,
        );

        assert_eq!(
            BookChapterVerseId::new(40, 1, 1)?.as_verse(),
            23146,
        );

        assert_eq!(
            BookChapterVerseId::new(40, 1, 25)?.as_verse(),
            23170,
        );

        assert_eq!(
            BookChapterVerseId::new(40, 2, 1)?.as_verse(),
            23171,
        );

        Ok(())
    }

    #[test]
    fn from_id_str() -> Result<(), String> {
        assert_eq!(
            BookChapterVerseId::from_id_str("01001001")?,
            BookChapterVerseId::new(1, 1, 1)?
        );

        assert_eq!(
            BookChapterVerseId::from_id_str("43011035")?,
            BookChapterVerseId::new(43, 11, 35)?
        );

        Ok(())
    }

    #[test]
    fn as_id_str() -> Result<(), String> {
        assert_eq!(
            BookChapterVerseId::new(1, 1, 1 )?.as_id_string(),
            String::from("01001001")
        );

        assert_eq!(
            BookChapterVerseId::new(43, 11, 35)?.as_id_string(),
            String::from("43011035")
        );

        Ok(())
    }
}
