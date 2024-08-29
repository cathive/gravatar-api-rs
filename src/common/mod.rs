use sha2::{Digest, Sha256};

/// Creates a SHA256 hash of the given string and returns
/// it as a hexadecimal string.
pub fn email_hash(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(email);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_hash() {
        assert_eq!(
            email_hash("anonymous@example.com"),
            "f807b5609eae64257bf4877652ea49fee40ac2451c152c12fa596ffeda647157"
        );
    }
}
