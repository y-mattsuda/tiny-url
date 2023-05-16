use std::format;

use chrono::{DateTime, Local};
use derive_new::new;

use super::Id;

#[derive(new, Debug, Clone, PartialEq)]
pub struct LongUrl(pub String);

impl TryFrom<String> for LongUrl {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(LongUrl::new(value))
    }
}

#[derive(new, Debug, Clone, PartialEq)]
pub struct ShortUrl(pub String);

impl TryFrom<String> for ShortUrl {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(ShortUrl::new(value))
    }
}

impl TryFrom<LongUrl> for ShortUrl {
    type Error = anyhow::Error;
    fn try_from(lu: LongUrl) -> Result<Self, Self::Error> {
        let hashed = crc32fast::hash(lu.0.as_bytes());
        Ok(Self::new(format!("{:x}", hashed)))
    }
}

#[derive(new, Debug)]
pub struct Url {
    pub id: Id<Self>,
    pub long: LongUrl,
    pub short: ShortUrl,
    pub create_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new, Debug)]
pub struct NewUrl {
    pub id: Id<Self>,
    pub long: LongUrl,
    pub short: ShortUrl,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_long_url_to_short_url() {
        let long_url = LongUrl::try_from("https://www.google.com".to_string()).unwrap();
        let short_url = ShortUrl::try_from(long_url).unwrap();
        assert_eq!(short_url.0, "331e5b6b");
    }
}
