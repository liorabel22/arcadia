pub fn get_announce_url(passkey_upper: i64, passkey_lower: i64, tracker_url: &str) -> String {
    let passkey = ((passkey_upper as u64 as u128) << 64) | (passkey_lower as u64 as u128);

    format!("{}/{passkey:x}/announce", tracker_url.trim_end_matches('/'))
}
