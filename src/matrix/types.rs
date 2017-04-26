use std::default::Default;

//  primitive telegram
pub type Integer = i32;

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub body: String,
    pub msgtype: String,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            body: "body".to_string(),
            msgtype: "m.text".to_string()
        }
    }
}
//#[derive(Serialize, Deserialize)]
//pub struct Emote {
//    pub body: String,
//    pub msgtype: Msgtype::Emote,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Notice {
//    pub body: String,
//    pub msgtype: Msgtype::Notice,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Image {
//    pub body: String,
//    pub info: Option<ImageInfo>,
//    pub thumbnail_info: Option<ImageInfo>,
//    pub url: String,
//    pub thumbnail_url: Option<String>,
//    pub msgtype: Msgtype::Image,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct File {
//    pub body: String,
//    pub info: Option<FileInfo>,
//    pub thumbnail_info: Option<ImageInfo>,
//    pub url: String,
//    pub filename: String,
//    pub thumbnail_url: Option<String>,
//    pub msgtype: Msgtype::File,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Location {
//    pub body: String,
//    pub thumbnail_info: Option<ImageInfo>,
//    pub thumbnail_url: Option<String>,
//    pub msgtype: Msgtype::Location,
//    pub geo_uri: String
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Video {
//    pub body: String,
//    pub info: Option<VideoInfo>,
//    pub msgtype: Msgtype::Video,
//    pub url: String,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct Audio {
//    pub body: String,
//    pub info: Option<AudioInfo>,
//    pub msgtype: Msgtype::Audio,
//    pub url: String,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct ImageInfo {
//    pub mimetype: String,
//    pub h: Integer,
//    pub w: Integer,
//    pub size: Integer,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct FileInfo {
//    pub mimetype: String,
//    pub size: Integer,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct VideoInfo {
//    pub mimetype: String,
//    pub thumbnail_info: ImageInfo,
//    pub h: Integer,
//    pub thumbnail_url: String,
//    pub w: Integer,
//    pub duration: Integer,
//    pub size: Integer,
//}
//
//#[derive(Serialize, Deserialize)]
//pub struct AudioInfo {
//    pub duration: Integer,
//    pub mimetype: String,
//    pub size: Integer,
//}

// Matrix User Profile Information, See: https://matrix.org/docs/spec/client_server/r0.2.0.html#id168
#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub avatar_url: Option<String>,
    pub displayname: Option<String>,
}

