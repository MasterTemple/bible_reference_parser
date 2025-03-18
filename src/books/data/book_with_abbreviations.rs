use derive_more::{Deref, DerefMut, IntoIterator};
use serde::{Deserialize, Serialize};

static BOOKS_WITH_ABBREVIATIONS_JSON: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/books/data/books_with_abbreviations.json"));

/**
Example:
```jsonc
[
  {
    "id": 1,
    "book": "Genesis",
    "abbreviation": "Gn",
    "abbreviations": [
      "gen",
      "ge",
      "gn"
    ]
  },
  // ...
]
```
*/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookWithAbbreviations {
    /// - book id, starting at 1
    /// - Genesis = 1
    /// - Matthew = 40
    #[serde(alias = "num")]
    #[serde(alias = "number")]
    pub id: u8,

    /// - the display name
    /// - case is kept
    /// - does not need to be repeated in abbreviations
    #[serde(alias = "book")]
    #[serde(alias = "book_name")]
    #[serde(alias = "display_name")]
    pub name: String,

    /// - the display abbreviation
    /// - case is kept
    /// - does not need to be repeated in abbreviations
    /// - TODO: if not provided, the first abbreviations
    #[serde(alias = "abbr")]
    #[serde(alias = "abbrv")]
    #[serde(alias = "abbrev")]
    pub abbreviation: String,

    /// - does not need to be repeated in abbreviations
    /// - meant for matching/parsing references
    #[serde(alias = "abbrs")]
    #[serde(alias = "abbrvs")]
    #[serde(alias = "abbrevs")]
    #[serde(default)]
    pub abbreviations: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(Deref, DerefMut, IntoIterator)]
pub struct BookWithAbbreviationsList(pub Vec<BookWithAbbreviations>);

impl Default for BookWithAbbreviationsList {
    fn default() -> Self {
        serde_json::from_str(&BOOKS_WITH_ABBREVIATIONS_JSON)
            .map_err(|_| format!("Could not parse default file")).unwrap()
    }
}
