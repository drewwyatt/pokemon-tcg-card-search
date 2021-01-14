use regex::{Captures, Regex};
use std::convert::TryFrom;
use web_sys::HtmlInputElement;
use yew::NodeRef;

use crate::models::PokemonError;

#[derive(Debug)]
pub enum Search {
    SetNumber(String, u8),
}

pub trait Searchable<T> {
    fn search(&self, query: &Search) -> Option<&T>;
    fn identify(&self, query: &Search) -> Option<String>;
}

impl TryFrom<NodeRef> for Search {
    type Error = PokemonError;

    fn try_from(node: NodeRef) -> Result<Search, PokemonError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([a-zA-Z\d]+)(?:[\W]+)(\d+)$").unwrap();
        }

        let input: String = node.cast::<HtmlInputElement>().unwrap().value();
        let captures: Captures = RE.captures(&input).ok_or(PokemonError::InvalidRegex)?; // TODO: match here and fallback to CardName or something

        let ptcgo_code = captures
            .get(1)
            .ok_or(PokemonError::InvalidRegex)?
            .as_str()
            .to_uppercase();
        let card_number: u8 = captures
            .get(2)
            .ok_or(PokemonError::InvalidRegex)?
            .as_str()
            .parse::<u8>()
            .map_err(|_| PokemonError::InvalidRegex)?;

        Ok(Search::SetNumber(String::from(ptcgo_code), card_number))
    }
}
