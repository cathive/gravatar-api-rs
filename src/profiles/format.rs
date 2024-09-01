use std::fmt;

/// The data format to be used when retrieving profiles.
///
/// See <https://docs.gravatar.com/api/profiles/#data-formats>.
#[derive(Clone, Debug)]
pub enum Format {
    /// JSON
    /// See <https://docs.gravatar.com/api/profiles/json/>.
    Json,

    /// XML
    /// See <https://docs.gravatar.com/api/profiles/xml/>.
    Xml,

    /// PHP
    /// See <https://docs.gravatar.com/api/profiles/php/>.
    Php,

    /// VCF/vCard
    /// See <https://docs.gravatar.com/api/profiles/vcf-vcard/>.
    Vcard,

    // QR code image in PNG format
    // See <https://docs.gravatar.com/api/profiles/qr-codes/>
    QrCode,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Format::Json => "json",
            Format::Xml => "xml",
            Format::Php => "php",
            Format::Vcard => "vcf",
            Format::QrCode => "qr",
        };
        f.write_str(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(Format::Json.to_string(), "json");
        assert_eq!(Format::Xml.to_string(), "xml");
        assert_eq!(Format::Php.to_string(), "php");
        assert_eq!(Format::Vcard.to_string(), "vcf");
        assert_eq!(Format::QrCode.to_string(), "qr");
    }
}
