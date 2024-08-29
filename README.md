# Gravatar API Client for Rust

A small library to access the Gravatar API,
inspired by [rust-gravatar](https://github.com/chowdhurya/rust-gravatar/).

## Example Usage

```rust
extern crate gravatar_api;
use gravatar_api::avatars;

let url = avatars::Avatar::builder("john.doe@example.com")
    .size(512)
    .default(avatars::Default::RoboHash)
    .rating(avatars::Rating::G)
    .build()
    .image_url();
assert_eq!(
    url.as_str(),
    "https://www.gravatar.com/avatar/836f82db99121b3481011f16b49dfa5fbc714a0d1b1b9f784a1ebbbf5b39577f?s=512&r=g&d=robohash"
);
```
