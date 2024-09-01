// See https://docs.gravatar.com/api/avatars/

use reqwest;
use url::Url;

use crate::common::email_hash;

mod default;
mod rating;

pub use default::Default;
pub use rating::Rating;

const BASE_URL: &str = "https://www.gravatar.com/";

/// Representation of a single Gravatar image URL.
#[derive(Clone, Debug)]
pub struct Avatar {
    email: String,
    pub size: Option<u16>,
    pub default: Option<Default>,
    pub force_default: Option<bool>,
    pub rating: Option<Rating>,
}

impl Avatar {
    pub fn builder(email: &str) -> AvatarBuilder {
        AvatarBuilder::new(email)
    }

    /// Returns the URL of the Gravatar image.
    pub fn image_url(self: &Self) -> Url {
        let mut str = format!("{}avatar/{}", BASE_URL, email_hash(&self.email));
        if let Some(size) = self.size {
            str.push_str(&format!("?s={}", size));
        }
        if let Some(rating) = &self.rating {
            str.push_str(&format!("&r={}", rating.to_string()));
        }
        if let Some(default) = &self.default {
            str.push_str(&format!("&d={}", default.to_string()));
        }
        if let Some(force_default) = self.force_default {
            str.push_str(&format!("&f={}", force_default.to_string()));
        }
        Url::parse(&str).unwrap()
    }

    pub fn get_image_bytes(&self) -> reqwest::Result<bytes::Bytes> {
        let url = self.image_url();
        match reqwest::blocking::get(url) {
            Ok(response) => response.bytes(),
            Err(e) => Err(e),
        }
    }

    pub async fn get_image_bytes_async(&self) -> reqwest::Result<bytes::Bytes> {
        let url = self.image_url();
        match reqwest::get(url).await {
            Ok(response) => response.bytes().await,
            Err(e) => Err(e),
        }
    }
}

// Builder for Avatar instances.
#[derive(core::default::Default)]
pub struct AvatarBuilder {
    email: String,
    size: Option<u16>,
    default: Option<Default>,
    force_default: Option<bool>,
    rating: Option<Rating>,
}

impl AvatarBuilder {
    pub fn new(email: &str) -> AvatarBuilder {
        AvatarBuilder {
            email: email.to_string(),
            ..core::default::Default::default()
        }
    }

    pub fn size(mut self, size: u16) -> AvatarBuilder {
        self.size = Some(size);
        self
    }

    /// Gravatar allows users to self-rate their images so that they can indicate
    /// if an image is appropriate for a certain audience.
    /// By default, only ‘G’ rated images are displayed unless you indicate that
    /// you would like to see higher ratings.
    ///
    /// If the requested email hash does not have an image meeting the requested
    /// rating level, then the default image is returned (or the specified default,
    /// as per above).
    pub fn rating(mut self, rating: Rating) -> AvatarBuilder {
        self.rating = Some(rating);
        self
    }

    /// Sets the default / fallback image to be used if no Gravatar image
    /// for the given email address can be found.
    pub fn default(mut self, default: Default) -> AvatarBuilder {
        self.default = Some(default);
        self
    }

    /// If for some reason you wanted to force the default image to always load,
    /// you can do that by passing `true` to this method.
    pub fn force_default(mut self, force_default: bool) -> AvatarBuilder {
        self.force_default = Some(force_default);
        self
    }

    /// Builds the Avatar instance.
    pub fn build(self) -> Avatar {
        Avatar {
            email: self.email,
            size: self.size,
            default: self.default,
            force_default: self.force_default,
            rating: self.rating,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_builder() {
        let email = "anonymous@example.com";

        let builder = Avatar::builder(email);
        assert_eq!(email, builder.build().email);
    }
}
