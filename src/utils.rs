use std::borrow::Cow;
use std::str::FromStr;

use url::Url;

use types::{MsgCommand, QueryCommand};
use types::kitsu::*;
use types::telegram::*;

named!(pub parse_message<&str, MsgCommand>,
  alt!(
    map!(tag!("/list"), |_| MsgCommand::List) |
    map!(tag!("/update"), |_| MsgCommand::Update)
  )
);

named!(pub parse_query<&str, QueryCommand>,
  do_parse!(
    tag!("/") >>
    kitsu_id: map_res!(take_until!("/"), i64::from_str)  >>
    command: alt!(
      do_parse!(
        tag!("/offset/") >>
        offset: map_res!(take_until!("/"), i64::from_str)  >>
        (QueryCommand::Offset{ kitsu_id, offset })
      ) |
      do_parse!(
        tag!("/detail/") >>
        anime_id: map_res!(take_until!("/"), i64::from_str)  >>
        (QueryCommand::Detail{ kitsu_id, anime_id })
      )
    ) >>
    (command)
  )
);

pub fn get_offset(url: Option<String>) -> Option<String> {
  url.map_or(None, |x| match Url::parse(&x) {
    Ok(url) => url
      .query_pairs()
      .find(|&(ref key, _)| key == &Cow::Borrowed("page[offset]"))
      .map_or(None, |(_, offset)| Some(offset.to_string())),
    _ => None,
  })
}

pub fn parse_anime_detail(
  kitsu_id: i64,
  pair: Option<(Entries, Anime)>,
) -> (String, Vec<Vec<InlineKeyboardButton>>) {
  let navigate = vec![
    InlineKeyboardButton::with_callback_data(
      String::from("Back to List"),
      format!("/{}/offset/0/", kitsu_id),
    ),
  ];
  let text = match pair {
    None => format!("Error: No Anime Found :("),
    Some((entry, anime)) => {
      let anime_attr = anime.attributes;
      let entry_attr = entry.attributes;
      format!(
        "<b>Title</b>: {}\n\
        <b>JapaneseTitle</b>: {}\n\
        <b>Subtype</b>: {:?}\n\
        <b>Status</b>: {:?}\n\
        <b>Progress</b>: {:?} [{}/{}]",
        anime_attr.canonical_title,
        anime_attr.titles.ja_jp.unwrap_or(String::from("null")),
        anime_attr.subtype.unwrap_or(AnimeSubtype::Unknown),
        anime_attr.status.unwrap_or(AnimeStatus::Unknown),
        entry_attr.status,
        entry_attr.progress,
        anime_attr.episode_count.unwrap_or(99),
      )
    },
  };
  (text, vec![navigate])
}

pub fn parse_anime_list(
  kitsu_id: i64,
  prev: Option<String>,
  next: Option<String>,
  pairs: Option<(Vec<Entries>, Vec<Anime>)>,
) -> (String, Vec<Vec<InlineKeyboardButton>>) {
  let mut index = vec![];
  let mut navigate = vec![];
  if let Some(offset) = get_offset(prev) {
    navigate.push(InlineKeyboardButton::with_callback_data(
      String::from("Prev"),
      format!("/{}/offset/{}/", kitsu_id, offset),
    ))
  }
  if let Some(offset) = get_offset(next) {
    navigate.push(InlineKeyboardButton::with_callback_data(
      String::from("Next"),
      format!("/{}/offset/{}/", kitsu_id, offset),
    ))
  }
  let mut text = String::new();
  match pairs {
    None => text = format!("No Anime :("),
    Some((entries, animes)) => {
      for (i, (entry, anime)) in entries.iter().zip(animes.iter()).enumerate() {
        let anime_attr: &AnimeAttributes = &anime.attributes;
        let entry_attr = &entry.attributes;
        text.push_str(&format!(
          "<b>{}|</b> {}\n\
           {:indent$}{}\n\
           {:indent$}<b>{:?} [{}/{}]</b>\n",
          i, anime_attr.canonical_title,
          "", match anime_attr.titles.ja_jp {
            Some(ref title) => title,
            None => "null"
          },
          "", entry_attr.status, entry_attr.progress, anime_attr.episode_count.unwrap_or(99),
          indent=4
        ));
        index.push(InlineKeyboardButton::with_callback_data(
          format!("{} {}", i, anime_attr.canonical_title),
          format!("/{}/detail/{}/", kitsu_id, anime.id),
        ));
      }
    }
  };
  (text, vec![index, navigate])
}
