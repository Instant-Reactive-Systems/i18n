/// Converts a Unicode langid into a conforming ISO-639 code.
pub fn langid_to_iso639(langid: &str) -> String {
    let splitter = if langid.contains('_') {
        '_'
    } else if langid.contains('-') {
        '-'
    } else {
        return langid.to_lowercase();
    };

    langid
        .split(splitter)
        .next()
        .expect("at least one should exist here")
        .to_string()
}

/// Extracts the country code from a Unicode langid.
pub fn langid_to_country_code(langid: &str) -> Option<String> {
    let splitter = if langid.contains('_') {
        '_'
    } else if langid.contains('-') {
        '-'
    } else {
        return None;
    };

    langid.split(splitter).nth(1).map(ToString::to_string)
}
