//! Which section of the page `errors` explains an error, told from the error's
//! own words. One table, first match wins, matched without regard to case.

/// The section for an error no row of `CASES` matches.
pub const GENERAL: &str = "errors#an-error-with-no-page-of-its-own";

/// A kind of error: its section, and the words that tell it. Order matters:
/// a driver that will not build also says dpkg could not configure it, and a
/// full disk says the same, so the more specific kinds come first. A kept-back
/// list may name a driver package (`virtualbox-dkms`), so it comes before the driver.
pub const CASES: &[(&str, &[&str])] = &[
    ("errors#the-package-database-is-locked", &["could not get lock", "unable to acquire the dpkg", "unable to lock the administration directory", "is another process using it", "dpkg frontend lock", "var/lib/dpkg/lock", "var/lib/apt/lists/lock"]),
    ("errors#there-is-no-space-left", &["no space left on device", "not enough disk space", "not enough free space", "enospc", "insufficient disk space", "disk full"]),
    ("errors#updates-were-kept-back", &["kept back:"]),
    ("errors#a-driver-would-not-build-for-the-new-kernel", &["dkms autoinstall", "dkms: ", "dkms) will not build", "bad return status for module build", "module build for kernel"]),
    ("errors#packages-have-unmet-dependencies", &["unmet dependencies", "held broken packages", "apt --fix-broken", "apt-get -f install", "is not going to be installed", "but it is not installable"]),
    ("errors#a-package-would-not-configure", &["dpkg: error processing", "errors were encountered while processing", "post-installation script returned", "pre-installation script returned", "dpkg was interrupted", "dpkg --configure -a", "package is in a very bad inconsistent state", "dependency problems - leaving unconfigured"]),
    ("errors#a-download-failed", &["failed to fetch", "hash sum mismatch", "file has unexpected size", "size mismatch", "404  not found", "404 not found", "some index files failed to download", "unable to fetch some archives", "temporary failure resolving", "could not resolve", "connection timed out", "connection failed", "bad header line"]),
];

/// The section whose words fit `error`, if any.
pub fn classify(error: &str) -> Option<&'static str> {
    let low = error.to_lowercase();
    CASES.iter().find(|(_, needles)| needles.iter().any(|n| low.contains(n))).map(|(address, _)| *address)
}

/// The address to open for an error: the one its words tell; else the one
/// the failure line named; else, when there are words, the general one.
pub fn address_for(named: &str, error: &str) -> String {
    if let Some(a) = classify(error) {
        return a.to_string();
    }
    if named.trim().is_empty() && !error.trim().is_empty() {
        return GENERAL.to_string();
    }
    named.to_string()
}
