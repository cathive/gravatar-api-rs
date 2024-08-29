# Gravatar API Client for Rust

## Example Usage

```rust
use gravatar_api::avatars;

fn main() {
    println!(
        "{}",
        avatars::Avatar::builder("john.doe@example.com")
            .size(512)
            .default(avatars::Default::RoboHash)
            .rating(avatars::Rating::G)
            .build()
            .image_url()
    );
}
``
