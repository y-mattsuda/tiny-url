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

impl LongUrl {
    /// Generate a 5-character long hash from the long URL.
    /// Attention! The return value of this method can collide!
    /// So, application layer must handle collisions by searching for existing short URLs.
    pub fn gen_hash(&self) -> String {
        format!("{:x}", crc32fast::hash(self.0.as_bytes()))
            .chars()
            .take(5)
            .collect()
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
    use std::println;

    use super::*;

    #[test]
    fn test_try_from_long_url_to_hash() {
        let long_url = LongUrl::try_from("https://example.com".to_string()).unwrap();
        let hash = long_url.gen_hash();
        println!("long: {}, hash: {}", long_url.0, hash);
        assert_eq!(hash.len(), 5);
    }
}
