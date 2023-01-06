use serde::{Deserialize, Serialize};

/// <https://scryfall.com/docs/api/lists>
#[derive(Serialize, Deserialize, Debug)]
pub struct List<T> {
    /// An array of the requested objects, in a specific order.
    pub data: Vec<T>,
    /// True if this List is paginated and
    /// there is a page beyond the current page.
    pub has_more: bool,
    /// If there is a page beyond the current page,
    /// this field will contain a full API URI to that page.
    /// You may submit a HTTP GET request to that URI to continue paginating
    /// forward on this List.
    pub next_page: Option<String>,
    /// If this is a list of Card objects, this field will contain
    /// the total number of cards found across all pages.
    pub total_cards: Option<u64>,
    /// An array of human-readable warnings issued when generating this list,
    /// as strings.
    /// Warnings are non-fatal issues that the API discovered with your input.
    /// In general, they indicate that the List will not contain
    /// the all of the information you requested.
    /// You should fix the warnings and re-submit your request.
    pub warnings: Option<Vec<String>>,
}

/// <https://scryfall.com/docs/api/bulk-data>
#[derive(Serialize, Deserialize, Debug)]
pub struct Bulk {
    /// A unique ID for this bulk item.
    pub id: String,
    /// The Scryfall API URI for this file.
    pub uri: String,
    /// A computer-readable string for the kind of bulk item.
    #[serde(alias = "type")]
    pub type_: String,
    /// A human-readable name for this file.
    pub name: String,
    /// A human-readable description for this file.
    pub description: String,
    /// The URI that hosts this bulk file for fetching.
    pub download_uri: String,
    /// The time when this file was last updated.
    pub updated_at: String,
    /// The size of this file in integer bytes.
    pub size: u64,
    /// The MIME type of this file.
    pub content_type: String,
    /// The Content-Encoding encoding that will be used to transmit this file
    /// when you download it.
    pub content_encoding: String,
}

/// <https://scryfall.com/docs/api/sets>
#[derive(Serialize, Deserialize, Debug)]
pub struct Set {
    /// A unique ID for this set on Scryfall that will not change.
    pub id: String,
    /// The unique three to five-letter code for this set.
    pub code: String,
    /// The unique code for this set on MTGO,
    /// which may differ from the regular code.
    pub mtgo_code: Option<String>,
    /// This set’s ID on TCGplayer’s API, also known as the groupId.
    pub tcgplayer_id: Option<u64>,
    /// The English name of the set.
    pub name: String,
    /// A computer-readable classification for this set. See below.
    pub set_type: String,
    /// The date the set was released or the first card was printed in the set
    /// (in GMT-8 Pacific time).
    pub released_at: Option<String>,
    /// The block code for this set, if any.
    pub block_code: Option<String>,
    /// The block or group name code for this set, if any.
    pub block: Option<String>,
    /// The set code for the parent set, if any.
    /// promo and token sets often have a parent set.
    pub parent_set_code: Option<String>,
    /// The number of cards in this set.
    pub card_count: u64,
    /// The denominator for the set’s printed collector numbers.
    pub printed_size: Option<u64>,
    /// True if this set was only released in a video game.
    pub digital: bool,
    /// True if this set contains only foil cards.
    pub foil_only: bool,
    /// True if this set contains only nonfoil cards.
    pub nonfoil_only: bool,
    /// A link to this set’s permapage on Scryfall’s website.
    pub scryfall_uri: String,
    /// A link to this set object on Scryfall’s API.
    pub uri: String,
    /// A URI to an SVG file for this set’s icon on Scryfall’s CDN.
    /// Hotlinking this image isn’t recommended,
    /// because it may change slightly over time.
    /// You should download it and use it locally
    /// for your particular user interface needs.
    pub icon_svg_uri: String,
    /// A Scryfall API URI that you can request to begin paginating
    /// over the cards in this set.
    pub search_uri: String,
}
