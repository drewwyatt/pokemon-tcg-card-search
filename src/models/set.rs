use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    code: String,
    expanded_legal: bool,
    logo_url: String,
    name: String,
    ptcgo_code: Option<String>,
    #[serde(with = "set_format")]
    release_date: NaiveDate,
    series: String,
    standard_legal: bool,
    symbol_url: String,
    total_cards: u8,
    #[serde(with = "set_format")]
    updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetResponse {
    pub sets: Vec<Set>,
}

mod set_format {
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};
    use crate::models::DateFormatType;

    pub trait DateFormatStringProviding where Self: DateFormatType {
        fn format_string() -> &'static str;
    }

    impl DateFormatStringProviding for NaiveDateTime {
        fn format_string() -> &'static str {
            "%m/%d/%Y %H:%M:%S"
        }
    }

    impl DateFormatStringProviding for NaiveDate {
        fn format_string() -> &'static str {
            "%m/%d/%Y"
        }
    }

    pub fn serialize<Date, S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        Date: DateFormatStringProviding,
        S: Serializer,
    {
        let s = format!("{}", date.format(Date::format_string()));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, Date, D>(deserializer: D) -> Result<Date, D::Error>
    where
        Date: DateFormatStringProviding,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::parse_from_str(&s, Date::format_string()).map_err(serde::de::Error::custom)
    }
}
