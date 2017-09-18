#[serde(untagged)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Json {
  AnimeEntry {
    data: Vec<Entry>,
    links: Links,
    included: Vec<Anime>,
  },
  Entry { data: Entry },
  Error { errors: Vec<ApiError> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
  pub title: String,
  pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
  Anime,
  Users,
  Manga,
  LibraryEntries,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anime {
  #[serde(default = "String::new")] pub id: String,
  pub attributes: Option<AnimeAttributes>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimeAttributes {
  pub canonical_title: String,
  pub episode_count: Option<u32>,
  pub status: Option<AnimeStatus>,
  pub subtype: Option<AnimeSubtype>,
  pub titles: AnimeTitles,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnimeSubtype {
  ONA,
  OVA,
  TV,
  Unknown,
  #[serde(rename = "movie")] Movie,
  #[serde(rename = "music")] Music,
  #[serde(rename = "special")] Special,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimeStatus {
  Current,
  Finished,
  Tba,
  Unreleased,
  Upcoming,
  Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeTitles {
  pub ja_jp: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
  id: i32,
  pub attributes: UserAttributes,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAttributes {
  pub name: String,
  pub life_spent_on_anime: i32,
  pub title_language_preference: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
  pub id: String,
  #[serde(rename = "type")] pub kind: Type,
  pub attributes: Option<EntryAttributes>,
  pub relationships: Option<Relationships>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryAttributes {
  #[serde(skip_serializing_if = "Option::is_none")] pub progress: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")] pub status: Option<EntryStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryStatus {
  OnHold,
  Current,
  Dropped,
  Planned,
  Completed,
  Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
  pub count: i32,
  pub status_counts: MetaStatusCounts,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaStatusCounts {
  current: Option<i32>,
  dropped: Option<i32>,
  on_hold: Option<i32>,
  planned: Option<i32>,
  completed: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
  pub prev: Option<String>,
  pub next: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationships {
  pub anime: Option<Anime>,
}
