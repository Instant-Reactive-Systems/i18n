/// Converts a Unicode langid into its respective country flag.
pub fn langid_to_flag(langid: &str) -> Option<&'static str> {
    let country_code = crate::utils::langid_to_country_code(langid)?;
    match country_code.to_uppercase().as_str() {
        "AD" => Some("🇦🇩"),
        "AE" => Some("🇦🇪"),
        "AF" => Some("🇦🇫"),
        "AG" => Some("🇦🇬"),
        "AI" => Some("🇦🇮"),
        "AL" => Some("🇦🇱"),
        "AM" => Some("🇦🇲"),
        "AO" => Some("🇦🇴"),
        "AQ" => Some("🇦🇶"),
        "AR" => Some("🇦🇷"),
        "AS" => Some("🇦🇸"),
        "AT" => Some("🇦🇹"),
        "AU" => Some("🇦🇺"),
        "AW" => Some("🇦🇼"),
        "AX" => Some("🇦🇽"),
        "AZ" => Some("🇦🇿"),
        "BA" => Some("🇧🇦"),
        "BB" => Some("🇧🇧"),
        "BD" => Some("🇧🇩"),
        "BE" => Some("🇧🇪"),
        "BF" => Some("🇧🇫"),
        "BG" => Some("🇧🇬"),
        "BH" => Some("🇧🇭"),
        "BI" => Some("🇧🇮"),
        "BJ" => Some("🇧🇯"),
        "BL" => Some("🇧🇱"),
        "BM" => Some("🇧🇲"),
        "BN" => Some("🇧🇳"),
        "BO" => Some("🇧🇴"),
        "BQ" => Some("🇧🇶"),
        "BR" => Some("🇧🇷"),
        "BS" => Some("🇧🇸"),
        "BT" => Some("🇧🇹"),
        "BV" => Some("🇧🇻"),
        "BW" => Some("🇧🇼"),
        "BY" => Some("🇧🇾"),
        "BZ" => Some("🇧🇿"),
        "CA" => Some("🇨🇦"),
        "CC" => Some("🇨🇨"),
        "CD" => Some("🇨🇩"),
        "CF" => Some("🇨🇫"),
        "CG" => Some("🇨🇬"),
        "CH" => Some("🇨🇭"),
        "CI" => Some("🇨🇮"),
        "CK" => Some("🇨🇰"),
        "CL" => Some("🇨🇱"),
        "CM" => Some("🇨🇲"),
        "CN" => Some("🇨🇳"),
        "CO" => Some("🇨🇴"),
        "CR" => Some("🇨🇷"),
        "CU" => Some("🇨🇺"),
        "CV" => Some("🇨🇻"),
        "CW" => Some("🇨🇼"),
        "CX" => Some("🇨🇽"),
        "CY" => Some("🇨🇾"),
        "CZ" => Some("🇨🇿"),
        "DE" => Some("🇩🇪"),
        "DJ" => Some("🇩🇯"),
        "DK" => Some("🇩🇰"),
        "DM" => Some("🇩🇲"),
        "DO" => Some("🇩🇴"),
        "DZ" => Some("🇩🇿"),
        "EC" => Some("🇪🇨"),
        "EE" => Some("🇪🇪"),
        "EG" => Some("🇪🇬"),
        "EH" => Some("🇪🇭"),
        "ER" => Some("🇪🇷"),
        "ES" => Some("🇪🇸"),
        "ET" => Some("🇪🇹"),
        "FI" => Some("🇫🇮"),
        "FJ" => Some("🇫🇯"),
        "FK" => Some("🇫🇰"),
        "FM" => Some("🇫🇲"),
        "FO" => Some("🇫🇴"),
        "FR" => Some("🇫🇷"),
        "GA" => Some("🇬🇦"),
        "GB" => Some("🇬🇧"),
        "GD" => Some("🇬🇩"),
        "GE" => Some("🇬🇪"),
        "GF" => Some("🇬🇫"),
        "GG" => Some("🇬🇬"),
        "GH" => Some("🇬🇭"),
        "GI" => Some("🇬🇮"),
        "GL" => Some("🇬🇱"),
        "GM" => Some("🇬🇲"),
        "GN" => Some("🇬🇳"),
        "GP" => Some("🇬🇵"),
        "GQ" => Some("🇬🇶"),
        "GR" => Some("🇬🇷"),
        "GS" => Some("🇬🇸"),
        "GT" => Some("🇬🇹"),
        "GU" => Some("🇬🇺"),
        "GW" => Some("🇬🇼"),
        "GY" => Some("🇬🇾"),
        "HK" => Some("🇭🇰"),
        "HM" => Some("🇭🇲"),
        "HN" => Some("🇭🇳"),
        "HR" => Some("🇭🇷"),
        "HT" => Some("🇭🇹"),
        "HU" => Some("🇭🇺"),
        "ID" => Some("🇮🇩"),
        "IE" => Some("🇮🇪"),
        "IL" => Some("🇮🇱"),
        "IM" => Some("🇮🇲"),
        "IN" => Some("🇮🇳"),
        "IO" => Some("🇮🇴"),
        "IQ" => Some("🇮🇶"),
        "IR" => Some("🇮🇷"),
        "IS" => Some("🇮🇸"),
        "IT" => Some("🇮🇹"),
        "JE" => Some("🇯🇪"),
        "JM" => Some("🇯🇲"),
        "JO" => Some("🇯🇴"),
        "JP" => Some("🇯🇵"),
        "KE" => Some("🇰🇪"),
        "KG" => Some("🇰🇬"),
        "KH" => Some("🇰🇭"),
        "KI" => Some("🇰🇮"),
        "KM" => Some("🇰🇲"),
        "KN" => Some("🇰🇳"),
        "KP" => Some("🇰🇵"),
        "KR" => Some("🇰🇷"),
        "KW" => Some("🇰🇼"),
        "KY" => Some("🇰🇾"),
        "KZ" => Some("🇰🇿"),
        "LA" => Some("🇱🇦"),
        "LB" => Some("🇱🇧"),
        "LC" => Some("🇱🇨"),
        "LI" => Some("🇱🇮"),
        "LK" => Some("🇱🇰"),
        "LR" => Some("🇱🇷"),
        "LS" => Some("🇱🇸"),
        "LT" => Some("🇱🇹"),
        "LU" => Some("🇱🇺"),
        "LV" => Some("🇱🇻"),
        "LY" => Some("🇱🇾"),
        "MA" => Some("🇲🇦"),
        "MC" => Some("🇲🇨"),
        "MD" => Some("🇲🇩"),
        "ME" => Some("🇲🇪"),
        "MF" => Some("🇲🇫"),
        "MG" => Some("🇲🇬"),
        "MH" => Some("🇲🇭"),
        "MK" => Some("🇲🇰"),
        "ML" => Some("🇲🇱"),
        "MM" => Some("🇲🇲"),
        "MN" => Some("🇲🇳"),
        "MO" => Some("🇲🇴"),
        "MP" => Some("🇲🇵"),
        "MQ" => Some("🇲🇶"),
        "MR" => Some("🇲🇷"),
        "MS" => Some("🇲🇸"),
        "MT" => Some("🇲🇹"),
        "MU" => Some("🇲🇺"),
        "MV" => Some("🇲🇻"),
        "MW" => Some("🇲🇼"),
        "MX" => Some("🇲🇽"),
        "MY" => Some("🇲🇾"),
        "MZ" => Some("🇲🇿"),
        "NA" => Some("🇳🇦"),
        "NC" => Some("🇳🇨"),
        "NE" => Some("🇳🇪"),
        "NF" => Some("🇳🇫"),
        "NG" => Some("🇳🇬"),
        "NI" => Some("🇳🇮"),
        "NL" => Some("🇳🇱"),
        "NO" => Some("🇳🇴"),
        "NP" => Some("🇳🇵"),
        "NR" => Some("🇳🇷"),
        "NU" => Some("🇳🇺"),
        "NZ" => Some("🇳🇿"),
        "OM" => Some("🇴🇲"),
        "PA" => Some("🇵🇦"),
        "PE" => Some("🇵🇪"),
        "PF" => Some("🇵🇫"),
        "PG" => Some("🇵🇬"),
        "PH" => Some("🇵🇭"),
        "PK" => Some("🇵🇰"),
        "PL" => Some("🇵🇱"),
        "PM" => Some("🇵🇲"),
        "PN" => Some("🇵🇳"),
        "PR" => Some("🇵🇷"),
        "PS" => Some("🇵🇸"),
        "PT" => Some("🇵🇹"),
        "PW" => Some("🇵🇼"),
        "PY" => Some("🇵🇾"),
        "QA" => Some("🇶🇦"),
        "RE" => Some("🇷🇪"),
        "RO" => Some("🇷🇴"),
        "RS" => Some("🇷🇸"),
        "RU" => Some("🇷🇺"),
        "RW" => Some("🇷🇼"),
        "SA" => Some("🇸🇦"),
        "SB" => Some("🇸🇧"),
        "SC" => Some("🇸🇨"),
        "SD" => Some("🇸🇩"),
        "SE" => Some("🇸🇪"),
        "SG" => Some("🇸🇬"),
        "SH" => Some("🇸🇭"),
        "SI" => Some("🇸🇮"),
        "SJ" => Some("🇸🇯"),
        "SK" => Some("🇸🇰"),
        "SL" => Some("🇸🇱"),
        "SM" => Some("🇸🇲"),
        "SN" => Some("🇸🇳"),
        "SO" => Some("🇸🇴"),
        "SR" => Some("🇸🇷"),
        "SS" => Some("🇸🇸"),
        "ST" => Some("🇸🇹"),
        "SV" => Some("🇸🇻"),
        "SX" => Some("🇸🇽"),
        "SY" => Some("🇸🇾"),
        "SZ" => Some("🇸🇿"),
        "TC" => Some("🇹🇨"),
        "TD" => Some("🇹🇩"),
        "TF" => Some("🇹🇫"),
        "TG" => Some("🇹🇬"),
        "TH" => Some("🇹🇭"),
        "TJ" => Some("🇹🇯"),
        "TK" => Some("🇹🇰"),
        "TL" => Some("🇹🇱"),
        "TM" => Some("🇹🇲"),
        "TN" => Some("🇹🇳"),
        "TO" => Some("🇹🇴"),
        "TR" => Some("🇹🇷"),
        "TT" => Some("🇹🇹"),
        "TV" => Some("🇹🇻"),
        "TW" => Some("🇹🇼"),
        "TZ" => Some("🇹🇿"),
        "UA" => Some("🇺🇦"),
        "UG" => Some("🇺🇬"),
        "UM" => Some("🇺🇲"),
        "US" => Some("🇺🇸"),
        "UY" => Some("🇺🇾"),
        "UZ" => Some("🇺🇿"),
        "VA" => Some("🇻🇦"),
        "VC" => Some("🇻🇨"),
        "VE" => Some("🇻🇪"),
        "VG" => Some("🇻🇬"),
        "VI" => Some("🇻🇮"),
        "VN" => Some("🇻🇳"),
        "VU" => Some("🇻🇺"),
        "WF" => Some("🇼🇫"),
        "WS" => Some("🇼🇸"),
        "YE" => Some("🇾🇪"),
        "YT" => Some("🇾🇹"),
        "ZA" => Some("🇿🇦"),
        "ZM" => Some("🇿🇲"),
        "ZW" => Some("🇿🇼"),
        _ => None,
    }
}
