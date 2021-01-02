use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    pub code: String,
    pub expanded_legal: bool,
    pub logo_url: String,
    pub name: String,
    pub ptcgo_code: Option<String>,
    #[serde(with = "set_format")]
    pub release_date: NaiveDate,
    pub series: String,
    pub standard_legal: bool,
    pub symbol_url: String,
    pub total_cards: u8,
    #[serde(with = "set_format")]
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetResponse {
    pub sets: Vec<Set>,
}

mod set_format {
    use crate::models::DateFormatType;
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub trait DateFormatStringProviding
    where
        Self: DateFormatType,
    {
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

    pub fn serialize<Date, Ser>(date: &Date, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Date: DateFormatStringProviding,
        Ser: Serializer,
    {
        let s = format!("{}", date.format(Date::format_string()));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, Date, De>(deserializer: De) -> Result<Date, De::Error>
    where
        Date: DateFormatStringProviding,
        De: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::parse_from_str(&s, Date::format_string()).map_err(serde::de::Error::custom)
    }
}
