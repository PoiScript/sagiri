pub mod kitsu;
pub mod matrix;
pub mod telegram;

use hyper_tls::HttpsConnector;
use hyper::client::{self, HttpConnector};

pub type Client = client::Client<HttpsConnector<HttpConnector>>;
