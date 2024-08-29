extern crate sha2;
extern crate url;

#[path = "avatars/mod.rs"]
mod _avatars;

#[path = "common/mod.rs"]
mod _common;

pub use public::*;
pub mod public {
    pub mod avatars {
        pub use crate::_avatars::{Avatar, AvatarBuilder, Rating, _Default as Default};
    }
}
