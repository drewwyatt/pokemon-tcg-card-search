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
                true
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
        html! {
            <>
                <main>
                    <fieldset>
                        <legend>{ "Search bar" }</legend>
                        <SearchBar />
                    </fieldset>
                    <section>
                        <fieldset>
                            <legend>{ "Sets" }</legend>
                            {self.view_sets()}
                        </fieldset>
                        <footer>
                            <button onclick=self.link.callback(|_| Msg::FetchSets)>{ "Fetch Sets" }</button>
                        </footer>
                    </section>
                </main>
            </>
        }
    }
}

impl App {
    fn view_sets(&self) -> Html {
        match &self.sets {
            FetchState::Success(sets) => {
                html! { <ul>{for sets.iter().map(|s| self.view_set(s))}</ul> }
            }
            FetchState::Fetching => html! { <h2>{ "Fetching..." }</h2> },
            FetchState::Failed(e) => {
                html! { <><h2>{ "Error!" }</h2><pre>{ format!("{:?}", e) }</pre></> }
            }
            FetchState::Idle => html! { <h2>{ "👇 Click the button to fetch 👇" }</h2> },
        }
    }

    fn view_set(&self, set: &Set) -> Html {
        match &set.ptcgo_code {
            Some(code) => html! {
                <li>
                    <strong>{code}</strong>
                    { '\u{00a0}' }
                    <span>{&set.name}</span>
                </li>
            },
            None => html! { <></> },
        }
    }
}
