use log::*;
use yew::prelude::*;
use yew::services::ConsoleService;
use yewtil::future::LinkFuture;

use crate::api::PokemonAPI;
use crate::components::SearchBar;
use crate::models::{FetchState, PokemonError, Set};

pub struct App {
    link: ComponentLink<Self>,
    sets: FetchState<Vec<Set>>,
}

pub enum Msg {
    FetchSets,
    LoadSets(Vec<Set>),
    LoadError(PokemonError),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let sets = FetchState::Idle;
        App { link, sets }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchSets => {
                self.link.send_future(async {
                    match PokemonAPI::sets().await {
                        Ok(sets) => Msg::LoadSets(sets),
                        Err(e) => Msg::LoadError(e),
                    }
                });
                self.sets = FetchState::Fetching;
                false
            }
            Msg::LoadSets(sets) => {
                self.sets = FetchState::Success(sets);
                true
            }
            Msg::LoadError(e) => {
                self.sets = FetchState::Failed(e);
                true
            }
        }
    }

    fn view(&self) -> Html {
        info!("rendered!");
        let loggable_sets = format!("{:?}", self.sets);
        ConsoleService::log(&loggable_sets);
        let sets_msg = match self.sets {
            FetchState::Success(_) => "Success",
            FetchState::Failed(_) => "Failed",
            FetchState::Fetching => "Fetching",
            FetchState::Idle => "Idle",
        };
        html! {
            <>
                <main>
                    <p>{ "This is the main" }</p>
                    <fieldset>
                        <legend>{ "Search bar" }</legend>
                        <SearchBar />
                    </fieldset>
                    <fieldset>
                        <legend>{ "Sets" }</legend>
                        <h1>{sets_msg}</h1>
                    </fieldset>
                </main>
                <footer>
                    <button onclick=self.link.callback(|_| Msg::FetchSets)>{ "Fetch Sets" }</button>
                </footer>
            </>
        }
    }
}
