use crate::models::{PokemonError, Set, SetResponse};

pub struct PokemonAPI {}

impl PokemonAPI {
    pub async fn sets() -> Result<Vec<Set>, PokemonError> {
        let request_url = url("/sets");
        let response = reqwest::get(&request_url).await?;
        let sets_res: SetResponse = serde_json::from_str(&response.text().await?)?;
        Ok(sets_res.sets)
    }
}

fn url(path: &str) -> String {
    format!("https://api.pokemontcg.io/v1{}", path)
}
