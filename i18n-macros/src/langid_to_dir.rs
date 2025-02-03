/// Converts a Unicode langid to the language's respective writing direction.
pub fn langid_to_dir(langid: &str) -> &'static str {
    match langid {
        "aa" => "ltr",
        "ab" => "ltr",
        "ae" => "ltr",
        "af" => "ltr",
        "ak" => "ltr",
        "am" => "ltr",
        "an" => "ltr",
        "ar" => "rtl",
        "as" => "ltr",
        "av" => "ltr",
        "ay" => "ltr",
        "az" => "ltr",
        "ba" => "ltr",
        "be" => "ltr",
        "bg" => "ltr",
        "bi" => "ltr",
        "bm" => "auto",
        "bn" => "ltr",
        "bo" => "ltr",
        "br" => "ltr",
        "bs" => "ltr",
        "ca" => "ltr",
        "ce" => "ltr",
        "ch" => "ltr",
        "co" => "ltr",
        "cr" => "ltr",
        "cs" => "ltr",
        "cu" => "ltr",
        "cv" => "ltr",
        "cy" => "ltr",
        "da" => "ltr",
        "de" => "ltr",
        "dv" => "rtl",
        "dz" => "ltr",
        "ee" => "ltr",
        "el" => "ltr",
        "en" => "ltr",
        "eo" => "ltr",
        "es" => "ltr",
        "et" => "ltr",
        "eu" => "ltr",
        "fa" => "rtl",
        "ff" => "ltr",
        "fi" => "ltr",
        "fj" => "ltr",
        "fo" => "ltr",
        "fr" => "ltr",
        "fy" => "ltr",
        "ga" => "ltr",
        "gd" => "ltr",
        "gl" => "ltr",
        "gn" => "ltr",
        "gu" => "ltr",
        "gv" => "ltr",
        "ha" => "ltr",
        "he" => "rtl",
        "hi" => "ltr",
        "ho" => "ltr",
        "hr" => "ltr",
        "ht" => "ltr",
        "hu" => "ltr",
        "hy" => "ltr",
        "hz" => "ltr",
        "ia" => "ltr",
        "id" => "ltr",
        "ie" => "ltr",
        "ig" => "ltr",
        "ii" => "ltr",
        "ik" => "ltr",
        "io" => "ltr",
        "is" => "ltr",
        "it" => "ltr",
        "iu" => "ltr",
        "ja" => "auto", // (top to bottom)
        "jv" => "ltr",
        "ka" => "ltr",
        "kg" => "ltr",
        "ki" => "ltr",
        "kj" => "ltr",
        "kk" => "ltr",
        "kl" => "ltr",
        "km" => "ltr",
        "kn" => "ltr",
        "ko" => "auto", // (top to bottom)
        "kr" => "ltr",
        "ks" => "rtl",
        "ku" => "rtl",
        "kv" => "ltr",
        "kw" => "ltr",
        "ky" => "ltr",
        "la" => "ltr",
        "lb" => "ltr",
        "lg" => "ltr",
        "li" => "ltr",
        "ln" => "ltr",
        "lo" => "ltr",
        "lt" => "ltr",
        "lu" => "ltr",
        "lv" => "ltr",
        "mg" => "ltr",
        "mh" => "ltr",
        "mi" => "ltr",
        "mk" => "ltr",
        "ml" => "ltr",
        "mn" => "auto", // (top to bottom)
        "mr" => "ltr",
        "ms" => "ltr",
        "mt" => "ltr",
        "my" => "ltr",
        "na" => "ltr",
        "nb" => "ltr",
        "nd" => "ltr",
        "ne" => "ltr",
        "ng" => "ltr",
        "nl" => "ltr",
        "nn" => "ltr",
        "no" => "ltr",
        "nr" => "ltr",
        "nv" => "ltr",
        "ny" => "ltr",
        "oc" => "ltr",
        "oj" => "ltr",
        "om" => "ltr",
        "or" => "ltr",
        "os" => "ltr",
        "pa" => "rtl",
        "pi" => "ltr",
        "pl" => "ltr",
        "ps" => "rtl",
        "pt" => "ltr",
        "qu" => "ltr",
        "rm" => "ltr",
        "rn" => "ltr",
        "ro" => "ltr",
        "ru" => "ltr",
        "rw" => "ltr",
        "sa" => "ltr",
        "sc" => "ltr",
        "sd" => "rtl",
        "se" => "ltr",
        "sg" => "ltr",
        "si" => "ltr",
        "sk" => "ltr",
        "sl" => "ltr",
        "sm" => "ltr",
        "sn" => "ltr",
        "so" => "ltr",
        "sq" => "ltr",
        "sr" => "ltr",
        "ss" => "ltr",
        "st" => "ltr",
        "su" => "ltr",
        "sv" => "ltr",
        "sw" => "ltr",
        "ta" => "ltr",
        "te" => "ltr",
        "tg" => "ltr",
        "th" => "ltr",
        "ti" => "ltr",
        "tk" => "rtl",
        "tl" => "ltr",
        "tn" => "ltr",
        "to" => "ltr",
        "tr" => "ltr",
        "ts" => "ltr",
        "tt" => "ltr",
        "tw" => "ltr",
        "ty" => "ltr",
        "ug" => "rtl",
        "uk" => "ltr",
        "ur" => "rtl",
        "uz" => "ltr",
        "ve" => "ltr",
        "vi" => "auto", // (top to bottom)
        "vo" => "ltr",
        "wa" => "ltr",
        "wo" => "ltr",
        "xh" => "ltr",
        "yi" => "rtl",
        "yo" => "ltr",
        "za" => "auto", // (top to bottom)
        "zh" => "auto", // (top to bottom)
        "zu" => "ltr",
        _ => "auto",
    }
}
