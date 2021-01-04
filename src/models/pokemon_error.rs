#[derive(Debug)]
pub enum PokemonError {
    NetworkError(reqwest::Error),
    DeserializationError(serde_json::Error),
    DateParseError(chrono::ParseError),
    InvalidRegex,
    // Unknown,
}

impl From<reqwest::Error> for PokemonError {
    fn from(error: reqwest::Error) -> Self {
        PokemonError::NetworkError(error)
    }
}

impl From<serde_json::Error> for PokemonError {
    fn from(error: serde_json::Error) -> Self {
        PokemonError::DeserializationError(error)
    }
}

impl From<chrono::ParseError> for PokemonError {
    fn from(error: chrono::ParseError) -> Self {
        PokemonError::DateParseError(error)
    }
}
