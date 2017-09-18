use std::borrow::Cow;
use std::str::FromStr;

use url::Url;

use types::{MsgCommand, QueryCommand};
use types::kitsu::*;
use types::telegram::*;

named!(pub parse_message<&str, MsgCommand>,
  alt!(
    map!(tag!("/list"), |_| MsgCommand::List) |
    map!(tag!("/update"), |_| MsgCommand::Update) |
    map!(tag!("/version"), |_| MsgCommand::Version)
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
      ) |
      do_parse!(
        tag!("/progress/") >>
        anime_id: map!(take_until!("/"), String::from)  >>
        tag!("/") >>
        entry_id: map!(take_until!("/"), String::from)  >>
        tag!("/") >>
        progress: map_res!(take_until!("/"), i64::from_str)  >>
        (QueryCommand::Progress{ kitsu_id, anime_id, entry_id, progress })
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
  pair: Option<(Entry, Anime)>,
) -> (String, Vec<Vec<InlineKeyboardButton>>) {
  let mut buttons = Vec::new();
  let text = match pair {
    None => format!("Error: No Anime Found :("),
    Some((entry, anime)) => {
      let anime_attr = anime.attributes.unwrap();
      let entry_attr = entry.attributes.unwrap();
      buttons.push(vec![
        InlineKeyboardButton::with_callback_data(
          format!("Make {} Complete", entry_attr.progress.unwrap_or(0) + 1),
          format!(
            "/{}/progress/{}/{}/{}/",
            kitsu_id,
            anime.id,
            entry.id,
            entry_attr.progress.unwrap_or(0) + 1
          ),
        ),
      ]);
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
        entry_attr.status.unwrap_or(EntryStatus::Unknown),
        entry_attr.progress.unwrap_or(0),
        anime_attr.episode_count.unwrap_or(99),
      )
    }
  };
  buttons.push(vec![
    InlineKeyboardButton::with_callback_data(
      String::from("Back to List"),
      format!("/{}/offset/0/", kitsu_id),
    ),
  ]);
  (text, buttons)
}

pub fn parse_anime_list(
  kitsu_id: i64,
  prev: Option<String>,
  next: Option<String>,
  entries: Vec<Entry>,
  animes: Vec<Anime>,
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
  for (
    i,
    (
      &Entry { attributes: ref entry_attr, .. },
      &Anime {
        id: ref anime_id,
        attributes: ref anime_attr,
        ..
      },
    ),
  ) in entries.iter().zip(animes.iter()).enumerate()
  {
    match (entry_attr, anime_attr) {
      (&Some(ref entry_attr), &Some(ref anime_attr)) => {
        text.push_str(&format!("<b>{}| {}</b>", i, anime_attr.canonical_title));
        text.push_str(&format!(
          " <i>{}</i>",
          anime_attr
            .titles
            .ja_jp
            .as_ref()
            .unwrap_or(&String::from("null"))
        ));
        text.push_str(&format!(
          "\n{:?} [{}/{}]\n",
          entry_attr.status.as_ref().unwrap_or(&EntryStatus::Unknown),
          entry_attr.progress.unwrap_or(0),
          anime_attr.episode_count.unwrap_or(99)
        ));
        index.push(InlineKeyboardButton::with_callback_data(
          format!("{} {}", i, anime_attr.canonical_title),
          format!("/{}/detail/{}/", kitsu_id, anime_id),
        ));
      }
      _ => {
        text.push_str(&format!("<b>{}|</b> can't get attributes :(\n", i));
        index.push(InlineKeyboardButton::with_callback_data(
          format!("{} can't get title :(", i),
          format!("/{}/detail/{}/", kitsu_id, anime_id),
        ));
      }
    }
  }
  text.push_str("<i>Choose an anime from the list above.</i>");
  (text, vec![index, navigate])
}
