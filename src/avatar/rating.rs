/// The maximum rating level for which Gravatar will show the user's image instead of the specified
/// default.
///
/// See <https://en.gravatar.com/site/implement/images/#rating>.
#[derive(Clone, Debug)]
pub enum Rating {
    /// Show "G"-rated images only,
    /// suitable for display on all websites with any audience type.
    G,

    /// Show "PG"-rated images or lower only,
    /// may contain rude gestures, provocatively dressed individuals, the lesser swear words,
    /// or mild violence.
    Pg,

    /// Show "R"-rated images or lower only,
    /// may contain such things as harsh profanity, intense violence, nudity, or hard drug use.
    R,

    /// Show all images, up to and including "X"-rated ones,
    /// may contain sexual imagery or extremely disturbing violence.
    X,
}
