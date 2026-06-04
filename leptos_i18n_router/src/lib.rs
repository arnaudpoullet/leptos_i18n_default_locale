#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![deny(warnings)]
//! This crate contain anything related to routing for the `leptos_i18n` crate.

mod components;
mod routing;

pub use components::I18nRoute;

/// Whether the default locale gets a URL prefix like the others.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrefixDefault {
    /// Default locale is served unprefixed (`/path`). Current behavior.
    #[default]
    Never,
    /// Every locale is prefixed, including the default (`/fr/path`).
    ///
    /// An unprefixed request is redirected to the resolved locale's prefixed
    /// URL, regardless of how that locale was resolved.
    Always,
    /// Every locale is prefixed, but the unprefixed apex (and other unprefixed
    /// paths) are *rendered* rather than redirected — letting you show a
    /// language splash screen — **unless** the user has an explicit locale
    /// cookie, in which case they are redirected to that locale's prefix.
    ///
    /// In other words: a first-time visitor with no cookie stays on the
    /// unprefixed URL (show them a chooser); a returning visitor with a cookie
    /// is sent to `/{their_locale}/…`.
    AlwaysSplash,
}

/// Create a localized path (one or more static segments) based on a locale.
///
/// ```rust, ignore
/// <Route path=i18n_path!(Locale, |locale| td_string!(locale, routes.some_path)) view=.. />
/// ```
#[macro_export]
macro_rules! i18n_path {
    ($t:ty, $func:expr) => {{ $crate::__private::make_i18n_path::<$t, _>($func) }};
}

#[doc(hidden)]
pub mod __private {
    pub use crate::routing::make_i18n_path;
}
