use crate::models::PokemonError;

#[derive(Debug)]
pub enum FetchState<T> {
    Idle,
    Fetching,
    Success(T),
    Failed(PokemonError),
}
