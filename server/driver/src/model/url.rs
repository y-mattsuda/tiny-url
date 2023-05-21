use kernel::model::url::Url;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct JsonUrlToShorten {
    #[validate(url)]
    pub long_url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonUrl {
    id: String,
    long_url: String,
    short_url: String,
}

impl From<Url> for JsonUrl {
    fn from(u: Url) -> Self {
        JsonUrl {
            id: u.id.value.to_string(),
            long_url: u.long.0,
            short_url: u.short.0,
        }
    }
}
