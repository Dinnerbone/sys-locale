//! A library to safely and easily obtain the current locale on the system or for an application.
//!
//! This library currently supports the following platforms:
//! - Android
//! - iOS
//! - macOS
//! - Linux, BSD, and other UNIX variations
//! - WebAssembly on the web (via the `js` feature)
//! - Windows
#![cfg_attr(
    any(
        not(unix),
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ),
    no_std
)]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
use android as provider;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use apple as provider;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
mod unix;
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
use unix as provider;

#[cfg(all(target_family = "wasm", feature = "js", not(unix)))]
mod wasm;
#[cfg(all(target_family = "wasm", feature = "js", not(unix)))]
use wasm as provider;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows as provider;

#[cfg(not(any(unix, all(target_family = "wasm", feature = "js", not(unix)), windows)))]
mod provider {
    pub fn get() -> alloc::vec::Vec<alloc::string::String> {
        alloc::vec::Vec::new()
    }
}

/// Returns the active locale for the system or application.
///
/// This may be equivalent to `get_locales().into_iter().next()` (the first entry),
/// depending on the platform.
///
/// # Returns
///
/// Returns `Some(String)` with a BCP-47 language tag inside. If the locale
/// couldn't be obtained, `None` is returned instead.
///
/// # Example
///
/// ```no_run
/// use sys_locale::get_locale;
///
/// let current_locale = get_locale().unwrap_or_else(|| String::from("en-US"));
///
/// println!("The locale is {}", current_locale);
/// ```
pub fn get_locale() -> Option<String> {
    get_locales().into_iter().next()
}

/// Returns the preferred locales for the system or application, in descending order of preference.
///
/// # Returns
///
/// Returns a `Vec` with any number of BCP-47 language tags inside.
/// If no locale preferences could be obtained, the vec will be empty.
///
/// # Example
///
/// ```no_run
/// use sys_locale::get_locales;
///
/// let locales = get_locales();
/// let fallback = "en-US".to_string();
///
/// println!("The most preferred locale is {}", locales.first().unwrap_or(&fallback));
/// println!("The least preferred locale is {}", locales.last().unwrap_or(&fallback));
/// ```
pub fn get_locales() -> Vec<String> {
    provider::get()
}

#[cfg(test)]
mod tests {
    use super::get_locales;
    extern crate std;

    #[test]
    fn can_obtain_locale() {
        let locales = get_locales();
        assert!(!locales.is_empty(), "locales vec was empty");
        for (i, locale) in locales.into_iter().enumerate() {
            assert!(!locale.is_empty(), "locale string {} was empty", i);
            assert!(
                !locale.ends_with('\0'),
                "locale {} contained trailing NUL",
                i
            );
        }
    }
}
