use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;

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

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div>
                <main>
                    <p>{ "This is the main" }</p>
                    <p>{ "This is a second paragraph" }</p>
                </main>
                <footer>
                    <p>{ "This is the footer" }</p>
                </footer>
            </div>
        }
    }
}
