use crate::models::{Search, Searchable, Set};
use std::convert::TryFrom;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct SearchBar {
    link: ComponentLink<Self>,
    props: Props,
    text_input: NodeRef,
}

pub enum Msg {
    Submit,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub sets: Vec<Set>,
}

impl Component for SearchBar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let text_input = NodeRef::default();
        SearchBar {
            link,
            props,
            text_input,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                let text = Search::try_from(self.text_input.clone()).unwrap(); // TODO: get rid of unwrap when i'm not tryint to go to sleep
                let card_id = self.props.sets.identify(&text);
                ConsoleService::log(&format!("text: {:?} card_id: {:?}", text, card_id));
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <form onsubmit=self.link.callback(|e: web_sys::FocusEvent| {
                e.prevent_default();
                Msg::Submit
            })>
                <fieldset>
                    <legend>{ "Search Bar" }</legend>
                    <input placeholder="Enter a card number" ref=self.text_input.clone() />
                    <button type="submit">{ "Search" }</button>
                </fieldset>
            </form>
        }
    }
}
