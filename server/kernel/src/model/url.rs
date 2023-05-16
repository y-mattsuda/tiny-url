use std::format;

use chrono::{DateTime, Local};
use derive_new::new;

use super::Id;

#[derive(new, Debug)]
pub struct LongUrl(pub String);

#[derive(new, Debug)]
pub struct ShortUrl(pub String);

impl From<LongUrl> for ShortUrl {
    fn from(value: LongUrl) -> Self {
        let hashed = crc32fast::hash(value.0.as_bytes());
        Self::new(format!("{:x}", hashed))
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
    fn test_shorten_url() {
        let long = LongUrl::new("https://www.google.com".to_string());
        let short = ShortUrl::from(long);
        // CRC-32 hash function
        // ref: https://emn178.github.io/online-tools/crc32.html
        assert_eq!(short.0, "331e5b6b".to_string());
    }
}
