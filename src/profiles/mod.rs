const BASE_URL: &str = "https://gravatar.com/";

use url::Url;

use crate::common::email_hash;

mod format;

pub use format::Format;

/// Representation of a single Gravatar profile URL.
#[derive(Clone, Debug)]
pub struct Profile {
    email: String,
}

impl Profile {
    pub fn builder(email: &str) -> ProfileBuilder {
        ProfileBuilder::new(email)
    }

    /// Returns the URL of the Gravatar image.
    pub fn profile_url(self: &Self, format: Format) -> Url {
        let str = format!(
            "{}{}.{}",
            BASE_URL,
            email_hash(&self.email),
            format.to_string()
        );
        Url::parse(&str).unwrap()
    }
}

// Builder for Profile instances.
#[derive(core::default::Default)]
pub struct ProfileBuilder {
    email: String,
}

impl ProfileBuilder {
    pub fn new(email: &str) -> ProfileBuilder {
        ProfileBuilder {
            email: email.to_string(),
            ..core::default::Default::default()
        }
    }
    /// Builds the Profile instance.
    pub fn build(self) -> Profile {
        Profile { email: self.email }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_builder() {
        let email = "anonymous@example.com";

        let builder = Profile::builder(email);
        let profile = builder.build();
        assert_eq!(email, profile.email);
        assert_eq!("https://gravatar.com/f807b5609eae64257bf4877652ea49fee40ac2451c152c12fa596ffeda647157.json", profile.profile_url(Format::Json).to_string());
        assert_eq!("https://gravatar.com/f807b5609eae64257bf4877652ea49fee40ac2451c152c12fa596ffeda647157.xml", profile.profile_url(Format::Xml).to_string());
        assert_eq!("https://gravatar.com/f807b5609eae64257bf4877652ea49fee40ac2451c152c12fa596ffeda647157.php", profile.profile_url(Format::Php).to_string());
        assert_eq!("https://gravatar.com/f807b5609eae64257bf4877652ea49fee40ac2451c152c12fa596ffeda647157.vcf", profile.profile_url(Format::Vcard).to_string());
        assert_eq!("https://gravatar.com/f807b5609eae64257bf4877652ea49fee40ac2451c152c12fa596ffeda647157.qr", profile.profile_url(Format::QrCode).to_string());
    }
}
