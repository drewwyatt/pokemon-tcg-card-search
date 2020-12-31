#[path = "components/mod.rs"]
mod components;

use components::SearchBar;
use log::*;
use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
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
                    <fieldset>
                        <legend>{ "Search bar" }</legend>
                        <SearchBar />
                    </fieldset>
                </main>
                <footer>
                    <p>{ "This is the footer" }</p>
                </footer>
            </div>
        }
    }
}
