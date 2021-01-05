use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::models::{Search, Searchable};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl Searchable<Set> for Vec<Set> {
    fn identify(&self, query: &Search) -> Option<String> {
        match query {
            Search::SetNumber(_, card_number) => match self.search(query) {
                Some(set) => {
                    let mut id = set.code.clone();
                    id.push_str("-");
                    id.push_str(&card_number.to_string());
                    Some(id)
                }
                None => None,
            },
            _ => None,
        }
    }

    fn search(&self, query: &Search) -> Option<&Set> {
        match query {
            Search::SetNumber(ptcgo_code, _) => {
                let comparable_code = Some(ptcgo_code.clone());
                self.iter().find(|set| set.ptcgo_code == comparable_code)
            }
            _ => None,
        }
    }
}

// > Anna's opponent uses Gengar & Mimikyu-GX's (TEU, 53) Horror House-GX attack. During Anna's next turn, she wants to use her Meganium's (LOT, 8) Quick-Ripening Herb Ability to evolve one of her Benched Pokémon. Can she do this?

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
