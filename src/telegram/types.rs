//  primitive telegram
pub type Integer = i64;
pub type Float = f32;

#[derive(Debug)]
pub enum Error {
    Api(String),
    Invalid(String),
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub error_code: Option<Integer>,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Integer,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}