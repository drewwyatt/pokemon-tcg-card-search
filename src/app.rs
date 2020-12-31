use log::*;
use serde_derive::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::{Area, StorageService};

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {};
        App { link, state }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div>
                <main>
                    <p>{ "This is the main" }</p>
                </main>
                <footer>
                    <p>{ "This is the footer" }</p>
                </footer>
            </div>
        }
    }
}
