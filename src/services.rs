//! Accounts and remote services: signing in (OAuth, GNOME Online Accounts,
//! Microsoft's broker), Microsoft Graph, cloud storage, translation, grammar
//! checking, and the pictures fetched for senders.

pub(crate) mod avatar;
pub(crate) mod cloud;
pub(crate) mod goa;
pub(crate) mod grammar;
pub(crate) mod graph_api;
pub(crate) mod graph_labels;
pub(crate) mod logo;
pub(crate) mod ms_broker;
pub(crate) mod oauth;
pub(crate) mod remote_content;
pub(crate) mod translate;

/// Lowercase hexadecimal text of `bytes`. A digest is a byte array that no longer prints with
/// `{:x}` (sha2 0.11), and a hash in a URL or a marker file is hex.
pub(crate) fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut out, b| {
            let _ = write!(out, "{b:02x}");
            out
        })
}

#[cfg(test)]
mod hex_tests {
    use super::hex;

    #[test]
    fn hex_is_lowercase_and_pads_each_byte() {
        assert_eq!(hex(&[0x00, 0x0a, 0xff]), "000aff");
        assert_eq!(hex(&[]), "");
    }

    #[test]
    fn a_sha256_digest_prints_as_the_usual_sixty_four_characters() {
        use sha2::Digest;
        assert_eq!(
            hex(&sha2::Sha256::digest(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
