use crate::models::Set;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct SearchBar {
    link: ComponentLink<Self>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let text_input = NodeRef::default();
        SearchBar { link, text_input }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit => {
                let text = self.text_input.cast::<HtmlInputElement>().unwrap().value();
                ConsoleService::log(&format!("text: {:?}", text));
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
